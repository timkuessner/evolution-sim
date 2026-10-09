use bevy::prelude::*;

use crate::config::{CreatureConfig, FoodConfig, SimSet};
use crate::creature::{Creature, Energy};
use crate::world::WorldBounds;

#[derive(Component)]
pub struct Food;

#[derive(Component)]
pub struct Respawning(Timer);

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FoodConfig>()
            .add_systems(Startup, spawn_food)
            .add_systems(Update, respawn_food.in_set(SimSet::Needs))
            .add_systems(Update, eat_food.in_set(SimSet::Interaction));
    }
}

fn spawn_food(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<FoodConfig>,
    bounds: Res<WorldBounds>,
) {
    let mut rng = rand::rng();
    let mesh = meshes.add(Circle::new(config.radius));
    let material = materials.add(Color::srgb(1.0, 0.7, 0.2));

    for _ in 0..config.count {
        let pos = bounds.random_position(&mut rng, config.radius);
        commands.spawn((
            Food,
            Mesh2d(mesh.clone()),
            MeshMaterial2d(material.clone()),
            Transform::from_translation(pos.extend(0.0)),
        ));
    }
}

fn eat_food(
    mut commands: Commands,
    mut creatures: Query<(&Transform, &mut Energy), With<Creature>>,
    foods: Query<(Entity, &Transform), (With<Food>, Without<Respawning>)>,
    food_cfg: Res<FoodConfig>,
    creature_cfg: Res<CreatureConfig>,
    mut eaten: Local<Vec<Entity>>,
) {
    eaten.clear();
    let reach = creature_cfg.radius + food_cfg.radius;

    for (creature_transform, mut energy) in &mut creatures {
        let creature_pos = creature_transform.translation.truncate();
        for (food_entity, f_transform) in &foods {
            if eaten.contains(&food_entity) {
                continue;
            }
            if creature_pos.distance(f_transform.translation.truncate()) < reach {
                energy.0 = (energy.0 + food_cfg.energy_value).min(creature_cfg.max_energy);
                eaten.push(food_entity);
                commands.entity(food_entity).insert((
                    Respawning(Timer::from_seconds(food_cfg.respawn_secs, TimerMode::Once)),
                    Visibility::Hidden,
                ));
                break;
            }
        }
    }
}

fn respawn_food(
    mut commands: Commands,
    mut q: Query<(Entity, &mut Respawning, &mut Transform, &mut Visibility), With<Food>>,
    bounds: Res<WorldBounds>,
    config: Res<FoodConfig>,
    time: Res<Time>,
) {
    let mut rng = rand::rng();
    for (entity, mut respawning, mut transform, mut visibility) in &mut q {
        if respawning.0.tick(time.delta()).is_finished() {
            let pos = bounds.random_position(&mut rng, config.radius);
            transform.translation = pos.extend(0.0);
            *visibility = Visibility::Visible;
            commands.entity(entity).remove::<Respawning>();
        }
    }
}