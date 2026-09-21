// build.rs
use std::env;
use std::fs;
use std::path::Path;
use chrono::Utc;

fn main() {
    // Get the output directory (set by Cargo)
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set by Cargo");
    let dest_path = Path::new(&out_dir).join("greeting.rs");

    let now = Utc::now();
    let name = "Alice";

    // Generate Rust code
    let greeting = format!(
        r#"
        pub fn greet() -> &'static str {{
            "Hello, {}! This project was built on {}."
        }}
        "#,
        name,
        now.format("%Y-%m-%d %H:%M:%S UTC")
    );

    // Write to OUT_DIR
    fs::write(&dest_path, greeting)
        .expect("Failed to write generated file");

    // Tell Cargo when to re-run the build
    println!("cargo:rerun-if-changed=build.rs");
}