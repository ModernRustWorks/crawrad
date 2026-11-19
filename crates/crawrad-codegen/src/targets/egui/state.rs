//! Egui target component state derivation from the catalogue.

use crawrad_ir::{Document, Node, Value};
use crawrad_registry::ComponentDesc;

use crate::emission::why_not_set;
use crate::targets::shared::rust_type_of;
use crawrad_note::note;

#[note("targets-egui-state-field")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Field {
    /// Field name override (empty to keep prop key).
    pub name: &'static str,
    /// Rust type override.
    pub ty: &'static str,
}

#[note("targets-egui-state-field-map")]
pub const FIELD_MAP: &[(&str, &str, Field)] = &[
    (
        "ComboBox",
        "item_index",
        Field {
            name: "selected",
            ty: "Option<usize>",
        },
    ),
    (
        "Grid",
        "col_count",
        Field {
            name: "col_count",
            ty: "usize",
        },
    ),
    (
        "Grid",
        "default_col_width",
        Field {
            name: "default_col_width",
            ty: "f32",
        },
    ),
    (
        "Grid",
        "default_row_height",
        Field {
            name: "default_row_height",
            ty: "f32",
        },
    ),
    (
        "Grid",
        "fixed_cols",
        Field {
            name: "fixed_cols",
            ty: "usize",
        },
    ),
    (
        "Grid",
        "fixed_rows",
        Field {
            name: "fixed_rows",
            ty: "usize",
        },
    ),
    (
        "Grid",
        "row_count",
        Field {
            name: "row_count",
            ty: "usize",
        },
    ),
    (
        "ImageList",
        "images_data",
        Field {
            name: "images",
            ty: "Vec<ImageItem>",
        },
    ),
    (
        "ListBox",
        "item_index",
        Field {
            name: "selected",
            ty: "Option<usize>",
        },
    ),
    (
        "ListView",
        "items",
        Field {
            name: "items",
            ty: "Vec<ListViewItem>",
        },
    ),
    (
        "PageControl",
        "active_page",
        Field {
            name: "active_page",
            ty: "usize",
        },
    ),
    (
        "Panel",
        "border_width",
        Field {
            name: "border_width",
            ty: "f32",
        },
    ),
    (
        "ScrollBar",
        "large_change",
        Field {
            name: "large_change",
            ty: "i64",
        },
    ),
    (
        "ScrollBar",
        "max",
        Field {
            name: "max",
            ty: "i64",
        },
    ),
    (
        "ScrollBar",
        "min",
        Field {
            name: "min",
            ty: "i64",
        },
    ),
    (
        "ScrollBar",
        "position",
        Field {
            name: "position",
            ty: "i64",
        },
    ),
    (
        "ScrollBar",
        "small_change",
        Field {
            name: "small_change",
            ty: "i64",
        },
    ),
    (
        "Shape",
        "shape",
        Field {
            name: "shape_kind",
            ty: "String",
        },
    ),
    (
        "Timer",
        "interval",
        Field {
            name: "interval",
            ty: "u64",
        },
    ),
    (
        "TreeView",
        "items",
        Field {
            name: "items",
            ty: "Vec<crawrad_runtime::tree::TreeNode>",
        },
    ),
];

