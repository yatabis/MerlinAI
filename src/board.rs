type Bitboard = [u64; 4];

const LEFT_BOUND: u64 = 0x0004010040100401;
const RIGHT_BOUND: u64 = 0x0802008020080200;
const LOWER_BOUND: u64 = 0x00000000000003FF;

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

impl Mino {
    const fn spawn(&self) -> u64 {
        match self {
            Mino::I => 0x0000000007800000,
            Mino::O => 0x0000000C03000000,
            Mino::S => 0x0000000C01800000,
            Mino::Z => 0x0000000603000000,
            Mino::J => 0x0000000203800000,
            Mino::L => 0x0000000803800000,
            Mino::T => 0x0000000403800000,
            _ => 0,
        }
    }

    const fn north(&self) -> u64 {
        match self {
            Mino::I => 0x0000000000F00000,
            Mino::S => 0x0000000000600C00,
            Mino::Z => 0x0000000000301800,
            Mino::J => 0x0000000000101C00,
            Mino::L => 0x0000000000401C00,
            Mino::T => 0x0000000000201C00,
            _ => 0,
        }
    }

    const fn east(&self) -> u64 {
        match self {
            Mino::I => 0x0000000100401004,
            Mino::S => 0x0000000000201804,
            Mino::Z => 0x0000000000401802,
            Mino::J => 0x0000000000600802,
            Mino::L => 0x0000000000200806,
            Mino::T => 0x0000000000201802,
            _ => 0,
        }
    }

    const fn south(&self) -> u64 {
        match self {
            Mino::I => 0x0000000000003C00,
            Mino::S => 0x0000000000001803,
            Mino::Z => 0x0000000000000C06,
            Mino::J => 0x0000000000001C04,
            Mino::L => 0x0000000000001C01,
            Mino::T => 0x0000000000001C02,
            _ => 0,
        }
    }
    const fn west(&self) -> u64 {
        match self {
            Mino::I => 0x0000000080200802,
            Mino::S => 0x0000000000100C02,
            Mino::Z => 0x0000000000200C01,
            Mino::J => 0x0000000000200803,
            Mino::L => 0x0000000000300802,
            Mino::T => 0x0000000000200C02,
            _ => 0,
        }
    }
}

pub const MINO_LIST: [Mino; 7] = [Mino::I, Mino::O, Mino::S, Mino::Z, Mino::J, Mino::L, Mino::T];

#[derive(Eq, PartialEq)]
enum Rotation {
    North,
    East,
    South,
    West,
}

impl Rotation {
    const fn clockwise(&self) -> Rotation {
        match self {
            Rotation::North => Rotation::East,
            Rotation::East => Rotation::South,
            Rotation::South => Rotation::West,
            Rotation::West => Rotation::North,
        }
    }

    const fn counterclockwise(&self) -> Rotation {
        match self {
            Rotation::North => Rotation::West,
            Rotation::East => Rotation::North,
            Rotation::South => Rotation::East,
            Rotation::West => Rotation::South,
        }
    }

    fn srs(&self, next: Rotation, mino: Mino) -> [i16; 5] {
        if mino == Mino::I {
            match self {
                Rotation::North => {
                    match next {
                        Rotation::East => [-2, 3, -13, 33, -21],
                        Rotation::West => [-1, 3, 17, -27, 8],
                        _ => [0, 0, 0, 0, 0],
                    }
                },
                Rotation::East => {
                    match next {
                        Rotation::North => [2, -3, 13, -33, 21],
                        Rotation::South => [-1, 3, 17, -27, 8],
                        _ => [0, 0, 0, 0, 0],
                    }
                },
                Rotation::South => {
                    match next {
                        Rotation::East => [1, -3, -17, 27, -8],
                        Rotation::West => [2, -3, 13, -33, 21],
                        _ => [0, 0, 0, 0, 0],
                    }
                },
                Rotation::West => {
                    match next {
                        Rotation::North => [1, -3, -17, 27, -8],
                        Rotation::South => [-2, 3, -13, 33, -21],
                        _ => [0, 0, 0, 0, 0],
                    }
                },
            }
        } else {
            match self {
                Rotation::East => [1, -10, 29, 1, -21],
                Rotation::West => [-1, -10, 31, -1, -19],
                _ => {
                    if next == Rotation::East {
                        [-1, 10, -29, -1, 21]
                    } else {
                        [1, 10, -31, 1, 19]
                    }
                }
            }
        }
    }
}

