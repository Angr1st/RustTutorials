use std::io;

fn main() {
    println!("Convert Fahrenheit to Celsius.");

    println!("Please input your temperature.");

    let mut temperature = String::new();

    io::stdin().read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: f64 = temperature.trim().parse()
        .expect("Please type a number!");

    let temp_celsius = to_celsius(temperature);

    println!("The temperature in Celsius is: {}", temp_celsius);

    println!("The temperature in Fahrenheit is: {}", to_fahrenheit( temp_celsius));
}

fn to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * (5.0/9.0)
}

fn to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}