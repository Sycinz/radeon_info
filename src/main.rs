use std::process::Command;

fn main() {
    println!("This is a radeon_info program in testing version");
    println!("{}", get_gpu_info("frequency"));
    println!("{}", get_gpu_info("frequency")); // Other info_types will be added later
    println!("{}", get_gpu_info("frequency"));
}

fn get_gpu_info(info_type: &str) -> String {
    let mut command = String::from("cat ");
    let path = String::from("/sys/class/drm/card1/device/pp_dpm_sclk");

    command += &path;

    if info_type == "frequency" {
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
        panic!("PANICCCCC AAAAAAAAAAAAAAAAAA");
    }
}