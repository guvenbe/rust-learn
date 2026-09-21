use std::process::Command;

fn main() {
    println!("Running project checklist...\n");

    // Check formatting
    let fmt = Command::new("cargo").args(["fmt", "--check"]).output().unwrap();
    if fmt.status.success() {
        println!("OK - Code is formatted");
    } else {
        println!("ERROR - Code needs formatting");
    }

    // Run clippy
    let clippy = Command::new("cargo").args(["clippy", "--all-targets", "--", "-D", "warnings"]).output().unwrap();
    if clippy.status.success() {
        println!("OK - Clippy passed");
    } else {
        println!("ERROR - Clippy found issues");
    }

    // Run tests
    let test = Command::new("cargo").arg("test").output().unwrap();
    if test.status.success() {
        println!("OK - All tests passed");
    } else {
        println!("ERROR - Tests failed");
    }

    println!("\nChecklist complete!");
}