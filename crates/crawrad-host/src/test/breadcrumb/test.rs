//! The breadcrumb's items: the path segments and the symbol.

use super::{METHOD, TYPE, path_items, symbol_item};
use std::path::Path;

const CODE: &str = "use x;\n\nimpl FormMain {\n    pub fn btn_ok_click(&mut self) {\n        self.a = 1;\n    }\n}\n\nfn helper<T>(t: T) {}\n";

/// **Inside a method, the method; on the impl's line, the type; above
/// everything, nothing; a generic fn's name stops at `<`.**
#[test]
fn the_nearest_symbol_above_the_caret() {
    assert_eq!(
        symbol_item(CODE, 4),
        Some(format!("{METHOD} btn_ok_click()"))
    );
    assert_eq!(symbol_item(CODE, 2), Some(format!("{TYPE} impl FormMain")));
    assert_eq!(symbol_item(CODE, 0), None);
    assert_eq!(symbol_item(CODE, 8), Some(format!("{METHOD} helper()")));
}

/// Inside the project: relative to its root, every segment.
#[test]
fn a_file_in_the_project_is_relative_to_its_root() {
    let root = Path::new("C:/work/calc");
    let file = Path::new("C:/work/calc/app/src/form_main.rs");
    assert_eq!(
        path_items(file, Some(root)),
        vec!["🦀 app", "src", "form_main.rs"]
    );
}

/// Outside it: the last four parts, never the drive; a non-Rust file is 📄.
#[test]
fn a_file_outside_keeps_its_last_four_parts() {
    let file = Path::new("C:/a/b/c/d/e/notes.md");
    assert_eq!(path_items(file, None), vec!["📄 c", "d", "e", "notes.md"]);
    assert_eq!(path_items(Path::new("C:/x.rs"), None), vec!["🦀 x.rs"]);
}
