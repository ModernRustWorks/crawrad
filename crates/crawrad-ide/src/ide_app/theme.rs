//! The **Makepad** shell theme — the single place deciding its appearance.
//!
//! This file is the entry point for the Makepad theme, partitioned into thematic
//! submodules under `theme/` according to ADR 0043 to respect the strict ceiling of
//! 1,500 lines per file.
//!
//! # Submodules
//!
//! - `tokens`: Palette colors, spacing metrics, typography, and icons.
//! - `widgets`: Visual primitives, buttons, labels, status bars, and menus.
//! - `class_icons`: Makepad shader drawing component icons (Toolbox/Structure).
//! - `workspace`: Rows with icons, tabs, text inputs, scrollbars, and chat cards.
//! - `inspector_widgets`: Object Inspector property rows and editors, pad, and alignment.
//! - `dialog_templates`: Modal dialog templates and tokens, ImageList, Options, and New Project.
//! - `explorer_dock`: Explorer rows, status badges, Package Manager, and Dock.

use makepad_widgets::*;

#[path = "theme/class_icons.rs"]
pub mod class_icons;
#[path = "theme/dialog_templates.rs"]
pub mod dialog_templates;
#[path = "theme/explorer_dock.rs"]
pub mod explorer_dock;
#[path = "theme/inspector_widgets.rs"]
pub mod inspector_widgets;
#[path = "theme/tokens.rs"]
pub mod tokens;
#[path = "theme/widgets.rs"]
pub mod widgets;
#[path = "theme/workspace.rs"]
pub mod workspace;

pub fn live_design(cx: &mut Cx) {
    tokens::live_design(cx);
    widgets::live_design(cx);
    class_icons::live_design(cx);
    workspace::live_design(cx);
    inspector_widgets::live_design(cx);
    dialog_templates::live_design(cx);
    explorer_dock::live_design(cx);
}

use crawrad_note::note;
use crawrad_project::workspace::EditorTokenColors;
use makepad_widgets::{Vec4, vec4};

#[inline]
pub fn rgba_to_vec4(c: [f32; 4]) -> Vec4 {
    vec4(c[0], c[1], c[2], c[3])
}

#[note("ide-app-theme-editortokencolorsvec")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EditorTokenColorsVec {
    pub branch_keyword: Vec4,
    pub loop_keyword: Vec4,
    pub other_keyword: Vec4,
    pub typename: Vec4,
    pub function: Vec4,
    pub identifier: Vec4,
    pub comment: Vec4,
    pub number: Vec4,
    pub string: Vec4,
    pub constant: Vec4,
    pub delimiter: Vec4,
    pub delimiter_highlight: Vec4,
    pub punctuator: Vec4,
    pub bg: Vec4,
}

impl From<EditorTokenColors> for EditorTokenColorsVec {
    fn from(c: EditorTokenColors) -> Self {
        Self {
            branch_keyword: rgba_to_vec4(c.branch_keyword),
            loop_keyword: rgba_to_vec4(c.loop_keyword),
            other_keyword: rgba_to_vec4(c.other_keyword),
            typename: rgba_to_vec4(c.typename),
            function: rgba_to_vec4(c.function),
            identifier: rgba_to_vec4(c.identifier),
            comment: rgba_to_vec4(c.comment),
            number: rgba_to_vec4(c.number),
            string: rgba_to_vec4(c.string),
            constant: rgba_to_vec4(c.constant),
            delimiter: rgba_to_vec4(c.delimiter),
            delimiter_highlight: rgba_to_vec4(c.delimiter_highlight),
            punctuator: rgba_to_vec4(c.punctuator),
            bg: rgba_to_vec4(c.bg),
        }
    }
}
