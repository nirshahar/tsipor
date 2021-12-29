use bevy::prelude::*;

use crate::boid::Boid;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_assets.system())
            .add_system(spawn_boid.system());
    }
}

struct BoidTexture {
    texture: Handle<TextureAtlas>,
}

fn setup_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_assets: ResMut<Assets<TextureAtlas>>,
) {
    let handle: Handle<Texture> = asset_server.load("Untitled.png");
    let boid_texture =
        TextureAtlas::from_grid(handle, Vec2::new(21.0 /*138.0*/, 35.0 /*152.0*/), 1, 1);
    let boid_texture = texture_atlas_assets.add(boid_texture);

    commands.insert_resource(BoidTexture {
        texture: boid_texture,
    });

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_boid(
    mut commands: Commands,
    boid_texture: Res<BoidTexture>,
    window: Res<Windows>,
    btn: Res<Input<MouseButton>>,
) {
    if btn.just_pressed(MouseButton::Left) {
        let win = window.get_primary().expect("primary window doesn't exist");
        let pos = win.cursor_position();

        if let Some(pos) = pos {
            let transform =
                Transform::from_xyz(pos.x - win.width() / 2.0, pos.y - win.height() / 2.0, 0.0);

            commands
                .spawn()
                .insert_bundle(SpriteSheetBundle {
                    texture_atlas: boid_texture.texture.clone(),
                    transform: transform,
                    sprite: TextureAtlasSprite::new(0),
                    ..Default::default()
                })
                .insert(Boid::new());
        }
    }
}