struct MinoInfo {
    mino: Mino,
    position: i16,
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
                position: 0,
                rotation: Rotation::North,
                board: [0; 4],
            }
        }
    }

    pub fn spawn(&mut self, mino: Mino) {
        self.current = MinoInfo {
            mino,
            position: if mino == Mino::I { 194 } else { 204 },
            rotation: Rotation::North,
            board: [0, 0, 0, mino.spawn()]
        };
        if self.current.board[3] >> 10 & self.field[3] == 0 {
            self.current.board[3] >>= 10;
            self.current.position -= 10;
        }
    }

    pub fn move_left(&mut self) -> bool {
        if self.current.board[0] > 0 {
            if self.current.board[0] & LEFT_BOUND > 0 { return false; }
            if self.current.board[0] >> 1 & self.field[0] > 0 { return false; }
        }
        if self.current.board[1] > 0 {
            if self.current.board[1] & LEFT_BOUND > 0 { return false; }
            if self.current.board[1] >> 1 & self.field[1] > 0 { return false; }
        }
        if self.current.board[2] > 0 {
            if self.current.board[2] & LEFT_BOUND > 0 { return false; }
            if self.current.board[2] >> 1 & self.field[2] > 0 { return false; }
        }
        if self.current.board[3] > 0 {
            if self.current.board[3] & LEFT_BOUND > 0 { return false; }
            if self.current.board[3] >> 1 & self.field[3] > 0 { return false; }
        }
        self.current.board[0] >>= 1;
        self.current.board[1] >>= 1;
        self.current.board[2] >>= 1;
        self.current.board[3] >>= 1;
        self.current.position -= 1;
        true
    }

    pub fn move_right(&mut self) -> bool {
        if self.current.board[0] > 0 {
            if self.current.board[0] & RIGHT_BOUND > 0 { return false; }
            if self.current.board[0] << 1 & self.field[0] > 0 { return false; }
        }
        if self.current.board[1] > 0 {
            if self.current.board[1] & RIGHT_BOUND > 0 { return false; }
            if self.current.board[1] << 1 & self.field[1] > 0 { return false; }
        }
        if self.current.board[2] > 0 {
            if self.current.board[2] & RIGHT_BOUND > 0 { return false; }
            if self.current.board[2] << 1 & self.field[2] > 0 { return false; }
        }
        if self.current.board[3] > 0 {
            if self.current.board[3] & RIGHT_BOUND > 0 { return false; }
            if self.current.board[3] << 1 & self.field[3] > 0 { return false; }
        }
        self.current.board[0] <<= 1;
        self.current.board[1] <<= 1;
        self.current.board[2] <<= 1;
        self.current.board[3] <<= 1;
        self.current.position += 1;
        true
    }

    pub fn move_down(&mut self) -> bool {
        if self.current.board[0] > 0 {
            if self.current.board[0] & LOWER_BOUND > 0 { return false; }
            if self.current.board[0] >> 10 & self.field[0] > 0 { return false; }
        }
        if self.current.board[1] > 0 {
            if self.current.board[1] << 50 & self.field[0] > 0 { return false; }
            if self.current.board[1] >> 10 & self.field[1] > 0 { return false; }
        }
        if self.current.board[2] > 0 {
            if self.current.board[2] << 50 & self.field[1] > 0 { return false; }
            if self.current.board[2] >> 10 & self.field[2] > 0 { return false; }
        }
        if self.current.board[3] > 0 {
            if self.current.board[3] << 50 & self.field[2] > 0 { return false; }
            if self.current.board[3] >> 10 & self.field[3] > 0 { return false; }
        }
        if self.current.board[0] > 0 {
            self.current.board[0] >>= 10;
        }
        if self.current.board[1] > 0 {
            self.current.board[0] |= (self.current.board[1] & LOWER_BOUND) << 50;
            self.current.board[1] >>= 10;
        }
        if self.current.board[2] > 0 {
            self.current.board[1] |= (self.current.board[2] & LOWER_BOUND) << 50;
            self.current.board[2] >>= 10;
        }
        if self.current.board[3] > 0 {
            self.current.board[2] |= (self.current.board[3] & LOWER_BOUND) << 50;
            self.current.board[3] >>= 10;
        }
        self.current.position -= 10;
        true
    }

    pub fn rotate_clockwise(&mut self) -> bool {
        if self.current.mino == Mino::None || self.current.mino == Mino::O { return false; }
        let mino = match self.current.rotation {
            Rotation::North => self.current.mino.east(),
            Rotation::East => self.current.mino.south(),
            Rotation::South => self.current.mino.west(),
            Rotation::West => self.current.mino.north(),
        };
        let srs = self.current.rotation.srs(self.current.rotation.clockwise(), self.current.mino);
        self.current.rotation = self.current.rotation.clockwise();
        if self.set_rotation(mino) { return true; }
        self.current.position += srs[0];
        if self.set_rotation(mino) { return true; }
        self.current.position += srs[1];
        if self.set_rotation(mino) { return true; }
        self.current.position += srs[2];
        if self.set_rotation(mino) { return true; }
        self.current.position += srs[3];
        if self.set_rotation(mino) { return true; }
        self.current.position += srs[4];
        self.current.rotation = self.current.rotation.counterclockwise();
        false
    }

    pub fn rotate_counterclockwise(&mut self) -> bool {
        if self.current.mino == Mino::None || self.current.mino == Mino::O { return false; }
        let mino = match self.current.rotation {
            Rotation::North => self.current.mino.west(),
            Rotation::East => self.current.mino.north(),
            Rotation::South => self.current.mino.east(),
            Rotation::West => self.current.mino.south(),
        };
        let srs = self.current.rotation.srs(self.current.rotation.counterclockwise(), self.current.mino);
        self.current.rotation = self.current.rotation.counterclockwise();
        if self.set_rotation(mino) { return true; }
        self.current.position += srs[0];
        if self.set_rotation(mino) { return true; }
        self.current.position += srs[1];
        if self.set_rotation(mino) { return true; }
        self.current.position += srs[2];
        if self.set_rotation(mino) { return true; }
        self.current.position += srs[3];
        if self.set_rotation(mino) { return true; }
        self.current.position += srs[4];
        self.current.rotation = self.current.rotation.clockwise();
        false
    }

    fn set_rotation(&mut self, mino: u64) -> bool {
        if self.current.position < 10 { return false; }
        let mut rotated = [0; 4];
        if self.current.position < 11 {
            rotated[0] = mino >> 11 - self.current.position;
        } else if self.current.position < 71 {
            rotated[0] = mino << self.current.position - 11;
            rotated[1] = mino >> 71 - self.current.position;
        } else if self.current.position < 131 {
            rotated[1] = mino << self.current.position - 71;
            rotated[2] = mino >> 131 - self.current.position;
        } else if self.current.position < 191 {
            rotated[2] = mino << self.current.position - 131;
            rotated[3] = mino >> 191 - self.current.position;
        } else {
            rotated[3] = mino << self.current.position - 191;
        }
        if (rotated[0] | rotated[1] | rotated[2] | rotated[3]) & LEFT_BOUND > 0
            && (rotated[0] | rotated[1] | rotated[2] | rotated[3]) & RIGHT_BOUND > 0 {
            return false;
        }
        if rotated[0] & self.field[0] > 0 { return false; }
        if rotated[1] & self.field[1] > 0 { return false; }
        if rotated[2] & self.field[2] > 0 { return false; }
        if rotated[3] & self.field[3] > 0 { return false; }
        self.current.board = rotated;
        return true;
    }

    pub fn ground(&mut self) {
        if self.current.mino == Mino::None { return; }
        loop {
            if !self.move_down() {
                break;
            }
        }
        self.field[0] |= self.current.board[0];
        self.field[1] |= self.current.board[1];
        self.field[2] |= self.current.board[2];
        self.field[3] |= self.current.board[3];
    }
}
