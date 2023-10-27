use minimax::{Evaluation, Evaluator, Winner};

use crate::board::{Board, Moves};

pub struct TwentyFortyEight;

impl minimax::Game for TwentyFortyEight {
    type S = Board;
    type M = Moves;

    fn generate_moves(board: &Board, moves: &mut Vec<Moves>) {
        *moves = board.get_moves();
    }

    fn apply(board: &mut Board, moves: Moves) -> Option<Board> {
        let mut board = board.clone();
        board.apply_move(moves);
        Some(board)
    }

    fn get_winner(_: &Board) -> Option<Winner> {
        None
    }
}

// To run the search we need an evaluator.
pub struct TwentyFortyEightEvaluator;

impl Evaluator for TwentyFortyEightEvaluator {
    type G = TwentyFortyEight;

    fn evaluate(&self, board: &Board) -> Evaluation {
        let mut empty = 0;
        for y in 0..4 {
            for x in 0..4 {
                if board.data[y][x] == 0 {
                    empty += 1;
                }
            }
        }
        empty as Evaluation
    }
}
