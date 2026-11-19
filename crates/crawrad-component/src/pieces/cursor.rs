use std::fmt;
use std::str::FromStr;

/// Mouse cursor shape for UI elements.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum Cursor {
    /// Default system arrow or contextual cursor.
    #[default]
    Default,
    /// Standard pointing arrow.
    Arrow,
    /// Pointing hand, typically for links and clickable items.
    Hand,
    /// I-beam text selection cursor.
    Text,
    /// Crosshair cursor for precision selection.
    Crosshair,
    /// Move or 4-way pan cursor.
    Move,
    /// Wait or busy spinner cursor.
    Wait,
    /// Action not allowed or forbidden cursor.
    NotAllowed,
    /// Vertical resize cursor (North-South).
    ResizeNs,
    /// Horizontal resize cursor (East-West).
    ResizeEw,
    /// Diagonal resize cursor (NorthEast-SouthWest).
    ResizeNesw,
    /// Diagonal resize cursor (NorthWest-SouthEast).
    ResizeNwse,
}

impl Cursor {
    /// Returns the static string representation of this cursor.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Arrow => "arrow",
            Self::Hand => "hand",
            Self::Text => "text",
            Self::Crosshair => "crosshair",
            Self::Move => "move",
            Self::Wait => "wait",
            Self::NotAllowed => "not_allowed",
            Self::ResizeNs => "resize_ns",
            Self::ResizeEw => "resize_ew",
            Self::ResizeNesw => "resize_nesw",
            Self::ResizeNwse => "resize_nwse",
        }
    }

    /// Parses a cursor from a string slice.
    pub fn from_str_name(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "default" => Some(Self::Default),
            "arrow" => Some(Self::Arrow),
            "hand" | "pointer" => Some(Self::Hand),
            "text" | "ibeam" => Some(Self::Text),
            "crosshair" | "cross" => Some(Self::Crosshair),
            "move" | "size_all" | "sizeall" => Some(Self::Move),
            "wait" | "busy" | "hourglass" => Some(Self::Wait),
            "not_allowed" | "nodrop" | "forbidden" => Some(Self::NotAllowed),
            "resize_ns" | "sizens" | "row_resize" => Some(Self::ResizeNs),
            "resize_ew" | "sizewe" | "col_resize" => Some(Self::ResizeEw),
            "resize_nesw" | "sizenesw" => Some(Self::ResizeNesw),
            "resize_nwse" | "sizenwse" => Some(Self::ResizeNwse),
            _ => None,
        }
    }
}

impl fmt::Display for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Cursor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_name(s).ok_or(())
    }
}