#[note("targets-egui-state-skipped")]
pub const SKIPPED: &[(&str, &str, &str)] = &[
    (
        "",
        "action",
        "Action links are resolved at generation, not stored in state",
    ),
    (
        "BitBtn",
        "action",
        "Action links are resolved at generation, not stored in state",
    ),
    (
        "BitBtn",
        "kind",
        "BitBtn standard kind is resolved at render time, not kept in state",
    ),
    (
        "Button",
        "action",
        "Action links are resolved at generation, not stored in state",
    ),
    (
        "Grid",
        "data_source",
        "Data source binding is resolved at generation, not held in state",
    ),
    (
        "Grid",
        "design_rows",
        "Designer-only: how many real rows the canvas shows (#925); the app reads its own data",
    ),
    (
        "MainMenu",
        "images",
        "ImageList reference for MainMenu is resolved at generation, not kept in state",
    ),
    (
        "Memo",
        "scroll_bars",
        "Egui TextEdit handles scroll bars automatically",
    ),
    (
        "MenuItem",
        "action",
        "Action links are resolved at generation, not stored in state",
    ),
    (
        "PopupMenu",
        "images",
        "ImageList reference for PopupMenu is resolved at generation, not kept in state",
    ),
    (
        "ScrollBar",
        "kind",
        "ScrollBar orientation kind is resolved at layout/render time, not kept in state",
    ),
    (
        "SpeedButton",
        "action",
        "Action links are resolved at generation, not stored in state",
    ),
    (
        "ToolBar",
        "button_height",
        "ToolBar button dimensions are resolved at layout/render time",
    ),
    (
        "ToolBar",
        "button_width",
        "ToolBar button dimensions are resolved at layout/render time",
    ),
    (
        "TreeView",
        "images",
        "ImageList reference for TreeView is not held in egui state",
    ),
];

#[note("targets-egui-state-extra-fields")]
pub const EXTRA_FIELDS: &[(&str, &str, &str)] = &[
    ("DateTimePicker", "shown", "String"),
    ("Grid", "cells", "crawrad_runtime::string_grid::Cells"),
    ("Grid", "selected", "Option<(usize, usize)>"),
    ("Grid", "editor", "Option<String>"),
    ("ListBox", "selection", "Vec<usize>"),
    ("Memo", "lines", "Strings"),
    ("MessageDialog", "modal_result", "ModalResult"),
    ("MessageDialog", "answer", "MessageAnswer"),
    ("OpenDialog", "files", "Vec<String>"),
    ("Splitter", "offset", "f32"),
    ("Timer", "next_due", "Option<f64>"),
    ("TreeView", "selection", "Vec<usize>"),
    ("TreeView", "collapsed", "Vec<Vec<usize>>"),
];

