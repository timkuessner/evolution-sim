use bevy::prelude::*;
use rand::RngExt;

#[derive(Resource)]
pub struct WorldBounds {
    pub half_size: Vec2,
}

impl Default for WorldBounds {
    fn default() -> Self {
        Self {
            half_size: Vec2::new(400.0, 250.0),
        }
    }
}

impl WorldBounds {
    pub fn random_position(&self, rng: &mut impl rand::Rng, margin: f32) -> Vec2 {
        let max = self.half_size - Vec2::splat(margin);
        Vec2::new(
            rng.random_range(-max.x..max.x),
            rng.random_range(-max.y..max.y),
        )
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldBounds>()
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
