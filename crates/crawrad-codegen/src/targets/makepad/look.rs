//! **How a control paints its text and background**, as DATA: where
//! each class keeps its text and whether it has a background to colour.
//!
//! An unknown live field in `apply_over` is logged as an error by makepad
//! 1.0.0, so nothing is applied blind: a class is styled only through the
//! paths this table names. The values are read at run time, and `ParentFont`
//! / `ParentColor` choose between the control's own and the form's, which
//! the generator writes as two constants.

use crawrad_ir::{Document, Node};
use crawrad_note::note;

#[note("targets-makepad-look-look")]
const LOOK: &[(&str, &str, Option<bool>, &str)] = &[
    ("Button", "", Some(false), "Pressable"),
    ("Label", "{f}_text", Some(true), "Plain"),
    ("Edit", "", Some(false), "Pressable"),
    ("MaskEdit", "", Some(false), "Pressable"),
    ("DateTimePicker", "", Some(false), "Pressable"),
    ("Memo", "", Some(false), "Pressable"),
    // its draw_bg is the box itself: font only; the caption is in the box or,
    // left of it, in the lead label
    ("CheckBox", "{f}_box,{f}_lead:Plain", None, "Toggle"),
    ("RadioButton", "", None, "Toggle"),
    // the number's field
    ("SpinEdit", "{f}_edit", None, "Pressable"),
    ("Toggle", "", None, "Toggle"),
    ("ColorBox", "{f}_list", None, "Pressable"),
    ("LinkLabel", "", None, "Pressable"),
    ("Badge", "label", None, "Plain"),
    // the Card's Font is its title's; the subtitle stays the smaller muted line
    ("Card", "header.title", Some(false), "Plain"),
    // the Drawer's Font is its title's
    ("Drawer", "header.title", Some(false), "Plain"),
    ("CategoryPanel", "header", None, "Pressable"),
    ("Panel", "label", Some(false), "Plain"),
    ("GroupBox", "title", Some(false), "Plain"),
    // its simple text and each of its panels
    ("StatusBar", "*status", Some(false), "Plain"),
    // step 4: one text per tab, counted at generation (the TreeView is a
    // `CrawradListBox` and takes the whole look below)
    ("PageControl", "*tabs", Some(false), "Pressable"),
    // a background and no text
    ("ScrollBox", "-", Some(false), ""),
];

#[note("targets-makepad-look-text-paths")]
fn text_paths(doc: &Document, node: &Node, f: &str, text: &str) -> Vec<String> {
    match text {
        "" => vec![super::components::text_path(node, f)],
        "*tabs" => {
            let tabs = node
                .children
                .iter()
                .filter_map(|c| doc.node(c))
                .filter(|c| c.class == "TabSheet")
                .count()
                .max(1);
            (0..tabs)
                .map(|i| format!("{f}.tabs_header.tab_{i}"))
                .collect()
        }
        "-" => Vec::new(),
        "*status" => {
            let mut paths = vec![format!("{f}.lbl")];
            let panels = node
                .children
                .iter()
                .filter_map(|c| doc.node(c))
                .filter(|c| c.class == "StatusPanel")
                .count();
            paths.extend((0..panels).map(|i| format!("{f}.panels.p_{i}.t")));
            paths
        }
        t => vec![format!("{f}.{}", t.replace("{f}", f))],
    }
}

fn entry(class: &str) -> Option<(&'static str, Option<bool>, &'static str)> {
    LOOK.iter()
        .find(|(c, _, _, _)| *c == class)
        .map(|(_, t, b, st)| (*t, *b, *st))
}

#[note("targets-makepad-look-sync")]
pub fn sync(doc: &Document, node: &Node, f: &str) -> String {
    // our own widgets (ListBox, MultiSelect, ComboBox…) take the whole look
    // in one call and style each row themselves; each component says so
    if let Some(widget) = super::components::whole_look_widget(&node.class) {
        return whole_look(f, widget);
    }
    let Some((text, background, states)) = entry(&node.class) else {
        return String::new();
    };
    styled(doc, node, f, text, background, states)
}

/// Syncs the look of a component, routing inheriting components through their effective base.
pub fn sync_com(doc: &Document, reg: &crawrad_registry::Registry, node: &Node, f: &str) -> String {
    let effective = reg.effective_class(&node.class);
    if let Some(widget) = super::components::whole_look_widget(effective) {
        return whole_look(f, widget);
    }
    let Some((text, background, states)) = entry(effective) else {
        return String::new();
    };
    styled(doc, node, f, text, background, states)
}

#[note("targets-makepad-look-whole-look")]
pub fn whole_look(f: &str, widget: &str) -> String {
    format!(
        "        // the font and colours, the list's own or the form's\n\
         \x20       {{\n\
         \x20           let (font_size, text) = if self.form.{f}.parent_font {{ (CRAWRAD_FORM_FONT_SIZE, CRAWRAD_FORM_FONT_COLOR) }} else {{ (self.form.{f}.font.size, self.form.{f}.font.color) }};\n\
         \x20           let background = if self.form.{f}.parent_color {{ [0.0; 4] }} else {{ self.form.{f}.color.to_f32() }};\n\
         \x20           self.ui.{widget}(id!({f})).set_look(cx, crawrad_runtime_makepad::list::ListLook {{ font_size, text, background }});\n\
         \x20       }}\n"
    )
}

#[note("targets-makepad-look-styled")]
fn styled(
    doc: &Document,
    node: &Node,
    f: &str,
    text: &str,
    background: Option<bool>,
    states: &str,
) -> String {
    // several texts are `a,b`, each with its own states after a `:` (
    // the check box and the caption beside it)
    let paths: Vec<(String, &str)> = text
        .split(',')
        .flat_map(|part| {
            let (path, own) = part.split_once(':').unwrap_or((part, states));
            text_paths(doc, node, f, path)
                .into_iter()
                .map(move |p| (p, own))
        })
        .collect();
    let mut s = String::new();
    // a class with no text (the ScrollBox) has no font to apply
    if !paths.is_empty() {
        s.push_str(&format!(
            "        // the font, the control's own or the form's\n\
             \x20       {{\n\
             \x20           let (size, color) = if self.form.{f}.parent_font {{ (CRAWRAD_FORM_FONT_SIZE, CRAWRAD_FORM_FONT_COLOR) }} else {{ (self.form.{f}.font.size, self.form.{f}.font.color) }};\n"
        ));
        for (path, states) in &paths {
            s.push_str(&format!(
                "            let text = self.ui.widget(id!({path}));\n\
                 \x20           crawrad_runtime_makepad::look::font_size(cx, &text, size);\n\
                 \x20           crawrad_runtime_makepad::look::text_color(cx, &text, color, crawrad_runtime_makepad::look::TextStates::{states});\n"
            ));
        }
        s.push_str("        }\n");
    }
    if let Some(show_bg) = background {
        s.push_str(&format!(
            "        {{\n\
             \x20           let rgba = if self.form.{f}.parent_color {{ [0.0; 4] }} else {{ self.form.{f}.color.to_f32() }};\n\
             \x20           crawrad_runtime_makepad::look::background(cx, &self.ui.widget(id!({f})), rgba, {show_bg});\n\
             \x20       }}\n"
        ));
    }
    s
}