#[note("targets-egui-state-field-docs")]
pub const FIELD_DOCS: &[(&str, &str, &str)] = &[
    (
        "",
        "cursor",
        "The mouse pointer over the control (`Cursor`, #667).",
    ),
    (
        "BitBtn",
        "glyph",
        "The picture left of the caption (#454); `num_glyphs` pictures side by side,\nthe first one shown.",
    ),
    (
        "SpeedButton",
        "glyph",
        "The picture left of the caption (#454); `num_glyphs` pictures side by side,\nthe first one shown.",
    ),
    (
        "SpeedButton",
        "flat",
        "Without a frame until the pointer is over it (#454).",
    ),
    (
        "SpeedButton",
        "down",
        "Pressed: in a group, the one that is down.",
    ),
    (
        "SpeedButton",
        "group_index",
        "Its group among the SpeedButtons of the same parent; 0 = none.",
    ),
    (
        "SpeedButton",
        "allow_all_up",
        "The one that is down can be lifted, leaving the group all up.",
    ),
    (
        "ToolBar",
        "flat",
        "The bar's SpeedButtons draw flat, and show their caption (#457).",
    ),
    (
        "StatusBar",
        "simple_panel",
        "On: the bar shows `simple_text`; off: its panels.",
    ),
    (
        "Shape",
        "brush_color",
        "The fill (`Brush.Color`) and the outline (`Pen.Color`), #460.",
    ),
    (
        "Splitter",
        "offset",
        "How far it was dragged, in points (#461); its neighbours add it.",
    ),
    (
        "Edit",
        "char_case",
        "The case the typed text is held to (`CharCase`, #667).",
    ),
    (
        "MaskEdit",
        "char_case",
        "The case the typed text is held to (`CharCase`, #667).",
    ),
    (
        "MaskEdit",
        "edit_mask",
        "The mask the text is shaped by (`EditMask`, #71): `0` digit, `L`
letter, `>`/`<` case… — see `crawrad_runtime::mask`.",
    ),
    (
        "DateTimePicker",
        "kind",
        "The entry and display mode (`Kind`, #21): `dtkDate` or `dtkTime`.",
    ),
    (
        "DateTimePicker",
        "date_time",
        "The current value in ISO format (`DateTime`, #21): `YYYY-MM-DD` or `HH:MM:SS`.",
    ),
    (
        "DateTimePicker",
        "min_date",
        "The minimum allowed value in ISO format (`MinDate`, #21).",
    ),
    (
        "DateTimePicker",
        "max_date",
        "The maximum allowed value in ISO format (`MaxDate`, #21).",
    ),
    (
        "DateTimePicker",
        "shown",
        "The text currently shown in the box (#21).",
    ),
    (
        "CheckBox",
        "state",
        "`cbUnchecked`, `cbChecked` or `cbGrayed` (#445); `checked` is the\ntruth for on/off, and only `state` can say \"grayed\".",
    ),
    ("CheckBox", "allow_grayed", "A click can reach `cbGrayed`."),
    (
        "CheckBox",
        "alignment",
        "`taRightJustify` (the caption right of the box) or `taLeftJustify`.",
    ),
    (
        "ListBox",
        "selection",
        "Every row marked: with `multi_select` off, `selected` or nothing (#439).",
    ),
    (
        "ListBox",
        "sorted",
        "Keeps `items` in alphabetical order, ignoring case (#439).",
    ),
    (
        "ComboBox",
        "sorted",
        "Keeps `items` in alphabetical order, ignoring case (#440).",
    ),
    (
        "Grid",
        "table_name",
        "The table a grid bound to a `Connection` shows (#442).",
    ),
    (
        "PageControl",
        "tab_position",
        "`tpTop` or `tpBottom`: where the tabs sit (#449).",
    ),
    (
        "TabSheet",
        "tab_visible",
        "The page's tab is shown (#449); a hidden one is reached by `active_page`.",
    ),
    (
        "TreeView",
        "collapsed",
        "The paths of the folded nodes: their children are not shown.",
    ),
];

#[note("targets-egui-state-field-of")]
pub fn field_of(class: &str, key: &str, default: &Value) -> Field {
    for (c, k, f) in FIELD_MAP {
        if *c == class && *k == key {
            return *f;
        }
    }
    Field {
        name: "",
        ty: rust_type_of(default),
    }
}

#[note("targets-egui-state-why-skipped")]
pub fn why_skipped(class: &str, key: &str) -> Option<&'static str> {
    for (c, k, reason) in SKIPPED {
        if (c.is_empty() || *c == class) && *k == key {
            return Some(*reason);
        }
    }
    None
}

#[note("targets-egui-state-field-doc")]
pub fn field_doc(class: &str, field: &str) -> Option<&'static str> {
    for (c, f, doc) in FIELD_DOCS {
        if *c == class && *f == field {
            return Some(*doc);
        }
    }
    for (c, f, doc) in FIELD_DOCS {
        if c.is_empty() && *f == field {
            return Some(*doc);
        }
    }
    None
}

#[note("targets-egui-state-fields-of")]
pub fn fields_of(desc: &ComponentDesc) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for prop in &desc.props {
        if why_not_set(prop.key, prop.category).is_some() {
            continue;
        }
        if why_skipped(desc.class, prop.key).is_some() {
            continue;
        }
        let f = field_of(desc.class, prop.key, &prop.default);
        let name = if f.name.is_empty() {
            prop.key.to_string()
        } else {
            f.name.to_string()
        };
        out.push((name, f.ty.to_string()));
    }
    for (class, name, ty) in EXTRA_FIELDS {
        if *class == desc.class {
            out.push((name.to_string(), ty.to_string()));
        }
    }
    out
}

