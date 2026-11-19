use crate::error::ExtensionError;
use serde::{Deserialize, Serialize};

/// Basic information about a visual form in the project.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::FormInfo;
///
/// let form = FormInfo {
///     name: "MainForm".to_string(),
///     form_file: "src/main_form.crawform".to_string(),
///     code_file: "src/main_form.rs".to_string(),
/// };
/// assert_eq!(form.name, "MainForm");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormInfo {
    /// Form name (e.g., `"MainForm"`).
    pub name: String,
    /// Absolute path to the declarative design file (`.crawform`).
    pub form_file: String,
    /// Absolute path to the companion code file (`.rs`).
    pub code_file: String,
}

/// Project and module management services.
///
/// Project and module management services (analogous to the classic RAD project services).
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::{ExtensionError, FormInfo, ProjectService};
///
/// struct Dummy;
/// impl ProjectService for Dummy {
///     fn active_project_path(&self) -> Option<String> { Some("/path".into()) }
///     fn active_project_name(&self) -> Option<String> { Some("my_app".into()) }
///     fn list_project_files(&self) -> Result<Vec<String>, ExtensionError> { Ok(vec![]) }
///     fn list_forms(&self) -> Result<Vec<FormInfo>, ExtensionError> { Ok(vec![]) }
///     fn add_file(&mut self, _: &str) -> Result<(), ExtensionError> { Ok(()) }
///     fn request_build(&mut self) -> Result<(), ExtensionError> { Ok(()) }
/// }
///
/// let svc = Dummy;
/// assert_eq!(svc.active_project_name(), Some("my_app".into()));
/// ```
pub trait ProjectService: Send + Sync {
    /// Return the root directory path of the active project/workspace.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{ExtensionError, FormInfo, ProjectService};
    ///
    /// struct Dummy;
    /// impl ProjectService for Dummy {
    ///     fn active_project_path(&self) -> Option<String> { Some("/path".into()) }
    ///     fn active_project_name(&self) -> Option<String> { None }
    ///     fn list_project_files(&self) -> Result<Vec<String>, ExtensionError> { Ok(vec![]) }
    ///     fn list_forms(&self) -> Result<Vec<FormInfo>, ExtensionError> { Ok(vec![]) }
    ///     fn add_file(&mut self, _: &str) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn request_build(&mut self) -> Result<(), ExtensionError> { Ok(()) }
    /// }
    ///
    /// let svc = Dummy;
    /// assert_eq!(svc.active_project_path().as_deref(), Some("/path"));
    /// ```
    fn active_project_path(&self) -> Option<String>;

    /// Return the name of the active project crate or application.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{ExtensionError, FormInfo, ProjectService};
    ///
    /// struct Dummy;
    /// impl ProjectService for Dummy {
    ///     fn active_project_path(&self) -> Option<String> { None }
    ///     fn active_project_name(&self) -> Option<String> { Some("my_app".into()) }
    ///     fn list_project_files(&self) -> Result<Vec<String>, ExtensionError> { Ok(vec![]) }
    ///     fn list_forms(&self) -> Result<Vec<FormInfo>, ExtensionError> { Ok(vec![]) }
    ///     fn add_file(&mut self, _: &str) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn request_build(&mut self) -> Result<(), ExtensionError> { Ok(()) }
    /// }
    ///
    /// let svc = Dummy;
    /// assert_eq!(svc.active_project_name().as_deref(), Some("my_app"));
    /// ```
    fn active_project_name(&self) -> Option<String>;

    /// List all source code files belonging to the active project.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{ExtensionError, FormInfo, ProjectService};
    ///
    /// struct Dummy;
    /// impl ProjectService for Dummy {
    ///     fn active_project_path(&self) -> Option<String> { None }
    ///     fn active_project_name(&self) -> Option<String> { None }
    ///     fn list_project_files(&self) -> Result<Vec<String>, ExtensionError> {
    ///         Ok(vec!["src/main.rs".into()])
    ///     }
    ///     fn list_forms(&self) -> Result<Vec<FormInfo>, ExtensionError> { Ok(vec![]) }
    ///     fn add_file(&mut self, _: &str) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn request_build(&mut self) -> Result<(), ExtensionError> { Ok(()) }
    /// }
    ///
    /// let svc = Dummy;
    /// assert_eq!(svc.list_project_files().unwrap(), vec!["src/main.rs"]);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ExtensionError`] if project files cannot be listed.
    fn list_project_files(&self) -> Result<Vec<String>, ExtensionError>;

    /// List all visual forms registered in the active project.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{ExtensionError, FormInfo, ProjectService};
    ///
    /// struct Dummy;
    /// impl ProjectService for Dummy {
    ///     fn active_project_path(&self) -> Option<String> { None }
    ///     fn active_project_name(&self) -> Option<String> { None }
    ///     fn list_project_files(&self) -> Result<Vec<String>, ExtensionError> { Ok(vec![]) }
    ///     fn list_forms(&self) -> Result<Vec<FormInfo>, ExtensionError> { Ok(vec![]) }
    ///     fn add_file(&mut self, _: &str) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn request_build(&mut self) -> Result<(), ExtensionError> { Ok(()) }
    /// }
    ///
    /// let svc = Dummy;
    /// assert!(svc.list_forms().unwrap().is_empty());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ExtensionError`] if forms cannot be listed.
    fn list_forms(&self) -> Result<Vec<FormInfo>, ExtensionError>;

    /// Add a new file to the active project manifest/workspace.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{ExtensionError, FormInfo, ProjectService};
    ///
    /// struct Dummy;
    /// impl ProjectService for Dummy {
    ///     fn active_project_path(&self) -> Option<String> { None }
    ///     fn active_project_name(&self) -> Option<String> { None }
    ///     fn list_project_files(&self) -> Result<Vec<String>, ExtensionError> { Ok(vec![]) }
    ///     fn list_forms(&self) -> Result<Vec<FormInfo>, ExtensionError> { Ok(vec![]) }
    ///     fn add_file(&mut self, _: &str) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn request_build(&mut self) -> Result<(), ExtensionError> { Ok(()) }
    /// }
    ///
    /// let mut svc = Dummy;
    /// assert!(svc.add_file("src/utils.rs").is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ExtensionError`] if the file cannot be added.
    fn add_file(&mut self, relative_or_abs_path: &str) -> Result<(), ExtensionError>;

    /// Request a project build or check.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{ExtensionError, FormInfo, ProjectService};
    ///
    /// struct Dummy;
    /// impl ProjectService for Dummy {
    ///     fn active_project_path(&self) -> Option<String> { None }
    ///     fn active_project_name(&self) -> Option<String> { None }
    ///     fn list_project_files(&self) -> Result<Vec<String>, ExtensionError> { Ok(vec![]) }
    ///     fn list_forms(&self) -> Result<Vec<FormInfo>, ExtensionError> { Ok(vec![]) }
    ///     fn add_file(&mut self, _: &str) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn request_build(&mut self) -> Result<(), ExtensionError> { Ok(()) }
    /// }
    ///
    /// let mut svc = Dummy;
    /// assert!(svc.request_build().is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ExtensionError`] if the build request fails.
    fn request_build(&mut self) -> Result<(), ExtensionError>;
}
