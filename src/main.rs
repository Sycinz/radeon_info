use std::{fs::read_to_string, path::PathBuf};
use glob::{glob};

fn main() {
    println!("This is a radeon_info program in beta version\n");
    println!("{}", get_gpu_info("frequency").0);
    println!("{}", get_gpu_info("temperature").0); // Other info_types will be added later
}

fn get_gpu_info(info_type: &str) -> (String, std::io::Result<()>) {
    // Frequency case
    if info_type == "frequency" {
        let path = PathBuf::from("/sys/class/drm/card0/device/pp_dpm_sclk");
        let file_content = read_to_string(path).unwrap();

        let output = String::from(format!("Frequency of GPU:\n{}", &file_content));

        (output, Ok(()))

    // Temperature case
    } else if info_type == "temperature" {
        // Setting the temperature file path in Linux
        let path = PathBuf::from("/sys/class/drm/card*/device/hwmon/hwmon*/temp1_input");
        let temp_path = glob(path.to_str().unwrap()).unwrap();
        let temp_path = temp_path.into_iter().next().unwrap();

        let file_content = match read_to_string(temp_path.expect("error")) {
            Ok(content) => content,
            Err(e) => {
                return (String::from(format!("Could not read the temperature {:?}", &e)), Err(e));
            }
        };        

        let fixed_content = match file_content.trim().parse::<i32>() {
            Ok(num) => (num / 1000).to_string(), // Dividing by 1000 to get the temperature in Celsius
            Err(e) => {
                return (String::from("Could not parse the temperature value."), Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Parsing error")));
            }
        };

        let output = String::from(format!("Temperature of GPU: {}°C", fixed_content));
        (output, Ok(()))
    } else {
        // This case will probably never be reached
        panic!("Invalid info_type provided. Use 'frequency' or 'temperature'.");
    }
}