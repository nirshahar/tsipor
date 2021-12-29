use bevy::prelude::*;

use rand::random;

pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system_to_stage(CoreStage::PreUpdate, update_boids.system());
        app.add_system(apply_all_boid_behaviours.system());
    }
}

#[derive(Clone, Copy)]
pub struct Boid {
    vel: Vec2,
    accel: Vec2,
    vision: f32,
    max_steering: f32,
    max_speed: f32,
    seperation_vision: f32,
}

impl Boid {
    pub fn new() -> Self {
        Self {
            vel: (Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5) * 3.0)
                .clamp_length_min(2.0),
            accel: Vec2::ZERO,
            vision: 100.0,
            max_steering: 0.2,
            max_speed: 5.0,
            seperation_vision: 75.0,
        }
    }
}

fn update_boids(mut boids: Query<(&mut Boid, &mut Transform)>, window: Res<Windows>) {
    let win = window.get_primary().expect("no primary window exists");

    for (mut boid, mut transform) in boids.iter_mut() {
        let pos = transform.translation;

        let width = win.width() / 2.0;
        let height = win.height() / 2.0;

        transform.translation = pos + Vec3::new(boid.vel.x, boid.vel.y, 0.0);

        if pos.x > width {
            transform.translation.x = -width;
        } else if pos.x < -width {
            transform.translation.x = width;
        }

        if pos.y > height {
            transform.translation.y = -height;
        } else if pos.y < -height {
            transform.translation.y = height;
        }

        let acceleration = boid.accel;
        boid.vel += acceleration;
        boid.vel = boid.vel.clamp_length_max(boid.max_speed);
    }
}

fn collect_intersections<'a, T>(
    (boid, transform): (&Boid, &Transform),
    other_boids: T,
) -> Vec<(&'a Boid, &'a Transform)>
where
    T: Iterator<Item = (&'a Boid, &'a Transform)>,
{
    let mut collected_boids = Vec::new();

    for (other_boid, other_transform) in other_boids {
        if transform
            .translation
            .distance_squared(other_transform.translation)
            <= boid.vision * boid.vision
        {
            collected_boids.push((other_boid, other_transform));
        }
    }

    collected_boids
}

fn boid_behaviour_alignment(boids_in_vision: &Vec<(&Boid, &Transform)>) -> Vec2 {
    let mut avg_direction = Vec2::ZERO;

    for (boid, _) in boids_in_vision {
        avg_direction += boid.vel.normalize_or_zero();
    }

    avg_direction / (boids_in_vision.len() as f32)
}

fn boid_behaviour_cohesion(
    boid_pos: &Transform,
    boids_in_vision: &Vec<(&Boid, &Transform)>,
) -> Vec2 {
    let mut avg_pos = Vec3::ZERO;

    for (_, boid_pos) in boids_in_vision {
        avg_pos += boid_pos.translation;
    }
    avg_pos /= boids_in_vision.len() as f32;

    (avg_pos - boid_pos.translation).truncate()
}

fn boid_behaviour_seperation(
    (boid, boid_pos): (&Boid, &Transform),
    boids_in_vision: &Vec<(&Boid, &Transform)>,
) -> Vec2 {
    let pos = boid_pos.translation;

    let mut seperation_avg = Vec3::ZERO;
    let mut cnt = 0;

    for (_, other_boid_pos) in boids_in_vision {
        let offset = other_boid_pos.translation - pos;
        let len = offset.length();

        if *other_boid_pos != boid_pos && len < boid.seperation_vision {
            seperation_avg += -boid.vision * offset.normalize_or_zero() / len;
            cnt += 1;
        }
    }
    seperation_avg /= cnt as f32;

    seperation_avg.truncate()
}

fn apply_steering_force(boid: &mut Boid, force: Vec2) {
    boid.accel +=
        (force.normalize_or_zero() * boid.max_speed - boid.vel).clamp_length_max(boid.max_steering)
}

fn apply_all_boid_behaviours(mut boids: Query<(&mut Boid, &Transform)>) {
    let mut boids_behaviours = Vec::new();

    let cloned_boids: Vec<_> = boids.iter_mut().map(|(b, t)| (*b, t)).collect();

    for (boid, pos) in cloned_boids.iter() {
        let boids_in_vision =
            collect_intersections((boid, pos), cloned_boids.iter().map(|(b, t)| (b, *t)));

        if boids_in_vision.len() > 1 {
            let alignment = boid_behaviour_alignment(&boids_in_vision);
            let cohesion = boid_behaviour_cohesion(pos, &boids_in_vision);
            let seperation = boid_behaviour_seperation((boid, pos), &boids_in_vision);

            boids_behaviours.push(Some([alignment, cohesion, seperation]));
        } else {
            boids_behaviours.push(None);
        }
    }

    for ((mut boid, _), behaviours) in boids.iter_mut().zip(boids_behaviours.iter()) {
        if let Some(behaviours) = behaviours {
            boid.accel = Vec2::ZERO;
            for behaviour in behaviours {
                apply_steering_force(&mut boid, *behaviour);
            }
        }
    }
}
