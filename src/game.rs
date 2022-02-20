use std::collections::VecDeque;

use crate::board::{Bitboard, Board, Mino, TSpin};

const PERFECT_CLEAR: i32 = 10;
const DOUBLE: i32 = 1;
const TRIPLE: i32 = 2;
const TETRIS: i32 = 4;
const T_SPIN_SINGLE: i32 = 2;
const T_SPIN_DOUBLE: i32 = 4;
const T_SPIN_TRIPLE: i32 = 6;
const REN: [i32; 12] = [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 4, 5];

pub struct Game {
    pub board: Board,
    pub hold: Mino,
    pub next: VecDeque<Mino>,
    pub bag: [bool; 7],
    pub back_to_back: bool,
    pub ren: usize
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

    pub fn hard_drop(&mut self) -> Bitboard {
        let grounded = self.board.ground();
        let clears = self.board.clear_lines();
        if clears == 0 {
            self.ren = 0;
        }
        let attacks = self.calc_attacks(clears);
        println!("attack: {attacks}");
        if clears > 0 {
            self.back_to_back = clears == 4 || self.board.t_spin != TSpin::None;
        }
        if clears > 0 {
            self.ren += 1;
        }
        self.board.spawn(self.next.pop_front().unwrap());
        grounded
    }

    fn calc_attacks(&mut self, clears: u32) -> i32 {
        if self.board.field[0] == 0 &&self.board.field[1] == 0 &&self.board.field[2] == 0 &&self.board.field[3] == 0 {
            println!("Perfect Clear");
            return PERFECT_CLEAR;
        }
        let mut attacks = if self.board.t_spin == TSpin::Normal {
            match clears {
                1 => T_SPIN_SINGLE,
                2 => T_SPIN_DOUBLE,
                3 => T_SPIN_TRIPLE,
                _ => 0,
            }
        } else {
            match clears {
                2 => DOUBLE,
                3 => TRIPLE,
                4 => TETRIS + self.back_to_back as i32,
                _ => 0,
            }
        };
        let mut text = if clears == 4 {
            "Tetris ".to_string()
        } else {
            match self.board.t_spin {
                TSpin::Normal => "T-Spin ".to_string() + ["", "Single ", "Double ", "Triple "][clears as usize],
                TSpin::Mini => "T-Spin mini ".to_string() + ["", "Single ", "Double ", "Triple "][clears as usize],
                TSpin::None => "".to_string(),
            }
        };
        if self.back_to_back && clears == 4 {
            text += "Back to Back ";
        }
        if self.board.t_spin != TSpin::None && self.back_to_back {
            attacks += 1;
            text += "Back to Back ";
        }
        attacks += if self.ren < REN.len() { REN[self.ren] } else {REN[REN.len() - 1]};
        if self.ren > 0 {
            text += &format!("{}REN", self.ren);
        }
        if text != "" {
            println!("{text}");
        }
        attacks
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

    pub fn game_over(&self, grounded: Bitboard) -> bool {
        if self.board.current.board[3] & self.board.field[3] > 0 {
            return true;
        }
        if grounded[0] > 0 { return false; }
        if grounded[1] > 0 { return false; }
        if grounded[2] > 0 { return false; }
        if grounded[3] & 0x00000000000FFFFF > 0 { return false; }
        true
    }
}
