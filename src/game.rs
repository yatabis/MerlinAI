use std::collections::VecDeque;

use crate::board::{Board, Mino, MinoInfo, TSpin};

pub struct Game {
    pub board: Board,
    pub hold: Mino,
    pub next: VecDeque<Mino>,
    pub bag: [bool; 7],
    pub back_to_back: bool,
    pub ren: i32
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            hold: Mino::None,
            next: VecDeque::new(),
            bag: [true; 7],
            back_to_back: false,
            ren: 0,
        }
    }

    pub fn new_next(&mut self, mino: Mino) {
        self.next.push_back(mino);
        self.bag[mino as usize] = false;
        if !self.bag.contains(&true) {
            self.bag = [true; 7];
        }
    }

    pub fn move_left(&mut self) {
        self.board.move_left();
    }

    pub fn move_right(&mut self) {
        self.board.move_right();
    }

    pub fn rotate_clockwise(&mut self) {
        self.board.rotate_clockwise();
    }

    pub fn rotate_counterclockwise(&mut self) {
        self.board.rotate_counterclockwise();
    }

    pub fn soft_drop(&mut self) {
        self.board.move_down();
    }

    pub fn hard_drop(&mut self) -> MinoInfo {
        let ground_info = self.board.ground();
        let clears = self.board.clear_lines();
        if clears > 0 {
            self.back_to_back = clears == 4 || self.board.t_spin != TSpin::None;
        }
        if clears > 0 {
            self.ren += 1;
        } else {
            self.ren = 0;
        }
        self.board.spawn(self.next.pop_front().unwrap());
        ground_info
    }

    pub fn hold(&mut self) {
        if self.hold == Mino::None {
            self.hold = self.board.current.mino;
            self.board.spawn(self.next.pop_front().unwrap());
        } else {
            let hold_mino = self.hold;
            self.hold = self.board.current.mino;
            self.board.spawn(hold_mino);
        }
    }
}