#[note("targets-egui-state-struct-of")]
pub fn struct_of(desc: &ComponentDesc) -> String {
    let fields = fields_of(desc);
    if fields.is_empty() {
        return format!("pub struct {} {{}}\n", desc.class);
    }
    let mut out = format!("pub struct {} {{\n", desc.class);
    for (name, ty) in &fields {
        if let Some(doc) = field_doc(desc.class, name) {
            for line in doc.lines() {
                out.push_str("    /// ");
                out.push_str(line);
                out.push('\n');
            }
        }
        out.push_str(&format!("    pub {name}: {ty},\n"));
    }
    out.push_str("}\n");
    out
}

#[note("targets-egui-state-extra-inits")]
pub const EXTRA_INITS: &[(&str, &str, &str)] = &[
    ("Grid", "cells", "Default::default()"),
    ("Grid", "selected", "None"),
    ("Grid", "editor", "None"),
    ("MessageDialog", "modal_result", "ModalResult::None"),
    ("MessageDialog", "answer", "MessageAnswer::default()"),
    ("OpenDialog", "files", "Vec::new()"),
    ("Splitter", "offset", "0.0"),
    ("Timer", "next_due", "None"),
    ("TreeView", "selection", "Vec::new()"),
];

#[note("targets-egui-state-lookup-extra-init")]
pub fn lookup_extra_init(class: &str, field: &str) -> Option<&'static str> {
    for (c, f, init) in EXTRA_INITS {
        if *c == class && *f == field {
            return Some(*init);
        }
    }
    None
}

#[note("targets-egui-state-initfn")]
pub type InitFn = fn(&Node, Option<&Document>) -> String;

