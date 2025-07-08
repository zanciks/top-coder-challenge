use std::env;

use crate::trip::Trip;

mod tests;
mod trip;

fn main() {
    let args: Vec<String> = env::args().collect();
    let floats: Vec<f32> = args
        .iter()
        .skip(1)
        .map(|s| s.parse::<f32>().unwrap())
        .collect();
    let trip = Trip::new(floats[0] as i32, floats[1], floats[2]);
    println!("{}", trip.calculate_output());
}
