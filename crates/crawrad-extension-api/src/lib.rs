#![forbid(unsafe_code)]

//! # CrawRAD Extension API
//!
//! Declarative contracts, traits, and manifest specifications for extending the
//! CrawRAD IDE with custom menus, commands, dockable cards, tools, and lifecycle hooks.
//!
//! Inspired by the classic RAD IDE's open tools API, but fully safe, manifest-first, and idiomatic Rust.

pub mod contribution;
pub mod error;
pub mod manifest;
pub mod marks;
pub mod protocol;
pub mod services;
pub mod traits;
pub mod ui;
pub mod views;

#[cfg(test)]
mod tests;

pub use contribution::{
    ActionContribution, CardArea, CardContribution, MenuContribution, MenuPosition,
    ProjectTemplateContribution, ToolContribution,
};
pub use error::ExtensionError;
pub use manifest::{ExtensionContributions, ExtensionManifest};
pub use services::{
    ComponentInfo, ComponentRect, DesignerService, EditorSelection, EditorService, FormInfo,
    ProjectService, TextEdit, TextRange,
};
pub use traits::{CommandHandler, ExtensionContext, IdeExtension, MessageLevel};
pub use ui::{
    CloseCardParams, ContainerLayout, RenderCardParams, UiElementKind, UiElementPatch, UiEventKind,
    UiEventNotification, UiListItem, UiNode, UiTreeNode, UpdateElementParams,
};
pub use views::{ItemClicked, ItemLocation, SetTreeParams, ViewContribution, ViewItem};
