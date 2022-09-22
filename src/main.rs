use std::io;

fn main() {
    // Temp converter
    let mut input = String::new();

    println!("Enter a temperature in Farenheit.");
    io::stdin().read_line(&mut input).expect("Did not enter a temp");

    let numeric_temp = input.trim().parse::<f32>().unwrap();
    let multiplier = 0.5556;
    
    let temp_in_celsius = numeric_temp * multiplier;

    // TODO: Fix line break cause by trim method on line 10 above
    println!("{input} degrees Farenheit is equal to {temp_in_celsius} degrees Celsius!")


}
