use bevy::prelude::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

#[derive(Component)]
struct Tile;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(400.0), Val::Px(400.0)),
                        padding: UiRect {
                            top: Val::Px(10.0),
                            left: Val::Px(10.0),
                            ..default()
                        },
                        flex_wrap: FlexWrap::Wrap,
                        ..default()
                    },
                    color: Color::rgb(0.5, 0.5, 0.5).into(),
                    ..default()
                })
                .with_children(|parent| {
                    for _ in 0..16 {
                        parent
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(87.5), Val::Px(87.5)),
                                    margin: UiRect {
                                        bottom: Val::Px(10.0),
                                        right: Val::Px(10.0),
                                        ..default()
                                    },
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                color: Color::rgb(0.6, 0.6, 0.6).into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn_bundle(
                                    TextBundle::from_section(
                                        "2",
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 60.0,
                                            color: Color::WHITE,
                                        },
                                    )
                                );
                            });
                    }
                });
        });
}
