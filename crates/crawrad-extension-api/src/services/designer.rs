use crate::error::ExtensionError;
use serde::{Deserialize, Serialize};

/// 2D bounding rectangle of a component in design canvas units.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::ComponentRect;
///
/// let rect = ComponentRect::new(10.0, 20.0, 100.0, 50.0);
/// assert_eq!(rect.width, 100.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ComponentRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl ComponentRect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

/// Information about a component currently placed on a form.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::{ComponentInfo, ComponentRect};
///
/// let info = ComponentInfo {
///     name: "btnOk".to_string(),
///     class_name: "Button".to_string(),
///     parent_name: None,
///     rect: ComponentRect::new(0.0, 0.0, 80.0, 30.0),
///     is_visual: true,
/// };
/// assert_eq!(info.name, "btnOk");
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentInfo {
    /// Instance name (e.g., `"btnSalvar"` or `"gridPrincipal"`).
    pub name: String,
    /// Class name (e.g., `"Button"`, `"Grid"`, `"Edit"`, `"Timer"`).
    pub class_name: String,
    /// Parent component name, or `None` if placed directly on the root form.
    pub parent_name: Option<String>,
    /// Bounding rectangle on the canvas.
    pub rect: ComponentRect,
    /// Whether this component is visual or non-visual.
    pub is_visual: bool,
}

/// Visual designer inspection and modification services.
///
/// Visual designer inspection and modification services (analogous to the classic RAD form editor services).
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::{ComponentInfo, DesignerService, ExtensionError};
///
/// struct MockDesigner;
/// impl DesignerService for MockDesigner {
///     fn active_form_name(&self) -> Option<String> { Some("Form1".into()) }
///     fn list_components(&self) -> Result<Vec<ComponentInfo>, ExtensionError> { Ok(vec![]) }
///     fn get_property(&self, _: &str, _: &str) -> Result<Option<String>, ExtensionError> { Ok(None) }
///     fn set_property(&mut self, _: &str, _: &str, _: &str) -> Result<(), ExtensionError> { Ok(()) }
///     fn selected_components(&self) -> Vec<String> { vec![] }
///     fn select_components(&mut self, _: &[&str]) -> Result<(), ExtensionError> { Ok(()) }
///     fn delete_selected(&mut self) -> Result<(), ExtensionError> { Ok(()) }
/// }
///
/// let designer = MockDesigner;
/// assert_eq!(designer.active_form_name(), Some("Form1".into()));
/// ```
pub trait DesignerService: Send + Sync {
    /// Return the name of the form currently active in the visual designer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{ComponentInfo, DesignerService, ExtensionError};
    ///
    /// struct Dummy;
    /// impl DesignerService for Dummy {
    ///     fn active_form_name(&self) -> Option<String> { Some("MainForm".into()) }
    ///     fn list_components(&self) -> Result<Vec<ComponentInfo>, ExtensionError> { Ok(vec![]) }
    ///     fn get_property(&self, _: &str, _: &str) -> Result<Option<String>, ExtensionError> { Ok(None) }
    ///     fn set_property(&mut self, _: &str, _: &str, _: &str) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn selected_components(&self) -> Vec<String> { vec![] }
    ///     fn select_components(&mut self, _: &[&str]) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn delete_selected(&mut self) -> Result<(), ExtensionError> { Ok(()) }
    /// }
    ///
    /// let svc = Dummy;
    /// assert_eq!(svc.active_form_name().as_deref(), Some("MainForm"));
    /// ```
    fn active_form_name(&self) -> Option<String>;

    /// List all components on the active form.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{ComponentInfo, DesignerService, ExtensionError};
    ///
    /// struct Dummy;
    /// impl DesignerService for Dummy {
    ///     fn active_form_name(&self) -> Option<String> { None }
    ///     fn list_components(&self) -> Result<Vec<ComponentInfo>, ExtensionError> { Ok(vec![]) }
    ///     fn get_property(&self, _: &str, _: &str) -> Result<Option<String>, ExtensionError> { Ok(None) }
    ///     fn set_property(&mut self, _: &str, _: &str, _: &str) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn selected_components(&self) -> Vec<String> { vec![] }
    ///     fn select_components(&mut self, _: &[&str]) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn delete_selected(&mut self) -> Result<(), ExtensionError> { Ok(()) }
    /// }
    ///
    /// let svc = Dummy;
    /// assert!(svc.list_components().unwrap().is_empty());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ExtensionError`] if the active form cannot be inspected.
    fn list_components(&self) -> Result<Vec<ComponentInfo>, ExtensionError>;

