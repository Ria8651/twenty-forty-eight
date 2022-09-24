use bevy::prelude::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateBoardEvent>()
            .insert_resource(Board {
                data: vec![
                    vec![0, 1, 1, 2],
                    vec![1, 0, 2, 1],
                    vec![1, 1, 2, 1],
                    vec![2, 3, 4, 5],
                ],
            })
            .add_startup_system(setup)
            .add_system(update_board);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub data: Vec<Vec<u8>>,
}

impl Board {
    pub fn swipe(&mut self, direction: Direction) {
        for a in 0..4 {
            let mut last = (a, 0);
            for b in 1..4 {
                let pos = abtoxy(a, b, direction);
                let current = self.data[pos.y][pos.x];

                let last_pos = abtoxy(last.0, last.1, direction);
                let last_value = self.data[last_pos.y][last_pos.x];

                if current != 0 {
                    if current == last_value {
                        self.data[pos.y][pos.x] = 0;
                        self.data[last_pos.y][last_pos.x] += 1;
                        last = (last.0, last.1 + 1);
                    } else if last_value == 0 {
                        self.data[pos.y][pos.x] = 0;
                        self.data[last_pos.y][last_pos.x] = current;
                    } else {
                        last = (last.0, last.1 + 1);
                        let last_pos = abtoxy(last.0, last.1, direction);
                        if last_pos != pos {
                            self.data[pos.y][pos.x] = 0;
                            self.data[last_pos.y][last_pos.x] = current;
                        }
                    }
                }
            }
        }
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

#[derive(Clone, Copy)]
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

fn update_board(
    board: Res<Board>,
    mut update_event: EventReader<UpdateBoardEvent>,
    // mut tile_query: Query<&mut Tile>,
    mut tile_text_query: Query<(&TileText, &mut Text)>,
) {
    for _ in update_event.iter() {
        for (tile_text, mut text) in tile_text_query.iter_mut() {
            let exp = board.data[3 - tile_text.0 as usize / 4][tile_text.0 as usize % 4];
            if exp == 0 {
                text.sections[0].value = "".to_string();
            } else {
                let value = 1u32 << exp as u32;
                text.sections[0].value = value.to_string();
            }
        }
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
                                            font_size: 60.0,
                                            color: Color::WHITE,
                                        },
                                    ))
                                    .insert(TileText(i));
                            });
                    }
                });
        });

    update_event.send(UpdateBoardEvent);
}
