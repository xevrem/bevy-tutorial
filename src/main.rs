use bevy::{app::AppExit, prelude::*, render::camera::ScalingMode, winit::WinitSettings};

mod ascii;
mod audio;
mod combat;
mod debug;
mod fadeout;
mod graphics;
mod player;
mod start_menu;
mod tilemap;
mod npc;

use ascii::AsciiPlugin;
use audio::GameAudioPlugin;
use combat::CombatPlugin;
use debug::DebugPlugin;
use fadeout::FadeoutPlugin;
use graphics::GraphicsPlugin;
use player::PlayerPlugin;
use start_menu::MainMenuPlugin;
use tilemap::TileMapPlugin;
use npc::NpcPlugin;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    StartMenu,
    Overworld,
    Combat,
}

fn main() {
    let height = 900.0;

    App::new()
        .add_state(GameState::StartMenu)
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height,
            title: "Bevy Tutorial".to_string(),
            present_mode: bevy::window::PresentMode::Fifo,
            resizable: false,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_plugin(AsciiPlugin)
        .add_plugin(CombatPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(FadeoutPlugin)
        .add_plugin(GameAudioPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(TileMapPlugin)
        .add_plugin(GraphicsPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(NpcPlugin)
        .add_system(check_for_exit)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;
    camera.orthographic_projection.right = 1.0 * RESOLUTION;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

fn check_for_exit(
    keyboard: Res<Input<KeyCode>>,
    mut events: EventWriter<AppExit>,
) {
    if keyboard.pressed(KeyCode::LControl) && keyboard.pressed(KeyCode::Q) {
        events.send(AppExit);
    }
}
