#[derive(Debug, Eq, PartialEq)]
pub enum  Key {
    Left,
    Right,
    Clockwise,
    Counterclockwise,
    SoftDrop,
    HardDrop,
    Hold,
    Retry,
    Exit,
    None,
    Unknown,
}

impl Key {
    pub fn new(code: [libc::c_char; 1]) -> Key {
        match code[0] {
            115 => Key::Left,
            102 => Key::Right,
            108 => Key::Clockwise,
            106 => Key::Counterclockwise,
            100 => Key::SoftDrop,
            32 => Key::HardDrop,
            59 | 97 => Key::Hold,
            114 => Key::Retry,
            113 => Key::Exit,
            0 => Key::None,
            _ => Key::Unknown,
        }
    }
}
