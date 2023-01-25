#![allow(dead_code)]

// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the HotShot Query Service library.
//
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU
// General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program. If not,
// see <https://www.gnu.org/licenses/>.

use hotshot_types::traits::metrics;
use itertools::Itertools;
use prometheus::{
    core::{AtomicU64, Collector, GenericCounter, GenericGauge},
    Encoder, Opts, TextEncoder,
};
use snafu::Snafu;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Snafu)]
pub enum MetricsError {
    NoSuchSubgroup {
        path: Vec<String>,
    },
    NoSuchMetric {
        namespace: Vec<String>,
        label: String,
    },
    Prometheus {
        source: prometheus::Error,
    },
}

impl From<prometheus::Error> for MetricsError {
    fn from(source: prometheus::Error) -> Self {
        Self::Prometheus { source }
    }
}

#[derive(Clone, Debug, Default)]
struct Registry {
    prometheus: prometheus::Registry,
    // There is nothing in the Prometheus standard corresponding to our notion of a [Label] (which
    // is like a gauge whose value is text). As a workaround we serialize labels using special
    // Prometheus comments, like `# LABEL <name> "<text>"`. `labels` collects _all_ labels (as a map
    // from fully-qualified name to label value) in this tree of metric groups.
    //
    // We store all the labels here, rather than storing the labels for each sub-group in the
    // appropriate [PrometheusMetrics], to match the behavior of Prometheus-native metrics, where
    // all metrics from any related subgroup are serialied when [PrometheusMetrics::prometheus] is
    // called on any subgroup.
    labels: Arc<RwLock<HashMap<Vec<String>, Label>>>,
}

impl Registry {
    fn register(&self, metric: Box<dyn Collector>) {
        self.prometheus.register(metric).unwrap();
    }

    fn register_label(&self, fq_name: Vec<String>, value: Label) {
        self.labels.write().unwrap().insert(fq_name, value);
    }

    fn get_label(&self, fq_name: &[String]) -> Option<Label> {
        self.labels.read().unwrap().get(fq_name).cloned()
    }

    fn export(&self) -> Result<String, MetricsError> {
        // First write all the labels as Prometheus comments.
        let mut labels = self
            .labels
            .read()
            .unwrap()
            .iter()
            .map(|(fq_name, label)| format!("# LABEL {} {}", fq_name.iter().join("_"), label.get()))
            .join("\n");
        labels.push('\n');

        // Now append the Prometheus metrics.
        let mut buffer = labels.into_bytes();
        let encoder = TextEncoder::new();
        let metric_families = self.prometheus.gather();
        encoder.encode(&metric_families, &mut buffer)?;
        String::from_utf8(buffer).map_err(|err| MetricsError::Prometheus {
            source: prometheus::Error::Msg(format!(
                "could not convert Prometheus output to UTF-8: {}",
                err
            )),
        })
    }
}

/// A Prometheus-based implementation of a [Metrics](metrics::Metrics) registry.
///
/// [PrometheusMetrics] provides a collection of metrics including [Counter], [Gauge], [Histogram],
/// and [Label]. These metrics can be created and associated with a [PrometheusMetrics] collection
/// and then used as handles for updating and populating. The [PrometheusMetrics] registry can then
/// be used to collect all of the associated metrics and export them in the Prometheus text format.
///
/// This implementation provides a few features beyond the basic [prometheus] features. It supports
/// hierarchical namespaces; any [PrometheusMetrics] can be used to derive a subgroup with a certain
/// name. The subgroup is then related to the parent, and any [PrometheusMetrics] in the tree of
/// related groups can be used to collect _all_ registered metrics. The namespacing will be
/// reflected in the fully qualified name of each metric in the Prometheus output. The subgroup
/// relationship is pure and deterministic -- calling [subgroup](PrometheusMetrics::subgroup) with
/// the same subgroup name will always return a handle to the same underlying [PrometheusMetrics]
/// object.
///
/// [PrometheusMetrics] also supports querying for individual metrics by name, unlike
/// [prometheus::Registry]. This provides a programming interface for inspecting the values of
/// specific metrics at run-time, if that is preferrable to exporting all metrics wholesale.
#[derive(Clone, Debug, Default)]
pub(crate) struct PrometheusMetrics {
    metrics: Registry,
    namespace: Vec<String>,
    children: Arc<RwLock<HashMap<String, PrometheusMetrics>>>,
    counters: Arc<RwLock<HashMap<String, Counter>>>,
    gauges: Arc<RwLock<HashMap<String, Gauge>>>,
    histograms: Arc<RwLock<HashMap<String, Histogram>>>,
}

