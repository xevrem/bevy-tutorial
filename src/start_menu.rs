use bevy::{prelude::*, ui::FocusPolicy};

struct UiAssets {
    font: Handle<Font>,
    button: Handle<Image>,
    button_pressed: Handle<Image>,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_menu);
    }
}

fn setup_menu(mut commands: Commands, assets: Res<AssetServer>) {
    let ui_assets = UiAssets {
        font: assets.load("QuattrocentoSans-Bold.ttf"),
        button: assets.load("button.png"),
        button_pressed: assets.load("button_pressed.png"),
    };
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(20.0), Val::Percent(10.0)),
                margin: Rect::all(Val::Auto),
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ImageBundle {
                    style: Style {
                        size: Size::new(
                            Val::Percent(100.0),
                            Val::Percent(100.0),
                        ),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    image: ui_assets.button.clone().into(),
                    ..default()
                })
                .insert(FocusPolicy::Pass)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Start Game",
                            TextStyle {
                                font: ui_assets.font.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            default()
                        ),
                        focus_policy: FocusPolicy::Pass,
                        ..default()
                    });
                });
        });
        commands.insert_resource(ui_assets);
}
