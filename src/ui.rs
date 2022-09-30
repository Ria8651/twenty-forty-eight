use super::{
    board::{Board, UpdateBoardEvent},
    record::{load_board_from_file, RecordEvent, RecordInfo},
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContext, EguiPlugin,
};
use futures_lite::future;
use std::path::PathBuf;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .insert_resource(UIState {
                board_selector: 0,
                loaded_recording: None,
            })
            .add_system(ui_system);
    }
}

struct UIState {
    board_selector: usize,
    loaded_recording: Option<Vec<u8>>,
}

fn ui_system(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    event_info: Res<RecordInfo>,
    mut record_event: EventWriter<RecordEvent>,
    mut ui_state: ResMut<UIState>,
    mut board: ResMut<Board>,
    mut events: EventWriter<UpdateBoardEvent>,
    mut file_dialog: Query<(Entity, &mut SelectedFile)>,
) {
    egui::Window::new("Settings")
        .anchor(Align2::RIGHT_TOP, [-5.0, 5.0])
        .show(egui_context.ctx_mut(), |ui| {
            if ui.button("Reset board").clicked() {
                *board = Board::new();
                events.send(UpdateBoardEvent);
            }

            match event_info.recording {
                true => {
                    if ui.button("Stop Recording").clicked() {
                        record_event.send(RecordEvent::Stop);
                    }
                }
                false => {
                    if ui.button("Start Recording").clicked() {
                        record_event.send(RecordEvent::Start);
                    }
                }
            }

            if ui.button("Load file").clicked() {
                let thread_pool = bevy::tasks::AsyncComputeTaskPool::get();
                let task = thread_pool.spawn(async move {
                    rfd::FileDialog::new()
                        .add_filter("2048 recording", &["tfer"])
                        .pick_file()
                });
                commands.spawn().insert(SelectedFile(task));
            }

            if ui_state.loaded_recording != None {
                let recording_length = ui_state.loaded_recording.as_ref().unwrap().len() / 17 - 1;
                let slider = egui::Slider::new(&mut ui_state.board_selector, 0..=recording_length)
                    .text("Board index");
                    
                if ui.add(slider).changed() {
                    let file = ui_state.loaded_recording.as_ref().unwrap();
                    *board = load_board_from_file(file, ui_state.board_selector).input;
                    events.send(UpdateBoardEvent);
                }
            }
        });

    // check for file dialog completion
    for (entity, mut selected_file) in file_dialog.iter_mut() {
        if let Some(result) = future::block_on(future::poll_once(&mut selected_file.0)) {
            let path = result.unwrap();
            commands.entity(entity).despawn();

            let file = std::fs::read(path).unwrap();
            ui_state.loaded_recording = Some(file);
        }
    }
}

// file loading stuff
#[derive(Component)]
struct SelectedFile(bevy::tasks::Task<Option<PathBuf>>);
