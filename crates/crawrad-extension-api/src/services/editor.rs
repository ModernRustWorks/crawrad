use crate::error::ExtensionError;
use serde::{Deserialize, Serialize};

/// Represents a range of text inside an editor document (1-based line and column).
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::TextRange;
///
/// let range = TextRange::new(1, 1, 1, 5);
/// assert_eq!(range.start_line, 1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextRange {
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
}

impl TextRange {
    pub fn new(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Self {
        Self {
            start_line,
            start_col,
            end_line,
            end_col,
        }
    }
}

/// A text edit operation to be applied to an editor buffer.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::{TextEdit, TextRange};
///
/// let edit = TextEdit::new(TextRange::new(1, 1, 1, 5), "hello");
/// assert_eq!(edit.new_text, "hello");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextEdit {
    pub range: TextRange,
    pub new_text: String,
}

impl TextEdit {
    pub fn new(range: TextRange, new_text: impl Into<String>) -> Self {
        Self {
            range,
            new_text: new_text.into(),
        }
    }
}

/// Current cursor or selection state inside the active code editor.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::{EditorSelection, TextRange};
///
/// let sel = EditorSelection {
///     range: TextRange::new(1, 1, 1, 5),
///     selected_text: "hello".to_string(),
/// };
/// assert_eq!(sel.selected_text, "hello");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorSelection {
    pub range: TextRange,
    pub selected_text: String,
}

/// Editor services exposed to extensions.
///
/// Editor services exposed to extensions (analogous to the classic RAD editor services).
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::{EditorSelection, EditorService, ExtensionError, TextEdit};
///
/// struct Dummy;
/// impl EditorService for Dummy {
///     fn active_file(&self) -> Option<String> { Some("main.rs".into()) }
///     fn open_file(&mut self, _: &str, _: Option<(usize, usize)>) -> Result<(), ExtensionError> { Ok(()) }
///     fn get_content(&self, _: &str) -> Result<String, ExtensionError> { Ok(String::new()) }
///     fn get_lines(&self, _: &str, _: usize, _: usize) -> Result<Vec<String>, ExtensionError> { Ok(vec![]) }
///     fn apply_edit(&mut self, _: &str, _: TextEdit) -> Result<(), ExtensionError> { Ok(()) }
///     fn active_selection(&self) -> Option<EditorSelection> { None }
/// }
///
/// let svc = Dummy;
/// assert_eq!(svc.active_file(), Some("main.rs".into()));
/// ```
pub trait EditorService: Send + Sync {
    /// Return the path of the file currently focused in the editor, if any.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{EditorSelection, EditorService, ExtensionError, TextEdit};
    ///
    /// struct Dummy;
    /// impl EditorService for Dummy {
    ///     fn active_file(&self) -> Option<String> { Some("main.rs".into()) }
    ///     fn open_file(&mut self, _: &str, _: Option<(usize, usize)>) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn get_content(&self, _: &str) -> Result<String, ExtensionError> { Ok(String::new()) }
    ///     fn get_lines(&self, _: &str, _: usize, _: usize) -> Result<Vec<String>, ExtensionError> { Ok(vec![]) }
    ///     fn apply_edit(&mut self, _: &str, _: TextEdit) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn active_selection(&self) -> Option<EditorSelection> { None }
    /// }
    ///
    /// let svc = Dummy;
    /// assert_eq!(svc.active_file().as_deref(), Some("main.rs"));
    /// ```
    fn active_file(&self) -> Option<String>;

    /// Open a file in the IDE editor at the specified optional line and column.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{EditorSelection, EditorService, ExtensionError, TextEdit};
    ///
    /// struct Dummy;
    /// impl EditorService for Dummy {
    ///     fn active_file(&self) -> Option<String> { None }
    ///     fn open_file(&mut self, _: &str, _: Option<(usize, usize)>) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn get_content(&self, _: &str) -> Result<String, ExtensionError> { Ok(String::new()) }
    ///     fn get_lines(&self, _: &str, _: usize, _: usize) -> Result<Vec<String>, ExtensionError> { Ok(vec![]) }
    ///     fn apply_edit(&mut self, _: &str, _: TextEdit) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn active_selection(&self) -> Option<EditorSelection> { None }
    /// }
    ///
    /// let mut svc = Dummy;
    /// assert!(svc.open_file("src/lib.rs", Some((10, 1))).is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ExtensionError`] if opening the file fails.
    fn open_file(
        &mut self,
        path: &str,
        position: Option<(usize, usize)>,
    ) -> Result<(), ExtensionError>;

