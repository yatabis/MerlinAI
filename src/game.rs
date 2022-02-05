use std::collections::VecDeque;

use crate::board::{Board, Mino, MINO_LIST};

pub struct Game {
    board: Board,
    hold: Mino,
    next: VecDeque<Mino>,
    bag: [bool; 7],
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            hold: Mino::None,
            next: VecDeque::new(),
            bag: [true; 7],
        }
    }
}
