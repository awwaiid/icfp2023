use ncollide2d::query::{Ray, RayCast};
use nphysics2d::object::{BodyHandle, ColliderHandle};
use nphysics2d::world::CollisionWorld;

fn ray_intersects_world(world: &CollisionWorld<f64, BodyHandle>, placement: &Position) -> bool {
    let stage_bottom_left = world
        .colliders()
        .find(|&(_, collider)| collider.user_data() == Some(&"stage".to_string()))
        .map(|(handle, _)| {
            let collider = world.collider(*handle).unwrap();
            let position = collider.position();
            let shape = collider.shape();
            let half_extents = shape.half_extents();
            let bottom_left = position.translation.vector - half_extents;
            (bottom_left.x, bottom_left.y)
        })
        .unwrap();

    let stage_top_right = (
        stage_bottom_left.0
            + world
                .colliders()
                .find(|&(_, collider)| collider.user_data() == Some(&"stage".to_string()))
                .map(|(handle, _)| {
                    let collider = world.collider(*handle).unwrap();
                    let shape = collider.shape();
                    let half_extents = shape.half_extents();
                    2.0 * half_extents.x
                })
                .unwrap(),
        stage_bottom_left.1
            + world
                .colliders()
                .find(|&(_, collider)| collider.user_data() == Some(&"stage".to_string()))
                .map(|(handle, _)| {
                    let collider = world.collider(*handle).unwrap();
                    let shape = collider.shape();
                    let half_extents = shape.half_extents();
                    2.0 * half_extents.y
                })
                .unwrap(),
    );

    let ray = Ray::new(
        [placement.x, placement.y].into(),
        [
            (stage_bottom_left.0 + stage_top_right.0) / 2.0,
            stage_top_right.1 + 1.0,
        ]
        .into(),
    );

    let mut closest_intersection = f64::INFINITY;
    let mut intersects = false;

    for (_, collider) in world.colliders() {
        if let Some(pillar) = collider
            .user_data()
            .and_then(|data| data.parse::<usize>().ok())
        {
            if let Some(intersection) =
                collider
                    .shape()
                    .toi_and_normal_with_ray(&collider.position(), &ray, true)
            {
                if intersection.0 < closest_intersection {
                    closest_intersection = intersection.0;
                    intersects = true;
                }
            }
        }
    }

    intersects
}
