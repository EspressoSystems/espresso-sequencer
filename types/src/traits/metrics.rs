//! The [`Metrics`] trait is used to collect information from multiple components in the entire system.
//!
//! This trait can be used to spawn the following traits:
//! - [`Counter`]: an ever-increasing value (example usage: total bytes send/received)
//! - [`Gauge`]: a value that store the latest value, and can go up and down (example usage: amount of users logged in)
//! - [`Histogram`]: stores multiple float values based for a graph (example usage: CPU %)
//! - [`Label`]: Stores the last string (example usage: current version, network online/offline)

use std::fmt::Debug;

use dyn_clone::DynClone;

/// The metrics type.
pub trait Metrics: Send + Sync + DynClone + Debug {
    /// Create a [`Counter`] with an optional `unit_label`.
    ///
    /// The `unit_label` can be used to indicate what the unit of the value is, e.g. "kb" or "seconds"
    fn create_counter(&self, label: String, unit_label: Option<String>) -> Box<dyn Counter>;
    /// Create a [`Gauge`] with an optional `unit_label`.
    ///
    /// The `unit_label` can be used to indicate what the unit of the value is, e.g. "kb" or "seconds"
    fn create_gauge(&self, label: String, unit_label: Option<String>) -> Box<dyn Gauge>;
    /// Create a [`Histogram`] with an optional `unit_label`.
    ///
    /// The `unit_label` can be used to indicate what the unit of the value is, e.g. "kb" or "seconds"
    fn create_histogram(&self, label: String, unit_label: Option<String>) -> Box<dyn Histogram>;
    /// Create a [`Label`].
    fn create_label(&self, label: String) -> Box<dyn Label>;

    /// Create a subgroup with a specified prefix.
    fn subgroup(&self, subgroup_name: String) -> Box<dyn Metrics>;
}

/// Use this if you're not planning to use any metrics. All methods are implemented as a no-op
#[derive(Clone, Copy, Debug, Default)]
pub struct NoMetrics;

impl NoMetrics {
    /// Create a new `Box<dyn Metrics>` with this [`NoMetrics`]
    #[must_use]
    pub fn boxed() -> Box<dyn Metrics> {
        Box::<Self>::default()
    }
}

impl Metrics for NoMetrics {
    fn create_counter(&self, _: String, _: Option<String>) -> Box<dyn Counter> {
        Box::new(NoMetrics)
    }

    fn create_gauge(&self, _: String, _: Option<String>) -> Box<dyn Gauge> {
        Box::new(NoMetrics)
    }

    fn create_histogram(&self, _: String, _: Option<String>) -> Box<dyn Histogram> {
        Box::new(NoMetrics)
    }

    fn create_label(&self, _: String) -> Box<dyn Label> {
        Box::new(NoMetrics)
    }

    fn subgroup(&self, _: String) -> Box<dyn Metrics> {
        Box::new(NoMetrics)
    }
}

impl Counter for NoMetrics {
    fn add(&self, _: usize) {}
}
impl Gauge for NoMetrics {
    fn set(&self, _: usize) {}
    fn update(&self, _: i64) {}
}
impl Histogram for NoMetrics {
    fn add_point(&self, _: f64) {}
}
impl Label for NoMetrics {
    fn set(&self, _: String) {}
}

/// An ever-incrementing counter
pub trait Counter: Send + Sync + Debug + DynClone {
    /// Add a value to the counter
    fn add(&self, amount: usize);
}
/// A gauge that stores the latest value.
pub trait Gauge: Send + Sync + Debug + DynClone {
    /// Set the gauge value
    fn set(&self, amount: usize);

    /// Update the guage value
    fn update(&self, delts: i64);
}

/// A histogram which will record a series of points.
pub trait Histogram: Send + Sync + Debug + DynClone {
    /// Add a point to this histogram.
    fn add_point(&self, point: f64);
}

/// A label that stores the last string value.
pub trait Label: Send + Sync + DynClone {
    /// Set the label value
    fn set(&self, value: String);
}
dyn_clone::clone_trait_object!(Metrics);
dyn_clone::clone_trait_object!(Gauge);
dyn_clone::clone_trait_object!(Counter);
dyn_clone::clone_trait_object!(Histogram);
dyn_clone::clone_trait_object!(Label);

#[cfg(test)]
mod test {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    use super::*;

    #[derive(Debug, Clone)]
    struct TestMetrics {
        prefix: String,
        values: Arc<Mutex<Inner>>,
    }

