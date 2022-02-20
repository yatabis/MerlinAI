pub type Bitboard = [u64; 4];

const LEFT_BOUND: u64 = 0x0004010040100401;
const RIGHT_BOUND: u64 = 0x0802008020080200;
const LOWER_BOUND: u64 = 0x00000000000003FF;
const BOARD_MASK: u64 = 0x0FFFFFFFFFFFFFFF;

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

    pub const fn north(&self) -> u64 {
        match self {
            Mino::I => 0x0000000000F00000,
            Mino::S => 0x0000000000600C00,
            Mino::Z => 0x0000000000301800,
            Mino::J => 0x0000000000101C00,
            Mino::L => 0x0000000000401C00,
            Mino::T => 0x0000000000201C00,
            Mino::O => 0x0000000000601800,
            Mino::None => 0,
        }
    }

    pub const fn east(&self) -> u64 {
        match self {
            Mino::I => 0x0000000100401004,
            Mino::S => 0x0000000000201804,
            Mino::Z => 0x0000000000401802,
            Mino::J => 0x0000000000600802,
            Mino::L => 0x0000000000200806,
            Mino::T => 0x0000000000201802,
            Mino::O => 0x0000000000601800,
            Mino::None => 0,
        }
    }

    pub const fn south(&self) -> u64 {
        match self {
            Mino::I => 0x0000000000003C00,
            Mino::S => 0x0000000000001803,
            Mino::Z => 0x0000000000000C06,
            Mino::J => 0x0000000000001C04,
            Mino::L => 0x0000000000001C01,
            Mino::T => 0x0000000000001C02,
            Mino::O => 0x0000000000601800,
            Mino::None => 0,
        }
    }
    pub const fn west(&self) -> u64 {
        match self {
            Mino::I => 0x0000000080200802,
            Mino::S => 0x0000000000100C02,
            Mino::Z => 0x0000000000200C01,
            Mino::J => 0x0000000000200803,
            Mino::L => 0x0000000000300802,
            Mino::T => 0x0000000000200C02,
            Mino::O => 0x0000000000601800,
            Mino::None => 0,
        }
    }
}

