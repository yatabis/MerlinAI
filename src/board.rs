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
}
