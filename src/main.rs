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
}
