use bevy::{prelude::*, winit::WinitSettings};
use board::BoardPlugin;

mod board;

fn main() {
    App::new()
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugin(BoardPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
