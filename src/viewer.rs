use std::io::Write;
use std::process::ChildStdin;
use crate::board::{Bitboard, Mino};

use crate::game::Game;

pub struct Viewer {
    mino: Mino,
    field: [u8; 240],
    current: [u8; 240],
    ghost: [u8; 240],
}

impl Viewer {
    pub fn new() -> Viewer {
        Viewer {
            mino: Mino::None,
            field: [0; 240],
            current: [0; 240],
            ghost: [0; 240],
        }
    }

    pub fn update(&mut self, game: &mut Game) {
        self.mino = game.board.current.mino;
        let ghost = game.board.ghost();
        for i in 0..240 {
            self.current[i] = if game.board.current.board[i / 60] >> i % 60 & 1 > 0 { self.mino as u8 + 1 } else { 0 };
            self.ghost[i] = if ghost[i / 60] >> i % 60 & 1 > 0 { self.mino as u8 + 8 } else { 0 };
        }
    }
    
    pub fn ground(&mut self, grounded: Bitboard) {
        for i in 0..240 {
            if grounded[i / 60] >> i % 60 & 1 > 0 {
                self.field[i] = self.mino as u8 + 1;
            }
        }
    }

    pub fn clear_lines(&mut self) {
        let mut line = 0;
        for i in 0..24 {
            if self.field[i * 10..(i + 1) * 10].iter().min() > Some(&0) { continue; }
            for j in 0..10 {
                self.field[line * 10 + j] = self.field[i * 10 + j];
            }
            line += 1;
        }
        for i in line * 10..240 {
            self.field[i] = 0;
        }
    }

    pub fn write(&mut self, game: &Game, input: &mut ChildStdin) {
        let mut map = String::new();
        for i in (0..21).rev() {
            for j in 0..10 {
                let mut cell = self.current[i * 10 + j];
                if cell == 0 { cell = self.ghost[i * 10 + j]; }
                if cell == 0 { cell = self.field[i * 10 + j]; }
                map += &(cell.to_string() + ",");
            }
        }
        map += &format!(
            "{},{},{},{},{},{},{},{},{},{}",
            game.hold as usize + 1,
            game.next[0] as usize + 1,
            game.next[1] as usize + 1,
            game.next[2] as usize + 1,
            game.next[3] as usize + 1,
            game.next[4] as usize + 1,
            game.attacks,
            game.back_to_back as usize,
            game.ren,
            game.effect,
        );
        write!(input, "{}\n", map).unwrap();
    }
}