#[note("targets-egui-state-init-map")]
pub const INIT_MAP: &[(&str, &str, InitFn, &str)] = &[
    (
        "",
        "popup_menu",
        init_popup_menu,
        "PopupMenu reference is resolved to the menu component's identifier",
    ),
    (
        "Label",
        "focus_control",
        init_focus_control,
        "Label FocusControl reference is resolved to the target control's identifier",
    ),
    (
        "Badge",
        "associate",
        init_badge_associate,
        "Badge associate reference is resolved to the target component's identifier",
    ),
    (
        "BitBtn",
        "text",
        init_bit_btn_text,
        "BitBtn caption derives from standard kind unless custom caption is set",
    ),
    (
        "Button",
        "text",
        init_action_or_caption_text,
        "Button text may come from linked Action or stripped accelerator",
    ),
    (
        "SpeedButton",
        "text",
        init_action_or_caption_text,
        "SpeedButton text may come from linked Action or stripped accelerator",
    ),
    (
        "MenuItem",
        "text",
        init_action_or_caption_text,
        "MenuItem text may come from linked Action or stripped accelerator",
    ),
    (
        "Edit",
        "text",
        init_edit_text,
        "Edit text is cased according to its char_case property",
    ),
    (
        "MaskEdit",
        "text",
        init_mask_edit_text,
        "MaskEdit text is cased, then shaped by its edit_mask at run time",
    ),
    (
        "DateTimePicker",
        "shown",
        init_date_time_picker_shown,
        "DateTimePicker shown is initialized from date_time formatted by kind mask",
    ),
    (
        "MessageDialog",
        "text",
        init_verbatim_text,
        "MessageDialog message text preserves verbatim characters",
    ),
    (
        "ListColumn",
        "text",
        init_verbatim_text,
        "ListColumn header text preserves verbatim characters",
    ),
    (
        "StatusPanel",
        "text",
        init_verbatim_text,
        "StatusPanel text preserves verbatim characters",
    ),
    (
        "ComboBox",
        "text",
        init_verbatim_text,
        "ComboBox text preserves verbatim characters",
    ),
    (
        "",
        "text",
        init_caption_text,
        "Control caption strips accelerator markers",
    ),
    (
        "CheckBox",
        "state",
        init_check_box_state,
        "CheckBox state defaults to cbChecked when checked is true",
    ),
    (
        "ImageList",
        "images",
        init_image_list_images,
        "ImageList images_data is converted into a vector of ImageItem structs",
    ),
    (
        "ListBox",
        "selection",
        init_list_box_selection,
        "ListBox selection contains the selected row index if item_index >= 0",
    ),
    (
        "ListView",
        "items",
        init_list_view_items,
        "ListView items are converted into a vector of ListViewItem structs",
    ),
    (
        "TreeView",
        "items",
        init_tree_view_items,
        "TreeView items are converted into a vector of TreeNode structs",
    ),
    (
        "TreeView",
        "collapsed",
        init_tree_view_collapsed,
        "TreeView collapsed paths are computed from items whose expanded is false",
    ),
    (
        "Memo",
        "lines",
        init_memo_lines,
        "Memo lines is initialized from the text property",
    ),
    (
        "Grid",
        "cells",
        init_grid_cells,
        "Grid cells are initialized from cell_text if unbound",
    ),
    (
        "Breadcrumb",
        "items",
        init_items_constant,
        "Breadcrumb items are loaded from the generated items constant",
    ),
    (
        "CheckListBox",
        "items",
        init_items_constant,
        "CheckListBox items are loaded from the generated items constant",
    ),
    (
        "CheckListBox",
        "checked_items",
        init_checked_items_constant,
        "CheckListBox checked_items are loaded from the generated checked items constant",
    ),
    (
        "Chips",
        "items",
        init_items_constant,
        "Chips items are loaded from the generated items constant",
    ),
    (
        "ComboBox",
        "items",
        init_items_constant,
        "ComboBox items are loaded from the generated items constant",
    ),
    (
        "ListBox",
        "items",
        init_items_constant,
        "ListBox items are loaded from the generated items constant",
    ),
    (
        "MultiSelect",
        "items",
        init_items_constant,
        "MultiSelect items are loaded from the generated items constant",
    ),
    (
        "MultiSelect",
        "selected_items",
        init_selected_items_constant,
        "MultiSelect selected_items are loaded from the generated selected items constant",
    ),
    (
        "RadioGroup",
        "items",
        init_items_constant,
        "RadioGroup items are loaded from the generated items constant",
    ),
];

#[note("targets-egui-state-lookup-init-map")]
pub fn lookup_init_map(class: &str, field: &str) -> Option<InitFn> {
    for (c, f, func, _) in INIT_MAP {
        if *c == class && *f == field {
            return Some(*func);
        }
    }
    for (c, f, func, _) in INIT_MAP {
        if c.is_empty() && *f == field {
            return Some(*func);
        }
    }
    None
}

fn field_name(n: &Node) -> String {
    crate::identificador(n.name()).unwrap_or_else(|_| n.name().to_string())
}

fn action_text(n: &Node, doc: Option<&Document>) -> Option<String> {
    let doc = doc?;
    let act_id = match n.props.get("action")? {
        crawrad_ir::Value::Ref(id) if !id.trim().is_empty() => id,
        _ => return None,
    };
    let act = doc.node(act_id).filter(|a| a.class == "Action")?;
    let text = act.text().filter(|t| !t.is_empty())?;
    Some(crate::sem_acelerador(text))
}

fn init_popup_menu(n: &Node, doc: Option<&Document>) -> String {
    let popup = match n.props.get("popup_menu") {
        Some(crawrad_ir::Value::Ref(m)) => doc
            .and_then(|d| d.node(m))
            .filter(|m| m.class == "PopupMenu")
            .and_then(|m| crate::identificador(m.name()).ok())
            .unwrap_or_default(),
        _ => String::new(),
    };
    format!("{}.to_string()", crate::literal(&popup))
}

