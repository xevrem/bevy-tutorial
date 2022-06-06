use bevy::prelude::*;

use crate::{TILE_SIZE, combat::EnemyType};

pub struct CharacterSheet {
    pub handle: Handle<TextureAtlas>,
    pub player_up: [usize; 3],
    pub player_down: [usize; 3],
    pub player_left: [usize; 3],
    pub player_right: [usize; 3],
    pub bat_frames: [usize; 3],
    pub ghost_frames: [usize; 3],
}

pub enum FacingDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
pub struct PlayerGraphics {
    pub facing: FacingDirection,
}

#[derive(Component)]
pub struct FrameAnimation {
    pub timer: Timer,
    pub frames: Vec<usize>,
    pub current_frame: usize,
}

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(
            StartupStage::PreStartup,
            Self::load_graphics,
        )
        .add_system(Self::frame_animation)
        .add_system(Self::update_player_graphics);
    }
}

impl GraphicsPlugin {
    fn load_graphics(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        let image = assets.load("characters.png");
        let atlas = TextureAtlas::from_grid_with_padding(
            image,
            Vec2::splat(16.0),
            12,
            8,
            Vec2::splat(2.0),
        );
        let atlas_handle = texture_atlases.add(atlas);

        let columns: usize = 12;

        commands.insert_resource(CharacterSheet {
            handle: atlas_handle,
            player_up: [columns * 3 + 6, columns * 3 + 7, columns * 3 + 8],
            player_down: [6, 7, 8],
            player_left: [columns * 1 + 6, columns * 1 + 7, columns * 1 + 8],
            player_right: [columns * 2 + 6, columns * 2 + 7, columns * 2 + 8],
            bat_frames: [columns * 4 + 3, columns * 4 + 4, columns * 4 + 5],
            ghost_frames: [columns * 4 + 6, columns * 4 + 7, columns * 4 + 8],
        });
    }

    fn update_player_graphics(
        mut sprites_query: Query<
            (&PlayerGraphics, &mut FrameAnimation),
            Changed<PlayerGraphics>,
        >,
        characters: Res<CharacterSheet>,
    ) {
        for (graphics, mut animation) in sprites_query.iter_mut() {
            animation.frames = match graphics.facing {
                FacingDirection::Up => characters.player_up.to_vec(),
                FacingDirection::Down => characters.player_down.to_vec(),
                FacingDirection::Left => characters.player_left.to_vec(),
                FacingDirection::Right => characters.player_right.to_vec(),
            }
        }
    }

    fn frame_animation(
        mut sprites_query: Query<(
            &mut TextureAtlasSprite,
            &mut FrameAnimation,
        )>,
        time: Res<Time>,
    ) {
        for (mut sprite, mut animation) in sprites_query.iter_mut() {
            animation.timer.tick(time.delta());

            if animation.timer.just_finished() {
                animation.current_frame =
                    (animation.current_frame + 1) % animation.frames.len();
                sprite.index = animation.frames[animation.current_frame];
            }
        }
    }
}

pub fn spawn_enemy_sprite(
    commands: &mut Commands,
    characters: &CharacterSheet,
    translation: Vec3,
    enemy_type: &EnemyType,
) -> Entity {
    let mut sprite = match enemy_type {
        EnemyType::Bat => TextureAtlasSprite::new(characters.bat_frames[0]),
        EnemyType::Ghost => TextureAtlasSprite::new(characters.ghost_frames[0]),
    };
    sprite.custom_size = Some(Vec2::splat(0.5));

    let animation = match enemy_type {
        EnemyType::Bat => FrameAnimation {
            timer: Timer::from_seconds(0.2, true),
            frames: characters.bat_frames.to_vec(),
            current_frame: 0,
        },
        EnemyType::Ghost => FrameAnimation {
            timer: Timer::from_seconds(0.2, true),
            frames: characters.ghost_frames.to_vec(),
            current_frame: 0,
        },
    };

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: characters.handle.clone(),
            transform: Transform {
                translation,
                ..default()
            },
            ..default()
        })
        .insert(animation)
        .id()
}
