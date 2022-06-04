use bevy::{
    prelude::*, render::camera::Camera2d, sprite::collide_aabb::collide,
};
use bevy_inspector_egui::Inspectable;

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    combat::CombatStats,
    fadeout::create_fadeout,
    graphics::{
        CharacterSheet, FacingDirection, FrameAnimation, PlayerGraphics,
    },
    tilemap::{EncounterSpawner, TileCollider},
    GameState, TILE_SIZE,
};

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct EncounterTracker {
    timer: Timer,
}

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
    just_moved: bool,
    pub active: bool,
    pub exp: usize,
}

impl Player {
    pub fn give_exp(&mut self, exp: usize, stats: &mut CombatStats) -> bool {
        self.exp += exp;
        if self.exp >= 50 {
            stats.health += 2;
            stats.max_health += 2;
            stats.attack += 2;
            stats.defense += 2;
            self.exp -= 50;
            return true;
        }
        false
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_resume(GameState::Overworld).with_system(show_player),
        )
        .add_system_set(
            SystemSet::on_pause(GameState::Overworld).with_system(hide_player),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Overworld)
                .with_system(player_encounter_checking.after(player_movement))
                .with_system(camera_follow.after(player_movement))
                .with_system(player_movement),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Overworld).with_system(spawn_player),
        );
    }
}

fn show_player(
    mut player_query: Query<(&mut Visibility, &mut Player)>,
    children_query: Query<&Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    let (mut player_vis, mut player) = player_query.single_mut();
    player_vis.is_visible = true;
    player.active = true;

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
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &mut EncounterTracker, &Transform)>,
    encounter_query: Query<
        &Transform,
        (With<EncounterSpawner>, Without<Player>),
    >,
    time: Res<Time>,
    ascii: Res<AsciiSheet>,
) {
    let (mut player, mut encounter_tracker, player_transform) =
        player_query.single_mut();
    let player_translation = player_transform.translation;

    if player.just_moved
        && encounter_query.iter().any(|&transform| {
            wall_collision_check(player_translation, transform.translation)
        })
    {
        encounter_tracker.timer.tick(time.delta());

        if encounter_tracker.timer.just_finished() {
            println!("Change to combat");
            create_fadeout(&mut commands, Some(GameState::Combat), &ascii);
            player.active = false;
        }
    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn player_movement(
    mut player_query: Query<(&mut Player, &mut Transform, &mut PlayerGraphics)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, mut transform, mut graphics) = player_query.single_mut();
    player.just_moved = false;

    if !player.active {
        return;
    }

    let mut delta_y = 0.0;
    if keyboard.pressed(KeyCode::W) {
        delta_y += time.delta_seconds() * player.speed * TILE_SIZE;
    }
    if keyboard.pressed(KeyCode::S) {
        delta_y -= time.delta_seconds() * player.speed * TILE_SIZE;
    }

    let mut delta_x = 0.0;
    if keyboard.pressed(KeyCode::A) {
        delta_x -= time.delta_seconds() * player.speed * TILE_SIZE;
    }
    if keyboard.pressed(KeyCode::D) {
        delta_x += time.delta_seconds() * player.speed * TILE_SIZE;
    }

    let target = transform.translation + Vec3::new(0.0, delta_y, 0.0);
    if !wall_query
        .iter()
        .any(|&transform| wall_collision_check(target, transform.translation))
    {
        if delta_y != 0.0 {
            player.just_moved = true;
            if delta_y > 0.0 {
                graphics.facing = FacingDirection::Up;
            } else if delta_y < 0.0 {
                graphics.facing = FacingDirection::Down;
            }
        }
        transform.translation = target;
    }

    let target = transform.translation + Vec3::new(delta_x, 0.0, 0.0);
    if !wall_query
        .iter()
        .any(|&transform| wall_collision_check(target, transform.translation))
    {
        if delta_x != 0.0 {
            player.just_moved = true;
            if delta_x > 0.0 {
                graphics.facing = FacingDirection::Right;
            } else if delta_x < 0.0 {
                graphics.facing = FacingDirection::Left;
            }
        }
        transform.translation = target;
    }
}

fn wall_collision_check(
    target_player_pos: Vec3,
    wall_translation: Vec3,
) -> bool {
    let collision = collide(
        target_player_pos,
        Vec2::splat(TILE_SIZE * 0.9),
        wall_translation,
        Vec2::splat(TILE_SIZE),
    );
    collision.is_some()
}

pub fn spawn_player(mut commands: Commands, characters: Res<CharacterSheet>) {
    let player = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: characters.player_down[0],
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..default()
            },
            texture_atlas: characters.handle.clone(),
            transform: Transform::from_xyz(
                2.0 * TILE_SIZE,
                -2.0 * TILE_SIZE,
                900.0,
            ),
            ..default()
        })
        .insert(FrameAnimation {
            timer: Timer::from_seconds(0.2, true),
            frames: characters.player_down.to_vec(),
            current_frame: 0,
        })
        .insert(PlayerGraphics {
            facing: FacingDirection::Down,
        })
        .insert(Name::new("Player"))
        .insert(Player {
            speed: 3.0,
            just_moved: false,
            active: true,
            exp: 0,
        })
        .insert(CombatStats {
            health: 10,
            max_health: 10,
            attack: 2,
            defense: 1,
        })
        .insert(EncounterTracker {
            timer: Timer::from_seconds(1.0, true),
        })
        .id();
}
