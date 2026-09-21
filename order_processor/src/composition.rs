/// Composition over inheritance via traits.
pub trait Notifier { fn notify(&self, msg: &str); }
pub trait AuditLogger { fn log(&self, msg: &str); }

pub struct StdoutNotifier;
pub struct StdoutLogger;

impl Notifier for StdoutNotifier { fn notify(&self, msg: &str) { println!("[notify] {msg}"); } }
impl AuditLogger for StdoutLogger { fn log(&self, msg: &str) { println!("[audit] {msg}"); } }
