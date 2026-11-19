//! Declarative UI protocol for interactive panels and dockable cards in CrawRAD extensions.

use serde::{Deserialize, Serialize};

/// Direction of elements flow inside a container.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::ContainerLayout;
///
/// let layout = ContainerLayout::Vertical;
/// assert_eq!(layout, ContainerLayout::Vertical);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContainerLayout {
    Vertical,
    Horizontal,
}

/// An item inside a declarative list.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::UiListItem;
///
/// let item = UiListItem {
///     id: "item1".to_string(),
///     label: "First".to_string(),
///     secondary: None,
///     icon: None,
/// };
/// assert_eq!(item.id, "item1");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiListItem {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub secondary: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
}

/// A node inside a declarative tree.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::UiTreeNode;
///
/// let node = UiTreeNode {
///     id: "root".to_string(),
///     label: "Root Node".to_string(),
///     icon: None,
///     children: Vec::new(),
///     expanded: false,
/// };
/// assert_eq!(node.id, "root");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTreeNode {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub children: Vec<UiTreeNode>,
    #[serde(default)]
    pub expanded: bool,
}

/// Component type and type-specific attributes for a UI node.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::UiElementKind;
///
/// let kind = UiElementKind::Label {
///     text: "Hello".to_string(),
///     secondary: false,
/// };
/// assert!(matches!(kind, UiElementKind::Label { .. }));
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum UiElementKind {
    Container {
        layout: ContainerLayout,
        children: Vec<UiNode>,
    },
    Label {
        text: String,
        #[serde(default)]
        secondary: bool,
    },
    Button {
        text: String,
        #[serde(default)]
        primary: bool,
    },
    TextInput {
        text: String,
        #[serde(default)]
        placeholder: Option<String>,
    },
    Checkbox {
        label: String,
        checked: bool,
    },
    List {
        items: Vec<UiListItem>,
        #[serde(default)]
        selected_id: Option<String>,
    },
    Tree {
        nodes: Vec<UiTreeNode>,
        #[serde(default)]
        selected_id: Option<String>,
    },
}

/// A declarative UI node rendered in an extension card.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::UiNode;
///
/// let node = UiNode::label("lbl1", "Hello");
/// assert_eq!(node.id, "lbl1");
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UiNode {
    pub id: String,
    #[serde(default = "default_true")]
    pub visible: bool,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub tooltip: Option<String>,
    #[serde(flatten)]
    pub kind: UiElementKind,
}

fn default_true() -> bool {
    true
}

impl UiNode {
    pub fn container(
        id: impl Into<String>,
        layout: ContainerLayout,
        children: Vec<UiNode>,
    ) -> Self {
        Self {
            id: id.into(),
            visible: true,
            enabled: true,
            tooltip: None,
            kind: UiElementKind::Container { layout, children },
        }
    }

    pub fn label(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            visible: true,
            enabled: true,
            tooltip: None,
            kind: UiElementKind::Label {
                text: text.into(),
                secondary: false,
            },
        }
    }

    pub fn button(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            visible: true,
            enabled: true,
            tooltip: None,
            kind: UiElementKind::Button {
                text: text.into(),
                primary: false,
            },
        }
    }

    pub fn text_input(
        id: impl Into<String>,
        text: impl Into<String>,
        placeholder: Option<String>,
    ) -> Self {
        Self {
            id: id.into(),
            visible: true,
            enabled: true,
            tooltip: None,
            kind: UiElementKind::TextInput {
                text: text.into(),
                placeholder,
            },
        }
    }

    pub fn checkbox(id: impl Into<String>, label: impl Into<String>, checked: bool) -> Self {
        Self {
            id: id.into(),
            visible: true,
            enabled: true,
            tooltip: None,
            kind: UiElementKind::Checkbox {
                label: label.into(),
                checked,
            },
        }
    }
}

/// A reactive patch updating element properties without recreating DOM.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::UiElementPatch;
///
/// let patch = UiElementPatch {
///     text: Some("Updated".to_string()),
///     ..Default::default()
/// };
/// assert_eq!(patch.text.as_deref(), Some("Updated"));
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UiElementPatch {
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub visible: Option<bool>,
    #[serde(default)]
    pub checked: Option<bool>,
    #[serde(default)]
    pub items: Option<Vec<UiListItem>>,
    #[serde(default)]
    pub selected_id: Option<String>,
}

/// User interaction events sent from IDE to the extension.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::UiEventKind;
///
/// let ev = UiEventKind::Click;
/// assert_eq!(ev, UiEventKind::Click);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum UiEventKind {
    Click,
    TextChanged {
        text: String,
    },
    CheckedChanged {
        checked: bool,
    },
    SelectionChanged {
        selected_id: String,
    },
    Custom {
        name: String,
        data: serde_json::Value,
    },
}

/// Envelope for UI event notification (IDE -> Extension).
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::{UiEventKind, UiEventNotification};
///
/// let notif = UiEventNotification {
///     card_id: "card1".to_string(),
///     element_id: "btn1".to_string(),
///     event: UiEventKind::Click,
/// };
/// assert_eq!(notif.card_id, "card1");
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UiEventNotification {
    pub card_id: String,
    pub element_id: String,
    #[serde(flatten)]
    pub event: UiEventKind,
}

