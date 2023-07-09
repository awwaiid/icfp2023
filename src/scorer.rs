use rapier2d::prelude::*;
// mod icfp;
use crate::icfp::*;

pub(crate) fn add_musician_to_physics(
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
    x: f32,
    y: f32,
    radius: f32,
    index: u128,
) -> RigidBodyHandle {
    let musician_body = RigidBodyBuilder::dynamic() // or ::dynamic()
        .translation(vector![x, y]) // Initial location
        .user_data(index)
        .build();

    let musician_body_handle: RigidBodyHandle = rigid_body_set.insert(musician_body);
    let musician_collider = ColliderBuilder::ball(radius).restitution(0.7).build();

    collider_set.insert_with_parent(musician_collider, musician_body_handle, rigid_body_set);

    musician_body_handle
}

pub fn player_scorer(
    problem: &Problem,
    solution: &Solution,
    player: &Position,
    player_index: usize,
) -> f32 {
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    // Loop over musicans and add them to the world
    for player in &solution.placements {
        add_musician_to_physics(
            &mut rigid_body_set,
            &mut collider_set,
            player.x,
            player.y,
            5.0,
            0,
        );
    }

    // Loop over pillars and add them to the world
    for pillar in &problem.pillars {
        add_musician_to_physics(
            &mut rigid_body_set,
            &mut collider_set,
            pillar.center[0],
            pillar.center[1],
            pillar.radius,
            0,
        );
    }

    // Set up the query
    let mut query_pipeline = QueryPipeline::new();
    query_pipeline.update(&rigid_body_set, &collider_set);

    // loop over attendees and trace a line to each placement if it intersects a pillar, or another placement, then 0, otherwise take the distance of the line
    let mut score = 0.0;

    for attendee in &problem.attendees {
        // eprintln!("Working on attendee");

        // eprintln!("Working on musician {} at ({},{})", i, player.x, player.y);

        let player_vector = vector![player.x - attendee.x, player.y - attendee.y];
        let ray = Ray::new(point![attendee.x, attendee.y], player_vector);
        let max_toi = 1.0; // Since we're going at full speed, we just have 1 step
                           // eprintln!("Max TOI: {}", max_toi);
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
            |_handle, _intersection| {
                // Callback called on each collider hit by the ray.
                // let hit_point = ray.point_at(intersection.toi);
                // let hit_normal = intersection.normal;
                // eprintln!(
                //     "Collider {:?} hit at point {} with normal {}",
                //     handle,
                //     hit_point,
                //     hit_normal
                // );
                collision_count += 1;
                true // Return `false` instead if we want to stop searching for other hits.
            },
        );

        // eprintln!("Collision count: {}", collision_count);

        if collision_count > 1 {
            // eprintln!("Collided with a second musician!");
        } else {
            let distance = ray.dir.norm();

            // eprintln!("distance = {}\n", distance);

            let player_instrument = problem.musicians[player_index];
            // eprintln!("player_instrument = {}\n", player_instrument);
            let attendee_instrument_preference = attendee.tastes[player_instrument as usize];
            // eprintln!(
            //     "attendee_instrument_preference = {}\n",
            //     attendee_instrument_preference
            // );
            let player_score =
                ((attendee_instrument_preference * 1000000.0) / (distance * distance)).ceil();
            // -5394855
            // -7530993
            // eprint!("player_score = {}\n", player_score);
            score += player_score;
        }
    }

    let player_physical_presence = 10.0;

    // validate that all players are on the stage with the stage_bottom_left and stage_width/height

    if player.x < problem.stage_bottom_left[0] + player_physical_presence
        || player.x > problem.stage_bottom_left[0] + problem.stage_width - player_physical_presence
        || player.y < problem.stage_bottom_left[1] + player_physical_presence
        || player.y > problem.stage_bottom_left[1] + problem.stage_height - player_physical_presence
    {
        // eprintln!("Player is off the stage!");
        score = 0.0;
    }

    score
}

pub fn scorer(problem: &Problem, solution: &Solution) -> f32 {
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    // Loop over musicans and add them to the world
    for player in &solution.placements {
        add_musician_to_physics(
            &mut rigid_body_set,
            &mut collider_set,
            player.x,
            player.y,
            5.0,
            0,
        );
    }

    // Loop over pillars and add them to the world
    for pillar in &problem.pillars {
        add_musician_to_physics(
            &mut rigid_body_set,
            &mut collider_set,
            pillar.center[0],
            pillar.center[1],
            pillar.radius,
            0,
        );
    }

    // Set up the query
    let mut query_pipeline = QueryPipeline::new();
    query_pipeline.update(&rigid_body_set, &collider_set);

    // loop over attendees and trace a line to each placement if it intersects a pillar, or another placement, then 0, otherwise take the distance of the line
    let mut score = 0.0;

    for attendee in &problem.attendees {
        // eprintln!("Working on attendee");
        for (i, player) in solution.placements.clone().iter().enumerate() {
            // eprintln!("Working on musician {} at ({},{})", i, player.x, player.y);

            let player_vector = vector![player.x - attendee.x, player.y - attendee.y];
            let ray = Ray::new(point![attendee.x, attendee.y], player_vector);
            let max_toi = 1.0; // Since we're going at full speed, we just have 1 step
                               // eprintln!("Max TOI: {}", max_toi);
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
                |_handle, _intersection| {
                    // Callback called on each collider hit by the ray.
                    // let hit_point = ray.point_at(intersection.toi);
                    // let hit_normal = intersection.normal;
                    // eprintln!(
                    //     "Collider {:?} hit at point {} with normal {}",
                    //     handle,
                    //     hit_point,
                    //     hit_normal
                    // );
                    collision_count += 1;
                    true // Return `false` instead if we want to stop searching for other hits.
                },
            );

            // eprintln!("Collision count: {}", collision_count);

            if collision_count > 1 {
                // eprintln!("Collided with a second musician!");
            } else {
                let distance = ray.dir.norm();

                // eprintln!("distance = {}\n", distance);

                let player_instrument = problem.musicians[i];
                // eprintln!("player_instrument = {}\n", player_instrument);
                let attendee_instrument_preference = attendee.tastes[player_instrument as usize];
                // eprintln!(
                //     "attendee_instrument_preference = {}\n",
                //     attendee_instrument_preference
                // );
                let player_score =
                    ((attendee_instrument_preference * 1000000.0) / (distance * distance)).ceil();
                // -5394855
                // -7530993
                // eprint!("player_score = {}\n", player_score);
                score += player_score;
            }
        }
    }

    let player_physical_presence = 10.0;

    // validate that all players are on the stage with the stage_bottom_left and stage_width/height
    for player in &solution.placements {
        if player.x < problem.stage_bottom_left[0] + player_physical_presence
            || player.x
                > problem.stage_bottom_left[0] + problem.stage_width - player_physical_presence
            || player.y < problem.stage_bottom_left[1] + player_physical_presence
            || player.y
                > problem.stage_bottom_left[1] + problem.stage_height - player_physical_presence
        {
            // eprintln!("Player is off the stage!");
            score = 0.0;
        }
    }

    score
}
