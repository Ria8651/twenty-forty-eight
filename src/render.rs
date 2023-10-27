use bevy::prelude::*;

use crate::BoardResource;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateBoardEvent>()
            .add_systems(Startup, setup)
            .add_systems(Update, update_board);
    }
}

#[derive(Event)]
pub struct UpdateBoardEvent;

#[derive(Component)]
struct Tile(u32);
#[derive(Component)]
struct TileText(u32);

#[derive(Component)]
struct ScoreText;
#[derive(Component)]
struct TimeText;

const fn color_map(exp: u8) -> Color {
    match exp {
        0 => Color::rgb(0.80, 0.76, 0.71),
        1 => Color::rgb(0.93, 0.90, 0.85),
        2 => Color::rgb(0.93, 0.89, 0.79),
        3 => Color::rgb(0.95, 0.7, 0.48),
        4 => Color::rgb(0.96, 0.59, 0.39),
        5 => Color::rgb(0.97, 0.49, 0.37),
        6 => Color::rgb(0.97, 0.37, 0.24),
        7..=11 => Color::rgb(0.93, 0.82, 0.45),
        _ => Color::rgb(0.4, 0.4, 0.4),
    }
}

fn update_board(
    board: Res<BoardResource>,
    mut update_event: EventReader<UpdateBoardEvent>,
    mut querys: ParamSet<(
        Query<(&Tile, &mut BackgroundColor)>,
        Query<(&TileText, &mut Text)>,
        Query<&mut Text, With<ScoreText>>,
        Query<&mut Text, With<TimeText>>,
    )>,
    time: Res<Time>,
) {
    for _ in update_event.iter() {
        for (tile, mut ui_colour) in querys.p0().iter_mut() {
            let exp = board.data[3 - tile.0 as usize / 4][tile.0 as usize % 4];
            *ui_colour = color_map(exp).into();
        }

        for (tile_text, mut text) in querys.p1().iter_mut() {
            let exp = board.data[3 - tile_text.0 as usize / 4][tile_text.0 as usize % 4];

            let string = if exp == 0 {
                "".to_string()
            } else {
                (1u32 << exp as u32).to_string()
            };
            text.sections[0].value = string;
            text.sections[0].style.color = if exp <= 3 {
                Color::rgb(0.47, 0.44, 0.40)
            } else {
                Color::rgb(0.98, 0.96, 0.95)
            };
        }

        let mut score_query = querys.p2();
        let mut score_text = score_query.single_mut();
        score_text.sections[0].value = board.score().to_string();

        let mut time_query = querys.p3();
        let mut time_text = time_query.single_mut();
        time_text.sections[0].value = format!("{:.2}", time.startup().elapsed().as_secs_f32());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut update_event: EventWriter<UpdateBoardEvent>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgb(0.98, 0.97, 0.94).into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(400.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Px(80.0),
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceBetween,
                                ..default()
                            },
                            background_color: Color::NONE.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section(
                                    "0",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 60.0,
                                        color: Color::rgb(0.47, 0.44, 0.40),
                                    },
                                ))
                                .insert(ScoreText);
                            parent
                                .spawn(TextBundle::from_section(
                                    "0",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 60.0,
                                        color: Color::rgb(0.47, 0.44, 0.40),
                                    },
                                ))
                                .insert(TimeText);
                        });

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(400.0),
                                height: Val::Px(400.0),
                                padding: UiRect {
                                    top: Val::Px(10.0),
                                    left: Val::Px(10.0),
                                    ..default()
                                },
                                flex_wrap: FlexWrap::WrapReverse,
                                flex_direction: FlexDirection::Row,
                                ..default()
                            },
                            background_color: Color::rgb(0.73, 0.68, 0.63).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            for i in 0..16 {
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Px(87.5),
                                            height: Val::Px(87.5),
                                            margin: UiRect {
                                                bottom: Val::Px(10.0),
                                                right: Val::Px(10.0),
                                                ..default()
                                            },
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        background_color: Color::rgb(0.6, 0.6, 0.6).into(),
                                        ..default()
                                    })
                                    .insert(Tile(i))
                                    .with_children(|parent| {
                                        parent
                                            .spawn(TextBundle::from_section(
                                                i.to_string(),
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 40.0,
                                                    color: Color::rgb(0.47, 0.44, 0.40),
                                                },
                                            ))
                                            .insert(TileText(i));
                                    });
                            }
                        });
                });
        });

    update_event.send(UpdateBoardEvent);
}
