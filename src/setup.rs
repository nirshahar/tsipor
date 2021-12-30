use bevy::prelude::*;
use bevy_canvas::common_shapes::Polygon;

use crate::boid_behaviour::Boid;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_camera.system())
            .add_system(spawn_boid.system());
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_boid(mut commands: Commands, window: Res<Windows>, btn: Res<Input<MouseButton>>) {
    if btn.just_pressed(MouseButton::Left) {
        let win = window.get_primary().expect("primary window doesn't exist");
        let pos = win.cursor_position();

        if let Some(pos) = pos {
            let transform =
                Transform::from_xyz(pos.x - win.width() / 2.0, pos.y - win.height() / 2.0, 0.0);

            let boid = Boid::new();

            let mut drawing_points = Vec::new();
            drawing_points
                .push(pos - Vec2::new(boid.draw_base_size / 2.0, boid.draw_height_size / 2.0));
            drawing_points
                .push(pos - Vec2::new(-boid.draw_base_size / 2.0, boid.draw_height_size / 2.0));
            drawing_points.push(pos - Vec2::new(0.0, -boid.draw_height_size / 2.0));

            let boid_sprite = Polygon {
                origin: Vec2::ZERO,
                points: drawing_points,
                closed: true,
            };
            commands
                .spawn()
                .insert(transform)
                .insert(boid_sprite)
                .insert(boid);
        }
    }
}
