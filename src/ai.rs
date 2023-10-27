use super::board::{Board, Direction};
use super::record::{load_board_from_file, InoutPair};
use nn::{HaltCondition, NN};

pub fn board_to_nn_in(board: &Board) -> Vec<f64> {
    let mut inputs = vec![0.0; 256];
    for y in 0..4 {
        for x in 0..4 {
            let exp = board.data[y][x] as usize;
            if exp > 0 {
                inputs[y * 64 + x * 16 + exp] = 1.0;
            }
        }
    }
    inputs
}

pub fn direction_to_nn_out(direction: &Direction) -> Vec<f64> {
    let mut outputs = vec![0.0; 4];
    match direction {
        Direction::Up => outputs[0] = 1.0,
        Direction::Down => outputs[1] = 1.0,
        Direction::Left => outputs[2] = 1.0,
        Direction::Right => outputs[3] = 1.0,
    }
    outputs
}

pub fn train() -> NN {
    let mut examples = Vec::new();

    let recording = std::fs::read("recordings/take1.tfer").unwrap();
    let boards = recording.len() / 17;
    for i in 0..boards {
        let InoutPair { input, output } = load_board_from_file(&recording, i);
        examples.push((board_to_nn_in(&input), direction_to_nn_out(&output)));
    }

    let mut net = NN::new(&[256, 64, 64, 4]);

    net.train(&examples)
        .halt_condition(HaltCondition::Epochs(1000))
        .log_interval(Some(100))
        .go();

    let json = net.to_json();
    std::fs::write("ai.json", json).unwrap();

    net
}
