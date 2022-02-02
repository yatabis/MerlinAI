#[derive(Debug, Eq, PartialEq)]
pub enum  Key {
    Left,
    Right,
    Clockwise,
    CounterClockwise,
    SoftDrop,
    HardDrop,
    Hold,
    Exit,
    None,
    Unknown,
}

impl Key {
    pub fn new(code: [libc::c_char; 1]) -> Key {
        match code[0] {
            115 => Key::Left,
            102 => Key::Right,
            106 => Key::Clockwise,
            108 => Key::CounterClockwise,
            100 => Key::SoftDrop,
            32 => Key::HardDrop,
            59 | 97 => Key::Hold,
            113 => Key::Exit,
            0 => Key::None,
            _ => Key::Unknown,
        }
    }
}
