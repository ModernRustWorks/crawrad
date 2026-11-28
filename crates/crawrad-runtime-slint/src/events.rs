//! Event notification and router types for Slint generated components.

/// Key event information forwarded from Slint callbacks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEventArgs {
    pub key: String,
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
}

impl KeyEventArgs {
    pub fn new(key: impl Into<String>, shift: bool, control: bool, alt: bool) -> Self {
        Self {
            key: key.into(),
            shift,
            control,
            alt,
        }
    }
}
