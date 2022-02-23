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
    Unknown,
}

impl Key {
    pub fn new(code: &str) -> Key {
        match code {
            "Left" => Key::Left,
            "Right" => Key::Right,
            "Clockwise" => Key::Clockwise,
            "Counterclockwise" => Key::Counterclockwise,
            "SoftDrop" => Key::SoftDrop,
            "HardDrop" => Key::HardDrop,
            "Hold" => Key::Hold,
            "Retry" => Key::Retry,
            "Exit" => Key::Exit,
            _ => Key::Unknown,
        }
    }
}
