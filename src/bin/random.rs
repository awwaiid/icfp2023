use rapier2d::prelude::*;

use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};

// A lovely little d! macro
macro_rules! d {
    ($($arg:tt)*) => (eprintln!($($arg)*));
    // ($($arg:tt)*) => {};
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Problem {
    room_width: f32,
    room_height: f32,
    stage_width: f32,
    stage_height: f32,
    stage_bottom_left: Vec<f32>,
    musicians: Vec<i64>,
    attendees: Vec<Attendee>,
    pillars: Vec<Pillar>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Attendee {
    x: f32,
    y: f32,
    tastes: Vec<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Pillar {
    center: Vec<f32>,
    radius: f32,
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

    d!("Trying to lay out stuff");
    let mut y = stage_y_min + 10.0;
    'iter: while y <= stage_y_max - 10.0 {
        let mut x = stage_x_min + 10.0;
        while x <= stage_x_max - 10.0 {
            if players.len() >= problem.musicians.len() {
                break 'iter;
            }
            players.push(Position {
                x: x as f32,
                y: y as f32,
            });
            x = x + 10.0;
        }
        y = y + 10.0;
    }

    players
}

fn add_musician_to_physics(rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet, x: f32, y: f32, radius: f32) {
    let musician_body = RigidBodyBuilder::fixed() // or ::dynamic()
        .translation(vector![x, y]) // Initial location
        .build();
    let musician_body_handle = rigid_body_set.insert(musician_body);
    let musician_collider = ColliderBuilder::ball(radius).restitution(0.0).build();
    collider_set.insert_with_parent(musician_collider, musician_body_handle, rigid_body_set);
}

fn scorer(problem: Problem, solution: Solution) -> f32 {
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

    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    // Loop over musicans and add them to the world
    // for (i, player) in solution.placements.clone().iter().enumerate() {
    for player in &solution.placements {
        add_musician_to_physics(&mut rigid_body_set, &mut collider_set, player.x, player.y, 5.0);
    }

    // let pillar_group = CollisionGroups::new().with_membership(&[2]);
    // Loop over pillars and add them to the world
    for pillar in problem.pillars {
        add_musician_to_physics(&mut rigid_body_set, &mut collider_set, pillar.center[0], pillar.center[1], pillar.radius);
    }

    // Set up the query
    let mut query_pipeline = QueryPipeline::new();
    query_pipeline.update(&rigid_body_set, &collider_set);

    // loop over attendees and trace a line to each placement if it intersects a pillar, or another placement, then 0, otherwise take the distance of the line
    let mut score = 0.0;

    for attendee in problem.attendees {
        d!("Working on attendee");
        for (i, player) in solution.placements.clone().iter().enumerate() {
            d!("Working on musician {} at ({},{})", i, player.x, player.y);

            let player_vector = vector![player.x - attendee.x, player.y - attendee.y];
            let ray = Ray::new(point![attendee.x, attendee.y], player_vector);
            let max_toi = 1.0; // Since we're going at full speed, we just have 1 step
            d!("Max TOI: {}", max_toi);
            let solid = true;
            let filter = QueryFilter::default();

            let mut collision_count = 0;
            query_pipeline.intersections_with_ray(
                &rigid_body_set,
                &collider_set,
                &ray,
                max_toi,
                solid,
                filter,
                |handle, intersection| {
                    // Callback called on each collider hit by the ray.
                    let hit_point = ray.point_at(intersection.toi);
                    let hit_normal = intersection.normal;
                    d!(
                        "Collider {:?} hit at point {} with normal {}",
                        handle, hit_point, hit_normal
                    );
                    collision_count += 1;
                    true // Return `false` instead if we want to stop searching for other hits.
                },
            );

            d!("Collision count: {}", collision_count);

            if collision_count > 1 {
                d!("Collided with a second musician!");
            } else {

                let distance = ray.dir.norm();

                d!("distance = {}\n", distance);

                let player_instrument = problem.musicians[i];
                d!("player_instrument = {}\n", player_instrument);
                let attendee_instrument_preference = attendee.tastes[player_instrument as usize];
                d!(
                    "attendee_instrument_preference = {}\n",
                    attendee_instrument_preference
                );
                let player_score =
                    ((attendee_instrument_preference * 1000000.0) / (distance * distance)).ceil();
                // -5394855
                // -7530993
                d!("player_score = {}\n", player_score);
                score += player_score;
            }
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

    d!("");
    d!("Score: {}", scorer(problem, solution));
    d!("");

    Ok(())
}