pub const MINO_LIST: [Mino; 7] = [Mino::I, Mino::O, Mino::S, Mino::Z, Mino::J, Mino::L, Mino::T];

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Rotation {
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

#[derive(Copy, Clone)]
pub struct MinoInfo {
    pub mino: Mino,
    pub position: i16,
    pub rotation: Rotation,
    pub board: Bitboard,
}

#[derive(Eq, PartialEq)]
pub enum TSpin {
    Normal,
    Mini,
    None,
}

pub struct Board {
    pub field: Bitboard,
    pub current: MinoInfo,
    pub t_spin: TSpin,
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
            },
            t_spin: TSpin::None,
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
        self.t_spin = TSpin::None;
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
        self.t_spin = TSpin::None;
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
        self.t_spin = TSpin::None;
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
        self.t_spin = TSpin::None;
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
        if self.set_rotation(mino, 0) { return true; }
        self.current.position += srs[0];
        if self.set_rotation(mino, 1) { return true; }
        self.current.position += srs[1];
        if self.set_rotation(mino, 2) { return true; }
        self.current.position += srs[2];
        if self.set_rotation(mino, 3) { return true; }
        self.current.position += srs[3];
        if self.set_rotation(mino, 4) { return true; }
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
        if self.set_rotation(mino, 0) { return true; }
        self.current.position += srs[0];
        if self.set_rotation(mino, 1) { return true; }
        self.current.position += srs[1];
        if self.set_rotation(mino, 2) { return true; }
        self.current.position += srs[2];
        if self.set_rotation(mino, 3) { return true; }
        self.current.position += srs[3];
        if self.set_rotation(mino, 4) { return true; }
        self.current.position += srs[4];
        self.current.rotation = self.current.rotation.clockwise();
        false
    }

    fn set_rotation(&mut self, mino: u64, srs_index: usize) -> bool {
        if self.current.rotation != Rotation::North && self.current.position < 10 { return false; }
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
        self.t_spin_check(srs_index);
        return true;
    }

    fn t_spin_check(&mut self, srs_index: usize) {
        if self.current.mino != Mino::T { return; }
        let normal = 0x0000000000500005;
        let mini = match self.current.rotation {
            Rotation::North => 0x0000000000000005,
            Rotation::East => 0x0000000000100001,
            Rotation::South => 0x0000000000500000,
            Rotation::West => 0x0000000000400004,
        };
        let mut normal_check = [0u64; 4];
        let mut mini_check = [0u64; 4];
        if self.current.position < 11 {
            normal_check[0] = normal >> 11 - self.current.position;
            mini_check[0] = mini >> 11 - self.current.position;
        } else if self.current.position < 71 {
            normal_check[0] = normal << self.current.position - 11;
            normal_check[1] = normal >> 71 - self.current.position;
            mini_check[0] = mini << self.current.position - 11;
            mini_check[1] = mini >> 71 - self.current.position;
        } else if self.current.position < 131 {
            normal_check[1] = normal << self.current.position - 71;
            normal_check[2] = normal >> 131 - self.current.position;
            mini_check[1] = mini << self.current.position - 71;
            mini_check[2] = mini >> 131 - self.current.position;
        } else if self.current.position < 191 {
            normal_check[2] = normal << self.current.position - 131;
            normal_check[3] = normal >> 191 - self.current.position;
            mini_check[2] = mini << self.current.position - 131;
            mini_check[3] = mini >> 191 - self.current.position;
        } else {
            normal_check[3] = normal << self.current.position - 191;
            mini_check[3] = mini << self.current.position - 191;
        }
        normal_check[0] &= BOARD_MASK;
        normal_check[1] &= BOARD_MASK;
        normal_check[2] &= BOARD_MASK;
        normal_check[3] &= BOARD_MASK;
        mini_check[0] &= BOARD_MASK;
        mini_check[1] &= BOARD_MASK;
        mini_check[2] &= BOARD_MASK;
        mini_check[3] &= BOARD_MASK;
        if self.current.position < 10 {
            if (normal_check[0] & self.field[0]).count_ones() == 1 {
                self.t_spin = TSpin::Mini;
            }
        } else if self.current.position % 10 == 0 {
            let count = (normal_check[0] & self.field[0] & !RIGHT_BOUND).count_ones()
                + (normal_check[1] & self.field[1] & !RIGHT_BOUND).count_ones()
                + (normal_check[2] & self.field[2] & !RIGHT_BOUND).count_ones()
                + (normal_check[3] & self.field[3] & !RIGHT_BOUND).count_ones();
            self.t_spin = match count {
                1 => if srs_index == 4 { TSpin::Normal } else { TSpin::Mini },
                2 => TSpin::Normal,
                _ => TSpin::None,
            }
        } else if self.current.position % 10 == 9 {
            let count = (normal_check[0] & self.field[0] & !LEFT_BOUND).count_ones()
                + (normal_check[1] & self.field[1] & !LEFT_BOUND).count_ones()
                + (normal_check[2] & self.field[2] & !LEFT_BOUND).count_ones()
                + (normal_check[3] & self.field[3] & !LEFT_BOUND).count_ones();
            self.t_spin = match count {
                1 => if srs_index == 4 { TSpin::Normal } else { TSpin::Mini },
                2 => TSpin::Normal,
                _ => TSpin::None,
            }
        } else {
            let normal_count = (normal_check[0] & self.field[0]).count_ones()
                + (normal_check[1] & self.field[1]).count_ones()
                + (normal_check[2] & self.field[2]).count_ones()
                + (normal_check[3] & self.field[3]).count_ones();
            let mini_count = (mini_check[0] & self.field[0]).count_ones()
                + (mini_check[1] & self.field[1]).count_ones()
                + (mini_check[2] & self.field[2]).count_ones()
                + (mini_check[3] & self.field[3]).count_ones();
            if normal_count >= 3 {
                self.t_spin = if mini_count < 2 || srs_index == 4 { TSpin::Normal } else { TSpin::Mini }
            }
        }
    }

    pub fn ground(&mut self) -> Bitboard {
        if self.current.mino == Mino::None { return [0; 4]; }
        loop {
            if !self.move_down() {
                break;
            }
        }
        self.field[0] |= self.current.board[0];
        self.field[1] |= self.current.board[1];
        self.field[2] |= self.current.board[2];
        self.field[3] |= self.current.board[3];
        self.current.board.clone()
    }

    pub fn clear_lines(&mut self) -> u32 {
        let mut clear = 0;
        let mut field = [0; 4];
        let mut line = 0;
        for i in 0..4 {
            for j in 0..6 {
                let current = self.field[i] >> 10 * j & LOWER_BOUND;
                if current == 0 {
                    self.field = field;
                    return clear;
                }
                if current == LOWER_BOUND {
                    clear += 1;
                } else {
                    field[line / 60] |= current << line % 60;
                    line += 10;
                }
            }
        }
        self.field = field;
        clear
    }

    pub fn ghost(&mut self) -> Bitboard {
        if self.current.mino == Mino::None { return [0; 4]; }
        let current = self.current;
        loop {
            if !self.move_down() {
                break;
            }
        }
        let ghost = self.current.board;
        self.current = current;
        ghost
    }
}
