#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Fullscreen {
    Off,
    On,
    Desktop,
}

impl From<bool> for Fullscreen {
    fn from(val: bool) -> Self {
        if val {
            Self::On
        } else {
            Self::Off
        }
    }
}
