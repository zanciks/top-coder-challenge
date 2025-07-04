use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

mod trip;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("public_cases.json")?;
    let reader = BufReader::new(file);
    let raw: Vec<Value> = serde_json::from_reader(reader)?;

    let trips: Vec<trip::Trip> = raw
        .into_iter()
        .filter_map(|v| v.get("input").cloned())
        .filter_map(|input| serde_json::from_value(input).ok())
        .collect();

    for trip in trips {
        println!("{}", trip);
    }

    Ok(())
}
