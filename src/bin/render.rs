use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use paisleys_paradox::icfp::Problem;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(print_ball_altitude)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle {
        transform: Transform::from_scale(Vec3::new(15.0, 15.0, 1.0))
            .mul_transform(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))),
        ..Default::default()
    });
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    // "room_width": 4200.0,
    // "room_height": 6234.0,
    // "stage_width": 198.0,
    // "stage_height": 909.0,
    // "stage_bottom_left": [
    //   1076.0,
    //   2395.0
    // ],

    let problem = Problem {
        room_width: 4200.0,
        room_height: 6234.0,
        stage_width: 198.0,
        stage_height: 909.0,
        stage_bottom_left: [1076.0, 2395.0].to_vec(),
        musicians: vec![],
        attendees: vec![],
        pillars: vec![],
    };
    let bottom = Collider::cuboid(0.1, problem.stage_width);
    let top = Collider::cuboid(0.1, problem.stage_height);
    let left = Collider::cuboid(problem.stage_height, 0.1);
    let right = Collider::cuboid(problem.stage_height, 0.1);

    commands
        .spawn(RigidBody::Fixed)
        .insert(bottom)
        .insert(GlobalTransform::from(Transform::from_xyz(
            problem.stage_bottom_left[0],
            problem.stage_bottom_left[1],
            0.0,
        )));

    // // Rectangle
    // commands.spawn(SpriteBundle {
    //     sprite: Sprite {
    //         color: Color::rgb(0.25, 0.25, 0.75),
    //         custom_size: Some(Vec2::new(problem.stage_width, 5.0)),
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(
    //         problem.stage_bottom_left[0],
    //         problem.stage_bottom_left[1],
    //         0.0,
    //     ),
    //     ..default()
    // });

    commands
        .spawn(RigidBody::Fixed)
        .insert(top)
        .insert(GlobalTransform::from(Transform::from_xyz(
            problem.stage_bottom_left[0],
            problem.stage_bottom_left[1] + problem.stage_height,
            0.0,
        )));

    commands
        .spawn(RigidBody::Fixed)
        .insert(left)
        .insert(GlobalTransform::from(Transform::from_xyz(
            problem.stage_bottom_left[0],
            problem.stage_bottom_left[1],
            0.0,
        )));

    commands
        .spawn(RigidBody::Fixed)
        .insert(right)
        .insert(GlobalTransform::from(Transform::from_xyz(
            problem.stage_bottom_left[0] + problem.stage_width,
            problem.stage_bottom_left[1],
            0.0,
        )));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(GlobalTransform::from(Transform::from_xyz(
            problem.stage_bottom_left[0] + problem.stage_width / 2.0,
            problem.stage_bottom_left[1] + problem.stage_height / 2.0,
            0.0,
        )));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}
