use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};

use minimax::{Evaluation, Evaluator, Winner};

use crate::board::{Board, Moves, Pos};

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

    fn zobrist_hash(board: &Board) -> u64 {
        let mut hasher = DefaultHasher::new();
        board.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Clone, Copy)]
pub struct TwentyFortyEightEvaluator;

impl Evaluator for TwentyFortyEightEvaluator {
    type G = TwentyFortyEight;

    fn evaluate(&self, board: &Board) -> Evaluation {
        // empty tiles
        // let mut empty_tiles = 0;
        // for y in 0..4 {
        //     for x in 0..4 {
        //         if board.data[y][x] == 0 {
        //             empty_tiles += 1;
        //         }
        //     }
        // }
        // empty_tiles as Evaluation

        // zig zag
        let mut biggest_tile = 0;
        for y in 0..4 {
            for x in 0..4 {
                if board.data[y][x] > biggest_tile {
                    biggest_tile = board.data[y][x];
                }
            }
        }

        let mut score = 0;
        let mut last_tile = biggest_tile;
        for pos in CHAIN.iter() {
            if board.data[pos.y][pos.x] <= last_tile {
                last_tile = board.data[pos.y][pos.x];
                score += 1 << last_tile;
            } else {
                score -= 1 << board.data[pos.y][pos.x];
            }
        }

        if board.player_to_move {
            score as Evaluation
        } else {
            -score as Evaluation
        }

        // board.score() as Evaluation
    }
}

const CHAIN: &[Pos] = &[
    Pos::new(0, 0),
    Pos::new(1, 0),
    Pos::new(2, 0),
    Pos::new(3, 0),
    Pos::new(3, 1),
    Pos::new(2, 1),
    Pos::new(1, 1),
    Pos::new(0, 1),
    Pos::new(0, 2),
    Pos::new(1, 2),
    Pos::new(2, 2),
    Pos::new(3, 2),
    Pos::new(3, 3),
    Pos::new(2, 3),
    Pos::new(1, 3),
    Pos::new(0, 3),
];
