use bevy::prelude::*;

pub mod boid;
pub mod setup;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(setup::SetupPlugin)
        .add_plugin(boid::BoidPlugin)
        .run();
}
