use std::io;

fn main() {
    println!("Temperature converter");

    loop {
        println!("Choose the conversion type:");
        println!("1: Celsius to Fahrenheit");
        println!("2: Fahrenheit to Celsius");
        println!("3: Celsius to Kelvin");
        println!("4: Kelvin to Celsius");
        println!("5: Kelvin to Fahrenheit");
        println!("6: Fahrenheit to Kelvin");
        println!("7: Quit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        if choice == 7 {
            break;
        }

        println!("Enter temperature value:");
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed ro read line");
        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid temperature");
                continue;
            }
        };

        let result = match choice {
            1 => celsius_to_fahrenheit(temp),
            2 => fahrenheit_to_celsius(temp),
            // 2 => celsius_kelvin_conversion(temp, choice),
            3 => celsius_to_kelvin(temp),
            // 3 => celsius_kelvin_conversion(temp, choice),
            4 => kelvin_to_celsius(temp),
            5 => kelvin_to_fahrenheit(temp),
            6 => fahrenheit_to_kelvin(temp),
            _ => {
                println!("Invalid choice");
                continue;
            }
        };

        println!("Converted temperature: {:.2}", result);
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

fn kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

fn celsius_kelvin_conversion(value: f64, choice: u32) -> Option<f64> {
    match choice {
        3 => Some(value + 273.15),
        4 => Some(value - 273.15),
        _ => None, // Indicates invalid choice, good to learn
    }
}

fn kelvin_to_fahrenheit(kelvin: f64) -> f64 {
    celsius_to_fahrenheit(kelvin_to_celsius(kelvin))
}

fn fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    celsius_to_kelvin(fahrenheit_to_celsius(fahrenheit))
}
