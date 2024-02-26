use std::{error, process::Command};

fn main() {
    println!("This is a radeon_info program in testing version\n");
    println!("{}", get_gpu_info("frequency"));
    println!("{}", get_gpu_info("temperature")); // Other info_types will be added later
}

fn get_gpu_info(info_type: &str) -> String {
    let mut command = String::from("cat ");
    let mut path = String::new();

    // Frequency case
    if info_type == "frequency" {
        path = String::from("/sys/class/drm/card*/device/pp_dpm_sclk");

        command += &path;

        println!("Frequency of GPU: ");
        let output = Command::new("bash")
            .arg("-c")
            .arg(command)
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout);
            String::from(result)
        } else {
            let result = String::from_utf8_lossy(&output.stderr);
            String::from(result)
        }
    // Temperature case
    } else if info_type == "temperature" {
        path = String::from("/sys/class/drm/card*/device/hwmon/hwmon*/temp1_input");

        command += &path;

        println!("Temperature of GPU: ");
        let output = Command::new("bash")
            .arg("-c")
            .arg(command)
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout);
            String::from(result)
        } else {
            let result = String::from_utf8_lossy(&output.stderr);
            String::from(result)
        }
    } else {
        panic!("Could not find GPU property.");
    }
}