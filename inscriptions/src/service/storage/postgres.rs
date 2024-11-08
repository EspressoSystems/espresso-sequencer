use futures::StreamExt;
use std::num::NonZero;
use std::time::SystemTime;

use alloy::sol_types::SolStruct;
use sqlx::Row;

use crate::service::espresso_inscription::{
    ChainDetails, EspressoInscription, InscriptionAndChainDetails,
};
use crate::service::ESPRESSO_INSCRIPTION_MESSAGE;

use super::{
    InscriptionPersistence, RecordConfirmedInscriptionAndChainDetailsError,
    RecordLastReceivedBlockError, RecordPendingPutInscriptionError,
    ResolvePendingPutInscriptionError, RetrieveLastReceivedBlockError,
    RetrieveLatestInscriptionAndChainDetailsError, RetrievePendingPutInscriptionsError,
};

pub struct PostgresPersistence {
    pool: sqlx::PgPool,
}

impl PostgresPersistence {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl InscriptionPersistence for PostgresPersistence {
    async fn record_pending_put_inscription(
        &self,
        inscription: &EspressoInscription,
    ) -> Result<(), RecordPendingPutInscriptionError> {
        tracing::debug!("Recording pending put inscription: {:?}", inscription);
        let mut conn = self.pool.begin().await?;
        let result = sqlx::query("INSERT INTO pending_put_inscription_request (ins_hash, ins_address, ins_time) VALUES ($1, $2, $3)")
            .bind(&inscription.eip712_hash_struct().0[..])
            .bind(inscription.address.to_string())
            // We shouldn't have a timestamp that gets close to i64::MAX, so
            // this should be a relatively safe cast.
            .bind(inscription.time as i64)
            .execute(&mut *conn)
            .await?;

        if result.rows_affected() != 1 {
            tracing::warn!("Failed to store pending put inscription: {:?}", inscription);
            panic!("Failed to store pending put inscription: {:?}", inscription);
        }

        // commit the transaction
        conn.commit().await?;

        Ok(())
    }

    async fn record_submit_put_inscription(
        &self,
        inscription: &EspressoInscription,
    ) -> Result<(), ResolvePendingPutInscriptionError> {
        tracing::debug!("Recording submit put inscription: {:?}", inscription);
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut conn = self.pool.begin().await?;

        let result = sqlx::query("INSERT INTO pending_put_inscriptions_event (ins_id, event_type, event_time) VALUES((SELECT id FROM pending_put_inscription_request WHERE ins_hash = $1), 'submit', $2)")
            .bind(&inscription.eip712_hash_struct().0[..])
            .bind(now as i64)
            .execute(&mut *conn)
            .await?;

        if result.rows_affected() != 1 {
            // This means that we could not store the event for a pending
            // put_inscription.
            // This might imply that the pending put inscription hasn't been
            // stored yet.
            tracing::warn!(
                "Failed to store event for 'submit' pending put inscription: {:?}",
                inscription
            );
        }

        // commit the transaction
        conn.commit().await?;

        Ok(())
    }

    async fn retrieve_pending_put_inscriptions(
        &self,
    ) -> Result<Vec<EspressoInscription>, RetrievePendingPutInscriptionsError> {
        tracing::debug!("Retrieving pending put inscriptions");
        // We shouldn't need a transaction, as we're just performing a read
        let mut conn = self.pool.acquire().await?;

        let mut rows = sqlx::query("SELECT ins_address, ins_time FROM pending_put_inscription_request r LEFT JOIN pending_put_inscriptions_event e ON e.ins_id = r.id AND e.event_type = 'confirmed' WHERE e.id IS NULL")
                    .fetch(&mut *conn);

        let mut put_inscriptions = Vec::new();

        while let Some(row) = rows.next().await {
            let row = row?;
            let ins_address_string: String = row.try_get("ins_address")?;
            let ins_time: i64 = row.try_get("ins_time")?;
            let inscription = EspressoInscription {
                address: ins_address_string.parse()?,
                message: ESPRESSO_INSCRIPTION_MESSAGE.to_string(),
                time: ins_time as u64,
            };

            put_inscriptions.push(inscription);
        }

        Ok(put_inscriptions)
    }

