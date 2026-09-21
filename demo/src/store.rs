use crate::domain::Telemetry;
use dashmap::DashMap;
use std::time::{SystemTime, Duration};

/// Concurrent in-memory store: device_id -> Vec<Telemetry>
/// Using DashMap to avoid a coarse global lock.
#[derive(Default)]
pub struct TelemetryStore {
    inner: DashMap<String, Vec<Telemetry>>,
    // Bounding the kept history would be easy to add (ring buffer), omitted for brevity.
}

impl TelemetryStore {
    pub fn new() -> Self {
        Self { inner: DashMap::new() }
    }

    /// Appends a new measurement for a device.
    pub fn push(&self, t: Telemetry) {
        let mut entry = self.inner.entry(t.device_id.clone()).or_default();
        entry.push(t);
    }

    /// Returns a copy of the last N values for a device (cheap-ish for small N).
    /// This keeps the API simple; one could also return a borrowed "view" if needed.
    pub fn last_n(&self, device_id: &str, n: usize) -> Vec<Telemetry> {
        self.inner
            .get(device_id)
            .map(|v| {
                let vec = &*v;
                let len = vec.len();
                let start = len.saturating_sub(n);
                vec[start..].to_vec()
            })
            .unwrap_or_default()
    }

    /// Compute an average over the last N values using iterators (zero-cost style).
    pub fn iter_last_n_average(&self, device_id: &str, n: usize) -> Option<f64> {
        let vals = self.last_n(device_id, n);
        let (sum, count) = vals.iter()
            .map(|t| t.value)
            .fold((0.0, 0), |(acc, c), v| (acc + v, c + 1));
        if count > 0 { Some(sum / (count as f64)) } else { None }
    }

    /// Finds measurements within a recent window.
    pub fn since(&self, device_id: &str, window: Duration) -> Vec<Telemetry> {
        let now = SystemTime::now();
        self.inner
            .get(device_id)
            .map(|v| v.iter().filter(|t| now.duration_since(t.timestamp).unwrap_or_default() <= window).cloned().collect())
            .unwrap_or_default()
    }
}
