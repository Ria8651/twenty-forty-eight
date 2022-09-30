use bevy::prelude::*; //, winit::WinitSettings
use board::{Board, BoardPlugin, Direction, Pos, UpdateBoardEvent};
use record::RecordPlugin;
use ui::UIPlugin;

mod board;
mod record;
mod ui;

fn main() {
    App::new()
        // .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugin(BoardPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(RecordPlugin)
        .add_startup_system(setup)
        .add_system(update)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn update(
    input: Res<Input<KeyCode>>,
    mut board: ResMut<Board>,
    mut events: EventWriter<UpdateBoardEvent>,
    mut record_event: EventWriter<record::RecordEvent>,
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

    if let Some(direction) = direction {
        board.swipe(direction);
        if *board != tmp_board {
            record_event.send(record::RecordEvent::AddMove(record::InoutPair {
                input: tmp_board.clone(),
                output: direction,
            }));
        }
    }

    if input.just_pressed(KeyCode::Space) {
        // print each of the scoreing types
        println!(
            "MaxScore: {}, MostEmpty: {}, Adjacentcy: {}",
            score(&board, Scoreing::MaxScore),
            score(&board, Scoreing::MostEmpty) * 100,
            score(&board, Scoreing::Adjacentcy)
        );
    }

    // ai player
    // match Technique::RecursiveScoring {
    //     Technique::DownLeft => {
    //         board.swipe(Direction::Down);
    //         board.swipe(Direction::Right);
    //         if *board == tmp_board {
    //             board.swipe(Direction::Left);
    //             board.swipe(Direction::Down);
    //             board.swipe(Direction::Right);
    //         }
    //     }
    //     Technique::RecursiveScoring => {
    //         // for _ in 0..10 {
    //         let tmp = board.clone();
    //         board.swipe(recursive_board_score(&tmp, 7, Scoreing::MaxScore).1);
    //         // }
    //     }
    // }

    if *board != tmp_board {
        events.send(UpdateBoardEvent);
    }
}

#[derive(Clone, Copy)]
enum Technique {
    DownLeft,
    RecursiveScoring,
}

#[derive(Clone, Copy)]
enum Scoreing {
    MaxScore,
    MostEmpty,
    Position,
    Adjacentcy,
    Random,
}

fn recursive_board_score(board: &Board, depth: u32, scoreing: Scoreing) -> (i32, Direction) {
    if depth == 0 {
        let mut board_score = 0;
        // board_score += score(board, Scoreing::MaxScore);
        board_score += score(board, Scoreing::MostEmpty) * 100;
        // board_score += score(board, Scoreing::Adjacentcy);
        // board_score += score(board, Scoreing::Position);

        // print each of the scoreing types
        // println!(
        //     "MaxScore: {}, MostEmpty: {}, Adjacentcy: {}",
        //     score(board, Scoreing::MaxScore),
        //     score(board, Scoreing::MostEmpty) * 100,
        //     score(board, Scoreing::Adjacentcy)
        // );

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

        // let penalty = match *direction {
        //     Direction::Up => 2,
        //     Direction::Down => 0,
        //     Direction::Left => 1,
        //     Direction::Right => 0,
        // };

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

fn score(board: &Board, scoreing: Scoreing) -> u32 {
    match scoreing {
        Scoreing::MaxScore => {
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
        Scoreing::MostEmpty => {
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
        Scoreing::Position => {
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
        Scoreing::Adjacentcy => {
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
        Scoreing::Random => rand::random::<u32>(),
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
