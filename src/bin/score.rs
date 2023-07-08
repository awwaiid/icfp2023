use std::fs;
use std::env;
use std::io::{self, Read, Write};

use paisleys_paradox::icfp::*;
use paisleys_paradox::scorer::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // if args.len() < 3 {
    //     eprintln!("Please provide a problem.json and solution.json");
    //     exit;
    // }

    let problem_filename = &args[1];
    eprintln!("Loading problem file {}", problem_filename);
    let problem_file = fs::read_to_string(problem_filename)
        .expect("Error reading problem file");
    let problem: Problem = serde_json::from_str(&problem_file)
        .expect("Failed to parse problem JSON");

    let solution_filename = &args[2];
    eprintln!("Loading solution file {}", solution_filename);
    let solution_file = fs::read_to_string(solution_filename)
        .expect("Error reading solution file");
    let solution: Solution = serde_json::from_str(&solution_file)
        .expect("Failed to parse solution JSON");

    eprintln!("Calculating score");
    println!("{}", scorer(problem, solution));

    Ok(())
}