fn init_focus_control(n: &Node, doc: Option<&Document>) -> String {
    let target = match n.props.get("focus_control") {
        Some(crawrad_ir::Value::Ref(t)) => doc
            .and_then(|d| d.node(t))
            .and_then(|t| crate::identificador(t.name()).ok())
            .unwrap_or_default(),
        _ => String::new(),
    };
    format!("{}.to_string()", crate::literal(&target))
}

fn init_badge_associate(n: &Node, doc: Option<&Document>) -> String {
    let assoc_raw = match n.props.get("associate") {
        Some(crawrad_ir::Value::Ref(s)) | Some(crawrad_ir::Value::Str(s)) => s.as_str(),
        _ => "",
    };
    let assoc_target = if !assoc_raw.is_empty() {
        doc.and_then(|d| {
            d.node(assoc_raw)
                .or_else(|| d.nodes.values().find(|nd| nd.name() == assoc_raw))
        })
    } else {
        None
    };
    let assoc_val = if let Some(target) = assoc_target {
        crate::identificador(target.name()).unwrap_or_else(|_| assoc_raw.to_string())
    } else {
        assoc_raw.to_string()
    };
    format!("{}.to_string()", crate::literal(&assoc_val))
}

fn init_bit_btn_text(n: &Node, doc: Option<&Document>) -> String {
    if let Some(act_text) = action_text(n, doc) {
        return format!("{}.to_string()", crate::literal(&act_text));
    }
    let (caption, _) =
        crate::targets::egui::components::bit_btn::bit_btn_caption_and_modal_result(n);
    format!("{}.to_string()", crate::literal(&caption))
}

fn init_action_or_caption_text(n: &Node, doc: Option<&Document>) -> String {
    if let Some(act_text) = action_text(n, doc) {
        return format!("{}.to_string()", crate::literal(&act_text));
    }
    format!(
        "{}.to_string()",
        crate::literal(&crate::targets::egui::layout::texto(n))
    )
}

#[note("targets-egui-state-cased-text-literal")]
fn cased_text_literal(n: &Node) -> String {
    crate::literal(&crawrad_ir::char_case::cased_for(
        n,
        &crate::targets::egui::layout::texto(n),
    ))
}

fn init_edit_text(n: &Node, _doc: Option<&Document>) -> String {
    format!("{}.to_string()", cased_text_literal(n))
}

#[note("targets-egui-state-init-mask-edit-text")]
fn init_mask_edit_text(n: &Node, doc: Option<&Document>) -> String {
    let mask = match n.props.get("edit_mask") {
        Some(crawrad_ir::Value::Str(s)) => s.as_str(),
        _ => "",
    };
    if mask.is_empty() {
        return init_edit_text(n, doc);
    }
    format!(
        "crawrad_runtime::mask::shown_text({}, {})",
        crate::literal(mask),
        cased_text_literal(n)
    )
}

fn init_date_time_picker_shown(n: &Node, _doc: Option<&Document>) -> String {
    let shown = crawrad_ir::date_time::shown_of(n);
    format!("{}.to_string()", crate::literal(&shown))
}

fn init_verbatim_text(n: &Node, _doc: Option<&Document>) -> String {
    let text = match n.props.get("text") {
        Some(crawrad_ir::Value::Str(s)) => s.as_str(),
        _ => "",
    };
    format!("{}.to_string()", crate::literal(text))
}

fn init_caption_text(n: &Node, _doc: Option<&Document>) -> String {
    format!(
        "{}.to_string()",
        crate::literal(&crate::targets::egui::layout::texto(n))
    )
}

fn init_check_box_state(n: &Node, _doc: Option<&Document>) -> String {
    let state = match n.props.get("state") {
        Some(crawrad_ir::Value::Enum(s)) => s.as_str(),
        _ if crate::targets::egui::layout::marcado(n) => "cbChecked",
        _ => "cbUnchecked",
    };
    format!("{}.to_string()", crate::literal(state))
}

