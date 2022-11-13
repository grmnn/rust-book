use std::io::{stdin};
fn main() {
    println!("Into which type to you want to convert? Celsius(1) or Fahrenheit(2)");
    let mut target_temperature = String::new();

    match stdin().read_line(&mut target_temperature) {
        Ok(_) => {
            println!("Target temperature is: {target_temperature}");
            &target_temperature = target_temperature.trim();
        },
        Err(_) => {
            println!("Something went wrong")
        }
    }

    

    println!("Target temperature is: {target_temperature}");
}
