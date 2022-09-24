use bevy::{prelude::*, winit::WinitSettings};
use board::{Board, BoardPlugin, UpdateBoardEvent, Direction};

mod board;

fn main() {
    App::new()
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugin(BoardPlugin)
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
) {
    // human player
    if input.just_pressed(KeyCode::Up) || input.just_pressed(KeyCode::W) {
        board.swipe(Direction::Up);
        events.send(UpdateBoardEvent);
    } else if input.just_pressed(KeyCode::Down) || input.just_pressed(KeyCode::S) {
        board.swipe(Direction::Down);
        events.send(UpdateBoardEvent);
    } else if input.just_pressed(KeyCode::Left) || input.just_pressed(KeyCode::A) {
        board.swipe(Direction::Left);
        events.send(UpdateBoardEvent);
    } else if input.just_pressed(KeyCode::Right) || input.just_pressed(KeyCode::D) {
        board.swipe(Direction::Right);
        events.send(UpdateBoardEvent);
    }

    // ai player
    let directions = vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right];
    let mut scores = Vec::new();
    for direction in directions.iter() {
        let mut new_board = board.clone();
        new_board.swipe(*direction);

        if new_board == *board {
            scores.push(-1);
        } else {
            scores.push(new_board.score() as i32);
        }
    }
    
    let mut max = 0;
    for i in 0..scores.len() {
        if scores[i] > scores[max] {
            max = i;
        }
    }

    board.swipe(directions[max]);
    events.send(UpdateBoardEvent);
}
