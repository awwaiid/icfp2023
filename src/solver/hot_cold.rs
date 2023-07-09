use crate::icfp::*;
use crate::scorer::*;
use rapier2d::prelude::*;

fn solve_once(problem: &Problem) -> Solution {
    let random_start = crate::solver::random::solve(&problem);

    let (mut rigid_body_set, mut collider_set, players) = setup_bodies(&random_start, &problem);

    // Set up the query
    // let mut qp = QueryPipeline::new();

    /* Create other structures necessary for the simulation. */
    let gravity = vector![10.0, 0.0];
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();

    let mut best_solution = random_start;
    let mut best_score = scorer(&problem, &best_solution);

    /* Run the game loop, stepping the simulation once per frame. */
    for i in 0..20000 {
        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            None,
            &physics_hooks,
            &event_handler,
        );

        // Build a new solution object

        let mut new_solution = Solution {
            placements: Vec::new(),
        };

        // At each step perturb the positions of the players looking for a vector that improves the players score the most

        // We need a
        // player scorer.
        // apply a force to each player in the direction of the best score.

        for (i, player) in players.iter().enumerate() {
            let body: &mut RigidBody = rigid_body_set.get_mut(*player).unwrap();
            let pos = body.position().translation.vector;
            let new_player_placement = Position { x: pos.x, y: pos.y };
            new_solution.placements.push(new_player_placement);

            let ps = player_scorer(&problem, &new_solution, &new_player_placement, i);

            // FIXME
            // loop over 360 degrees in 10 degree increments
            // for each angle, apply a force in that direction
            // measure the score
            // if the score is better, keep the force
            // if the score is worse, try the next angle
            // if the score is the same, try the next angle
            // if we've tried all angles, keep the best force

            let mut best_score = ps;
            let mut force = vector![0.0, 0.0];

            for angle in 0..360 {
                let angle_radians = angle as f32 * std::f32::consts::PI / 180.0;
                force = vector![angle_radians.cos(), angle_radians.sin()];
                // eprintln!("force {}", force);
                // compute the new position of the player outside of the physics engine
                let new_player_placement = Position {
                    x: new_player_placement.x + force.x,
                    y: new_player_placement.y + force.y,
                };

                let new_ps = player_scorer(&problem, &new_solution, &new_player_placement, i);
                if new_ps > best_score {
                    best_score = new_ps;
                }
            }

            body.add_force(force, true);
        }

        // Computing step

        let score = scorer(&problem, &new_solution);
        eprintln!("step: {} : {} ", i, score);
        if score > best_score {
            // eprintln!("  NEW WINNER");
            best_score = score;
            best_solution = new_solution;
        }

        // eprintln!("score {}", score);
    }

    best_solution
}

fn setup_bodies(
    solution: &Solution,
    problem: &Problem,
) -> (RigidBodySet, ColliderSet, Vec<RigidBodyHandle>) {
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    // build the walls of the stage
    let bottom = ColliderBuilder::cuboid(0.1, problem.stage_width).build();
    let top = ColliderBuilder::cuboid(0.1, problem.stage_height).build();
    let left = ColliderBuilder::cuboid(problem.stage_height, 0.1).build();
    let right = ColliderBuilder::cuboid(problem.stage_height, 0.1).build();

    let bottom_left = RigidBodyBuilder::fixed()
        .translation(vector![
            problem.stage_bottom_left[0],
            problem.stage_bottom_left[1]
        ])
        .build();

    let bottom_left_handle = rigid_body_set.insert(bottom_left);

    let top_left = RigidBodyBuilder::fixed()
        .translation(vector![
            problem.stage_bottom_left[0],
            problem.stage_bottom_left[1] + problem.stage_height
        ])
        .build();

    let top_left_handle = rigid_body_set.insert(top_left);

    let bottom_right = RigidBodyBuilder::fixed()
        .translation(vector![
            problem.stage_bottom_left[0] + problem.stage_width,
            problem.stage_bottom_left[1]
        ])
        .build();

    let bottom_right_handle = rigid_body_set.insert(bottom_right);

    // Add the walls to the world
    collider_set.insert_with_parent(top, top_left_handle, &mut rigid_body_set);
    collider_set.insert_with_parent(bottom, bottom_left_handle, &mut rigid_body_set);
    collider_set.insert_with_parent(left, bottom_right_handle, &mut rigid_body_set);
    collider_set.insert_with_parent(right, bottom_left_handle, &mut rigid_body_set);

    let mut musician_handles: Vec<RigidBodyHandle> = Vec::new();
    // Loop over musicans and add them to the world
    for (i, player) in solution.placements.iter().enumerate() {
        let handle = crate::scorer::add_musician_to_physics(
            &mut rigid_body_set,
            &mut collider_set,
            player.x,
            player.y,
            5.0,
            i as u128,
        );
        musician_handles.push(handle);
    }

    // Loop over pillars and add them to the world
    for pillar in &problem.pillars {
        crate::scorer::add_musician_to_physics(
            &mut rigid_body_set,
            &mut collider_set,
            pillar.center[0],
            pillar.center[1],
            pillar.radius,
            u128::MAX,
        );
    }

    (rigid_body_set, collider_set, musician_handles)
}

pub fn solve(problem: &Problem) -> Solution {
    let n = 1;
    let mut best_score = 0.0;
    let mut best_solution = Solution { placements: vec![] };
    for attempt in 0..n {
        eprint!("Attempt {}... ", attempt);
        let solution = solve_once(problem);
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