    /// Get the serialized string value of a component's property.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{ComponentInfo, DesignerService, ExtensionError};
    ///
    /// struct Dummy;
    /// impl DesignerService for Dummy {
    ///     fn active_form_name(&self) -> Option<String> { None }
    ///     fn list_components(&self) -> Result<Vec<ComponentInfo>, ExtensionError> { Ok(vec![]) }
    ///     fn get_property(&self, _: &str, _: &str) -> Result<Option<String>, ExtensionError> {
    ///         Ok(Some("test".into()))
    ///     }
    ///     fn set_property(&mut self, _: &str, _: &str, _: &str) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn selected_components(&self) -> Vec<String> { vec![] }
    ///     fn select_components(&mut self, _: &[&str]) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn delete_selected(&mut self) -> Result<(), ExtensionError> { Ok(()) }
    /// }
    ///
    /// let svc = Dummy;
    /// assert_eq!(svc.get_property("btn", "text").unwrap().as_deref(), Some("test"));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ExtensionError`] if the component does not exist.
    fn get_property(
        &self,
        component_name: &str,
        property_key: &str,
    ) -> Result<Option<String>, ExtensionError>;

    /// Set a property value on a component in design-time.
    ///
    /// Set a property value on a component in design-time, triggering undo/redo history.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{ComponentInfo, DesignerService, ExtensionError};
    ///
    /// struct Dummy;
    /// impl DesignerService for Dummy {
    ///     fn active_form_name(&self) -> Option<String> { None }
    ///     fn list_components(&self) -> Result<Vec<ComponentInfo>, ExtensionError> { Ok(vec![]) }
    ///     fn get_property(&self, _: &str, _: &str) -> Result<Option<String>, ExtensionError> { Ok(None) }
    ///     fn set_property(&mut self, _: &str, _: &str, _: &str) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn selected_components(&self) -> Vec<String> { vec![] }
    ///     fn select_components(&mut self, _: &[&str]) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn delete_selected(&mut self) -> Result<(), ExtensionError> { Ok(()) }
    /// }
    ///
    /// let mut svc = Dummy;
    /// assert!(svc.set_property("btn", "text", "OK").is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ExtensionError`] if the property cannot be set.
    fn set_property(
        &mut self,
        component_name: &str,
        property_key: &str,
        property_value: &str,
    ) -> Result<(), ExtensionError>;

    /// Return the list of component names currently selected by the user.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{ComponentInfo, DesignerService, ExtensionError};
    ///
    /// struct Dummy;
    /// impl DesignerService for Dummy {
    ///     fn active_form_name(&self) -> Option<String> { None }
    ///     fn list_components(&self) -> Result<Vec<ComponentInfo>, ExtensionError> { Ok(vec![]) }
    ///     fn get_property(&self, _: &str, _: &str) -> Result<Option<String>, ExtensionError> { Ok(None) }
    ///     fn set_property(&mut self, _: &str, _: &str, _: &str) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn selected_components(&self) -> Vec<String> { vec!["btn1".into()] }
    ///     fn select_components(&mut self, _: &[&str]) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn delete_selected(&mut self) -> Result<(), ExtensionError> { Ok(()) }
    /// }
    ///
    /// let svc = Dummy;
    /// assert_eq!(svc.selected_components(), vec!["btn1".to_string()]);
    /// ```
    fn selected_components(&self) -> Vec<String>;

    /// Programmatically select components in the designer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{ComponentInfo, DesignerService, ExtensionError};
    ///
    /// struct Dummy;
    /// impl DesignerService for Dummy {
    ///     fn active_form_name(&self) -> Option<String> { None }
    ///     fn list_components(&self) -> Result<Vec<ComponentInfo>, ExtensionError> { Ok(vec![]) }
    ///     fn get_property(&self, _: &str, _: &str) -> Result<Option<String>, ExtensionError> { Ok(None) }
    ///     fn set_property(&mut self, _: &str, _: &str, _: &str) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn selected_components(&self) -> Vec<String> { vec![] }
    ///     fn select_components(&mut self, _: &[&str]) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn delete_selected(&mut self) -> Result<(), ExtensionError> { Ok(()) }
    /// }
    ///
    /// let mut svc = Dummy;
    /// assert!(svc.select_components(&["btn1"]).is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ExtensionError`] if named components cannot be selected.
    fn select_components(&mut self, names: &[&str]) -> Result<(), ExtensionError>;

    /// Delete selected components from the active form.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{ComponentInfo, DesignerService, ExtensionError};
    ///
    /// struct Dummy;
    /// impl DesignerService for Dummy {
    ///     fn active_form_name(&self) -> Option<String> { None }
    ///     fn list_components(&self) -> Result<Vec<ComponentInfo>, ExtensionError> { Ok(vec![]) }
    ///     fn get_property(&self, _: &str, _: &str) -> Result<Option<String>, ExtensionError> { Ok(None) }
    ///     fn set_property(&mut self, _: &str, _: &str, _: &str) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn selected_components(&self) -> Vec<String> { vec![] }
    ///     fn select_components(&mut self, _: &[&str]) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn delete_selected(&mut self) -> Result<(), ExtensionError> { Ok(()) }
    /// }
    ///
    /// let mut svc = Dummy;
    /// assert!(svc.delete_selected().is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ExtensionError`] if deleting components fails.
    fn delete_selected(&mut self) -> Result<(), ExtensionError>;
}
