use bevy::prelude::*;

use crate::{ascii::AsciiSheet, GameState};

#[derive(Component)]
pub struct ScreenFade {
    alpha: f32,
    sent: bool,
    next_state: Option<GameState>,
    timer: Timer,
}

pub struct FadeoutPlugin;

impl Plugin for FadeoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(fadeout);
    }
}

fn ui_fadeout(
    fade_query: Query<&ScreenFade>,
    mut ui_query: Query<&mut UiColor>,
    mut text_query: Query<&mut Text>,
) {
    if let Some(fade) = fade_query.iter().next() {
        for mut ui_color in ui_query.iter_mut() {
            ui_color.0.set_a(1.0 - fade.alpha);
        }
        for mut text in text_query.iter_mut() {
            for mut section in text.sections.iter_mut() {
                section.style.color.set_a(1.0 - fade.alpha);
            }
        }
    }
}

fn fadeout(
    mut commands: Commands,
    mut fade_query: Query<(Entity, &mut ScreenFade, &mut TextureAtlasSprite)>,
    mut state: ResMut<State<GameState>>,
    time: Res<Time>,
) {
    for (entity, mut fade, mut sprite) in fade_query.iter_mut() {
        fade.timer.tick(time.delta());
        if fade.timer.percent() < 0.5 {
            fade.alpha = fade.timer.percent() * 2.0;
        } else {
            fade.alpha = fade.timer.percent_left() * 2.0;
        }
        sprite.color.set_a(fade.alpha);

        if fade.timer.percent() > 0.5 && !fade.sent {
            if let Some(next_state) = fade.next_state {
                state.set(next_state).unwrap();
            } else{
                state.pop().unwrap();
            }
            fade.sent = true;
        }

        if fade.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn create_fadeout(
    commands: &mut Commands,
    next_state: Option<GameState>,
    ascii: &Res<AsciiSheet>,
) {
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.color = Color::rgba(0.1, 0.1, 0.15, 0.0);
    sprite.custom_size = Some(Vec2::splat(100000.0));
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 999.0),
                ..default()
            },
            ..default()
        })
        .insert(ScreenFade {
            alpha: 0.0,
            sent: false,
            next_state,
            timer: Timer::from_seconds(1.0, false),
        })
        .insert(Name::new("Fadeout"));
}