    impl TestMetrics {
        fn sub(&self, name: String) -> Self {
            let prefix = if self.prefix.is_empty() {
                name
            } else {
                format!("{}-{name}", self.prefix)
            };
            Self {
                prefix,
                values: Arc::clone(&self.values),
            }
        }
    }

    impl Metrics for TestMetrics {
        fn create_counter(
            &self,
            label: String,
            _unit_label: Option<String>,
        ) -> Box<dyn super::Counter> {
            Box::new(self.sub(label))
        }

        fn create_gauge(
            &self,
            label: String,
            _unit_label: Option<String>,
        ) -> Box<dyn super::Gauge> {
            Box::new(self.sub(label))
        }

        fn create_histogram(
            &self,
            label: String,
            _unit_label: Option<String>,
        ) -> Box<dyn super::Histogram> {
            Box::new(self.sub(label))
        }

        fn create_label(&self, label: String) -> Box<dyn super::Label> {
            Box::new(self.sub(label))
        }

        fn subgroup(&self, subgroup_name: String) -> Box<dyn Metrics> {
            Box::new(self.sub(subgroup_name))
        }
    }

    impl Counter for TestMetrics {
        fn add(&self, amount: usize) {
            *self
                .values
                .lock()
                .unwrap()
                .counters
                .entry(self.prefix.clone())
                .or_default() += amount;
        }
    }

    impl Gauge for TestMetrics {
        fn set(&self, amount: usize) {
            *self
                .values
                .lock()
                .unwrap()
                .gauges
                .entry(self.prefix.clone())
                .or_default() = amount;
        }
        fn update(&self, delta: i64) {
            let mut values = self.values.lock().unwrap();
            let value = values.gauges.entry(self.prefix.clone()).or_default();
            let signed_value = i64::try_from(*value).unwrap_or(i64::MAX);
            *value = usize::try_from(signed_value + delta).unwrap_or(0);
        }
    }

    impl Histogram for TestMetrics {
        fn add_point(&self, point: f64) {
            self.values
                .lock()
                .unwrap()
                .histograms
                .entry(self.prefix.clone())
                .or_default()
                .push(point);
        }
    }

    impl Label for TestMetrics {
        fn set(&self, value: String) {
            *self
                .values
                .lock()
                .unwrap()
                .labels
                .entry(self.prefix.clone())
                .or_default() = value;
        }
    }

    #[derive(Default, Debug)]
    struct Inner {
        counters: HashMap<String, usize>,
        gauges: HashMap<String, usize>,
        histograms: HashMap<String, Vec<f64>>,
        labels: HashMap<String, String>,
    }

    #[test]
    fn test() {
        let values = Arc::default();
        // This is all scoped so all the arcs should go out of scope
        {
            let metrics: Box<dyn Metrics> = Box::new(TestMetrics {
                prefix: String::new(),
                values: Arc::clone(&values),
            });

            let gauge = metrics.create_gauge("foo".to_string(), None);
            let counter = metrics.create_counter("bar".to_string(), None);
            let histogram = metrics.create_histogram("baz".to_string(), None);

            gauge.set(5);
            gauge.update(-2);

            for i in 0..5 {
                counter.add(i);
            }

            for i in 0..10 {
                histogram.add_point(f64::from(i));
            }

            let sub = metrics.subgroup("child".to_string());

            let sub_gauge = sub.create_gauge("foo".to_string(), None);
            let sub_counter = sub.create_counter("bar".to_string(), None);
            let sub_histogram = sub.create_histogram("baz".to_string(), None);

            sub_gauge.set(10);

            for i in 0..5 {
                sub_counter.add(i * 2);
            }

            for i in 0..10 {
                sub_histogram.add_point(f64::from(i) * 2.0);
            }
        }

        // The above variables are scoped so they should be dropped at this point
        // One of the rare times we can use `Arc::try_unwrap`!
        let values = Arc::try_unwrap(values).unwrap().into_inner().unwrap();
        assert_eq!(values.gauges["foo"], 3);
        assert_eq!(values.counters["bar"], 10); // 0..5
        assert_eq!(
            values.histograms["baz"],
            vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
        );

        assert_eq!(values.gauges["child-foo"], 10);
        assert_eq!(values.counters["child-bar"], 20); // 0..5 *2
        assert_eq!(
            values.histograms["child-baz"],
            vec![0.0, 2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0]
        );
    }
}
