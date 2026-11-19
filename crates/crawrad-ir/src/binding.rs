//! Declarative data binding definitions for the `.crawform` format (ADR 0041).

use crawrad_note::note;
use serde::{Deserialize, Serialize};

#[note("binding-bindingmode")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BindingMode {
    /// Read-only in UI: Source -> Target.
    #[default]
    OneWay,
    /// Bidirectional: Source <-> Target.
    TwoWay,
    /// Write-only from UI: Target -> Source.
    OneWayToSource,
    /// Initial one-time copy on form load.
    OneTime,
}

impl BindingMode {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::OneWay => "one_way",
            Self::TwoWay => "two_way",
            Self::OneWayToSource => "one_way_to_source",
            Self::OneTime => "one_time",
        }
    }
}

#[note("binding-bindingdef")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BindingDef {
    /// Source data component or model identifier (e.g. "ds_clients", "customer_model").
    pub source: String,
    /// Source field or column name (e.g. "NAME", "price"). Omitted for full collection bindings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// Binding synchronization mode.
    #[serde(default)]
    pub mode: BindingMode,
    /// Optional converter identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub converter: Option<String>,
}

impl BindingDef {
    #[note("binding-bindingdef-new")]
    pub fn new(source: impl Into<String>, mode: BindingMode) -> Self {
        Self {
            source: source.into(),
            field: None,
            mode,
            converter: None,
        }
    }

    #[note("binding-bindingdef-with-field")]
    pub fn with_field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }

    #[note("binding-bindingdef-with-converter")]
    pub fn with_converter(mut self, converter: impl Into<String>) -> Self {
        self.converter = Some(converter.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binding_serialization_roundtrip() {
        let b = BindingDef::new("ds_orders", BindingMode::TwoWay)
            .with_field("TOTAL")
            .with_converter("CurrencyConverter");

        let json = serde_json::to_string(&b).unwrap();
        let deserialized: BindingDef = serde_json::from_str(&json).unwrap();
        assert_eq!(b, deserialized);
    }
}
