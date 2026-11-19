//! Makepad component unified mapping and aggregation module.
//!
//! Each component submodule encapsulates its full lifecycle:
//! - Struct definition (autonomously derived from `crawrad-registry` descriptor)
//! - Default state initialization (`emit_default`)
//! - UI synchronization (`emit_sync`)
//! - Action/event dispatch (`emit_actions`)
//! - Makepad DSL emission (`emit_dsl`)

pub mod action_list;
pub mod badge;
pub mod bevel;
pub mod bit_btn;
pub mod breadcrumb;
pub mod button;
pub mod card;
pub mod category_panel;
pub mod check_list_box;
pub mod checkbox;
pub mod chips;
pub mod codegen;
pub mod color_box;
pub mod combobox;
pub mod date_time_picker;
pub mod drawer;
pub mod edit;
pub mod grid;
pub mod group_box;
pub mod image;
pub mod image_list;
pub mod label;
pub mod link_label;
pub mod list_box;
pub mod list_view;
pub mod main_menu;
pub mod mask_edit;
pub mod memo;
pub mod message_dialog;
pub mod multi_select;
pub mod open_dialog;
pub mod page_control;
pub mod pagination;
pub mod paint_box;
pub mod panel;
pub mod popup_menu;
pub mod progress_bar;
pub mod radio_button;
pub mod radio_group;
pub mod save_dialog;
pub mod scroll_bar;
pub mod scroll_box;
pub mod shape;
pub mod speed_button;
pub mod spin_edit;
pub mod splitter;
pub mod status_bar;
pub mod toggle;

pub mod timer;
pub mod tool_bar;
pub mod track_bar;
pub mod tree_view;
pub mod types;
pub mod util;

pub use codegen::{ComponentCodegen, EnterKey, MakepadComponentCodegen};

use crate::Erro;
use crawrad_ir::{Document, Node};
use crawrad_note::note;

static REGISTRY: &[&'static dyn MakepadComponentCodegen] = &[
    &action_list::ActionListCodegen,
    &action_list::ActionCodegen,
    &button::ButtonCodegen,
    &label::LabelCodegen,
    &image::ImageCodegen,
    &image_list::ImageListCodegen,
    &edit::EditCodegen,
    &mask_edit::MaskEditCodegen,
    &memo::MemoCodegen,
    &checkbox::CheckBoxCodegen,
    &radio_button::RadioButtonCodegen,
    &radio_group::RadioGroupCodegen,
    &color_box::ColorBoxCodegen,
    &combobox::ComboBoxCodegen,
    &date_time_picker::DateTimePickerCodegen,
    &list_box::ListBoxCodegen,
    &list_view::ListViewCodegen,
    &list_view::ListColumnCodegen,
    &grid::GridCodegen,
    &panel::PanelCodegen,
    &group_box::GroupBoxCodegen,
    &page_control::PageControlCodegen,
    &page_control::TabSheetCodegen,
    &timer::TimerCodegen,
    &scroll_bar::ScrollBarCodegen,
    &scroll_box::ScrollBoxCodegen,
    &tree_view::TreeViewCodegen,
    &bit_btn::BitBtnCodegen,
    &speed_button::SpeedButtonCodegen,
    &spin_edit::SpinEditCodegen,
    &toggle::ToggleCodegen,
    &link_label::LinkLabelCodegen,
    &bevel::BevelCodegen,
    &badge::BadgeCodegen,
    &card::CardCodegen,
    &drawer::DrawerCodegen,
    &category_panel::CategoryPanelCodegen,
    &chips::ChipsCodegen,
    &breadcrumb::BreadcrumbCodegen,
    &pagination::PaginationCodegen,
    &multi_select::MultiSelectCodegen,
    &check_list_box::CheckListBoxCodegen,
    &main_menu::MainMenuCodegen,
    &main_menu::MenuItemCodegen,
    &popup_menu::PopupMenuCodegen,
    &tool_bar::ToolBarCodegen,
    &status_bar::StatusBarCodegen,
    &status_bar::StatusPanelCodegen,
    &progress_bar::ProgressBarCodegen,
    &track_bar::TrackBarCodegen,
    &paint_box::PaintBoxCodegen,
    &shape::ShapeCodegen,
    &open_dialog::OpenDialogCodegen,
    &save_dialog::SaveDialogCodegen,
    &message_dialog::MessageDialogCodegen,
    &splitter::SplitterCodegen,
];

