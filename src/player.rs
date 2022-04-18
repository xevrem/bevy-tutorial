use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    tilemap::{EncounterSpawner, TileCollider},
    GameState, TILE_SIZE,
};

pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
    just_moved: bool,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Overworld).with_system(show_player))
            .add_system_set(SystemSet::on_exit(GameState::Overworld).with_system(hide_player))
            .add_system_set(
                SystemSet::on_update(GameState::Overworld)
                    .with_system(player_encounter_checking.after("movement"))
                    .with_system(camera_follow.after("movement"))
                    .with_system(player_movement.label("movement")),
            )
            .add_system_set(SystemSet::on_update(GameState::Combat).with_system(test_exit_combat))
            .add_startup_system(spawn_player);
    }
}

fn test_exit_combat(mut keyboard: ResMut<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        println!("Changing to Overworld");
        state.set(GameState::Overworld).unwrap();
        keyboard.clear();
    }
}

fn show_player(
    mut player_query: Query<&mut Visibility, With<Player>>,
    children_query: Query<&Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    let mut player_vis = player_query.single_mut();
    player_vis.is_visible = true;

    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = true;
            }
        }
    }
}

fn hide_player(
    mut player_query: Query<&mut Visibility, With<Player>>,
    children_query: Query<&Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    let mut player_vis = player_query.single_mut();
    player_vis.is_visible = false;

    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = false;
            }
        }
    }
}

fn player_encounter_checking(
    player_query: Query<(&Player, &Transform)>,
    encounter_query: Query<&Transform, (With<EncounterSpawner>, Without<Player>)>,
    mut state: ResMut<State<GameState>>,
) {
    let (player, player_transform) = player_query.single();
    let player_translation = player_transform.translation;

    if player.just_moved
        && encounter_query
            .iter()
            .any(|&transform| wall_collision_check(player_translation, transform.translation))
    {
        println!("Change to combat");
        state
            .set(GameState::Combat)
            .expect("Failed to change states");
    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn player_movement(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, mut transform) = player_query.single_mut();
    player.just_moved = false;

    let mut delta_y = 0.0;
    if keyboard.pressed(KeyCode::W) {
        delta_y += time.delta_seconds() * player.speed * TILE_SIZE;
    }
    if keyboard.pressed(KeyCode::S) {
        delta_y -= time.delta_seconds() * player.speed * TILE_SIZE;
    }

    let mut delta_x = 0.0;
    if keyboard.pressed(KeyCode::A) {
        delta_x += time.delta_seconds() * player.speed * TILE_SIZE;
    }
    if keyboard.pressed(KeyCode::D) {
        delta_x -= time.delta_seconds() * player.speed * TILE_SIZE;
    }

    let target = transform.translation + Vec3::new(delta_x, 0.0, 0.0);
    if !wall_query
        .iter()
        .any(|&transform| wall_collision_check(target, transform.translation))
    {
        if delta_x != 0.0 {
            player.just_moved = true;
        }
        transform.translation = target;
    }

    let target = transform.translation + Vec3::new(0.0, delta_y, 0.0);
    if !wall_query
        .iter()
        .any(|&transform| wall_collision_check(target, transform.translation))
    {
        if delta_y != 0.0 {
            player.just_moved = true;
        }
        transform.translation = target;
    }
}

fn wall_collision_check(target_player_pos: Vec3, wall_translation: Vec3) -> bool {
    let collision = collide(
        target_player_pos,
        Vec2::splat(TILE_SIZE * 0.9),
        wall_translation,
        Vec2::splat(TILE_SIZE),
    );
    collision.is_some()
}

pub fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        1,
        Color::rgb(0.3, 0.3, 0.9),
        Vec3::new(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0),
    );
    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player {
            speed: 3.0,
            just_moved: false,
        });

    let mut background_sprite = TextureAtlasSprite::new(0);
    background_sprite.color = Color::rgb(0.5, 0.5, 0.5);
    background_sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    let background = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        0,
        Color::rgb(0.5, 0.5, 0.5),
        Vec3::new(0.0, 0.0, -1.0),
    );

    commands.entity(background).insert(Name::new("Background"));

    commands.entity(player).push_children(&[background]);
}
