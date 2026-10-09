use bevy::prelude::*;

#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone)]
pub enum SimSet {
    Movement,
    Needs,
    Interaction,
    Cleanup,
}

#[derive(Resource)]
pub struct CreatureConfig {
    pub count: usize,
    pub radius: f32,
    pub max_energy: f32,
    pub energy_drain: f32,
    pub max_start_speed: f32,
}

impl Default for CreatureConfig {
    fn default() -> Self {
        Self {
            count: 20,
            radius: 10.0,
            max_energy: 100.0,
            energy_drain: 5.0,
            max_start_speed: 100.0,
        }
    }
}

#[derive(Resource)]
pub struct FoodConfig {
    pub count: usize,
    pub radius: f32,
    pub energy_value: f32,
    pub respawn_secs: f32,
}

impl Default for FoodConfig {
    fn default() -> Self {
        Self {
            count: 30,
            radius: 5.0,
            energy_value: 30.0,
            respawn_secs: 5.0,
        }
    }
}
