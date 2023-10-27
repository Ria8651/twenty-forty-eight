use crate::{
    board::{Board, UpdateBoardEvent},
    record::{load_board_from_file, RecordEvent, RecordInfo},
    ScoringMethod,
};
use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::{EguiContexts, EguiPlugin},
    egui,
    reflect_inspector::ui_for_value,
    DefaultInspectorConfigPlugin,
};
use futures_lite::future;
use std::path::PathBuf;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EguiPlugin, DefaultInspectorConfigPlugin))
            .insert_resource(UIState {
                board_selector: 0,
                loaded_recording: None,
            })
            .init_resource::<UiSettings>()
            .register_type::<UiSettings>()
            .add_systems(Update, ui_system);
    }
}

#[derive(Resource, Reflect, Default)]
struct UiSettings {
    scoring_method: ScoringMethod,
}

#[derive(Resource)]
struct UIState {
    board_selector: usize,
    loaded_recording: Option<Vec<u8>>,
}

fn ui_system(
    mut commands: Commands,
    mut contexts: EguiContexts,
    event_info: Res<RecordInfo>,
    mut record_event: EventWriter<RecordEvent>,
    mut ui_state: ResMut<UIState>,
    mut ui_settings: ResMut<UiSettings>,
    mut board: ResMut<Board>,
    mut events: EventWriter<UpdateBoardEvent>,
    mut file_dialog: Query<(Entity, &mut SelectedFile)>,
    type_registry: Res<AppTypeRegistry>,
) {
    egui::Window::new("Settings").show(contexts.ctx_mut(), |ui| {
        ui_for_value(ui_settings.as_mut(), ui, &type_registry.read());

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
            commands.spawn(SelectedFile(task));
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
