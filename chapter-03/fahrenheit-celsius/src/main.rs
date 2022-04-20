use std::io;

fn main() {
    println!("Please input a temperature.");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: f32 = number.trim().parse().expect("That's not a number");

    println!(
        "{} celsius is {} fahrenheit",
        number,
        celsius_to_fahrenheit(number)
    );

    println!(
        "{} fahrenheit is {} celsius",
        number,
        fahrenheit_to_celsius(number)
    );
}

fn celsius_to_fahrenheit(t: f32) -> f32 {
    t * (9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(t: f32) -> f32 {
    t * (5.0 / 9.0) - 32.0
}
