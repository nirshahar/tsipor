use bevy::prelude::*;
use bevy_canvas;

pub mod boid_behaviour;
pub mod boid_draw;
pub mod setup;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_canvas::CanvasPlugin)
        .add_plugin(setup::SetupPlugin)
        .add_plugin(boid_behaviour::BoidPlugin)
        .add_plugin(boid_draw::BoidDrawPlugin)
        .run();
}