/// Envelope for Render Card request (Extension -> IDE).
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::{RenderCardParams, UiNode};
///
/// let params = RenderCardParams {
///     card_id: "card1".to_string(),
///     root: UiNode::label("lbl", "Hello"),
/// };
/// assert_eq!(params.card_id, "card1");
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderCardParams {
    pub card_id: String,
    pub root: UiNode,
}

/// Envelope for Update Element request (Extension -> IDE).
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::{UiElementPatch, UpdateElementParams};
///
/// let params = UpdateElementParams {
///     card_id: "card1".to_string(),
///     element_id: "btn1".to_string(),
///     patch: UiElementPatch::default(),
/// };
/// assert_eq!(params.card_id, "card1");
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateElementParams {
    pub card_id: String,
    pub element_id: String,
    pub patch: UiElementPatch,
}

/// Envelope for Close Card request (Extension -> IDE).
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::CloseCardParams;
///
/// let params = CloseCardParams {
///     card_id: "card1".to_string(),
/// };
/// assert_eq!(params.card_id, "card1");
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloseCardParams {
    pub card_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_node_constructors() {
        let label = UiNode::label("lbl_1", "Hello World");
        assert_eq!(label.id, "lbl_1");
        assert!(label.visible);
        assert!(label.enabled);
        match &label.kind {
            UiElementKind::Label { text, secondary } => {
                assert_eq!(text, "Hello World");
                assert!(!secondary);
            }
            _ => panic!("Expected Label"),
        }

        let button = UiNode::button("btn_1", "Submit");
        match &button.kind {
            UiElementKind::Button { text, primary } => {
                assert_eq!(text, "Submit");
                assert!(!primary);
            }
            _ => panic!("Expected Button"),
        }

        let input = UiNode::text_input("txt_1", "initial value", Some("Enter text...".into()));
        match &input.kind {
            UiElementKind::TextInput { text, placeholder } => {
                assert_eq!(text, "initial value");
                assert_eq!(placeholder.as_deref(), Some("Enter text..."));
            }
            _ => panic!("Expected TextInput"),
        }

        let checkbox = UiNode::checkbox("chk_1", "Enable Feature", true);
        match &checkbox.kind {
            UiElementKind::Checkbox { label, checked } => {
                assert_eq!(label, "Enable Feature");
                assert!(checked);
            }
            _ => panic!("Expected Checkbox"),
        }

        let container = UiNode::container(
            "ct_1",
            ContainerLayout::Vertical,
            vec![label, button, input, checkbox],
        );
        match container.kind {
            UiElementKind::Container { layout, children } => {
                assert_eq!(layout, ContainerLayout::Vertical);
                assert_eq!(children.len(), 4);
            }
            _ => panic!("Expected Container"),
        }
    }

    #[test]
    fn test_ui_tree_serialization_roundtrip() {
        let tree = UiNode::container(
            "root",
            ContainerLayout::Horizontal,
            vec![
                UiNode::label("lbl", "Status: Active"),
                UiNode::button("btn_refresh", "Refresh"),
            ],
        );

        let json = serde_json::to_string(&tree).expect("failed to serialize UiNode");
        let parsed: UiNode = serde_json::from_str(&json).expect("failed to deserialize UiNode");
        assert_eq!(parsed, tree);
    }

    #[test]
    fn test_ui_element_patch_serialization() {
        let patch = UiElementPatch {
            text: Some("Updated Text".into()),
            enabled: Some(false),
            visible: Some(true),
            checked: Some(true),
            items: None,
            selected_id: Some("item_2".into()),
        };

        let json = serde_json::to_string(&patch).expect("failed to serialize patch");
        let parsed: UiElementPatch =
            serde_json::from_str(&json).expect("failed to deserialize patch");
        assert_eq!(parsed, patch);
    }

    #[test]
    fn test_ui_event_notification_serialization() {
        let event = UiEventNotification {
            card_id: "my_card".into(),
            element_id: "btn_save".into(),
            event: UiEventKind::Click,
        };

        let json = serde_json::to_string(&event).expect("failed to serialize event");
        let parsed: UiEventNotification =
            serde_json::from_str(&json).expect("failed to deserialize event");
        assert_eq!(parsed.card_id, "my_card");
        assert_eq!(parsed.element_id, "btn_save");
        assert_eq!(parsed.event, UiEventKind::Click);

        let text_event = UiEventNotification {
            card_id: "card_1".into(),
            element_id: "input_search".into(),
            event: UiEventKind::TextChanged {
                text: "query text".into(),
            },
        };
        let text_json = serde_json::to_string(&text_event).expect("failed to serialize text event");
        let parsed_text: UiEventNotification =
            serde_json::from_str(&text_json).expect("failed to deserialize text event");
        assert_eq!(
            parsed_text.event,
            UiEventKind::TextChanged {
                text: "query text".into()
            }
        );
    }
}