fn init_image_list_images(n: &Node, _doc: Option<&Document>) -> String {
    let icons = crate::targets::egui::components::image_list::extrair_icones(n);
    if icons.is_empty() {
        "Vec::new()".to_string()
    } else {
        let mut s = String::from("vec![\n");
        for (name, symbol) in &icons {
            s.push_str(&format!(
                "                ImageItem {{\n\
                 \x20                   name: {}.to_string(),\n\
                 \x20                   symbol: {}.to_string(),\n\
                 \x20               }},\n",
                crate::literal(name),
                crate::literal(symbol)
            ));
        }
        s.push_str("            ]");
        s
    }
}

fn init_list_box_selection(n: &Node, _doc: Option<&Document>) -> String {
    match n.props.get("item_index") {
        Some(crawrad_ir::Value::Int(i)) if *i >= 0 => format!("vec![{i}]"),
        _ => "Vec::new()".to_string(),
    }
}

fn init_list_view_items(n: &Node, _doc: Option<&Document>) -> String {
    let itens = match n.props.get("items") {
        Some(v) => crawrad_ir::Item::list_from(v),
        None => Vec::new(),
    };
    if itens.is_empty() {
        return "Vec::new()".to_string();
    }
    let mut out = String::from("vec![\n");
    for i in itens {
        let subs: Vec<String> = i
            .children
            .iter()
            .map(|c| format!("{}.to_string()", crate::literal(&c.text)))
            .collect();
        out.push_str(&format!(
            "                ListViewItem {{ text: {}.to_string(), sub_items: vec![{}] }},\n",
            crate::literal(&i.text),
            subs.join(", ")
        ));
    }
    out.push_str("            ]");
    out
}

fn init_tree_view_items(n: &Node, doc: Option<&Document>) -> String {
    let items = match n.props.get("items") {
        Some(v) => crawrad_ir::Item::list_from(v),
        None => Vec::new(),
    };
    let symbols = doc.map_or_else(Vec::new, |d| crate::targets::shared::image_symbols(d, n));
    let items_literal: Vec<String> = items
        .iter()
        .map(|it| crate::targets::shared::tree_node_literal(it, &symbols))
        .collect();
    format!("vec![{}]", items_literal.join(", "))
}

fn init_tree_view_collapsed(n: &Node, _doc: Option<&Document>) -> String {
    let items = match n.props.get("items") {
        Some(v) => crawrad_ir::Item::list_from(v),
        None => Vec::new(),
    };
    let paths = crate::targets::shared::collapsed_paths(&items);
    if paths.is_empty() {
        "Vec::new()".to_string()
    } else {
        let paths_str: Vec<String> = paths
            .iter()
            .map(|p| {
                let elems: Vec<String> = p.iter().map(|i| i.to_string()).collect();
                format!("vec![{}]", elems.join(", "))
            })
            .collect();
        format!("vec![{}]", paths_str.join(", "))
    }
}

fn init_memo_lines(n: &Node, _doc: Option<&Document>) -> String {
    let text_literal = crate::literal(&crate::targets::egui::layout::texto(n));
    format!("Strings::from_text({text_literal})")
}

fn init_grid_cells(n: &Node, _doc: Option<&Document>) -> String {
    crate::grid_cells_init(n)
}

fn init_items_constant(n: &Node, _doc: Option<&Document>) -> String {
    format!(
        "Strings::from_slice({}_ITEMS)",
        field_name(n).to_uppercase()
    )
}

fn init_checked_items_constant(n: &Node, _doc: Option<&Document>) -> String {
    format!(
        "Strings::from_slice({}_CHECKED_ITEMS)",
        field_name(n).to_uppercase()
    )
}

fn init_selected_items_constant(n: &Node, _doc: Option<&Document>) -> String {
    format!(
        "Strings::from_slice({}_SELECTED_ITEMS)",
        field_name(n).to_uppercase()
    )
}

