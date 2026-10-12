// 1. Fahrenheit converter

// Prompt for a temperature, convert °F → °C ((f - 32) * 5/9 as f64), then back to °F. let bindings for each step, immutability throughout.

use std::{io::{self, Write}};


fn main () {

    print!("Enter tempreature: ");
    let mut tempreature = String::new();
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut tempreature).expect("something went wrong");

    print!("buffer is {tempreature}");
    let tempreature: i32 = match tempreature.trim().parse::<i32>() {
        Ok(n) => n,
        Err(err) => {
            panic!("error: {}", err);
        }
    };

    let farenhit = ctof(tempreature);
    println!("{tempreature}C to F = {farenhit}");

    let celcius = ftoc(tempreature);
    println!("{tempreature}F to C = {celcius}");

}

fn ftoc(tempreature: i32) -> f32 {
    (tempreature - 32) as f32 * 5.0/9.0
}

fn ctof(tempreature: i32) -> f32 {
    (tempreature * 9/5) as f32 + 32.0
}