impl PrometheusMetrics {
    /// Export all metrics in the Prometheus text format.
    pub fn prometheus(&self) -> Result<String, MetricsError> {
        self.metrics.export()
    }

    /// Get a counter in this sub-group by name.
    pub fn get_counter(&self, label: &str) -> Result<Counter, MetricsError> {
        self.get_metric(&self.counters, label)
    }

    /// Get a gauge in this sub-group by name.
    pub fn get_gauge(&self, label: &str) -> Result<Gauge, MetricsError> {
        self.get_metric(&self.gauges, label)
    }

    /// Get a histogram in this sub-group by name.
    pub fn get_histogram(&self, label: &str) -> Result<Histogram, MetricsError> {
        self.get_metric(&self.histograms, label)
    }

    /// Get a label in this sub-group by name.
    pub fn get_label(&self, name: &str) -> Result<Label, MetricsError> {
        let mut fq_name = self.namespace.clone();
        fq_name.push(name.to_string());
        self.metrics
            .get_label(&fq_name)
            .ok_or_else(|| MetricsError::NoSuchMetric {
                namespace: self.namespace.clone(),
                label: name.to_string(),
            })
    }

    /// Get a (possibly nested) subgroup of this group by its path.
    pub fn get_subgroup<I>(&self, path: I) -> Result<PrometheusMetrics, MetricsError>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let mut curr = self.clone();
        for seg in path.into_iter() {
            let next = curr
                .children
                .read()
                .unwrap()
                .get(seg.as_ref())
                .ok_or_else(|| MetricsError::NoSuchSubgroup {
                    path: {
                        let mut path = curr.namespace.clone();
                        path.push(seg.as_ref().to_string());
                        path
                    },
                })?
                .clone();
            curr = next;
        }
        Ok(curr)
    }

    fn get_metric<M: Clone>(
        &self,
        metrics: &Arc<RwLock<HashMap<String, M>>>,
        label: &str,
    ) -> Result<M, MetricsError> {
        metrics
            .read()
            .unwrap()
            .get(label)
            .cloned()
            .ok_or_else(|| MetricsError::NoSuchMetric {
                namespace: self.namespace.clone(),
                label: label.to_string(),
            })
    }

    fn metric_opts(&self, label: String, unit_label: Option<String>) -> Opts {
        let help = unit_label.unwrap_or_else(|| label.clone());
        let mut opts = Opts::new(label, help);
        let mut group_names = self.namespace.iter();
        if let Some(namespace) = group_names.next() {
            opts = opts
                .namespace(namespace.clone())
                .subsystem(group_names.join("_"));
        }
        opts
    }
}

impl metrics::Metrics for PrometheusMetrics {
    fn create_counter(
        &self,
        label: String,
        unit_label: Option<String>,
    ) -> Box<dyn metrics::Counter> {
        let counter = Counter::new(&self.metrics, self.metric_opts(label.clone(), unit_label));
        self.counters
            .write()
            .unwrap()
            .insert(label, counter.clone());
        Box::new(counter)
    }

    fn create_gauge(&self, label: String, unit_label: Option<String>) -> Box<dyn metrics::Gauge> {
        let gauge = Gauge::new(&self.metrics, self.metric_opts(label.clone(), unit_label));
        self.gauges.write().unwrap().insert(label, gauge.clone());
        Box::new(gauge)
    }

    fn create_histogram(
        &self,
        label: String,
        unit_label: Option<String>,
    ) -> Box<dyn metrics::Histogram> {
        let histogram = Histogram::new(&self.metrics, self.metric_opts(label.clone(), unit_label));
        self.histograms
            .write()
            .unwrap()
            .insert(label, histogram.clone());
        Box::new(histogram)
    }

