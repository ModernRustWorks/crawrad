pub mod designer;
pub mod editor;
pub mod project;

pub use designer::{ComponentInfo, ComponentRect, DesignerService};
pub use editor::{EditorSelection, EditorService, TextEdit, TextRange};
pub use project::{FormInfo, ProjectService};
