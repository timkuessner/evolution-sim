use bevy::prelude::*;
use rand::RngExt;

const WORLD_WIDTH: f32 = 800.0;
const WORLD_HEIGHT: f32 = 500.0;
const CREATURE_RADIUS: f32 = 10.0;
const FOOD_RADIUS: f32 = 5.0;

#[derive(Component)]
struct Creature;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Energy(f32);

#[derive(Component)]
struct Food {
    eaten: bool,
}

#[derive(Component)]
struct FoodRespawnTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_creature,
                bounce_creatures,
                drain_energy,
                eat_food,
                respawn_food,
                remove_dead_creatures,
            )
                .chain(),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let mut rng = rand::rng();

    let max_x = WORLD_WIDTH / 2.0 - CREATURE_RADIUS;
    let max_y = WORLD_HEIGHT / 2.0 - CREATURE_RADIUS;

    let creature_circle = meshes.add(Circle::new(CREATURE_RADIUS));
    let creature_material = materials.add(Color::srgb(1.0, 0.0, 1.0));

    for _ in 0..20 {
        let x = rng.random_range(-max_x..max_x);
        let y = rng.random_range(-max_y..max_y);

        let vx = rng.random_range(-100.0..100.0);
        let vy = rng.random_range(-100.0..100.0);

        commands.spawn((
            Creature,
            Velocity(Vec2::new(vx, vy)),
            Energy(100.0),
            Mesh2d(creature_circle.clone()),
            MeshMaterial2d(creature_material.clone()),
            Transform::from_xyz(x, y, 0.0),
        ));
    }

    let food_mesh = meshes.add(Circle::new(FOOD_RADIUS));
    let food_material = materials.add(Color::srgb(1.0, 0.7, 0.2));

    for _ in 0..30 {
        let x = rng.random_range(-max_x..max_x);
        let y = rng.random_range(-max_y..max_y);

        commands.spawn((
            Food { eaten: false },
            Mesh2d(food_mesh.clone()),
            MeshMaterial2d(food_material.clone()),
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

fn drain_energy(mut creatures: Query<&mut Energy, With<Creature>>, time: Res<Time>) {
    for mut energy in &mut creatures {
        energy.0 -= 5.0 * time.delta_secs();
    }
}

fn eat_food(
    mut commands: Commands,
    mut creatures: Query<(&Transform, &mut Energy), With<Creature>>,
    mut foods: Query<(Entity, &Transform, &mut Food), Without<FoodRespawnTimer>>,
) {
    for (creature_transform, mut energy) in &mut creatures {
        for (food_entity, food_transform, mut food) in &mut foods {
            if food.eaten {
                continue;
            }

            let distance = creature_transform
                .translation
                .truncate()
                .distance(food_transform.translation.truncate());

            if distance < CREATURE_RADIUS + FOOD_RADIUS {
                food.eaten = true;
                energy.0 = (energy.0 + 30.0).min(100.0);
                commands.entity(food_entity).insert((
                    FoodRespawnTimer(Timer::from_seconds(5.0, TimerMode::Once)),
                    Visibility::Hidden,
                ));
                break;
            }
        }
    }
}

fn respawn_food(
    mut commands: Commands,
    mut foods: Query<(
        Entity,
        &mut Food,
        &mut FoodRespawnTimer,
        &mut Transform,
        &mut Visibility,
    )>,
    time: Res<Time>,
) {
    let mut rng = rand::rng();

    let max_x = WORLD_WIDTH / 2.0 - CREATURE_RADIUS;
    let max_y = WORLD_HEIGHT / 2.0 - CREATURE_RADIUS;

    for (entity, mut food, mut timer, mut transform, mut visibility) in &mut foods {
        timer.0.tick(time.delta());

        if timer.0.is_finished() {
            transform.translation.x =
                rng.random_range(-max_x..max_x);
            transform.translation.y =
                rng.random_range(-max_y..max_y);

            food.eaten = false;
            *visibility = Visibility::Visible;

            commands.entity(entity).remove::<FoodRespawnTimer>();
        }
    }
}

fn remove_dead_creatures(
    mut commands: Commands,
    creatures: Query<(Entity, &Energy), With<Creature>>,
) {
    for (entity, energy) in &creatures {
        if energy.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
