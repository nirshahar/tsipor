use bevy::prelude::*;

pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system_to_stage(CoreStage::PreUpdate, update_boids.system());
        app.add_system(apply_all_boid_behaviours.system());
    }
}

pub struct Boid {
    vel: Vec2,
    accel: Vec2,
    vision: f32,
}

impl Boid {
    pub fn new(vision: f32) -> Self {
        Self {
            vel: Vec2::new(1.0, 0.0),
            accel: Vec2::ZERO,
            vision,
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
    }
}

fn collect_intersections<'a, T>(
    boid: (&Boid, &Transform),
    other_boids: T,
) -> Vec<(&'a Boid, &'a Transform)>
where
    T: Iterator<Item = (&'a Boid, &'a Transform)>,
{
    let (boid, transform) = boid;
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

fn boid_behaviour_cohesion(boids_in_vision: &Vec<(&Boid, &Transform)>) -> Vec3 {
    let mut avg_pos = Vec3::ZERO;

    for (_, boid_pos) in boids_in_vision {
        avg_pos += boid_pos.translation;
    }

    avg_pos / (boids_in_vision.len() as f32)
}

fn boid_behaviour_alignment(boids_in_vision: &Vec<(&Boid, &Transform)>) -> Vec3 {
    let mut avg_direction = Vec3::ZERO;

    for (boid, _) in boids_in_vision {
        avg_direction += boid.vel.extend(0.0);
    }

    avg_direction / (boids_in_vision.len() as f32)
}

fn boid_behaviour_seperation(boids_in_vision: &Vec<(&Boid, &Transform)>) -> Vec3 {
    todo!("compute seperation force");
}

fn apply_steering_force(mut boid: &Boid, force: Vec2) {
    todo!("apply steering to the boid");
}

fn apply_all_boid_behaviours(mut boids: Query<(&Boid, &Transform)>) {
    for boid in boids.iter() {
        let boids_in_vision = collect_intersections((&boid.0, boid.1), boids.iter());

        let cohesion = boid_behaviour_cohesion(&boids_in_vision);
        let alignment = boid_behaviour_alignment(&boids_in_vision);
        //let seperation = boid_behaviour_seperation(&boids_in_vision);

        todo!("use the cohesion, alignment and seperation behaviours to apply steering");
    }
}
