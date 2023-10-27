use bevy::prelude::*; //, winit::WinitSettings
use board::{Board, BoardPlugin, Direction, Pos, UpdateBoardEvent};
use record::{RecordEvent, RecordPlugin};
use ui::{UIPlugin, UiSettings};

mod board;
mod record;
mod ui;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BoardPlugin, UIPlugin, RecordPlugin))
        // .insert_resource(WinitSettings::desktop_app())
        .init_resource::<MoveTimer>()
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Resource, Default, Deref, DerefMut)]
struct MoveTimer(f32);

fn update(
    input: Res<Input<KeyCode>>,
    mut board: ResMut<Board>,
    mut events: EventWriter<UpdateBoardEvent>,
    mut record_event: EventWriter<RecordEvent>,
    mut move_timer: ResMut<MoveTimer>,
    time: Res<Time>,
    ui_settings: Res<UiSettings>,
) {
    let tmp_board = board.clone();

    // human player
    let mut direction = None;
    if input.just_pressed(KeyCode::Up) || input.just_pressed(KeyCode::W) {
        direction = Some(Direction::Up);
    }
    if input.just_pressed(KeyCode::Down) || input.just_pressed(KeyCode::S) {
        direction = Some(Direction::Down);
    }
    if input.just_pressed(KeyCode::Left) || input.just_pressed(KeyCode::A) {
        direction = Some(Direction::Left);
    }
    if input.just_pressed(KeyCode::Right) || input.just_pressed(KeyCode::D) {
        direction = Some(Direction::Right);
    }

    // algorithmic player
    if ui_settings.automatic {
        move_timer.0 += time.delta_seconds();
        if move_timer.0 > ui_settings.speed / 1000.0 {
            move_timer.0 = 0.0;

            direction = Some(recursive_board_score(&board, 7, ui_settings.scoring_method).1);
        }
    }

    if let Some(direction) = direction {
        board.swipe(direction);
        if *board != tmp_board {
            record_event.send(record::RecordEvent::AddMove(record::InoutPair {
                input: tmp_board.clone(),
                output: direction,
            }));
        }
    }

    if *board != tmp_board {
        events.send(UpdateBoardEvent);
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum Technique {
    DownLeft,
    RecursiveScoring,
}

#[derive(Clone, Copy, Default, Reflect)]
pub enum ScoringMethod {
    MostEmpty,
    #[default]
    ZigZag,
    MaxScore,
    Position,
    Adjacentcy,
    Random,
}

fn recursive_board_score(board: &Board, depth: u32, scoreing: ScoringMethod) -> (i32, Direction) {
    if depth == 0 {
        let mut board_score = 0;
        board_score += score(board, scoreing);

        return (board_score as i32, Direction::Up);
    }

    let directions = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
    let mut scores = Vec::new();
    for direction in directions.iter() {
        let mut new_board = board.clone();
        new_board.swipe(*direction);

        if new_board == *board {
            scores.push(i32::MIN);
        } else {
            scores.push(recursive_board_score(&new_board, depth - 1, scoreing).0);
        }
    }

    let mut max = 0;
    for i in 0..scores.len() {
        if scores[i] > scores[max] {
            max = i;
        }
    }

    (scores[max], directions[max])
}

fn score(board: &Board, scoreing: ScoringMethod) -> u32 {
    match scoreing {
        ScoringMethod::MaxScore => {
            let mut score = 0;
            for y in 0..4 {
                for x in 0..4 {
                    let exp = board.data[y][x] as u32;
                    if exp > 0 {
                        let x = 1 << exp;
                        score += exp * x - x;
                    }
                }
            }
            score
        }
        ScoringMethod::MostEmpty => {
            let mut empty = 0;
            for y in 0..4 {
                for x in 0..4 {
                    if board.data[y][x] == 0 {
                        empty += 1;
                    }
                }
            }
            empty
        }
        ScoringMethod::Position => {
            let mut score: u32 = 0;
            let chain = chain();

            // collect numbers by size
            use std::collections::HashMap;
            let mut numbers: HashMap<u8, Vec<Pos>> = HashMap::new();
            for y in 0..4 {
                for x in 0..4 {
                    let exp = board.data[y][x];
                    if exp > 0 {
                        if let Some(vec) = numbers.get_mut(&exp) {
                            vec.push(Pos::new(x, y))
                        } else {
                            numbers.insert(exp, vec![Pos::new(x, y)]);
                        }
                    }
                }
            }

            let max = numbers.keys().max().unwrap();
            let mut value = *max;
            for i in 0..chain.len() {
                if let Some(vec) = numbers.get(&value) {
                    for pos in vec.iter() {
                        let distance = (chain[i].x as i32 - pos.x as i32).abs()
                            + (chain[i].y as i32 - pos.y as i32).abs();
                        score -= 1 << distance as u32;
                    }
                }
                if value > 1 {
                    value -= 1;
                } else {
                    break;
                }
            }

            // // chain
            // let mut last = board.data[chain[0].y][chain[0].x];
            // for i in 1..8 {
            //     let current = board.data[chain[i].y][chain[i].x];
            //     if last > current {
            //         chain_score += 1 << last;
            //     } else if last < current {
            //         chain_score = chain_score.saturating_sub(1 << current);
            //         chain_score = chain_score.saturating_sub(1 << last);
            //     }
            //     last = current;
            // }

            // for i in 0..chain.len() {
            //     score += i as u32 * (1 << board.data[chain[i].y][chain[i].x]);
            // }

            score
        }
        ScoringMethod::Adjacentcy => {
            let mut score = 0;
            for y in 0..4 {
                for x in 0..4 {
                    let current = board.data[y][x] as u32;
                    if current > 0 {
                        let mut adjacent = 0;
                        if x > 0 {
                            adjacent += board.data[y][x - 1] as u32;
                        }
                        if x < 3 {
                            adjacent += board.data[y][x + 1] as u32;
                        }
                        if y > 0 {
                            adjacent += board.data[y - 1][x] as u32;
                        }
                        if y < 3 {
                            adjacent += board.data[y + 1][x] as u32;
                        }
                        score += current * adjacent;
                    }
                }
            }
            score
        }
        ScoringMethod::Random => rand::random::<u32>(),
        ScoringMethod::ZigZag => {
            // put the biggest tile in the bottom right corner and zig zag down
            let mut biggest_tile = 0;
            for y in 0..4 {
                for x in 0..4 {
                    if board.data[y][x] > biggest_tile {
                        biggest_tile = board.data[y][x];
                    }
                }
            }

            let mut score = 0;
            for pos in chain().iter() {
                if board.data[pos.y][pos.x] == biggest_tile {
                    score += 1 << biggest_tile;
                    biggest_tile -= 1;
                } else {
                    break;
                }
            }

            score
        }
    }
}

const fn chain() -> [Pos; 16] {
    [
        Pos::new(3, 0),
        Pos::new(2, 0),
        Pos::new(1, 0),
        Pos::new(0, 0),
        Pos::new(0, 1),
        Pos::new(1, 1),
        Pos::new(2, 1),
        Pos::new(3, 1),
        Pos::new(3, 2),
        Pos::new(2, 2),
        Pos::new(1, 2),
        Pos::new(0, 2),
        Pos::new(0, 3),
        Pos::new(1, 3),
        Pos::new(2, 3),
        Pos::new(3, 3),
    ]
}
