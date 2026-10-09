mod config;
mod creature;
mod food;
mod world;

use bevy::prelude::*;
use config::SimSet;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .configure_sets(
            Update,
            (SimSet::Movement, SimSet::Needs, SimSet::Interaction, SimSet::Cleanup).chain(),
        )
        .add_plugins((world::WorldPlugin, creature::CreaturePlugin, food::FoodPlugin))
        .run();
}