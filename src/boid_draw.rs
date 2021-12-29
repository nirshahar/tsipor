use bevy::prelude::*;
use bevy_canvas::{common_shapes::*, Canvas, DrawMode, StrokeOptions};

use crate::boid_behaviour::Boid;

pub struct BoidDrawPlugin;

impl Plugin for BoidDrawPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system_to_stage(CoreStage::PostUpdate, draw_boids.system());
    }
}

fn update_boid_sprite_pos(sprite: &mut Polygon, boid: &Boid, pos: Vec2) {
    let points = &mut sprite.points;
    points.clear();

    points.push(pos - Vec2::new(boid.draw_base_size / 2.0, boid.draw_height_size / 2.0));
    points.push(pos - Vec2::new(-boid.draw_base_size / 2.0, boid.draw_height_size / 2.0));
    points.push(pos - Vec2::new(0.0, -boid.draw_height_size / 2.0));
}

fn draw_boids(mut canvas: ResMut<Canvas>, mut boids: Query<(&Boid, &Transform, &mut Polygon)>) {
    for (boid, pos, mut boid_sprite) in boids.iter_mut() {
        let pos = pos.translation.truncate();

        update_boid_sprite_pos(&mut boid_sprite, boid, pos);

        canvas.draw(
            &*boid_sprite,
            DrawMode::Stroke(StrokeOptions::DEFAULT.with_line_width(2.0)),
            Color::BLACK,
        );
    }
}
