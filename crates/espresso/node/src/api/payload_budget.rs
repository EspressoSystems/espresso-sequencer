//! Admission control for handlers that load block payloads into memory.
//!
//! `large_object_range_limit` bounds how many blocks a request may cover, not how many bytes. A
//! [`PayloadBudget`] bounds the payload bytes loaded at once across all requests, and the bytes a
//! single range request may cover.

use std::{ops::Range, sync::Arc, time::Duration};

use espresso_api::error::AvailabilityError;
use espresso_types::SeqTypes;
use futures::StreamExt as _;
use hotshot_query_service::availability::{AvailabilityDataSource, BlockId};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

use super::state::FETCH_TIMEOUT;

/// Permits are KiB, so the budget fits the `u32` of `acquire_many`.
const UNIT: u64 = 1024;

/// Payload bytes a node loads at once, unless configured.
pub const DEFAULT_BUDGET_BYTES: u64 = 512 * 1024 * 1024;

/// Payload bytes one range request may cover. 100 blocks of the 1 MiB mainnet maximum fit.
pub const DEFAULT_RANGE_LIMIT_BYTES: u64 = 128 * 1024 * 1024;

/// How long a request waits for budget before it is turned away.
const DEFAULT_WAIT: Duration = Duration::from_secs(5);

#[derive(Clone, Debug)]
pub struct PayloadBudget {
    semaphore: Arc<Semaphore>,
    capacity: u32,
    wait: Duration,
    range_limit: u64,
}

/// Holds budget until dropped.
#[derive(Debug)]
pub struct PayloadPermit(#[allow(dead_code)] OwnedSemaphorePermit);

impl Default for PayloadBudget {
    fn default() -> Self {
        Self::new(DEFAULT_BUDGET_BYTES)
    }
}

impl PayloadBudget {
    pub fn new(bytes: u64) -> Self {
        let capacity = units(bytes);
        Self {
            semaphore: Arc::new(Semaphore::new(capacity as usize)),
            capacity,
            wait: DEFAULT_WAIT,
            range_limit: DEFAULT_RANGE_LIMIT_BYTES,
        }
    }

    pub fn with_wait(mut self, wait: Duration) -> Self {
        self.wait = wait;
        self
    }

    pub fn with_range_limit(mut self, bytes: u64) -> Self {
        self.range_limit = bytes;
        self
    }

    /// Reject a range request covering more payload bytes than the per-request limit.
    pub fn check_range(&self, bytes: u64) -> anyhow::Result<()> {
        if bytes > self.range_limit {
            return Err(AvailabilityError::RangeExceeded(format!(
                "range covers {bytes} payload bytes, exceeding limit {}",
                self.range_limit
            ))
            .into());
        }
        Ok(())
    }

    /// Wait for `bytes` of budget. A request larger than the whole budget takes all of it, so
    /// every block can be served when nothing else is in flight.
    pub async fn acquire(&self, bytes: u64) -> anyhow::Result<PayloadPermit> {
        let wanted = units(bytes).min(self.capacity);
        let acquire = self.semaphore.clone().acquire_many_owned(wanted);
        match tokio::time::timeout(self.wait, acquire).await {
            Ok(permit) => Ok(PayloadPermit(permit.expect("semaphore is never closed"))),
            Err(_) => Err(AvailabilityError::Overloaded(format!(
                "payload memory budget exhausted, {bytes} bytes not available within {:?}",
                self.wait
            ))
            .into()),
        }
    }

    pub async fn admit_range<S>(&self, ds: &S, range: Range<usize>) -> anyhow::Result<PayloadPermit>
    where
        S: AvailabilityDataSource<SeqTypes> + ?Sized,
    {
        self.admit_ranges(ds, [range]).await
    }

    /// Admit a range request: check its byte limit, then wait for budget. Sizes come from the
    /// headers, so nothing is loaded until the request is admitted.
    pub async fn admit_ranges<S>(
        &self,
        ds: &S,
        ranges: impl IntoIterator<Item = Range<usize>>,
    ) -> anyhow::Result<PayloadPermit>
    where
        S: AvailabilityDataSource<SeqTypes> + ?Sized,
    {
        let mut bytes = 0;
        for range in ranges {
            let mut headers = ds.get_header_range(range.clone()).await.enumerate();
            while let Some((i, fetch)) = headers.next().await {
                let header = fetch.with_timeout(FETCH_TIMEOUT).await.ok_or_else(|| {
                    AvailabilityError::NotFound(format!("header {} not found", range.start + i))
                })?;
                bytes += header.ns_table().payload_byte_len() as u64;
            }
        }
        self.check_range(bytes)?;
        self.acquire(bytes).await
    }

    /// Admit a single block. It is not subject to the range limit.
    pub async fn admit_block<S>(
        &self,
        ds: &S,
        id: BlockId<SeqTypes>,
    ) -> anyhow::Result<PayloadPermit>
    where
        S: AvailabilityDataSource<SeqTypes> + ?Sized,
    {
        let header = ds
            .get_header(id)
            .await
            .with_timeout(FETCH_TIMEOUT)
            .await
            .ok_or_else(|| AvailabilityError::NotFound(format!("header not found for {id}")))?;
        self.acquire(header.ns_table().payload_byte_len() as u64)
            .await
    }
}

fn units(bytes: u64) -> u32 {
    u32::try_from(bytes.div_ceil(UNIT).max(1)).unwrap_or(u32::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    const KIB: u64 = 1024;

    fn budget(kib: u64) -> PayloadBudget {
        PayloadBudget::new(kib * KIB).with_wait(Duration::from_millis(50))
    }

    fn is_overloaded(err: &anyhow::Error) -> bool {
        matches!(
            err.downcast_ref::<AvailabilityError>(),
            Some(AvailabilityError::Overloaded(_))
        )
    }

    #[tokio::test]
    async fn rejects_when_budget_is_held() {
        let budget = budget(100);
        let _held = budget.acquire(60 * KIB).await.unwrap();

        let err = budget.acquire(60 * KIB).await.unwrap_err();

        assert!(is_overloaded(&err), "{err}");
        budget.acquire(40 * KIB).await.unwrap();
    }

    #[tokio::test]
    async fn waiter_is_admitted_when_budget_is_released() {
        let budget = PayloadBudget::new(100 * KIB).with_wait(Duration::from_secs(5));
        let held = budget.acquire(100 * KIB).await.unwrap();

        let waiter = tokio::spawn({
            let budget = budget.clone();
            async move { budget.acquire(100 * KIB).await }
        });
        tokio::task::yield_now().await;
        drop(held);

        waiter.await.unwrap().unwrap();
    }

    #[tokio::test]
    async fn block_larger_than_budget_takes_all_of_it() {
        let budget = budget(10);

        let permit = budget.acquire(1000 * KIB).await.unwrap();

        assert!(is_overloaded(&budget.acquire(1).await.unwrap_err()));
        drop(permit);
        budget.acquire(1000 * KIB).await.unwrap();
    }

    #[tokio::test]
    async fn empty_payload_still_takes_a_permit() {
        let budget = budget(1);
        let _held = budget.acquire(0).await.unwrap();

        assert!(is_overloaded(&budget.acquire(0).await.unwrap_err()));
    }

    #[test]
    fn range_over_limit_is_range_exceeded() {
        let budget = budget(1000).with_range_limit(100 * KIB);

        budget.check_range(100 * KIB).unwrap();
        let err = budget.check_range(100 * KIB + 1).unwrap_err();

        assert!(matches!(
            err.downcast_ref::<AvailabilityError>(),
            Some(AvailabilityError::RangeExceeded(_))
        ));
    }
}
