use std::process::{Command, Output};

fn get_gpu_info(info_type: &str) -> String {
    let mut command = String::from("cat ");
    let mut path = String::new();

    // Frequency case
    if info_type == "frequency" {
        path = String::from("/sys/class/drm/card*/device/pp_dpm_sclk");

        command += &path;

        println!("Frequency of GPU: ");
        let output = command_execution(&command);

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
        let output = command_execution(&command);

            // When output is valid, converting from utf8_lossy to String
        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout);
            let str_result = String::from(result);

            /* Converting str_result without whitespaces, dividing by 1000 and
            and then returning as string */ 
            match str_result.trim().parse::<i32>() {
                Ok(num) => {
                    (num / 1000).to_string()
                },
                Err(error) => panic!("Could not parse to i32!")
            }

        } else {
            let result = String::from_utf8_lossy(&output.stderr);
            String::from(result)
        }
    } else {
        panic!("Could not find GPU property.");
    }
}

fn command_execution(command: &str) -> Output {
    let output = Command::new("bash")
            .arg("-c")
            .arg(command)
            .output()
            .expect("Failed to execute command");

        output
}

fn main() {
    println!("This is a radeon_info program in testing version\n");
    println!("{}", get_gpu_info("frequency"));
    println!("{}", get_gpu_info("temperature")); // Other info_types will be added later
}