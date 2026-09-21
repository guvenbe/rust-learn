//! Fearless concurrency: threads + async pipeline with Tokio.

use crate::domain::{Telemetry, synthetic_value};
use crate::store::TelemetryStore;
// use rand::Rng; // removed (unused)
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

/// Classic safe concurrent counter using Arc<Mutex<_>>.
pub fn safe_concurrent_counter() -> i32 {
    use std::thread;

    let counter = Arc::new(Mutex::new(0i32));
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        let h = thread::spawn(move || {
            let mut guard = c.lock().expect("poisoned");
            *guard += 1;
        });
        handles.push(h);
    }
    for h in handles { h.join().unwrap(); }

    // Avoid holding the MutexGuard across the function return boundary.
    let value = *counter.lock().unwrap();
    value
}

/// A small near real-world async pipeline:
/// - N producers simulate devices emitting telemetry
/// - 1 consumer aggregates into the store
pub struct Pipeline {
    pub store: Arc<TelemetryStore>,
    pub device_count: usize,
    pub producer_rate: Duration,
}

impl Pipeline {
    pub fn new(store: Arc<TelemetryStore>, device_count: usize, producer_rate: Duration) -> Self {
        Self { store, device_count, producer_rate }
    }

    /// Runs the pipeline for a bounded amount of time.
    pub async fn run_for(&self, total_duration: Duration) {
        use tokio::task::JoinHandle;
        let (tx, mut rx) = mpsc::channel::<Telemetry>(1024);

        // Spawn producers and keep their handles
        let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(self.device_count);
        for device_idx in 0..self.device_count {
            let device_id = format!("device-{:04}", device_idx);
            let tx = tx.clone();
            let rate = self.producer_rate;
            let h = tokio::spawn(async move {
                let mut tick: u64 = 0;
                loop {
                    let v = synthetic_value(tick, 25.0 + (device_idx as f64) * 0.05);
                    if tx.send(Telemetry::new(device_id.clone(), v)).await.is_err() {
                        break; // consumer closed
                    }
                    tick += 1;
                    sleep(rate).await;
                }
            });
            handles.push(h);
        }
        drop(tx); // drop the original sender (clones live in producers)

        let store = Arc::clone(&self.store);
        let consumer = tokio::spawn(async move {
            while let Some(t) = rx.recv().await {
                store.push(t);
            }
        });

        // Let the pipeline run for the requested duration
        sleep(total_duration).await;

        // Stop producers so all tx clones are dropped -> channel will close
        for h in handles {
            h.abort();
        }

        // Wait the consumer to drain remaining messages and exit
        let _ = consumer.await;
    }
}

/// Example of isolating CPU-bound parsing from async reactor using spawn_blocking.
pub async fn parse_heavy_payload(payload: String) -> usize {
    tokio::task::spawn_blocking(move || {
        // Fake CPU-bound work: count digits as if parsing a big JSON.
        payload.chars().filter(|c| c.is_ascii_digit()).count()
    })
    .await
    .expect("spawn_blocking failed")
}
