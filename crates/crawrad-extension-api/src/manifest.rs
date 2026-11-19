use crate::contribution::{
    ActionContribution, CardContribution, MenuContribution, ProjectTemplateContribution,
    ToolContribution,
};
use crate::error::ExtensionError;
use crate::views::ViewContribution;
use serde::{Deserialize, Serialize};

/// The contributions bundle declared by an extension in its manifest.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::ExtensionContributions;
///
/// let contribs = ExtensionContributions::default();
/// assert!(contribs.actions.is_empty());
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionContributions {
    /// Action commands added by the extension.
    #[serde(default)]
    pub actions: Vec<ActionContribution>,
    /// Menus and menu items contributed.
    #[serde(default)]
    pub menus: Vec<MenuContribution>,
    /// Independent dockable cards contributed.
    #[serde(default)]
    pub cards: Vec<CardContribution>,
    /// AI/Script tools contributed.
    #[serde(default)]
    pub tools: Vec<ToolContribution>,
    /// Project templates contributed.
    #[serde(default)]
    pub templates: Vec<ProjectTemplateContribution>,
    /// Trees drawn in the Extensions panel.
    #[serde(default)]
    pub views: Vec<ViewContribution>,
}

/// The declarative manifest for an extension (`manifest.json` or `extension.json`).
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::{ExtensionContributions, ExtensionManifest};
///
/// let manifest = ExtensionManifest {
///     id: "ext.sample".to_string(),
///     name: "Sample".to_string(),
///     version: "0.1.0".to_string(),
///     publisher: "Acme".to_string(),
///     description: String::new(),
///     entrypoint: None,
///     activation_events: Vec::new(),
///     contributions: ExtensionContributions::default(),
/// };
/// assert_eq!(manifest.id, "ext.sample");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionManifest {
    /// Unique identifier for the extension (e.g., `"crawrad.git-tools"`).
    pub id: String,
    /// Human-friendly display name.
    pub name: String,
    /// Semantic version (e.g., `"0.1.0"`).
    pub version: String,
    /// Publisher or author name.
    pub publisher: String,
    /// Brief description of the extension.
    #[serde(default)]
    pub description: String,
    /// Optional relative path to an external executable or script runner.
    #[serde(default)]
    pub entrypoint: Option<String>,
    /// Activation events triggering extension execution (e.g. `["*"]`, `["onCommand:git.status"]`).
    #[serde(default)]
    pub activation_events: Vec<String>,
    /// Declarative contributions to the IDE.
    #[serde(default)]
    pub contributions: ExtensionContributions,
}

impl ExtensionManifest {
    /// Indicates whether this extension runs as a decoupled external subprocess.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{ExtensionContributions, ExtensionManifest};
    ///
    /// let manifest = ExtensionManifest {
    ///     id: "ext.sample".to_string(),
    ///     name: "Sample".to_string(),
    ///     version: "0.1.0".to_string(),
    ///     publisher: "Acme".to_string(),
    ///     description: String::new(),
    ///     entrypoint: Some("bin/runner".to_string()),
    ///     activation_events: Vec::new(),
    ///     contributions: ExtensionContributions::default(),
    /// };
    /// assert!(manifest.is_external());
    /// ```
    pub fn is_external(&self) -> bool {
        self.entrypoint.is_some()
    }

    /// Checks if this extension declares an activation trigger for the given event.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{ExtensionContributions, ExtensionManifest};
    ///
    /// let manifest = ExtensionManifest {
    ///     id: "ext.sample".to_string(),
    ///     name: "Sample".to_string(),
    ///     version: "0.1.0".to_string(),
    ///     publisher: "Acme".to_string(),
    ///     description: String::new(),
    ///     entrypoint: None,
    ///     activation_events: vec!["onCommand:test".to_string()],
    ///     contributions: ExtensionContributions::default(),
    /// };
    /// assert!(manifest.should_activate_on("onCommand:test"));
    /// assert!(!manifest.should_activate_on("onCommand:other"));
    /// ```
    pub fn should_activate_on(&self, event: &str) -> bool {
        if self.activation_events.is_empty() {
            return true;
        }
        self.activation_events
            .iter()
            .any(|e| e == "*" || e == event)
    }

    /// Parse an extension manifest from a JSON string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::ExtensionManifest;
    ///
    /// let json = r#"{
    ///     "id": "ext.sample",
    ///     "name": "Sample",
    ///     "version": "0.1.0",
    ///     "publisher": "Acme"
    /// }"#;
    /// let manifest = ExtensionManifest::from_json_str(json).unwrap();
    /// assert_eq!(manifest.id, "ext.sample");
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ExtensionError::InvalidManifest`] if JSON deserialization or validation fails.
    pub fn from_json_str(json: &str) -> Result<Self, ExtensionError> {
        let manifest: Self = serde_json::from_str(json)
            .map_err(|e| ExtensionError::InvalidManifest(e.to_string()))?;
        manifest.validate()?;
        Ok(manifest)
    }

    /// Basic structural validation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{ExtensionContributions, ExtensionManifest};
    ///
    /// let manifest = ExtensionManifest {
    ///     id: "ext.sample".to_string(),
    ///     name: "Sample".to_string(),
    ///     version: "0.1.0".to_string(),
    ///     publisher: "Acme".to_string(),
    ///     description: String::new(),
    ///     entrypoint: None,
    ///     activation_events: Vec::new(),
    ///     contributions: ExtensionContributions::default(),
    /// };
    /// assert!(manifest.validate().is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ExtensionError::InvalidManifest`] if required fields are missing or empty.
    pub fn validate(&self) -> Result<(), ExtensionError> {
        if self.id.trim().is_empty() {
            return Err(ExtensionError::InvalidManifest("id cannot be empty".into()));
        }
        if self.name.trim().is_empty() {
            return Err(ExtensionError::InvalidManifest(
                "name cannot be empty".into(),
            ));
        }
        if self.version.trim().is_empty() {
            return Err(ExtensionError::InvalidManifest(
                "version cannot be empty".into(),
            ));
        }
        for action in &self.contributions.actions {
            if action.id.trim().is_empty() {
                return Err(ExtensionError::InvalidManifest(
                    "action id cannot be empty".into(),
                ));
            }
        }
        for menu in &self.contributions.menus {
            if menu.action_id.trim().is_empty() {
                return Err(ExtensionError::InvalidManifest(
                    "menu action_id cannot be empty".into(),
                ));
            }
        }
        for card in &self.contributions.cards {
            if card.id.trim().is_empty() {
                return Err(ExtensionError::InvalidManifest(
                    "card id cannot be empty".into(),
                ));
            }
        }
        for view in &self.contributions.views {
            if view.id.trim().is_empty() {
                return Err(ExtensionError::InvalidManifest(
                    "view id cannot be empty".into(),
                ));
            }
        }
        if let Some(ref ep) = self.entrypoint
            && ep.trim().is_empty()
        {
            return Err(ExtensionError::InvalidManifest(
                "entrypoint cannot be empty if specified".into(),
            ));
        }
        Ok(())
    }
}
