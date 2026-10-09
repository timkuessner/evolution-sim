use bevy::prelude::*;
use rand::RngExt;

use crate::config::{CreatureConfig, SimSet};
use crate::world::WorldBounds;

#[derive(Component)]
#[require(Velocity, Energy)]
pub struct Creature;

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Energy(pub f32);
impl Default for Energy {
    fn default() -> Self {
        Self(100.0)
    }
}

pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CreatureConfig>()
            .add_systems(Startup, spawn_creatures)
            .add_systems(
                Update,
                (move_creatures, bounce_creatures)
                    .chain()
                    .in_set(SimSet::Movement),
            )
            .add_systems(Update, drain_energy.in_set(SimSet::Needs))
            .add_systems(Update, remove_dead_creatures.in_set(SimSet::Cleanup));
    }
}

fn spawn_creatures(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<CreatureConfig>,
    bounds: Res<WorldBounds>,
) {
    let mut rng = rand::rng();
    let mesh = meshes.add(Circle::new(config.radius));
    let material = materials.add(Color::srgb(1.0, 0.0, 1.0));

    for _ in 0..config.count {
        let pos = bounds.random_position(&mut rng, config.radius);
        let speed = config.max_start_speed;
        let vel = Vec2::new(
            rng.random_range(-speed..speed),
            rng.random_range(-speed..speed),
        );

        commands.spawn((
            Creature,
            Velocity(vel),
            Energy(config.max_energy),
            Mesh2d(mesh.clone()),
            MeshMaterial2d(material.clone()),
            Transform::from_translation(pos.extend(0.0)),
        ));
    }
}

fn move_creatures(mut q: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut t, v) in &mut q {
        t.translation += v.0.extend(0.0) * time.delta_secs();
    }
}

fn bounce_creatures(
    mut q: Query<(&mut Transform, &mut Velocity), With<Creature>>,
    bounds: Res<WorldBounds>,
    config: Res<CreatureConfig>,
) {
    let max = bounds.half_size - Vec2::splat(config.radius);
    for (mut t, mut v) in &mut q {
        let p = &mut t.translation;
        if p.x.abs() > max.x {
            p.x = p.x.clamp(-max.x, max.x);
            v.0.x = -v.0.x.abs() * p.x.signum();
        }
        if p.y.abs() > max.y {
            p.y = p.y.clamp(-max.y, max.y);
            v.0.y = -v.0.y.abs() * p.y.signum();
        }
    }
}

fn drain_energy(mut q: Query<&mut Energy>, config: Res<CreatureConfig>, time: Res<Time>) {
    for mut e in &mut q {
        e.0 -= config.energy_drain * time.delta_secs();
    }
}

fn remove_dead_creatures(mut commands: Commands, q: Query<(Entity, &Energy), With<Creature>>) {
    for (entity, energy) in &q {
        if energy.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
