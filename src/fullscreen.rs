/// Represents the possible fullscreen states of a window.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Fullscreen {
    // Windowed (not fullscreen)
    Off,
    /// Fullscreen, with the resolution of the window
    On,
    /// Fullscreen, with the native resolution of the display
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