    fn create_label(&self, name: String) -> Box<dyn metrics::Label> {
        Box::new(Label::new(&self.metrics, self.metric_opts(name, None)))
    }

    fn subgroup(&self, subgroup_name: String) -> Box<dyn metrics::Metrics> {
        Box::new(
            self.children
                .write()
                .unwrap()
                .entry(subgroup_name.clone())
                .or_insert_with(|| Self {
                    metrics: self.metrics.clone(),
                    namespace: {
                        let mut namespace = self.namespace.clone();
                        namespace.push(subgroup_name);
                        namespace
                    },
                    ..Default::default()
                })
                .clone(),
        )
    }
}

/// A [Counter](metrics::Counter) metric.
#[derive(Clone, Debug)]
pub struct Counter(GenericCounter<AtomicU64>);

impl Counter {
    fn new(registry: &Registry, opts: Opts) -> Self {
        let counter = GenericCounter::with_opts(opts).unwrap();
        registry.register(Box::new(counter.clone()));
        Self(counter)
    }

    pub fn get(&self) -> usize {
        self.0.get() as usize
    }
}

impl metrics::Counter for Counter {
    fn add(&self, amount: usize) {
        self.0.inc_by(amount as u64);
    }
}

/// A [Gauge](metrics::Gauge) metric.
#[derive(Clone, Debug)]
pub struct Gauge(GenericGauge<AtomicU64>);

impl Gauge {
    fn new(registry: &Registry, opts: Opts) -> Self {
        let gauge = GenericGauge::with_opts(opts).unwrap();
        registry.register(Box::new(gauge.clone()));
        Self(gauge)
    }

    pub fn get(&self) -> usize {
        self.0.get() as usize
    }
}

impl metrics::Gauge for Gauge {
    fn set(&self, amount: usize) {
        self.0.set(amount as u64);
    }

    fn update(&self, delta: i64) {
        if delta >= 0 {
            self.0.add(delta as u64);
        } else {
            self.0.sub(-delta as u64);
        }
    }
}

/// A [Histogram](metrics::Histogram) metric.
#[derive(Clone, Debug)]
pub struct Histogram(prometheus::Histogram);

impl Histogram {
    fn new(registry: &Registry, opts: Opts) -> Self {
        let histogram = prometheus::Histogram::with_opts(opts.into()).unwrap();
        registry.register(Box::new(histogram.clone()));
        Self(histogram)
    }

    pub fn sample_count(&self) -> usize {
        self.0.get_sample_count() as usize
    }

    pub fn sum(&self) -> f64 {
        self.0.get_sample_sum()
    }

    pub fn mean(&self) -> f64 {
        self.sum() / (self.sample_count() as f64)
    }
}

impl metrics::Histogram for Histogram {
    fn add_point(&self, point: f64) {
        self.0.observe(point);
    }
}

/// A [Label](metrics::Label) metric.
#[derive(Clone, Debug)]
pub struct Label(Arc<RwLock<String>>);

impl Label {
    fn new(registry: &Registry, opts: Opts) -> Self {
        let label = Self(Default::default());
        let mut fq_name = vec![];
        if !opts.namespace.is_empty() {
            fq_name.extend(opts.namespace.split('_').map(String::from));
        }
        if !opts.subsystem.is_empty() {
            fq_name.extend(opts.subsystem.split('_').map(String::from));
        }
        fq_name.push(opts.name.clone());
        registry.register_label(fq_name, label.clone());
        label
    }

    pub fn get(&self) -> String {
        self.0.read().unwrap().clone()
    }
}

