use super::{Board, Direction};
use bevy::prelude::*;
use futures_lite::future;
use std::path::PathBuf;

pub struct RecordPlugin;

impl Plugin for RecordPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RecordEvent>()
            .insert_resource(RecordInfo {
                recording: false,
                save_location: "".into(),
                move_stack: Vec::new(),
            })
            .add_systems(Update, record_system);
    }
}

#[derive(Clone)]
pub struct InoutPair<A, B> {
    pub input: A,
    pub output: B,
}

#[derive(Resource)]
pub struct RecordInfo {
    pub recording: bool,
    save_location: PathBuf,
    move_stack: Vec<InoutPair<Board, Direction>>,
}

#[derive(Event)]
pub enum RecordEvent {
    Start,
    Stop,
    // inital board and correct direction
    AddMove(InoutPair<Board, Direction>),
}

#[derive(Component)]
struct SelectedFile(bevy::tasks::Task<Option<PathBuf>>);

fn record_system(
    mut commands: Commands,
    mut file_dialog: Query<(Entity, &mut SelectedFile)>,
    mut record_info: ResMut<RecordInfo>,
    mut record_event: EventReader<RecordEvent>,
) {
    for event in record_event.iter() {
        match event {
            RecordEvent::Start => {
                record_info.recording = true;

                let thread_pool = bevy::tasks::AsyncComputeTaskPool::get();
                let task = thread_pool.spawn(async move {
                    rfd::FileDialog::new()
                        .add_filter("2048 recording", &["tfer"])
                        .save_file()
                });
                commands.spawn(SelectedFile(task));

                println!("Started recording");
            }
            RecordEvent::Stop => {
                record_info.recording = false;
                println!("Saving recording");

                // save the recorded moves
                let mut file_output = Vec::new();
                for i in 0..record_info.move_stack.len() {
                    let InoutPair { input, output } = &record_info.move_stack[i];
                    input.serialize(&mut file_output);
                    output.serialize(&mut file_output);
                }

                use std::io::Write;
                let mut file = std::fs::File::create(record_info.save_location.clone()).unwrap();
                file.write_all(&file_output).unwrap();
            }
            RecordEvent::AddMove(inout_pair) => {
                if record_info.recording {
                    record_info.move_stack.push(inout_pair.clone());
                }
            }
        }
    }

    // check for file dialog completion
    for (entity, mut selected_file) in file_dialog.iter_mut() {
        if let Some(result) = future::block_on(future::poll_once(&mut selected_file.0)) {
            record_info.save_location = result.unwrap();
            commands.entity(entity).despawn();
        }
    }
}

pub fn load_board_from_file(file: &[u8], index: usize) -> InoutPair<Board, Direction> {
    // each board is 17 bytes
    let board_index = index * 17;
    let board_slice = &file[board_index..board_index + 17];

    let board = Board::deserialize(board_slice);
    let direction = Direction::deserialize(&board_slice[16..17]);

    InoutPair {
        input: board,
        output: direction,
    }
}

trait Searialize {
    fn serialize(&self, output: &mut Vec<u8>);
}

impl Searialize for Board {
    fn serialize(&self, output: &mut Vec<u8>) {
        for x in 0..4 {
            for y in 0..4 {
                output.push(self.data[y][x]);
            }
        }
    }
}

impl Searialize for Direction {
    fn serialize(&self, output: &mut Vec<u8>) {
        match self {
            Direction::Up => output.push(0),
            Direction::Down => output.push(1),
            Direction::Left => output.push(2),
            Direction::Right => output.push(3),
        }
    }
}

trait Deserialize {
    fn deserialize(input: &[u8]) -> Self;
}

impl Deserialize for Board {
    fn deserialize(input: &[u8]) -> Self {
        let mut data = vec![vec![0; 4]; 4];
        let mut i = 0;
        for x in 0..4 {
            for y in 0..4 {
                data[y][x] = input[i];
                i += 1;
            }
        }
        Board { data }
    }
}

impl Deserialize for Direction {
    fn deserialize(input: &[u8]) -> Self {
        match input[0] {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}
