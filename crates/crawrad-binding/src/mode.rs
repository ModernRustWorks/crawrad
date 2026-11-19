//! Canonical binding modes for CrawRAD (ADR 0041).
//!
//! Synchronizing properties between UI components and underlying data models
//! obeys four canonical directions.

use crawrad_note::note;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[note("mode-bindingmode")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum BindingMode {
    /// Read-only on screen: updates in the source automatically propagate to the target (UI).
    #[default]
    OneWay,

    /// Bidirectional synchronization: typing in the UI updates the source;
    /// external changes to the source update the UI.
    TwoWay,

    /// One-way dispatch: target (UI) updates propagate to the source,
    /// without being overwritten by the source.
    OneWayToSource,

    /// One-time initialization: reads from source to target once during binding setup,
    /// without ongoing event subscriptions or observation overhead.
    OneTime,
}

impl BindingMode {
    #[note("mode-bindingmode-propagates-source-to-target")]
    #[inline]
    pub const fn propagates_source_to_target(&self) -> bool {
        matches!(self, Self::OneWay | Self::TwoWay | Self::OneTime)
    }

    #[note("mode-bindingmode-propagates-target-to-source")]
    #[inline]
    pub const fn propagates_target_to_source(&self) -> bool {
        matches!(self, Self::TwoWay | Self::OneWayToSource)
    }

    #[note("mode-bindingmode-is-continuous")]
    #[inline]
    pub const fn is_continuous(&self) -> bool {
        matches!(self, Self::OneWay | Self::TwoWay | Self::OneWayToSource)
    }

    #[note("mode-bindingmode-as-str")]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::OneWay => "OneWay",
            Self::TwoWay => "TwoWay",
            Self::OneWayToSource => "OneWayToSource",
            Self::OneTime => "OneTime",
        }
    }
}

impl std::fmt::Display for BindingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_propagation_rules() {
        assert!(BindingMode::OneWay.propagates_source_to_target());
        assert!(!BindingMode::OneWay.propagates_target_to_source());
        assert!(BindingMode::OneWay.is_continuous());

        assert!(BindingMode::TwoWay.propagates_source_to_target());
        assert!(BindingMode::TwoWay.propagates_target_to_source());
        assert!(BindingMode::TwoWay.is_continuous());

        assert!(!BindingMode::OneWayToSource.propagates_source_to_target());
        assert!(BindingMode::OneWayToSource.propagates_target_to_source());
        assert!(BindingMode::OneWayToSource.is_continuous());

        assert!(BindingMode::OneTime.propagates_source_to_target());
        assert!(!BindingMode::OneTime.propagates_target_to_source());
        assert!(!BindingMode::OneTime.is_continuous());
    }

    #[test]
    fn mode_display() {
        assert_eq!(BindingMode::OneWay.to_string(), "OneWay");
        assert_eq!(BindingMode::TwoWay.to_string(), "TwoWay");
        assert_eq!(BindingMode::OneWayToSource.to_string(), "OneWayToSource");
        assert_eq!(BindingMode::OneTime.to_string(), "OneTime");
    }
}
