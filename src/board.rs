use rand::Rng;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub data: Vec<Vec<u8>>,
    pub player_to_move: bool,
}

impl Board {
    pub fn new() -> Self {
        Board {
            data: vec![vec![0; 4]; 4],
            player_to_move: true,
        }
    }

    /// Returns true if the board was changed
    pub fn swipe(&mut self, direction: Swipe) -> bool {
        let mut changed = false;
        for a in 0..4 {
            let mut last = (a, 0);
            for b in 1..4 {
                let pos = direction.abtoxy(a, b);
                let current = self.data[pos.y][pos.x];

                let last_pos = direction.abtoxy(last.0, last.1);
                let last_value = self.data[last_pos.y][last_pos.x];

                if current != 0 {
                    if current == last_value {
                        self.data[pos.y][pos.x] = 0;
                        self.data[last_pos.y][last_pos.x] += 1;
                        last = (last.0, last.1 + 1);

                        changed = true;
                    } else if last_value == 0 {
                        self.data[pos.y][pos.x] = 0;
                        self.data[last_pos.y][last_pos.x] = current;

                        changed = true;
                    } else {
                        last = (last.0, last.1 + 1);
                        let last_pos = direction.abtoxy(last.0, last.1);
                        if last_pos != pos {
                            self.data[pos.y][pos.x] = 0;
                            self.data[last_pos.y][last_pos.x] = current;

                            changed = true;
                        }
                    }
                }
            }
        }

        self.player_to_move = false;

        changed
    }

    pub fn computer_move(&mut self) {
        self.add_random();
        self.player_to_move = true;
    }

    pub fn get_moves(&self) -> Vec<Moves> {
        match self.player_to_move {
            true => {
                let mut moves = Vec::new();
                for swipe in [Swipe::Up, Swipe::Down, Swipe::Left, Swipe::Right].iter() {
                    let mut board = self.clone();
                    if board.swipe(*swipe) {
                        moves.push(Moves::Player(*swipe));
                    }
                }
                moves
            }
            false => {
                let mut moves = Vec::new();
                for y in 0..4 {
                    for x in 0..4 {
                        if self.data[y][x] == 0 {
                            moves.push(Moves::Computer(ComputerMoves::PlaceTwo(Pos::new(x, y))));
                        }
                    }
                }
                moves
            }
        }
    }

    pub fn apply_move(&mut self, moves: Moves) {
        match moves {
            Moves::Player(swipe) => {
                self.swipe(swipe);
            }
            Moves::Computer(computer_moves) => match computer_moves {
                ComputerMoves::PlaceTwo(pos) => {
                    self.data[pos.y][pos.x] = 1;
                    self.player_to_move = true;
                }
                ComputerMoves::PlaceFour(pos) => {
                    self.data[pos.y][pos.x] = 2;
                    self.player_to_move = true;
                }
            },
        }
    }

    pub fn score(&self) -> u32 {
        let mut score = 0;
        for y in 0..4 {
            for x in 0..4 {
                let exp = self.data[y][x] as u32;
                if exp > 0 {
                    let x = 1 << exp;
                    score += exp * x - x;
                }
            }
        }
        score
    }

    pub fn add_random(&mut self) {
        let mut empty_tiles = Vec::new();
        for y in 0..4 {
            for x in 0..4 {
                if self.data[y][x] == 0 {
                    empty_tiles.push((x, y));
                }
            }
        }

        if empty_tiles.len() > 0 {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..empty_tiles.len());
            let (x, y) = empty_tiles[index];

            let tile_type = rng.gen_range(0..10);
            self.data[y][x] = match tile_type {
                0 => 2,
                _ => 1,
            };
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Swipe {
    Up,
    Down,
    Left,
    Right,
}

impl Swipe {
    fn abtoxy(&self, a: usize, b: usize) -> Pos {
        match self {
            Swipe::Up => Pos::new(a, b),
            Swipe::Down => Pos::new(a, 3 - b),
            Swipe::Left => Pos::new(b, a),
            Swipe::Right => Pos::new(3 - b, a),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputerMoves {
    PlaceTwo(Pos),
    PlaceFour(Pos),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Moves {
    Computer(ComputerMoves),
    Player(Swipe),
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        for y in 0..4 {
            for x in 0..4 {
                let tile = self.data[y][x] as u32;
                if tile == 0 {
                    board.push_str("    ");
                } else {
                    let tile = 1 << tile;
                    board.push_str(&format!("{:4}", tile));
                }
            }
            board.push('\n');
        }
        write!(f, "{}", board)
    }
}
