use crate::icfp::*;
use crate::scorer::*;
use rapier2d::prelude::*;

fn solve_once(problem: &Problem) -> Solution {
    let random_start = crate::solver::random::solve(&problem);

    let (mut rigid_body_set, mut collider_set, players) = setup_bodies(&random_start, &problem);

    // Set up the query
    // let mut qp = QueryPipeline::new();

    /* Create other structures necessary for the simulation. */
    let gravity = vector![1.0, -9.81];
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
    for _ in 0..200 {
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
        for player in &players {
            let body = rigid_body_set.get(*player).unwrap();
            let pos = body.position().translation.vector;

            new_solution
                .placements
                .push(Position { x: pos.x, y: pos.y });
        }

        let score = scorer(&problem, &new_solution);

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
    let n = 10;
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