    async fn record_confirmed_inscription_and_chain_details(
        &self,
        inscription_and_block_details: &InscriptionAndChainDetails,
    ) -> Result<(), RecordConfirmedInscriptionAndChainDetailsError> {
        tracing::debug!(
            "Recording confirmed inscription and chain details: {:?}",
            inscription_and_block_details
        );
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut conn = self.pool.begin().await?;

        let store_confirmed_inscription_result = sqlx::query("INSERT INTO confirmed_inscriptions (ins_hash, ins_address, ins_time, chain_block_height, chain_txn_offset) VALUES ($1, $2, $3, $4, $5)")
            .bind(&inscription_and_block_details.inscription.eip712_hash_struct().0[..])
            .bind(inscription_and_block_details.inscription.address.to_string())
            .bind(inscription_and_block_details.inscription.time as i64)
            .bind(inscription_and_block_details.chain_details.block as i64)
            .bind(inscription_and_block_details.chain_details.offset as i64)
            .execute(&mut *conn)
            .await?;

        if store_confirmed_inscription_result.rows_affected() != 1 {
            // This could mean that we're trying to store a confirmed
            // inscription that we've already stored.
            tracing::warn!(
                "Failed to store confirmed inscription: {:?}",
                inscription_and_block_details
            );
        }

        // If this fails... is that a problem?  It depends on the nature of the
        // problem
        let store_event_result = sqlx::query("INSERT INTO pending_put_inscriptions_event (ins_id, event_type, event_time) VALUES((SELECT id FROM pending_put_inscription_request WHERE ins_hash = $1), 'confirmed', $2)")
            .bind(&inscription_and_block_details.inscription.eip712_hash_struct().0[..])
            .bind(now as i64)
            .execute(&mut *conn)
            .await?;

        if store_event_result.rows_affected() != 1 {
            // It's not the end of the world if this happens, it just implies
            // that there is no corresponding pending put inscription entry.
            // so we'll just log this and move on.
            tracing::warn!(
                "Failed to store event for confirmed inscription: {:?}",
                inscription_and_block_details
            );
        }

        // commit the result
        conn.commit().await?;

        Ok(())
    }

    async fn retrieve_latest_inscription_and_chain_details(
        &self,
        number_of_inscriptions: NonZero<usize>,
    ) -> Result<Vec<InscriptionAndChainDetails>, RetrieveLatestInscriptionAndChainDetailsError>
    {
        tracing::debug!("Retrieving latest inscriptions and chain details");
        // We shouldn't need a transaction, as we're just performing a read
        let mut conn = self.pool.acquire().await?;

        let mut rows = sqlx::query("SELECT ins_address, ins_time, chain_block_height, chain_txn_offset FROM confirmed_inscriptions ORDER BY id DESC LIMIT $1")
            .bind(number_of_inscriptions.get() as i64)
            .fetch(&mut *conn);

        let mut inscription_and_chain_details = Vec::new();

        while let Some(row) = rows.next().await {
            let row = row?;
            let ins_address_string: String = row.try_get("ins_address")?;
            let ins_time: i64 = row.try_get("ins_time")?;
            let inscription = EspressoInscription {
                address: ins_address_string.parse()?,
                message: ESPRESSO_INSCRIPTION_MESSAGE.to_string(),
                time: ins_time as u64,
            };
            let block_height: i64 = row.try_get("chain_block_height")?;
            let txn_offset: i64 = row.try_get("chain_txn_offset")?;
            let chain_details = ChainDetails {
                block: block_height as u64,
                offset: txn_offset as u64,
            };

            inscription_and_chain_details.push(InscriptionAndChainDetails {
                inscription,
                chain_details,
            });
        }

        Ok(inscription_and_chain_details)
    }

    async fn record_last_received_block(
        &self,
        block: u64,
    ) -> Result<(), RecordLastReceivedBlockError> {
        tracing::debug!("Recording last received block: {}", block);
        let mut conn = self.pool.begin().await?;

        let result = sqlx::query(
            // Update the last read block if the new block number is greater than
            // the current block number.
            "UPDATE last_read_block SET block_number = $1 WHERE id = 0 AND $1 > block_number",
        )
        .bind(block as i64)
        .execute(&mut *conn)
        .await?;

        if result.rows_affected() != 1 {
            // We didn't actually store any update to the last read block.
            // This implies that we received a block before or equal to our
            // already stored block height.
            panic!(
                "attempt to record last block {}: it is not greater than the previous last read block",
                block
            );
        }

        // commit the result.
        conn.commit().await?;

        Ok(())
    }

    async fn retrieve_last_received_block(&self) -> Result<u64, RetrieveLastReceivedBlockError> {
        tracing::debug!("Retrieving last received block");
        // We shouldn't need a transaction, as we're just performing a read
        let mut conn = self.pool.acquire().await?;

        let row = sqlx::query("SELECT block_number FROM last_read_block")
            .fetch_one(&mut *conn)
            .await?;

        let block_number: i64 = row.try_get("block_number")?;

        Ok(block_number as u64)
    }
}