#[note("targets-makepad-components-for-class")]
pub fn for_class(class: &str) -> Option<&'static dyn MakepadComponentCodegen> {
    REGISTRY.iter().copied().find(|c| c.class_name() == class)
}

#[note("targets-makepad-components-whole-look-widget")]
pub fn whole_look_widget(class: &str) -> Option<&'static str> {
    for_class(class).and_then(|c| c.whole_look_widget())
}

#[note("targets-makepad-components-text-path")]
pub fn text_path(node: &Node, field: &str) -> String {
    match for_class(&node.class) {
        Some(c) => c.text_path(node, field),
        None => field.to_string(),
    }
}

#[note("targets-makepad-components-app-state")]
pub fn app_state(
    doc: &Document,
    node: &Node,
    field: &str,
) -> Result<Option<codegen::AppState>, crate::Erro> {
    match for_class(&node.class) {
        Some(c) => c.app_state_doc(doc, node, field),
        None => Ok(None),
    }
}

#[note("targets-makepad-components-has-enabled-field")]
pub fn has_enabled_field(class: &str) -> bool {
    for_class(class).is_some_and(|c| c.struct_definition().contains("pub enabled: bool,"))
}

#[note("targets-makepad-components-classes")]
pub fn classes() -> Vec<&'static str> {
    REGISTRY.iter().map(|c| c.class()).collect()
}

#[note("targets-makepad-components-component-structs")]
pub fn component_structs(doc: &Document, reg: &crawrad_registry::Registry) -> String {
    let mut out = String::new();
    let mut used_classes = std::collections::BTreeSet::new();
    for node in doc.nodes.values() {
        used_classes.insert(node.class.as_str());
        used_classes.insert(reg.effective_class(&node.class));
    }
    let mut added = true;
    while added {
        added = false;
        for class in used_classes.clone() {
            if let Some(desc) = reg.get(class)
                && let Some(item) = desc.item_class()
                && used_classes.insert(item)
            {
                added = true;
            }
        }
    }
    // PageControl has TabSheet children, but the catalogue declares its
    // children as Children::Area (geometry-derived container) rather than
    // a typed item collection.
    if used_classes.contains("PageControl") {
        used_classes.insert("TabSheet");
    }
    // ImageItem is an internal target struct in this backend, not a catalogue class.
    if used_classes.contains("ImageList") {
        used_classes.insert("ImageItem");
    }

    let needs_strings = used_classes.iter().any(|c| {
        reg.get(c).is_some_and(|desc| {
            desc.props.iter().any(|prop| {
                matches!(
                    prop.kind,
                    crawrad_registry::PropKind::Collection {
                        item_class: None,
                        ..
                    } | crawrad_registry::PropKind::Multiline
                ) || matches!(prop.default, crawrad_ir::Value::List(_))
            })
        })
    });
    if needs_strings {
        out.push_str(&types::STRINGS_TYPE_DEF.replace("\r\n", "\n"));
    }
    out.push_str(&crate::targets::compartilhado::texto_das_pecas(doc, reg));
    for comp in REGISTRY {
        if used_classes.contains(comp.class()) {
            let def = comp.struct_definition();
            if !def.is_empty() {
                out.push('\n');
                out.push_str(&def);
            }
        }
    }
    out.push('\n');
    out
}

#[note("targets-makepad-components-emit-default")]
pub fn emit_default(node: &Node) -> String {
    if let Some(comp) = for_class(&node.class) {
        comp.emit_default(node)
    } else {
        format!("{}::default()", node.class)
    }
}

#[note("targets-makepad-components-emit-default-com")]
pub fn emit_default_com(
    reg: &crawrad_registry::Registry,
    doc: &crawrad_ir::Document,
    node: &Node,
) -> String {
    let effective = reg.effective_class(&node.class);
    if let Some(comp) = for_class(effective) {
        return comp.emit_default_doc(doc, node);
    }
    let Some(desc) = terceiro(reg, &node.class) else {
        return emit_default(node);
    };
    third_party_literal(reg, doc, node, desc)
}