impl metrics::Label for Label {
    fn set(&self, value: String) {
        *self.0.write().unwrap() = value;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing::setup_test;
    use metrics::Metrics;

    #[test]
    fn test_prometheus_metrics() {
        setup_test();

        let metrics = PrometheusMetrics::default();

        // Register one metric of each type.
        let counter = metrics.create_counter("counter".into(), None);
        let gauge = metrics.create_gauge("gauge".into(), None);
        let histogram = metrics.create_histogram("histogram".into(), None);
        let label = metrics.create_label("label".into());

        // Set the metric values.
        counter.add(20);
        gauge.set(42);
        histogram.add_point(20f64);
        label.set("value".into());

        // Check the values.
        assert_eq!(metrics.get_counter("counter").unwrap().get(), 20);
        assert_eq!(metrics.get_gauge("gauge").unwrap().get(), 42);
        assert_eq!(
            metrics.get_histogram("histogram").unwrap().sample_count(),
            1
        );
        assert_eq!(metrics.get_histogram("histogram").unwrap().sum(), 20f64);
        assert_eq!(metrics.get_histogram("histogram").unwrap().mean(), 20f64);
        assert_eq!(metrics.get_label("label").unwrap().get(), "value");

        // Set the metric values again, to be sure they update properly.
        counter.add(22);
        gauge.set(100);
        histogram.add_point(22f64);
        label.set("another".into());

        // Check the updated values.
        assert_eq!(metrics.get_counter("counter").unwrap().get(), 42);
        assert_eq!(metrics.get_gauge("gauge").unwrap().get(), 100);
        assert_eq!(
            metrics.get_histogram("histogram").unwrap().sample_count(),
            2
        );
        assert_eq!(metrics.get_histogram("histogram").unwrap().sum(), 42f64);
        assert_eq!(metrics.get_histogram("histogram").unwrap().mean(), 21f64);
        assert_eq!(metrics.get_label("label").unwrap().get(), "another");

        // Export to a Prometheus string.
        let string = metrics.prometheus().unwrap();
        // Make sure the output makes sense.
        let lines = string.lines().collect::<Vec<_>>();
        assert!(lines.contains(&"counter 42"));
        assert!(lines.contains(&"gauge 100"));
        assert!(lines.contains(&"histogram_sum 42"));
        assert!(lines.contains(&"histogram_count 2"));
        assert!(lines.contains(&"# LABEL label another"));
    }

    #[test]
    fn test_namespace() {
        setup_test();

        let metrics = PrometheusMetrics::default();
        let subgroup1 = metrics.subgroup("subgroup1".into());
        let subgroup2 = subgroup1.subgroup("subgroup2".into());
        let counter = subgroup2.create_counter("counter".into(), None);
        let label = subgroup2.create_label("label".into());
        counter.add(42);
        label.set("value".into());

        // Check namespacing.
        assert_eq!(
            metrics.get_subgroup(["subgroup1"]).unwrap().namespace,
            ["subgroup1"]
        );
        assert_eq!(
            metrics
                .get_subgroup(["subgroup1", "subgroup2"])
                .unwrap()
                .namespace,
            ["subgroup1", "subgroup2"]
        );
        assert_eq!(
            metrics
                .get_subgroup(["subgroup1"])
                .unwrap()
                .get_subgroup(["subgroup2"])
                .unwrap()
                .namespace,
            ["subgroup1", "subgroup2"]
        );

        // Check different ways of accessing the counter.
        assert_eq!(
            metrics
                .get_subgroup(["subgroup1", "subgroup2"])
                .unwrap()
                .get_counter("counter")
                .unwrap()
                .get(),
            42
        );
        assert_eq!(
            metrics
                .get_subgroup(["subgroup1"])
                .unwrap()
                .get_subgroup(["subgroup2"])
                .unwrap()
                .get_counter("counter")
                .unwrap()
                .get(),
            42
        );

        // Check fully-qualified counter name in export.
        assert!(metrics
            .prometheus()
            .unwrap()
            .lines()
            .contains(&"subgroup1_subgroup2_counter 42"));

        // Check different ways of accessing the label.
        assert_eq!(
            metrics
                .get_subgroup(["subgroup1", "subgroup2"])
                .unwrap()
                .get_label("label")
                .unwrap()
                .get(),
            "value"
        );
        assert_eq!(
            metrics
                .get_subgroup(["subgroup1"])
                .unwrap()
                .get_subgroup(["subgroup2"])
                .unwrap()
                .get_label("label")
                .unwrap()
                .get(),
            "value"
        );

        // Check fully-qualified label name in export.
        assert!(metrics
            .prometheus()
            .unwrap()
            .lines()
            .contains(&"# LABEL subgroup1_subgroup2_label value"));
    }
}
