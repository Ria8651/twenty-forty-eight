use bevy::prelude::*;
use board::{Board, Moves, Swipe};
use evaluators::TwentyFortyEightEvaluator;
use minimax::{Negamax, Strategy};
use record::{InoutPair, RecordEvent, RecordPlugin};
use render::{BoardPlugin, UpdateBoardEvent};
use ui::{UIPlugin, UiSettings};

mod board;
mod evaluators;
mod record;
mod render;
mod ui;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BoardPlugin, UIPlugin, RecordPlugin))
        .init_resource::<MoveTimer>()
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

#[derive(Resource, Deref, DerefMut)]
pub struct BoardResource(Board);

fn setup(mut commands: Commands) {
    let mut board = Board::new();
    board.add_random();
    board.add_random();
    commands.insert_resource(BoardResource(board));
    commands.spawn(Camera2dBundle::default());
}

#[derive(Resource, Default, Deref, DerefMut)]
struct MoveTimer(f32);

fn update(
    input: Res<Input<KeyCode>>,
    mut board: ResMut<BoardResource>,
    mut events: EventWriter<UpdateBoardEvent>,
    mut record_event: EventWriter<RecordEvent>,
    mut move_timer: ResMut<MoveTimer>,
    time: Res<Time>,
    ui_settings: Res<UiSettings>,
) {
    // human player
    let mut swipe = None;
    if input.just_pressed(KeyCode::Up) || input.just_pressed(KeyCode::W) {
        swipe = Some(Swipe::Up);
    }
    if input.just_pressed(KeyCode::Down) || input.just_pressed(KeyCode::S) {
        swipe = Some(Swipe::Down);
    }
    if input.just_pressed(KeyCode::Left) || input.just_pressed(KeyCode::A) {
        swipe = Some(Swipe::Left);
    }
    if input.just_pressed(KeyCode::Right) || input.just_pressed(KeyCode::D) {
        swipe = Some(Swipe::Right);
    }

    // algorithmic player
    if ui_settings.automatic {
        move_timer.0 += time.delta_seconds();
        if move_timer.0 > ui_settings.speed / 1000.0 {
            move_timer.0 = 0.0;

            let mut strategy = Negamax::new(TwentyFortyEightEvaluator, ui_settings.depth);
            if let Some(best_move) = strategy.choose_move(board.as_ref()) {
                match best_move {
                    Moves::Player(new_swipe) => {
                        swipe = Some(new_swipe);
                    }
                    Moves::Computer(_) => panic!("Wrong players turn!"),
                }
            }
        }
    }

    if let Some(swipe) = swipe {
        if board.swipe(swipe) {
            board.computer_move();

            events.send(UpdateBoardEvent);
            record_event.send(RecordEvent::AddMove(InoutPair {
                input: board.clone(),
                output: swipe,
            }));
        }
    }
}

// fn main() {
//     let mut start = Board::new();
//     start.add_random();
//     start.add_random();

//     let mut strategy = Negamax::new(TwentyFortyEightEvaluator, 8);

//     loop {
//         let best_move = strategy.choose_move(&start).unwrap();
//         start.apply_move(best_move);

//         println!("{}", start);
//     }
// }
