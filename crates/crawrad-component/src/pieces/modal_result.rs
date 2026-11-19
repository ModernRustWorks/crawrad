use std::fmt;
use std::str::FromStr;

/// Dialog and modal window result codes.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum ModalResult {
    /// No modal result (window remains open).
    #[default]
    None,
    /// Confirmation result (OK button).
    Ok,
    /// Cancellation result (Cancel button or ESC key).
    Cancel,
    /// Abort operation result.
    Abort,
    /// Retry operation result.
    Retry,
    /// Ignore error result.
    Ignore,
    /// Affirmative response (Yes button).
    Yes,
    /// Negative response (No button).
    No,
    /// Close window result.
    Close,
}

impl ModalResult {
    /// Returns the static string representation of this modal result.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Ok => "ok",
            Self::Cancel => "cancel",
            Self::Abort => "abort",
            Self::Retry => "retry",
            Self::Ignore => "ignore",
            Self::Yes => "yes",
            Self::No => "no",
            Self::Close => "close",
        }
    }

    /// Parses a modal result from a string slice.
    pub fn from_str_name(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "none" | "mrnone" => Some(Self::None),
            "ok" | "mrok" => Some(Self::Ok),
            "cancel" | "mrcancel" => Some(Self::Cancel),
            "abort" | "mrabort" => Some(Self::Abort),
            "retry" | "mrretry" => Some(Self::Retry),
            "ignore" | "mrignore" => Some(Self::Ignore),
            "yes" | "mryes" => Some(Self::Yes),
            "no" | "mrno" => Some(Self::No),
            "close" | "mrclose" => Some(Self::Close),
            _ => None,
        }
    }

    /// Converts this modal result to its traditional integer code.
    pub const fn as_i32(&self) -> i32 {
        match self {
            Self::None => 0,
            Self::Ok => 1,
            Self::Cancel => 2,
            Self::Abort => 3,
            Self::Retry => 4,
            Self::Ignore => 5,
            Self::Yes => 6,
            Self::No => 7,
            Self::Close => 8,
        }
    }

    /// Resolves a modal result from its traditional integer code.
    pub const fn from_i32(val: i32) -> Option<Self> {
        match val {
            0 => Some(Self::None),
            1 => Some(Self::Ok),
            2 => Some(Self::Cancel),
            3 => Some(Self::Abort),
            4 => Some(Self::Retry),
            5 => Some(Self::Ignore),
            6 => Some(Self::Yes),
            7 => Some(Self::No),
            8 => Some(Self::Close),
            _ => None,
        }
    }

    /// Returns true if this is None (no modal result).
    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns true if this is an OK or Yes affirmative result.
    pub const fn is_affirmative(&self) -> bool {
        matches!(self, Self::Ok | Self::Yes)
    }

    /// Returns true if this is a Cancel or Abort result.
    pub const fn is_dismissive(&self) -> bool {
        matches!(self, Self::Cancel | Self::Abort | Self::Close)
    }
}

impl fmt::Display for ModalResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ModalResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_name(s).ok_or(())
    }
}
