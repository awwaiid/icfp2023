use crate::icfp::*;
use crate::scorer::*;
use rand::Rng;
use std::collections::HashSet;

fn solve_once(problem: &Problem) -> Solution {
    let mut rng = rand::thread_rng();
    let mut players: Vec<Position> = Vec::new();

    let mut used_locations = HashSet::new();

    let stage_x_min = problem.stage_bottom_left[0];
    let stage_y_min = problem.stage_bottom_left[1];

    let stage_cols = ((problem.stage_width - 20.0) / 10.0).ceil() as u32;
    let stage_rows = ((problem.stage_height - 20.0) / 10.0).ceil() as u32;

    eprintln!("Need to place {} musicians", problem.musicians.len());
    for _n in 0..(problem.musicians.len()) {
        // eprintln!("Placing musician {}", n);
        let mut found_spot = false;
        while !found_spot {
            let col = rng.gen_range(0..stage_cols);
            let row = rng.gen_range(0..stage_rows);
            if !used_locations.contains(&(col, row)) {
                used_locations.insert((col, row));
                found_spot = true;
                players.push(Position {
                    x: stage_x_min + (col as f32 * 10.0) + 10.0,
                    y: stage_y_min + (row as f32 * 10.0) + 10.0,
                });
            }
        }
    }

    Solution {
        placements: players,
    }
}

pub fn solve(problem: &Problem) -> Solution {
    let n = 1000;
    let mut best_score = 0.0;
    let mut best_solution = Solution { placements: vec![] };
    for attempt in 0..n {
        eprint!("Attempt {}... ", attempt);
        let solution = solve_once(&problem);
        let score = scorer(&problem, &solution);
        eprintln!("score {}", score);
        if score > best_score {
            eprintln!("  NEW WINNER");
            best_score = score;
            best_solution = solution;
        }
    }
    eprintln!("best score {}", best_score);
    best_solution
}
