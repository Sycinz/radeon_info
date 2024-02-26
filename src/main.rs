use std::process::Command;

fn main() {
    let output = Command::new("bash")
        .arg("-c")
        .arg("echo Hello from Rust!")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        println!("Command output: {}", result);
    } else {
        let result = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed with error: {}", result);
    }
}