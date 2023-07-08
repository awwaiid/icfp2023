use ncollide2d::pipeline::object::CollisionGroups;
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

    fn build_collision_world() -> CollisionWorld<f64, BodyHandle> {
        let mut world = CollisionWorld::new(0.02);

        let rigid_body = RigidBodyDesc::new().build();
        fn build_collision_world(musicians: &[i64]) -> CollisionWorld<f64, BodyHandle> {
            let mut world = CollisionWorld::new(0.02);

            // A ball / circle with a diameter of 10.0
            let circle_shape = Ball::new(5.0);

            let musiciansGroup = CollisionGroups::new().with_membership(&[0]);

            // Loop over musicans and add them to the world
            for placement in solution.placements {
                world.add(placement, circle_shape.clone(), musiciansGroup, 0, 0);
            }

            // Loop over pillars and add them to the world
            for pillar in problem.pillars {
                let rigid_body = RigidBodyDesc::new()
                    .translation(Vector2::new(pillar.center[0], pillar.center[1]))
                    .build();
                let collider = ColliderDesc::new(ShapeHandle::new(Ball::new(pillar.radius)))
                    .density(1.0)
                    .collision_groups(CollisionGroups::new().with_membership(&[1]))
                    .build(BodyHandle(world.add_rigid_body(rigid_body)));
                world.add_collider(collider);
            }

            world
        }

        world
    }

    let mut world = build_collision_world();

    // loop over attendees and trace a line to each placement if it intersects a pillar, or another placement, then 0, otherwise take the distance of the line
    let mut score = 0.0;

    for attendee in problem.attendees {
        let mut min_distance = f64::MAX;
        for placement in solution.placements {
            let ray = Ray::new(
                Point2::new(attendee.x, attendee.y),
                Vector2::new(placement.x - attendee.x, placement.y - attendee.y),
            );
            let mut interferences = Vec::new();
            world.interferences_with_ray(&ray, &mut interferences);
            for interference in interferences {
                match interference {
                    RayIntersection::Feature(id) => {
                        if id == 1 {
                            // pillar
                            min_distance = 0.0;
                        }
                    }
                    RayIntersection::Collider(id) => {
                        if id == 0 {
                            // placement
                            min_distance = 0.0;
                        }
                    }
                }
            }
            if min_distance != 0.0 {
                min_distance = min_distance.min(ray.dir.norm());
            }
        }
        score += min_distance;
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

    eprintln!("");
    eprintln!("Score: {}", scorer(&problem, &solution));
    eprintln!("");

    io::stdout().write_all(output.as_bytes())?;
    Ok(())
}
