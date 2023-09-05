use std::time::Duration;

use hotshot::traits::NetworkReliability;
use rand::{
    distributions::{Bernoulli, Uniform},
    prelude::Distribution,
};

/// A synchronous network. Packets may be delayed, but are guaranteed
/// to arrive within `timeout` ns
#[derive(Clone, Copy, Debug, Default)]
pub struct SynchronousNetwork {
    /// Max delay of packet before arrival
    timeout_ms: u64,
    /// Lowest value in milliseconds that a packet may be delayed
    delay_low_ms: u64,
}

impl NetworkReliability for SynchronousNetwork {
    /// never drop a packet
    fn sample_keep(&self) -> bool {
        true
    }
    fn sample_delay(&self) -> Duration {
        Duration::from_millis(
            Uniform::new_inclusive(self.delay_low_ms, self.timeout_ms)
                .sample(&mut rand::thread_rng()),
        )
    }
}

/// An asynchronous network. Packets may be dropped entirely
/// or delayed for arbitrarily long periods
/// probability that packet is kept = `keep_numerator` / `keep_denominator`
/// packet delay is obtained by sampling from a uniform distribution
/// between `delay_low_ms` and `delay_high_ms`, inclusive
#[derive(Debug, Clone, Copy)]
pub struct AsynchronousNetwork {
    /// numerator for probability of keeping packets
    keep_numerator: u32,
    /// denominator for probability of keeping packets
    keep_denominator: u32,
    /// lowest value in milliseconds that a packet may be delayed
    delay_low_ms: u64,
    /// highest value in milliseconds that a packet may be delayed
    delay_high_ms: u64,
}

impl NetworkReliability for AsynchronousNetwork {
    fn sample_keep(&self) -> bool {
        Bernoulli::from_ratio(self.keep_numerator, self.keep_denominator)
            .unwrap()
            .sample(&mut rand::thread_rng())
    }
    fn sample_delay(&self) -> Duration {
        Duration::from_millis(
            Uniform::new_inclusive(self.delay_low_ms, self.delay_high_ms)
                .sample(&mut rand::thread_rng()),
        )
    }
}

/// An partially synchronous network. Behaves asynchronously
/// until some arbitrary time bound, GST,
/// then synchronously after GST
#[derive(Debug, Clone, Copy)]
pub struct PartiallySynchronousNetwork {
    /// asynchronous portion of network
    asynchronous: AsynchronousNetwork,
    /// synchronous portion of network
    synchronous: SynchronousNetwork,
    /// time when GST occurs
    gst: std::time::Duration,
    /// when the network was started
    start: std::time::Instant,
}

impl NetworkReliability for PartiallySynchronousNetwork {
    /// never drop a packet
    fn sample_keep(&self) -> bool {
        true
    }
    fn sample_delay(&self) -> Duration {
        // act asyncronous before gst
        if self.start.elapsed() < self.gst {
            if self.asynchronous.sample_keep() {
                self.asynchronous.sample_delay()
            } else {
                // assume packet was "dropped" and will arrive after gst
                self.synchronous.sample_delay() + self.gst
            }
        } else {
            // act syncronous after gst
            self.synchronous.sample_delay()
        }
    }
}

impl Default for AsynchronousNetwork {
    // disable all chance of failure
    fn default() -> Self {
        AsynchronousNetwork {
            keep_numerator: 1,
            keep_denominator: 1,
            delay_low_ms: 0,
            delay_high_ms: 0,
        }
    }
}

impl Default for PartiallySynchronousNetwork {
    fn default() -> Self {
        PartiallySynchronousNetwork {
            synchronous: SynchronousNetwork::default(),
            asynchronous: AsynchronousNetwork::default(),
            gst: std::time::Duration::new(0, 0),
            start: std::time::Instant::now(),
        }
    }
}

impl SynchronousNetwork {
    /// create new `SynchronousNetwork`
    pub fn new(timeout: u64, delay_low_ms: u64) -> Self {
        SynchronousNetwork {
            timeout_ms: timeout,
            delay_low_ms,
        }
    }
}

impl AsynchronousNetwork {
    /// create new `AsynchronousNetwork`
    pub fn new(
        keep_numerator: u32,
        keep_denominator: u32,
        delay_low_ms: u64,
        delay_high_ms: u64,
    ) -> Self {
        AsynchronousNetwork {
            keep_numerator,
            keep_denominator,
            delay_low_ms,
            delay_high_ms,
        }
    }
}

impl PartiallySynchronousNetwork {
    /// create new `PartiallySynchronousNetwork`
    pub fn new(
        asynchronous: AsynchronousNetwork,
        synchronous: SynchronousNetwork,
        gst: std::time::Duration,
    ) -> Self {
        PartiallySynchronousNetwork {
            asynchronous,
            synchronous,
            gst,
            start: std::time::Instant::now(),
        }
    }
}
