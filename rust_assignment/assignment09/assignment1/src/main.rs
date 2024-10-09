// Declare a constant for the freezing point of water in Fahrenheit
const FREEZING_POINT_FAHRENHEIT: f64 = 32.0;

// Function to convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_FAHRENHEIT) * 5.0 / 9.0
}

// Function to convert Celsius to Fahrenheit
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + FREEZING_POINT_FAHRENHEIT
}

fn main() {
    // Declare a mutable variable with a temperature in Fahrenheit
    let mut fahrenheit_temp: f64 = 32.0;

    // Convert it to Celsius using the function and print the result
    let celsius_temp = fahrenheit_to_celsius(fahrenheit_temp);
    println!("{:.2}°F in celsius is {:.2}°C", fahrenheit_temp, celsius_temp);

    // Use a loop to convert and print the next 5 integer temperatures
    for temp in 0..5 {
        fahrenheit_temp += 1.0; // Increment the temperature by 1
        let celsius_temp = fahrenheit_to_celsius(fahrenheit_temp);
        println!("{:.2}°F in celcius is {:.2}°C", fahrenheit_temp, celsius_temp);
    }
}
