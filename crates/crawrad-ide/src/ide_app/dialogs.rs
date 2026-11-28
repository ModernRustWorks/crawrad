//! THE DIALOG CATALOG of the shell

#[path = "dialogs/acp_diagnostic.rs"]
pub mod acp_diagnostic;
#[path = "dialogs/actions.rs"]
mod actions;
#[path = "dialogs/build.rs"]
pub mod build;
#[path = "dialogs/build_run.rs"]
pub mod build_run;
#[path = "dialogs/color.rs"]
mod color;
#[path = "dialogs/dsl.rs"]
pub mod dsl;
#[path = "dialogs/exit.rs"]
mod exit;
pub(crate) use exit::AfterSaving;
#[path = "dialogs/environment.rs"]
pub mod environment;
#[path = "dialogs/fields.rs"]
mod fields;
#[path = "dialogs/icons.rs"]
mod icons;
#[path = "dialogs/imagelist.rs"]
mod imagelist;
#[path = "dialogs/inherit_form.rs"]
mod inherit_form;
#[path = "dialogs/items.rs"]
mod items;
#[path = "dialogs/lines.rs"]
mod lines;
#[path = "dialogs/mcp.rs"]
mod mcp;
#[path = "dialogs/memory.rs"]
mod memory;
#[path = "dialogs/new_component.rs"]
mod new_component;
#[path = "dialogs/new_project.rs"]
pub mod new_project;
#[path = "dialogs/notes.rs"]
mod notes;
#[path = "dialogs/options.rs"]
pub mod options;
#[path = "dialogs/pad.rs"]
mod pad;
#[path = "dialogs/permission.rs"]
mod permission;
#[path = "dialogs/project_options.rs"]
pub mod project_options;
#[path = "dialogs/quick_open.rs"]
pub mod quick_open;
#[path = "dialogs/shrink.rs"]
pub mod shrink;
#[path = "dialogs/templates.rs"]
mod templates;
