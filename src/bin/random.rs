use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{self, Read, Write};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Problem {
    room_width: f64,
    room_height: f64,
    stage_width: f64,
    stage_height: f64,
    stage_bottom_left: Vec<f64>,
    musicians: Vec<i64>,
    attendees: Vec<Attendee>,
    pillars: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Attendee {
    x: f64,
    y: f64,
    tastes: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Position {
    x: f64,
    y: f64
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Solution {
    placements: Vec<Position>
}

fn trivial(problem: Problem) -> Vec<Position> {
    let mut players: Vec<Position> = Vec::new();

    let stage_x_min = problem.stage_bottom_left[0];
    let stage_y_min = problem.stage_bottom_left[1];
    let stage_x_max = stage_x_min + problem.stage_width;
    let stage_y_max = stage_y_min + problem.stage_height;

    let mut y = stage_y_min + 10.0;
    'iter:
    while y < stage_y_max - 10.0 {
        let mut x = stage_x_min + 10.0;
        while x < stage_x_max - 10.0 {
            if players.len() >= problem.musicians.len() {
                break 'iter;
            }
            players.push(Position { x: x as f64, y: y as f64 });
            x = x + 10.0;
        }
        y = y + 10.0;
    }

    players
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let problem: Problem = serde_json::from_str(&buffer).expect("Failed to parse JSON");

    let players = trivial(problem);

    let solution = Solution {
        placements: players
    };

    let output = serde_json::to_string(&solution).expect("Failed to generate JSON");

    io::stdout().write_all(output.as_bytes())?;
    Ok(())
}
