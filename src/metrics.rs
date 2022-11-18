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
    core::{AtomicU64, GenericCounter, GenericGauge},
    Encoder, Opts, Registry, TextEncoder,
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
    labels: Arc<RwLock<HashMap<String, Label>>>,
}

impl PrometheusMetrics {
    /// Export all metrics in the Prometheus text format.
    pub fn prometheus(&self) -> Result<String, MetricsError> {
        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        let metric_families = self.metrics.gather();
        encoder.encode(&metric_families, &mut buffer)?;
        String::from_utf8(buffer).map_err(|err| MetricsError::Prometheus {
            source: prometheus::Error::Msg(format!(
                "could not convert Prometheus output to UTF-8: {}",
                err
            )),
        })
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
        self.get_metric(&self.labels, name)
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
        let mut opts = Opts::new(label, unit_label.unwrap_or_default());
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
        let label = Label::default();
        self.labels.write().unwrap().insert(name, label.clone());
        Box::new(label)
    }

    fn subgroup(&self, subgroup_name: String) -> Box<dyn metrics::Metrics> {
        Box::new(
            self.children
                .write()
                .unwrap()
                .entry(subgroup_name)
                .or_default()
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
        registry.register(Box::new(counter.clone())).unwrap();
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
        registry.register(Box::new(gauge.clone())).unwrap();
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
}

/// A [Histogram](metrics::Histogram) metric.
#[derive(Clone, Debug)]
pub struct Histogram(prometheus::Histogram);

impl Histogram {
    fn new(registry: &Registry, opts: Opts) -> Self {
        let histogram = prometheus::Histogram::with_opts(opts.into()).unwrap();
        registry.register(Box::new(histogram.clone())).unwrap();
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
///
/// Note that there is no Prometheus equivalent of a [Label], so this metric does not export its
/// value with the Prometheus data; however, it can still be queried directly from a
/// [PrometheusMetrics] collection using [get_label](PrometheusMetrics::get_label).
#[derive(Clone, Debug, Default)]
pub struct Label(Arc<RwLock<String>>);

impl Label {
    pub fn get(&self) -> String {
        self.0.read().unwrap().clone()
    }
}

impl metrics::Label for Label {
    fn set(&self, value: String) {
        *self.0.write().unwrap() = value;
    }
}