    /// Get the complete textual content of a file buffer currently open or on disk.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{EditorSelection, EditorService, ExtensionError, TextEdit};
    ///
    /// struct Dummy;
    /// impl EditorService for Dummy {
    ///     fn active_file(&self) -> Option<String> { None }
    ///     fn open_file(&mut self, _: &str, _: Option<(usize, usize)>) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn get_content(&self, _: &str) -> Result<String, ExtensionError> { Ok("fn main() {}".into()) }
    ///     fn get_lines(&self, _: &str, _: usize, _: usize) -> Result<Vec<String>, ExtensionError> { Ok(vec![]) }
    ///     fn apply_edit(&mut self, _: &str, _: TextEdit) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn active_selection(&self) -> Option<EditorSelection> { None }
    /// }
    ///
    /// let svc = Dummy;
    /// assert_eq!(svc.get_content("src/main.rs").unwrap(), "fn main() {}");
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ExtensionError`] if reading the file content fails.
    fn get_content(&self, path: &str) -> Result<String, ExtensionError>;

    /// Get a 1-based line slice from an open buffer or file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{EditorSelection, EditorService, ExtensionError, TextEdit};
    ///
    /// struct Dummy;
    /// impl EditorService for Dummy {
    ///     fn active_file(&self) -> Option<String> { None }
    ///     fn open_file(&mut self, _: &str, _: Option<(usize, usize)>) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn get_content(&self, _: &str) -> Result<String, ExtensionError> { Ok(String::new()) }
    ///     fn get_lines(&self, _: &str, _: usize, _: usize) -> Result<Vec<String>, ExtensionError> {
    ///         Ok(vec!["first line".into()])
    ///     }
    ///     fn apply_edit(&mut self, _: &str, _: TextEdit) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn active_selection(&self) -> Option<EditorSelection> { None }
    /// }
    ///
    /// let svc = Dummy;
    /// assert_eq!(svc.get_lines("src/main.rs", 1, 1).unwrap(), vec!["first line"]);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ExtensionError`] if the line range is invalid or reading fails.
    fn get_lines(
        &self,
        path: &str,
        start_line: usize,
        end_line: usize,
    ) -> Result<Vec<String>, ExtensionError>;

    /// Apply a text edit to an open document buffer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{EditorSelection, EditorService, ExtensionError, TextEdit, TextRange};
    ///
    /// struct Dummy;
    /// impl EditorService for Dummy {
    ///     fn active_file(&self) -> Option<String> { None }
    ///     fn open_file(&mut self, _: &str, _: Option<(usize, usize)>) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn get_content(&self, _: &str) -> Result<String, ExtensionError> { Ok(String::new()) }
    ///     fn get_lines(&self, _: &str, _: usize, _: usize) -> Result<Vec<String>, ExtensionError> { Ok(vec![]) }
    ///     fn apply_edit(&mut self, _: &str, _: TextEdit) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn active_selection(&self) -> Option<EditorSelection> { None }
    /// }
    ///
    /// let mut svc = Dummy;
    /// let edit = TextEdit::new(TextRange::new(1, 1, 1, 1), "// comment\n");
    /// assert!(svc.apply_edit("src/main.rs", edit).is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ExtensionError`] if applying the edit fails.
    fn apply_edit(&mut self, path: &str, edit: TextEdit) -> Result<(), ExtensionError>;

    /// Get current selection in the active editor.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{EditorSelection, EditorService, ExtensionError, TextEdit};
    ///
    /// struct Dummy;
    /// impl EditorService for Dummy {
    ///     fn active_file(&self) -> Option<String> { None }
    ///     fn open_file(&mut self, _: &str, _: Option<(usize, usize)>) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn get_content(&self, _: &str) -> Result<String, ExtensionError> { Ok(String::new()) }
    ///     fn get_lines(&self, _: &str, _: usize, _: usize) -> Result<Vec<String>, ExtensionError> { Ok(vec![]) }
    ///     fn apply_edit(&mut self, _: &str, _: TextEdit) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn active_selection(&self) -> Option<EditorSelection> { None }
    /// }
    ///
    /// let svc = Dummy;
    /// assert!(svc.active_selection().is_none());
    /// ```
    fn active_selection(&self) -> Option<EditorSelection>;
}
