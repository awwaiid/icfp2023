use ncollide2d::na::{Isometry2, Point2, Vector2};
use ncollide2d::pipeline::object::CollisionGroups;
use ncollide2d::pipeline::GeometricQueryType;
use ncollide2d::query::Ray;
use ncollide2d::shape::{Ball, ShapeHandle};
use ncollide2d::world::CollisionWorld;
use serde::{Deserialize, Serialize};
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
    pillars: Vec<Pillar>,
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
    y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Pillar {
    center: Vec<f64>,
    radius: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Solution {
    placements: Vec<Position>,
}

fn trivial_solver(problem: &Problem) -> Vec<Position> {
    let mut players: Vec<Position> = Vec::new();

    let stage_x_min = problem.stage_bottom_left[0];
    let stage_y_min = problem.stage_bottom_left[1];
    let stage_x_max = stage_x_min + problem.stage_width;
    let stage_y_max = stage_y_min + problem.stage_height;

    let mut y = stage_y_min + 10.0;
    'iter: while y < stage_y_max - 10.0 {
        let mut x = stage_x_min + 10.0;
        while x < stage_x_max - 10.0 {
            if players.len() >= problem.musicians.len() {
                break 'iter;
            }
            players.push(Position {
                x: x as f64,
                y: y as f64,
            });
            x = x + 10.0;
        }
        y = y + 10.0;
    }

    players
}

fn scorer(problem: Problem, solution: Solution) -> f64 {
    // {
    //   "room_width": 4200.0,
    //   "room_height": 6234.0,
    //   "stage_width": 198.0,
    //   "stage_height": 909.0,
    //   "stage_bottom_left": [
    //     1076.0,
    //     2395.0
    //   ],
    //   "musicians": [
    //     0,
    //     1,
    //     2
    //   ],
    //   "pillars": []}

    let mut world = CollisionWorld::new(0.02);

    // A ball / circle with a diameter of 10.0
    let circle_shape = ShapeHandle::new(Ball::new(5.0));

    let musicians_group = CollisionGroups::new().with_membership(&[0]);
    let contacts_query = GeometricQueryType::Contacts(0.0, 0.0);

    // Loop over musicans and add them to the world
    for (i, player) in solution.placements.clone().iter().enumerate() {
        world.add(
            Isometry2::new(Vector2::new(player.x, player.y), 0.0),
            circle_shape.clone(),
            musicians_group,
            contacts_query,
            i,
        );
    }

    let pillar_group = CollisionGroups::new().with_membership(&[1]);
    // Loop over pillars and add them to the world
    for pillar in problem.pillars {
        let pillar_shape = ShapeHandle::new(Ball::new(pillar.radius));
        world.add(
            Isometry2::new(Vector2::new(pillar.center[0], pillar.center[1]), 0.0),
            pillar_shape,
            pillar_group,
            contacts_query,
            0,
        );
    }

    // loop over attendees and trace a line to each placement if it intersects a pillar, or another placement, then 0, otherwise take the distance of the line
    let mut score = 0.0;

    for attendee in problem.attendees {
        for (i, player) in solution.placements.clone().iter().enumerate() {
            let ray = Ray::new(
                Point2::new(attendee.x, attendee.y),
                Vector2::new(player.x - attendee.x, player.y - attendee.y),
            );

            if world
                .interferences_with_ray(&ray, 5.0_f64, &pillar_group)
                .count()
                > 0
            {
                break;
            }

            if world
                .interferences_with_ray(&ray, 5.0_f64, &musicians_group)
                .count()
                > 1
            {
                break;
            }

            let distance = ray.dir.norm();

            eprint!("distance = {}\n", distance);

            let player_instrument = problem.musicians[i];
            eprint!("player_instrument = {}\n", player_instrument);
            let attendee_instrument_preference = attendee.tastes[player_instrument as usize];
            eprint!(
                "attendee_instrument_preference = {}\n",
                attendee_instrument_preference
            );
            let player_score =
                ((attendee_instrument_preference * 1000000.0) / (distance * distance)).ceil();
            // -5394855
            // -7530993
            // eprint!("player_score = {}\n", player_score);
            score += player_score;
        }
    }

    score
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let problem: Problem = serde_json::from_str(&buffer).expect("Failed to parse JSON");

    let players = trivial_solver(&problem);

    let solution = Solution {
        placements: players,
    };

    let output = serde_json::to_string(&solution).expect("Failed to generate JSON");

    io::stdout().write_all(output.as_bytes())?;

    eprintln!("");
    eprintln!("Score: {}", scorer(problem, solution));
    eprintln!("");

    Ok(())
}
