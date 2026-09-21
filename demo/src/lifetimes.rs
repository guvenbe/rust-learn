//! Lifetime examples: explicit annotations and safe borrowed returns.

/// Returns the longer of two string slices; classic example of tying output to inputs.
pub fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

/// Borrowed "view" over a slice of values.
/// In real code, such views avoid copying on read paths.
pub struct TelemetryView<'a, T> {
    pub window: &'a [T],
}

impl<'a, T> TelemetryView<'a, T> {
    pub fn new(window: &'a [T]) -> Self { Self { window } }
    pub fn len(&self) -> usize { self.window.len() }
}
