use serde::{Deserialize, Serialize};

/// Target dock area where a contributed card/panel can be placed.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::CardArea;
///
/// let area = CardArea::Bottom;
/// assert_eq!(area, CardArea::Bottom);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardArea {
    /// Left sidebar (like Project Explorer or Structure tree).
    Left,
    /// Right sidebar (like Object Inspector or Palette).
    Right,
    /// Bottom panel (like Output, Terminal, or AI Chat).
    Bottom,
    /// Main central editor area.
    Editor,
}

/// An action that can be invoked via shortcut, menu, or command palette.
///
/// An action contribution that can be invoked via shortcut, menu, or command palette.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::ActionContribution;
///
/// let action = ActionContribution {
///     id: "my_ext.run".to_string(),
///     label: "Run".to_string(),
///     description: None,
///     shortcut: None,
///     icon: None,
/// };
/// assert_eq!(action.id, "my_ext.run");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionContribution {
    /// Semantic identifier (e.g., `"crawrad.git.status"` or `"my_extension.run"`).
    pub id: String,
    /// User-facing display title.
    pub label: String,
    /// Optional detailed description.
    #[serde(default)]
    pub description: Option<String>,
    /// Optional default keyboard shortcut (e.g., `"Ctrl+Shift+P"`).
    #[serde(default)]
    pub shortcut: Option<String>,
    /// Optional icon name or Codicon identifier.
    #[serde(default)]
    pub icon: Option<String>,
}

/// Position where a menu item appears inside a menu.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::MenuPosition;
///
/// let pos = MenuPosition::Bottom;
/// assert_eq!(pos, MenuPosition::Bottom);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum MenuPosition {
    Top,
    #[default]
    Bottom,
    Before(String),
    After(String),
}

/// A menu contribution placing an action into a target menu.
///
/// A menu contribution placing an action into a target top-level menu or context menu.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::{MenuContribution, MenuPosition};
///
/// let menu = MenuContribution {
///     target: "File".to_string(),
///     action_id: "my_ext.open".to_string(),
///     position: MenuPosition::Bottom,
///     group: None,
/// };
/// assert_eq!(menu.target, "File");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MenuContribution {
    /// Target menu name (e.g., `"File"`, `"Edit"`, `"View"`, `"Tools"`, `"Help"`).
    pub target: String,
    /// Action ID that this menu entry triggers.
    pub action_id: String,
    /// Placement position within the menu.
    #[serde(default)]
    pub position: MenuPosition,
    /// Optional menu group for visual separators.
    #[serde(default)]
    pub group: Option<String>,
}

/// A card/panel contribution that adds an independent UI card into the IDE dock.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::{CardArea, CardContribution};
///
/// let card = CardContribution {
///     id: "chat.ai".to_string(),
///     title: "AI Chat".to_string(),
///     area: CardArea::Bottom,
///     icon: None,
///     closeable: true,
/// };
/// assert_eq!(card.id, "chat.ai");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardContribution {
    /// Unique identifier for this card (e.g., `"terminal.integrated"` or `"chat.ai"`).
    pub id: String,
    /// Header title for the card.
    pub title: String,
    /// Default dock area where this card lives.
    pub area: CardArea,
    /// Optional icon for the card tab or activity rail.
    #[serde(default)]
    pub icon: Option<String>,
    /// Whether this card can be minimized into an icon or closed.
    #[serde(default = "default_true")]
    pub closeable: bool,
}

fn default_true() -> bool {
    true
}

/// A tool contribution exposed to AI or scripts (e.g. MCP tool or Python tool).
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::ToolContribution;
///
/// let tool = ToolContribution {
///     name: "query".to_string(),
///     description: "Queries data".to_string(),
///     schema: serde_json::json!({}),
/// };
/// assert_eq!(tool.name, "query");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolContribution {
    /// Tool name used in AI invocations (e.g., `"query_schema"`).
    pub name: String,
    /// What the tool accomplishes.
    pub description: String,
    /// JSON Schema describing tool input parameters.
    pub schema: serde_json::Value,
}

/// A project template contribution providing a scaffolded project model.
///
/// A project template contribution providing a scaffolded project model in the New Project dialog.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::ProjectTemplateContribution;
///
/// let tmpl = ProjectTemplateContribution {
///     id: "template.rust".to_string(),
///     name: "Rust App".to_string(),
///     description: "A simple app".to_string(),
///     icon: None,
///     category: None,
/// };
/// assert_eq!(tmpl.id, "template.rust");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectTemplateContribution {
    /// Semantic identifier (e.g., `"crawrad.formatter.template"`).
    pub id: String,
    /// User-facing display title in the New Project dialog.
    pub name: String,
    /// Brief description of the template.
    pub description: String,
    /// Emoji icon or icon name.
    #[serde(default)]
    pub icon: Option<String>,
    /// Target category index or name (e.g., `"Extensions"` or `"Custom"`).
    #[serde(default)]
    pub category: Option<String>,
}
