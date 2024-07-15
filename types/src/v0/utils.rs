use std::{
    cmp::{min, Ordering},
    fmt::{self, Debug, Display, Formatter},
    num::ParseIntError,
    str::FromStr,
    time::Duration,
};

use anyhow::Context;
use async_std::task::sleep;
use clap::Parser;
use derive_more::{Display, From, Into};
use futures::future::BoxFuture;
use rand::Rng;
use sequencer_utils::{impl_serde_from_string_or_integer, ser::FromStringOrInteger};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use time::{format_description::well_known::Rfc3339 as TimestampFormat, OffsetDateTime};

/// Information about the genesis state which feeds into the genesis block header.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct GenesisHeader {
    pub timestamp: Timestamp,
}

#[derive(Hash, Copy, Clone, Debug, Display, PartialEq, Eq, From, Into)]
#[display(fmt = "{}", "_0.format(&TimestampFormat).unwrap()")]
pub struct Timestamp(OffsetDateTime);

impl_serde_from_string_or_integer!(Timestamp);

impl Default for Timestamp {
    fn default() -> Self {
        Self::from_integer(0).unwrap()
    }
}

impl Timestamp {
    pub fn unix_timestamp(&self) -> u64 {
        self.0.unix_timestamp() as u64
    }

    pub fn max() -> anyhow::Result<Self> {
        Ok(Self(
            OffsetDateTime::from_unix_timestamp(i64::MAX).context("overflow")?,
        ))
    }
}

impl FromStringOrInteger for Timestamp {
    type Binary = u64;
    type Integer = u64;

    fn from_binary(b: Self::Binary) -> anyhow::Result<Self> {
        Self::from_integer(b)
    }

    fn from_integer(i: Self::Integer) -> anyhow::Result<Self> {
        let unix = i.try_into().context("timestamp out of range")?;
        Ok(Self(
            OffsetDateTime::from_unix_timestamp(unix).context("invalid timestamp")?,
        ))
    }

    fn from_string(s: String) -> anyhow::Result<Self> {
        Ok(Self(
            OffsetDateTime::parse(&s, &TimestampFormat).context("invalid timestamp")?,
        ))
    }

    fn to_binary(&self) -> anyhow::Result<Self::Binary> {
        Ok(self.unix_timestamp())
    }

    fn to_string(&self) -> anyhow::Result<String> {
        Ok(format!("{self}"))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ratio {
    pub numerator: u64,
    pub denominator: u64,
}

impl From<Ratio> for (u64, u64) {
    fn from(r: Ratio) -> Self {
        (r.numerator, r.denominator)
    }
}

impl Display for Ratio {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.numerator, self.denominator)
    }
}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ratio {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.numerator * other.denominator).cmp(&(other.numerator * self.denominator))
    }
}

#[derive(Debug, Snafu)]
pub enum ParseRatioError {
    #[snafu(display("numerator and denominator must be separated by :"))]
    MissingDelimiter,
    InvalidNumerator {
        err: ParseIntError,
    },
    InvalidDenominator {
        err: ParseIntError,
    },
}

impl FromStr for Ratio {
    type Err = ParseRatioError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (num, den) = s.split_once(':').ok_or(ParseRatioError::MissingDelimiter)?;
        Ok(Self {
            numerator: num
                .parse()
                .map_err(|err| ParseRatioError::InvalidNumerator { err })?,
            denominator: den
                .parse()
                .map_err(|err| ParseRatioError::InvalidDenominator { err })?,
        })
    }
}

#[derive(Clone, Debug, Snafu)]
pub struct ParseDurationError {
    reason: String,
}

pub fn parse_duration(s: &str) -> Result<Duration, ParseDurationError> {
    cld::ClDuration::from_str(s)
        .map(Duration::from)
        .map_err(|err| ParseDurationError {
            reason: err.to_string(),
        })
}

pub const MIN_RETRY_DELAY: Duration = Duration::from_millis(500);
pub const MAX_RETRY_DELAY: Duration = Duration::from_secs(5);
pub const BACKOFF_FACTOR: u32 = 2;
// Exponential backoff jitter as a fraction of the backoff delay, (numerator, denominator).
pub const BACKOFF_JITTER: (u64, u64) = (1, 10);

#[derive(Clone, Copy, Debug, Parser, PartialEq, Eq, PartialOrd, Ord)]
pub struct BackoffParams {
    /// Exponential backoff exponent.
    #[clap(
        long = "catchup-backoff-factor",
        env = "ESPRESSO_SEQUENCER_CATCHUP_BACKOFF_FACTOR",
        default_value = "4"
    )]
    factor: u32,

    /// Exponential backoff base delay.
    #[clap(
        long = "catchup-base-retry-delay",
        env = "ESPRESSO_SEQUENCER_CATCHUP_BASE_RETRY_DELAY",
        default_value = "20ms",
        value_parser = parse_duration
    )]
    base: Duration,

    /// Exponential max delay.
    #[clap(
        long = "catchup-max-retry-delay",
        env = "ESPRESSO_SEQUENCER_CATCHUP_MAX_RETRY_DELAY",
        default_value = "5s",
        value_parser = parse_duration
    )]
    max: Duration,

    /// Exponential backoff jitter as a ratio of the backoff delay, numerator:denominator.
    #[clap(
        long = "catchup-backoff-jitter",
        env = "ESPRESSO_SEQUENCER_CATCHUP_BACKOFF_JITTER",
        default_value = "1:10"
    )]
    jitter: Ratio,
}

impl Default for BackoffParams {
    fn default() -> Self {
        Self::parse_from(std::iter::empty::<String>())
    }
}

impl BackoffParams {
    pub async fn retry<S, T>(
        &self,
        mut state: S,
        f: impl for<'a> Fn(&'a mut S) -> BoxFuture<'a, anyhow::Result<T>>,
    ) -> T {
        let mut delay = self.base;
        loop {
            match f(&mut state).await {
                Ok(res) => break res,
                Err(err) => {
                    tracing::warn!(
                        "Retryable operation failed, will retry after {delay:?}: {err:#}"
                    );
                    sleep(delay).await;
                    delay = self.backoff(delay);
                }
            }
        }
    }

    #[must_use]
    pub fn backoff(&self, delay: Duration) -> Duration {
        if delay >= self.max {
            return self.max;
        }

        let mut rng = rand::thread_rng();

        // Increase the backoff by the backoff factor.
        let ms = (delay * self.factor).as_millis() as u64;

        // Sample a random jitter factor in the range [0, self.jitter].
        let jitter_num = rng.gen_range(0..self.jitter.numerator);
        let jitter_den = self.jitter.denominator;

        // Increase the delay by the jitter factor.
        let jitter = ms * jitter_num / jitter_den;
        let delay = Duration::from_millis(ms + jitter);

        // Bound the delay by the maximum.
        min(delay, self.max)
    }
}
