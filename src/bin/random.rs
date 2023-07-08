use serde_json::Value;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let v: Value = serde_json::from_str(&buffer).expect("Failed to parse JSON");

    let output = serde_json::to_string(&v).expect("Failed to generate JSON");

    io::stdout().write_all(output.as_bytes())?;
    Ok(())
}
