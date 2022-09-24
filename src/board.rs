use bevy::prelude::*;
use rand::Rng;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateBoardEvent>()
            // .insert_resource(Board {
            //     data: vec![
            //         vec![3, 2, 1, 1],
            //         vec![4, 5, 6, 7],
            //         vec![11, 10, 9, 8],
            //         vec![12, 13, 14, 15],
            //     ],
            // })
            .insert_resource(Board::new())
            .add_startup_system(setup)
            .add_system(update_board);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub data: Vec<Vec<u8>>,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            data: vec![vec![0; 4]; 4],
        };

        board.add_random();
        board.add_random();

        board
    }

    pub fn swipe(&mut self, direction: Direction) {
        let mut new_board = self.clone();
        for a in 0..4 {
            let mut last = (a, 0);
            for b in 1..4 {
                let pos = abtoxy(a, b, direction);
                let current = new_board.data[pos.y][pos.x];

                let last_pos = abtoxy(last.0, last.1, direction);
                let last_value = new_board.data[last_pos.y][last_pos.x];

                if current != 0 {
                    if current == last_value {
                        new_board.data[pos.y][pos.x] = 0;
                        new_board.data[last_pos.y][last_pos.x] += 1;
                        last = (last.0, last.1 + 1);
                    } else if last_value == 0 {
                        new_board.data[pos.y][pos.x] = 0;
                        new_board.data[last_pos.y][last_pos.x] = current;
                    } else {
                        last = (last.0, last.1 + 1);
                        let last_pos = abtoxy(last.0, last.1, direction);
                        if last_pos != pos {
                            new_board.data[pos.y][pos.x] = 0;
                            new_board.data[last_pos.y][last_pos.x] = current;
                        }
                    }
                }
            }
        }

        if new_board != *self {
            self.data = new_board.data;
            self.add_random();
        }
    }

    fn add_random(&mut self) {
        let mut empty_tiles = Vec::new();
        for y in 0..4 {
            for x in 0..4 {
                if self.data[y][x] == 0 {
                    empty_tiles.push((x, y));
                }
            }
        }

        if empty_tiles.len() > 0 {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..empty_tiles.len());
            let (x, y) = empty_tiles[index];

            let tile_type = rng.gen_range(0..10);
            self.data[y][x] = match tile_type {
                0 => 2,
                _ => 1,
            };
        }
    }

    pub fn score(&self) -> u32 {
        let mut score = 0;
        for y in 0..4 {
            for x in 0..4 {
                let exp = self.data[y][x] as u32;
                if exp > 0 {
                    let x = 1 << exp;
                    score += exp * x - x;
                }
            }
        }
        score
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

const fn abtoxy(a: usize, b: usize, direction: Direction) -> Pos {
    match direction {
        Direction::Up => Pos::new(a, b),
        Direction::Down => Pos::new(a, 3 - b),
        Direction::Left => Pos::new(b, a),
        Direction::Right => Pos::new(3 - b, a),
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct UpdateBoardEvent;

#[derive(Component)]
struct Tile(u32);
#[derive(Component)]
struct TileText(u32);

#[derive(Component)]
struct Score;

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
    board: Res<Board>,
    mut update_event: EventReader<UpdateBoardEvent>,
    mut querys: ParamSet<(
        Query<(&Tile, &mut UiColor)>,
        Query<(&TileText, &mut Text)>,
        Query<&mut Text, With<Score>>,
    )>,
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
        let mut score = score_query.single_mut();
        score.sections[0].value = board.score().to_string();
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut update_event: EventWriter<UpdateBoardEvent>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::rgb(0.98, 0.97, 0.94).into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(
                    TextBundle::from_section(
                        "0",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::rgb(0.47, 0.44, 0.40),
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    }),
                )
                .insert(Score);

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
                    color: Color::rgb(0.73, 0.68, 0.63).into(),
                    ..default()
                })
                .with_children(|parent| {
                    for i in 0..16 {
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
                            .insert(Tile(i))
                            .with_children(|parent| {
                                parent
                                    .spawn_bundle(TextBundle::from_section(
                                        i.to_string(),
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 40.0,
                                            color: Color::rgb(0.47, 0.44, 0.40),
                                        },
                                    ))
                                    .insert(TileText(i));
                            });
                    }
                });
        });

    update_event.send(UpdateBoardEvent);
}
