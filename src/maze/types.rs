use std::fmt;

pub enum Action {
    Up,
    Down,
    Right,
    Left,
    Quit,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Action::Up => write!(f, "Up"),
            Action::Down => write!(f, "Down"),
            Action::Right => write!(f, "Right"),
            Action::Left => write!(f, "Left"),
            Action::Quit => write!(f, "Quit"),
        }
    }
}