#[note("targets-egui-state-value-to-literal")]
pub fn value_to_literal(field_ty: &str, val: &Value) -> String {
    match field_ty {
        "bool" => match val {
            Value::Bool(b) => b.to_string(),
            _ => "false".to_string(),
        },
        "i64" => match val {
            Value::Int(i) => i.to_string(),
            _ => "0".to_string(),
        },
        "u64" => match val {
            Value::Int(i) if *i > 0 => format!("{}", *i as u64),
            _ => "0".to_string(),
        },
        "f32" => match val {
            Value::Float(f) => format!("{f:.1}"),
            Value::Int(i) => format!("{i}.0"),
            _ => "0.0".to_string(),
        },
        "usize" => match val {
            Value::Int(i) if *i >= 0 => format!("{i}"),
            _ => "0".to_string(),
        },
        "Option<usize>" => match val {
            Value::Int(i) if *i >= 0 => format!("Some({i})"),
            _ => "None".to_string(),
        },
        "String" => match val {
            Value::Str(s) | Value::Enum(s) => format!("{}.to_string()", crate::literal(s)),
            Value::Ref(s) => format!("{}.to_string()", crate::literal(s)),
            _ => "\"\"\".to_string()".to_string(),
        },
        "Strings" => match val {
            Value::List(items) => {
                let strs: Vec<String> = items
                    .iter()
                    .filter_map(|it| match it {
                        Value::Str(s) => Some(format!("{}.to_string()", crate::literal(s))),
                        _ => None,
                    })
                    .collect();
                format!("Strings::from_slice(&[{}])", strs.join(", "))
            }
            Value::Str(s) => format!("Strings::from_text({})", crate::literal(s)),
            _ => "Strings::new()".to_string(),
        },
        _ => crate::targets::shared::literal_da_peca(val)
            .unwrap_or_else(|| "Default::default()".to_string()),
    }
}

#[note("targets-egui-state-inits-of")]
pub fn inits_of<'a>(
    node: &Node,
    desc: &ComponentDesc,
    doc: impl Into<Option<&'a Document>>,
) -> Vec<(String, String)> {
    let doc = doc.into();
    let mut out = Vec::new();
    for prop in &desc.props {
        if why_not_set(prop.key, prop.category).is_some() {
            continue;
        }
        if why_skipped(desc.class, prop.key).is_some() {
            continue;
        }
        let f = field_of(desc.class, prop.key, &prop.default);
        let field_name = if f.name.is_empty() { prop.key } else { f.name };
        let lit = if let Some(init_fn) = lookup_init_map(desc.class, field_name) {
            init_fn(node, doc)
        } else {
            let val = node.props.get(prop.key).unwrap_or(&prop.default);
            value_to_literal(f.ty, val)
        };
        out.push((field_name.to_string(), lit));
    }
    for (class, name, _ty) in EXTRA_FIELDS {
        if *class == desc.class {
            let lit = if let Some(init_fn) = lookup_init_map(desc.class, name) {
                init_fn(node, doc)
            } else if let Some(extra_init) = lookup_extra_init(desc.class, name) {
                extra_init.to_string()
            } else {
                "Default::default()".to_string()
            };
            out.push((name.to_string(), lit));
        }
    }
    out
}

#[note("targets-egui-state-init-of")]
pub fn init_of<'a>(
    node: &Node,
    desc: &ComponentDesc,
    doc: impl Into<Option<&'a Document>>,
) -> String {
    let inits = inits_of(node, desc, doc);
    if inits.is_empty() {
        return format!("{} {{}}", desc.class);
    }
    let pairs: Vec<String> = inits
        .into_iter()
        .map(|(k, v)| format!("{k}: {v}"))
        .collect();
    format!("{} {{ {} }}", desc.class, pairs.join(", "))
}
