type Bitboard = [u64; 4];

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Mino {
    I,
    O,
    S,
    Z,
    J,
    L,
    T,
    None,
}

pub const MINO_LIST: [Mino; 7] = [Mino::I, Mino::O, Mino::S, Mino::Z, Mino::J, Mino::L, Mino::T];

const I_SPAWN: u64 = 0x0000000007800000;
const O_SPAWN: u64 = 0x0000000C03000000;
const S_SPAWN: u64 = 0x0000000C01800000;
const Z_SPAWN: u64 = 0x0000000603000000;
const J_SPAWN: u64 = 0x0000000203800000;
const L_SPAWN: u64 = 0x0000000803800000;
const T_SPAWN: u64 = 0x0000000403800000;

enum Rotation {
    North,
    East,
    South,
    West,
}

struct MinoInfo {
    mino: Mino,
    rotation: Rotation,
    board: Bitboard,
}

pub struct Board {
    field: Bitboard,
    current: MinoInfo,
}

impl Board {
    pub fn new() -> Board {
        Board {
            field: [0; 4],
            current: MinoInfo {
                mino: Mino::None,
                rotation: Rotation::North,
                board: [0; 4],
            }
        }
    }

    pub fn spawn(&mut self, mino: Mino) {
        self.current = MinoInfo {
            mino,
            rotation: Rotation::North,
            board: [
                0, 0, 0, match mino {
                    Mino::I => I_SPAWN,
                    Mino::O => O_SPAWN,
                    Mino::S => S_SPAWN,
                    Mino::Z => Z_SPAWN,
                    Mino::J => J_SPAWN,
                    Mino::L => L_SPAWN,
                    Mino::T => T_SPAWN,
                    Mino::None => 0,
                }
            ]
        };
        if self.current.board[3] >> 10 & self.field[3] == 0 {
            self.current.board[3] >>= 10;
        }
    }
