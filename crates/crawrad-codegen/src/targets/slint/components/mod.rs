//! Slint component mapping and registry module.

pub mod bevel;
pub mod bit_btn;
pub mod button;
pub mod checkbox;
pub mod codegen;
pub mod combobox;
pub mod edit;
pub mod group_box;
pub mod image;
pub mod label;
pub mod link_label;
pub mod list_box;
pub mod memo;
pub mod message_dialog;
pub mod open_dialog;
pub mod page_control;
pub mod paint_box;
pub mod panel;
pub mod progress_bar;
pub mod radio_button;
pub mod radio_group;
pub mod save_dialog;
pub mod shape;
pub mod speed_button;
pub mod spin_edit;
pub mod timer;
pub mod toggle;
pub mod track_bar;
pub mod util;

pub use codegen::SlintComponentCodegen;

use crate::Erro;
use crawrad_ir::{Document, Node};

static REGISTRY: &[&'static dyn SlintComponentCodegen] = &[
    &button::ButtonCodegen,
    &label::LabelCodegen,
    &edit::EditCodegen,
    &memo::MemoCodegen,
    &checkbox::CheckBoxCodegen,
    &toggle::ToggleCodegen,
    &combobox::ComboBoxCodegen,
    &list_box::ListBoxCodegen,
    &progress_bar::ProgressBarCodegen,
    &track_bar::TrackBarCodegen,
    &spin_edit::SpinEditCodegen,
    &panel::PanelCodegen,
    &group_box::GroupBoxCodegen,
    &radio_button::RadioButtonCodegen,
    &radio_group::RadioGroupCodegen,
    &page_control::PageControlCodegen,
    &page_control::TabSheetCodegen,
    &speed_button::SpeedButtonCodegen,
    &bit_btn::BitBtnCodegen,
    &link_label::LinkLabelCodegen,
    &image::ImageCodegen,
    &shape::ShapeCodegen,
    &paint_box::PaintBoxCodegen,
    &bevel::BevelCodegen,
    &timer::TimerCodegen,
    &open_dialog::OpenDialogCodegen,
    &save_dialog::SaveDialogCodegen,
    &message_dialog::MessageDialogCodegen,
];

pub fn for_class(class: &str) -> Option<&'static dyn SlintComponentCodegen> {
    REGISTRY.iter().copied().find(|c| c.class_name() == class)
}

pub fn classes() -> Vec<&'static str> {
    REGISTRY.iter().map(|c| c.class()).collect()
}

pub fn emit_children(
    doc: &Document,
    reg: &crawrad_registry::Registry,
    parent: &Node,
    indent: &str,
) -> Result<String, Erro> {
    let mut out = String::new();
    for child_id in &parent.children {
        if let Some(child) = doc.node(child_id) {
            let effective = reg.effective_class(&child.class);
            if for_class(effective).is_none() {
                continue;
            }
            let child_rect = child
                .rect()
                .unwrap_or(crawrad_ir::Rect::new(0.0, 0.0, 80.0, 24.0));
            let child_slint = emit_slint(
                doc,
                reg,
                child,
                child_rect.x,
                child_rect.y,
                child_rect.w,
                child_rect.h,
            )?;
            for line in child_slint.lines() {
                out.push_str(indent);
                out.push_str(line);
                out.push('\n');
            }
        }
    }
    Ok(out)
}

pub fn component_structs(doc: &Document, reg: &crawrad_registry::Registry) -> String {
    let mut out = String::new();
    let mut used_classes = std::collections::BTreeSet::new();
    for node in doc.nodes.values() {
        used_classes.insert(node.class.as_str());
        used_classes.insert(reg.effective_class(&node.class));
    }

    for class in used_classes {
        if let Some(c) = for_class(class) {
            out.push_str(&c.struct_definition());
            out.push('\n');
        }
    }
    out
}

pub fn emit_slint(
    // Source document
    doc: &Document,
    // Component registry
    reg: &crawrad_registry::Registry,
    // Component node
    node: &Node,
    // Coordinates and dimensions
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> Result<String, Erro> {
    if let Some(c) = for_class(&node.class) {
        c.emit_slint(doc, reg, node, x, y, w, h)
    } else {
        Ok(String::new())
    }
}

pub fn emit_default_com(_reg: &crawrad_registry::Registry, doc: &Document, node: &Node) -> String {
    if let Some(c) = for_class(&node.class) {
        c.emit_default_doc(doc, node)
    } else {
        format!("{}::default()", node.class)
    }
}

pub fn emit_actions(
    // Source document
    doc: &Document,
    // Target component node
    node: &Node,
    // Form struct field name
    field: &str,
    // Emitted nodes
    emitted: &[&Node],
) -> Result<String, Erro> {
    if let Some(c) = for_class(&node.class) {
        c.emit_actions(doc, node, field, emitted)
    } else {
        Ok(String::new())
    }
}

pub fn emit_sync(doc: &Document, node: &Node, field: &str) -> String {
    if let Some(c) = for_class(&node.class) {
        c.emit_sync(doc, node, field)
    } else {
        String::new()
    }
}

pub fn slint_callbacks(node: &Node) -> Vec<String> {
    if let Some(c) = for_class(&node.class) {
        c.slint_callbacks(node)
    } else {
        Vec::new()
    }
}

pub fn slint_properties(node: &Node) -> Vec<String> {
    if let Some(c) = for_class(&node.class) {
        c.slint_properties(node)
    } else {
        Vec::new()
    }
}
