use bevy::prelude::*;
use rand::RngExt;

#[derive(Component)]
struct Creature;

#[derive(Component)]
struct Velocity(Vec2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_creature)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let circle = meshes.add(Circle::new(20.0));
    let material = materials.add(Color::srgb(1.0, 0.0, 1.0));

    let mut rng = rand::rng();

    for _ in 0..20 {
        let x = rng.random_range(-400.0..400.0);
        let y = rng.random_range(-250.0..250.0);

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
