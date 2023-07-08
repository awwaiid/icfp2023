use std::env;
use std::io::{self, Read, Write};

use paisleys_paradox::icfp::*;
use paisleys_paradox::scorer::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let problem: Problem = serde_json::from_str(&buffer).expect("Failed to parse JSON");

    let args: Vec<String> = env::args().collect();
    let solver = args.get(1);
    let solver = match solver {
        Some(name) => name.as_str(),
        None => "trivial"
    };

    eprintln!("Solving problem with {}", solver);

    let solution = match solver {
        "trivial" => paisleys_paradox::solver::trivial::solve(&problem),
        _ => panic!("I should never get here!!!!")
    };

    let output = serde_json::to_string(&solution).expect("Failed to generate JSON");

    io::stdout().write_all(output.as_bytes())?;

    eprintln!("");
    eprintln!("Score: {}", scorer(problem, solution));
    eprintln!("");

    Ok(())
}