/// The struct literal of a third-party component's crate type, built from the
/// node's props; an inheriting class writes only its OWN props (the base's
/// live on the generated base widget).
pub fn third_party_literal(
    reg: &crawrad_registry::Registry,
    doc: &crawrad_ir::Document,
    node: &Node,
    desc: &crawrad_registry::ComponentDesc,
) -> String {
    let tipo = format!("{}::{}", desc.crate_module, desc.class);
    let mut inits = Vec::new();
    // **Was any COMPONENT property left out?**
    //
    // This is not the same question as `private_state`. That one says whether the MANIFEST
    // covers all fields; this one says whether the GENERATOR was able to write a value
    // for each one it covers.
    //
    // The two diverged on the first reference property: `Connection`
    // publishes `engine`, and the literal came out without the field — the user's project
    // did not compile, despite the correct manifest. The reference now becomes the target's
    // NAME, so today it no longer falls in here; the calculation remains for the next
    // `Value` variant that the target does not know how to write.
    let mut faltou = false;
    for prop in &desc.props {
        // `name` and `rect` belong to design and are not fields of the type: skipping
        // both leaves no hole in the literal.
        if crate::emission::why_not_set(prop.key, prop.category).is_some()
            || reg.is_inherited(desc, prop.key)
        {
            continue;
        }
        let valor = node.props.get(prop.key).unwrap_or(&prop.default);
        // **THE REFERENCE becomes the NAME of the referenced component**, and does not vanish.
        if let crawrad_ir::Value::Ref(id) = valor {
            inits.push(format!(
                "{}: {}",
                prop.key,
                crate::targets::compartilhado::literal_de_referencia(doc, id)
            ));
            continue;
        }
        // A piece from the box: `Edges`, `Color`. The type travels along.
        if let Some(lit) = crate::targets::compartilhado::literal_da_peca(valor) {
            inits.push(format!("{}: {lit}", prop.key));
            continue;
        }
        if matches!(
            valor,
            crawrad_ir::Value::Str(_)
                | crawrad_ir::Value::Bool(_)
                | crawrad_ir::Value::Int(_)
                | crawrad_ir::Value::Float(_)
        ) {
            inits.push(format!(
                "{}: {}",
                prop.key,
                util::value_to_rust_literal(valor)
            ));
        } else {
            faltou = true;
        }
    }
    if inits.is_empty() {
        return format!("{tipo}::default()");
    }
    if desc.private_state || faltou {
        // Some field was left without a value — either because the crate has state the manifest
        // does not declare, or because the generator did not know how to write one. The
        // crate's `Default` is what fills in the rest.
        return format!("{tipo} {{ {}, ..Default::default() }}", inits.join(", "));
    }
    // Every field has a written value — and then `..Default::default()` would have
    // no effect, would be `needless_update`, and would fail the user's project
    // under `-D warnings`.
    format!("{tipo} {{ {} }}", inits.join(", "))
}

#[note("targets-makepad-components-terceiro")]
pub fn terceiro<'a>(
    reg: &'a crawrad_registry::Registry,
    classe: &str,
) -> Option<&'a crawrad_registry::ComponentDesc> {
    reg.get(classe).filter(|d| !d.crate_module.is_empty())
}

#[note("targets-makepad-components-emit-sync")]
pub fn emit_sync(doc: &Document, node: &Node, field: &str) -> String {
    if let Some(comp) = for_class(&node.class) {
        comp.emit_sync(doc, node, field)
    } else {
        String::new()
    }
}

/// Emits sync logic for a component, routing inheriting components through their effective base.
pub fn emit_sync_com(
    reg: &crawrad_registry::Registry,
    doc: &Document,
    node: &Node,
    field: &str,
) -> String {
    let effective = reg.effective_class(&node.class);
    if let Some(comp) = for_class(effective) {
        comp.emit_sync_with_reg(reg, doc, node, field)
    } else {
        String::new()
    }
}

#[note("targets-makepad-components-emit-actions")]
pub fn emit_actions(
    doc: &Document,
    node: &Node,
    field: &str,
    emitted: &[&Node],
) -> Result<String, Erro> {
    if let Some(comp) = for_class(&node.class) {
        comp.emit_actions(doc, node, field, emitted)
    } else {
        Ok(String::new())
    }
}

