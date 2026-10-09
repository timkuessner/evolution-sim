use bevy::prelude::*;

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

    commands.spawn((
        Creature,
        Velocity(Vec2::new(10.0, 6.0)),
        Mesh2d(circle),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn move_creature(
    mut creatures: Query<(&mut Transform, &Velocity), With<Creature>>,
    time: Res<Time>
) {
    for (mut transform, velocity) in &mut creatures {
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    }
}
