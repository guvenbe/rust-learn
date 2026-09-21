/// RAII/Drop demo: shows deterministic cleanup at scope boundaries.
pub struct ExportBuffer {
    name: &'static str,
    buf: Vec<u8>,
}

impl ExportBuffer {
    pub fn new(name: &'static str, size: usize) -> Self {
        println!("alloc {name}: {size} bytes");
        Self { name, buf: vec![0; size] }
    }
    pub fn write_sample(&mut self) { if let Some(b) = self.buf.first_mut() { *b = b.wrapping_add(1); } }
}

impl Drop for ExportBuffer {
    fn drop(&mut self) { println!("free  {} (via Drop)", self.name); }
}

pub fn export_demo() {
    println!("-- entering export demo");
    {
        let mut a = ExportBuffer::new("export-A", 1_000_000);
        a.write_sample();
        {
            let mut b = ExportBuffer::new("export-B", 500_000);
            b.write_sample();
            println!("-- leaving inner block (export-B will drop now)");
        }
        println!("-- back to outer block (export-A still alive)");
    }
    println!("-- left outer block (export-A dropped)");
}
