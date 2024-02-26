use std::process::Command;

fn main() {
    let output = if cfg!(target_os = "linux") {
        Command::new("zsh")
            .args(["-c", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")
    };

    println!("{:?}", output);
}
