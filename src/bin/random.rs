use ncollide2d::world::CollisionWorld;
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
    y: f64,
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

fn scorer(_problem: &Problem, _solution: &Solution) -> f64 {
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

  use nphysics2d::object::{BodyHandle, ColliderDesc, RigidBodyDesc};
  use nphysics2d::world::{CollisionGroups, CollisionWorld};

  fn build_collision_world() -> CollisionWorld<f64, BodyHandle> {
    let mut world = CollisionWorld::new(0.02);

    let rigid_body = RigidBodyDesc::new().build();
fn build_collision_world(musicians: &[i64]) -> CollisionWorld<f64, BodyHandle> {
  let mut world = CollisionWorld::new(0.02);

  // A ball / circle with a diameter of 10.0
  let circle_shape = ShapeHandle::new(Ball::new(5.0));

  // Loop over musicans
  for musician in musicians {
    let rigid_body = RigidBodyDesc::new()
      .translation(Vector2::new(0.0, 0.0))
      .build();
    let collider = ColliderDesc::new(circle_shape.clone())
      .density(1.0)
      .collision_groups(CollisionGroups::new().with_membership(&[0]))
      .build(BodyHandle(world.add_rigid_body(rigid_body)));
    world.add_collider(collider);
  }

  world
}

    world
  }

  let mut world = build_collision_world();


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

    eprintln!("");
    eprintln!("Score: {}", scorer(&problem, &solution));
    eprintln!("");

    io::stdout().write_all(output.as_bytes())?;
    Ok(())
}