#[note("targets-makepad-components-emit-actions-com")]
pub fn emit_actions_com(
    reg: &crawrad_registry::Registry,
    doc: &Document,
    node: &Node,
    field: &str,
    emitted: &[&Node],
) -> Result<String, Erro> {
    let effective = reg.effective_class(&node.class);
    if let Some(comp) = for_class(effective) {
        return comp.emit_actions_with_reg(reg, doc, node, field, emitted);
    }
    if terceiro(reg, &node.class).is_none() {
        return Ok(String::new());
    }
    let mut code = String::new();
    for (evento, handler) in ligacoes_do_terceiro(node)? {
        let b = super::binding::binding_declarado(&evento).expect("it came from there");
        code.push_str(&super::binding::emitir(&b, field, &handler));
    }
    Ok(code)
}

#[note("targets-makepad-components-ligacoes-do-terceiro")]
pub fn ligacoes_do_terceiro(node: &Node) -> Result<Vec<(String, String)>, Erro> {
    crate::targets::compartilhado::ligacoes_do_declarado(node)
}

#[note("targets-makepad-components-emit-dsl")]
pub fn emit_dsl(
    doc: &Document,
    reg: &crawrad_registry::Registry,
    node: &Node,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> Result<String, Erro> {
    let effective = reg.effective_class(&node.class);
    if let Some(comp) = for_class(effective) {
        return comp.emit_dsl(doc, reg, node, x, y, w, h);
    }
    // **THE THIRD-PARTY COMPONENT DRAWS ITSELF**, via the manifest's verbs.
    //
    // Before this point it came out as an empty string, and an empty string was the answer
    // for everything this target did not register: the component did not appear in the
    // application and nothing warned about it. The non-visual component remains without a widget,
    // and now for a declared reason — it has no drawing.
    let Some(desc) = terceiro(reg, &node.class) else {
        return Ok(String::new());
    };
    let field = crate::identificador(node.name())?;
    let Some(d) = super::declarado::desenho(&field, desc, w, h) else {
        return Ok(String::new());
    };
    let pos = super::layout::posicao(x, y);
    // **THE CURSOR IS NOT DECORATION: WITHOUT IT THE CLICK NEVER ARRIVES.**
    //
    // Measured in the framework source (`makepad-widgets-1.0.0/src/view.rs:701`):
    //
    // ```text
    // if self.visible && self.cursor.is_some() || self.animator.live_ptr.is_some() {
    //     match event.hits_with_capture_overload(cx, self.area(), …)
    // ```
    //
    // A `<View>` **only hit-tests when it has a `cursor`** (or an animator).
    // Without this line, the `finger_up` queried by `handle_actions` never
    // happens, and the user handler is never called — no error, no warning.
    //
    // It is added **only when there is a wired handler**: a hand cursor over a
    // component that does not respond is the application promising what it does not deliver.
    let cursor = if ligacoes_do_terceiro(node)?.is_empty() {
        String::new()
    } else {
        "                cursor: Hand\n".to_string()
    };
    Ok(format!(
        "            {field} = <View> {{\n\
         \x20               {pos}\n\
         \x20               width: {w}, height: {h}\n\
         \x20               flow: Overlay\n\
         {cursor}\
         {}\
         \x20           }}\n",
        d.dsl
    ))
}

#[note("targets-makepad-components-emit-children")]
pub fn emit_children(
    doc: &Document,
    reg: &crawrad_registry::Registry,
    node: &Node,
    indent: &str,
) -> Result<String, Erro> {
    // A collection item is not a widget: it is owner state, and what emits it is the
    // owner's `Set` — never the DSL tree.
    if reg
        .get(&node.class)
        .is_some_and(|d| d.item_class().is_some())
    {
        return Ok(String::new());
    }
    let mut dsl = String::new();
    for child_id in &node.children {
        let Some(child) = doc.node(child_id) else {
            continue;
        };
        let r = child
            .rect()
            .unwrap_or(crawrad_ir::Rect::new(0.0, 0.0, 80.0, 24.0));
        for line in emit_dsl(doc, reg, child, r.x, r.y, r.w, r.h)?.lines() {
            dsl.push_str(indent);
            dsl.push_str(line);
            dsl.push('\n');
        }
    }
    Ok(dsl)
}

pub fn emitir_dsl(
    doc: &Document,
    reg: &crawrad_registry::Registry,
    node: &Node,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> Result<String, Erro> {
    emit_dsl(doc, reg, node, x, y, w, h)
}
