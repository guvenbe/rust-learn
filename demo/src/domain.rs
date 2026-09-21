use serde::{Deserialize, Serialize};
use std::time::{SystemTime, Duration};

/// Single telemetry measurement coming from a device.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Telemetry {
    pub device_id: String,
    pub timestamp: SystemTime,
    pub value: f64,
    /// Optional arbitrary tags (kept simple for the demo).
    pub tag: Option<String>,
}

impl Telemetry {
    pub fn new(device_id: impl Into<String>, value: f64) -> Self {
        Self {
            device_id: device_id.into(),
            timestamp: SystemTime::now(),
            value,
            tag: None,
        }
    }
}

/// Helper to generate synthetic, realisticish values (e.g., temperature).
pub fn synthetic_value(tick: u64, base: f64) -> f64 {
    // Small variations; pretend there's some periodicity and noise.
    let wobble = ((tick as f64) * 0.037).sin() * 0.5;
    base + wobble
}

/// A simple SLA-ish bound we can assert in tests or logs.
pub const MAX_INGEST_TO_STORE_LATENCY: Duration = Duration::from_millis(500);
