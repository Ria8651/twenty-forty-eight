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
    if input.just_pressed(KeyCode::Up) {
        board.swipe(Direction::Up);
        events.send(UpdateBoardEvent);
    } else if input.just_pressed(KeyCode::Down) {
        board.swipe(Direction::Down);
        events.send(UpdateBoardEvent);
    } else if input.just_pressed(KeyCode::Left) {
        board.swipe(Direction::Left);
        events.send(UpdateBoardEvent);
    } else if input.just_pressed(KeyCode::Right) {
        board.swipe(Direction::Right);
        events.send(UpdateBoardEvent);
    }
}
