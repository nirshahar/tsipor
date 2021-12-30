use std::f32::consts::*;

use bevy::prelude::*;
use bevy_canvas::{common_shapes::*, Canvas, DrawMode, StrokeOptions};

use crate::boid_behaviour::Boid;

pub struct BoidDrawPlugin;

impl Plugin for BoidDrawPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system_to_stage(CoreStage::PostUpdate, draw_boids.system());
    }
}

fn update_boid_sprite_pos(sprite: &mut Polygon, boid: &Boid, pos: Vec3) {
    let points = &mut sprite.points;
    let direction = boid.get_vel().normalize_or_zero();

    let rot = Quat::from_rotation_z(direction.y.atan2(direction.x) - FRAC_PI_2);

    let off_left_bottom = Vec3::new(boid.draw_base_size / 2.0, boid.draw_height_size / 2.0, 0.0);
    let off_right_bottom = Vec3::new(-boid.draw_base_size / 2.0, boid.draw_height_size / 2.0, 0.0);
    let off_top = Vec3::new(0.0, -boid.draw_height_size / 2.0, 0.0);

    points.clear();

    points.push((pos - rot * off_left_bottom).truncate());
    points.push((pos - rot * off_right_bottom).truncate());
    points.push((pos - rot * off_top).truncate());
}

fn draw_boids(mut canvas: ResMut<Canvas>, mut boids: Query<(&Boid, &Transform, &mut Polygon)>) {
    for (boid, pos, mut boid_sprite) in boids.iter_mut() {
        let pos = pos.translation;

        update_boid_sprite_pos(&mut boid_sprite, boid, pos);

        canvas.draw(
            &*boid_sprite,
            DrawMode::Stroke(StrokeOptions::DEFAULT.with_line_width(2.0)),
            Color::BLACK,
        );
    }
}
