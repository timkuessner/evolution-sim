use bevy::prelude::*;
use rand::RngExt;

const WORLD_WIDTH: f32 = 800.0;
const WORLD_HEIGHT: f32 = 500.0;
const CREATURE_RADIUS: f32 = 10.0;

#[derive(Component)]
struct Creature;

#[derive(Component)]
struct Velocity(Vec2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_creature, bounce_creatures).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let circle = meshes.add(Circle::new(CREATURE_RADIUS));
    let material = materials.add(Color::srgb(1.0, 0.0, 1.0));

    let mut rng = rand::rng();

    let max_x = WORLD_WIDTH / 2.0 - CREATURE_RADIUS;
    let max_y = WORLD_HEIGHT / 2.0 - CREATURE_RADIUS;

    for _ in 0..20 {
        let x = rng.random_range(-max_x..max_x);
        let y = rng.random_range(-max_y..max_y);

        let vx = rng.random_range(-100.0..100.0);
        let vy = rng.random_range(-100.0..100.0);

        commands.spawn((
            Creature,
            Velocity(Vec2::new(vx, vy)),
            Mesh2d(circle.clone()),
            MeshMaterial2d(material.clone()),
            Transform::from_xyz(x, y, 0.0),
        ));
    }
}

fn move_creature(
    mut creatures: Query<(&mut Transform, &Velocity), With<Creature>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in &mut creatures {
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    }
}

fn bounce_creatures(mut creatures: Query<(&mut Transform, &mut Velocity), With<Creature>>) {
    for (mut transform, mut velocity) in &mut creatures {
        let position = &mut transform.translation;

        let max_x = WORLD_WIDTH / 2.0 - CREATURE_RADIUS;
        let max_y = WORLD_HEIGHT / 2.0 - CREATURE_RADIUS;

        if position.x < -max_x {
            position.x = -max_x;
            velocity.0.x = velocity.0.x.abs();
        } else if position.x > max_x {
            position.x = max_x;
            velocity.0.x = -velocity.0.x.abs();
        }

        if position.y < -max_y {
            position.y = -max_y;
            velocity.0.y = velocity.0.y.abs();
        } else if position.y > max_y {
            position.y = max_y;
            velocity.0.y = -velocity.0.y.abs();
        }
    }
}
