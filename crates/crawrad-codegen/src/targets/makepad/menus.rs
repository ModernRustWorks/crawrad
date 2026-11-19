//! **The menus of a form** — the MainMenu's bar and every
//! drop-down list, generated from the `MenuItem` tree.
//!
//! The bar is a row of flat buttons along the top of the form — the strip the
//! designer draws —, and each list is a hidden view of flat buttons over
//! every control: a bar entry with items opens its list under it, an item
//! with items opens its own beside it, and a control's secondary click opens
//! the `PopupMenu` it names at the pointer
//! (`crawrad_runtime_makepad::menu`). A press outside the bar and the open lists
//! closes them, and so does a click on an item that has no items.
//!
//! Each item is a `ButtonFlat` named after the item, so its click is the
//! `Bind` table's `button` / `clicked`, the same as a `Button` — and a linked
//! `Action` runs through the same link.

use super::components::codegen::AppState;
use crate::{Erro, identificador, literal};
use crawrad_ir::{Document, Node};
use crawrad_note::note;

#[note("targets-makepad-menus-bar-height")]
pub const BAR_HEIGHT: f32 = 22.0;

#[note("targets-makepad-menus-list")]
pub struct List<'a> {
    /// The widget of the list: `<owner>_list`.
    pub id: String,
    pub owner: &'a Node,
    pub items: Vec<&'a Node>,
    /// The lists this one opened from, outermost first.
    pub ancestors: Vec<String>,
}

#[note("targets-makepad-menus-items-of")]
pub fn items_of<'a>(doc: &'a Document, owner: &Node) -> Vec<&'a Node> {
    owner
        .children
        .iter()
        .filter_map(|c| doc.node(c))
        .filter(|c| c.class == "MenuItem")
        .collect()
}

#[note("targets-makepad-menus-list-id")]
pub fn list_id(owner: &Node) -> Result<String, Erro> {
    Ok(format!("{}_list", identificador(owner.name())?))
}

fn collect<'a>(
    doc: &'a Document,
    owner: &'a Node,
    ancestors: Vec<String>,
    out: &mut Vec<List<'a>>,
) -> Result<(), Erro> {
    let items = items_of(doc, owner);
    if items.is_empty() {
        return Ok(());
    }
    let id = list_id(owner)?;
    let mut inner = ancestors.clone();
    inner.push(id.clone());
    for item in &items {
        collect(doc, item, inner.clone(), out)?;
    }
    out.push(List {
        id,
        owner,
        items,
        ancestors,
    });
    Ok(())
}

#[note("targets-makepad-menus-lists")]
pub fn lists<'a>(doc: &'a Document, emitted: &[&'a Node]) -> Result<Vec<List<'a>>, Erro> {
    let mut out = Vec::new();
    for n in emitted {
        match n.class.as_str() {
            "MainMenu" => {
                for top in items_of(doc, n) {
                    collect(doc, top, Vec::new(), &mut out)?;
                }
            }
            "PopupMenu" => collect(doc, n, Vec::new(), &mut out)?,
            _ => {}
        }
    }
    Ok(out)
}

#[note("targets-makepad-menus-close-lists")]
pub fn close_lists(lists: &[List], keep: &[String], pad: usize) -> String {
    let p = " ".repeat(pad);
    lists
        .iter()
        .filter(|l| !keep.contains(&l.id))
        .map(|l| {
            format!(
                "{p}crawrad_runtime_makepad::menu::close(cx, &self.ui.widget(id!({})));\n",
                l.id
            )
        })
        .collect()
}

#[note("targets-makepad-menus-caption")]
fn caption(item: &Node) -> String {
    super::layout::text(item)
}

#[note("targets-makepad-menus-is-separator")]
pub fn is_separator(item: &Node) -> bool {
    caption(item) == "-"
}

fn escaped(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

#[note("targets-makepad-menus-dsl")]
pub fn dsl(doc: &Document, emitted: &[&Node]) -> Result<String, Erro> {
    let mut out = String::new();
    for m in emitted.iter().filter(|n| n.class == "MainMenu") {
        let tops = items_of(doc, m);
        if tops.is_empty() {
            continue;
        }
        out.push_str(&format!(
            "                {}_bar = <View> {{\n\
             \x20                   margin: {{ left: 0.0, top: 0.0 }}\n\
             \x20                   width: Fill, height: {BAR_HEIGHT:.1},\n\
             \x20                   flow: Right,\n\
             \x20                   show_bg: true,\n\
             \x20                   draw_bg: {{ color: #2d2d30 }}\n",
            identificador(m.name())?
        ));
        for t in tops {
            out.push_str(&format!(
                "                    {} = <ButtonFlatter> {{\n\
                 \x20                       width: Fit, height: Fill,\n\
                 \x20                       padding: {{ left: 8.0, right: 8.0 }},\n\
                 \x20                       draw_text: {{ text_style: {{ font_size: 9.0 }} }}\n\
                 \x20                       draw_bg: {{ color_hover: #3e4250, color_down: #4a5068, color_focus: #0000, color_disabled: #0000 }}\n\
                 \x20                       text: \"{}\"\n\
                 \x20                   }}\n",
                identificador(t.name())?,
                escaped(&caption(t))
            ));
        }
        out.push_str("                }\n");
    }
    for l in lists(doc, emitted)? {
        // wide enough for the longest caption and its shortcut
        let longest = l
            .items
            .iter()
            .map(|i| caption(i).chars().count() + shortcut_text(i).chars().count())
            .max()
            .unwrap_or(0);
        let width = (longest as f32 * 7.5 + 64.0).max(140.0);
        out.push_str(&format!(
            "                {} = <RoundedView> {{\n\
             \x20                   visible: false,\n\
             \x20                   width: {width:.1}, height: Fit,\n\
             \x20                   flow: Down,\n\
             \x20                   padding: 3.0,\n\
             \x20                   show_bg: true,\n\
             \x20                   draw_bg: {{ color: #2b2d33, border_size: 1.0, border_color: #4a4d55, border_radius: 3.0 }}\n",
            l.id
        ));
        for i in &l.items {
            let f = identificador(i.name())?;
            if is_separator(i) {
                out.push_str(&format!(
                    "                    {f} = <View> {{\n\
                     \x20                       width: Fill, height: 1.0,\n\
                     \x20                       margin: {{ top: 3.0, bottom: 3.0 }}\n\
                     \x20                       show_bg: true,\n\
                     \x20                       draw_bg: {{ color: #5a5e68 }}\n\
                     \x20                   }}\n"
                ));
            } else {
                out.push_str(&format!(
                    "                    {f} = <ButtonFlatter> {{\n\
                     \x20                       width: Fill, height: 24.0,\n\
                     \x20                       align: {{ x: 0.0, y: 0.5 }},\n\
                     \x20                       padding: {{ left: 6.0, right: 10.0 }},\n\
                     \x20                       draw_text: {{ text_style: {{ font_size: 9.0 }} }}\n\
                     \x20                       draw_bg: {{ color_hover: #3e4250, color_down: #4a5068, color_focus: #0000, color_disabled: #0000 }}\n\
                     \x20                       text: \"{}\"\n\
                     \x20                   }}\n",
                    escaped(&caption(i))
                ));
            }
        }
        out.push_str("                }\n");
    }
    Ok(out)
}

#[note("targets-makepad-menus-shortcut-text")]
fn shortcut_text(item: &Node) -> String {
    match item.props.get("shortcut") {
        Some(crawrad_ir::Value::Shortcut(s)) => s.to_string(),
        _ => String::new(),
    }
}

#[note("targets-makepad-menus-none-open")]
pub fn none_open(doc: &Document, emitted: &[&Node]) -> Result<Option<String>, Erro> {
    let lists = lists(doc, emitted)?;
    if lists.is_empty() {
        return Ok(None);
    }
    let ids: Vec<String> = lists.iter().map(|l| format!("id!({})", l.id)).collect();
    Ok(Some(format!(
        "![{}].into_iter().any(|w| self.ui.widget(w).visible())",
        ids.join(", ")
    )))
}

#[note("targets-makepad-menus-app-state")]
pub fn app_state(doc: &Document, emitted: &[&Node]) -> Result<Option<AppState>, Erro> {
    let lists = lists(doc, emitted)?;
    if lists.is_empty() {
        return Ok(None);
    }
    let mut zones: Vec<String> = lists.iter().map(|l| format!("id!({})", l.id)).collect();
    for m in emitted.iter().filter(|n| n.class == "MainMenu") {
        if !items_of(doc, m).is_empty() {
            zones.push(format!("id!({}_bar)", identificador(m.name())?));
        }
    }
    Ok(Some(AppState {
        uses: String::new(),
        field: String::new(),
        handle: format!(
            "        // ---- THE MENUS: a press outside them closes them --\n\
             \x20       if let Event::MouseDown(e) = event {{\n\
             \x20           let inside = [{}]\n\
             \x20               .into_iter()\n\
             \x20               .any(|w| crawrad_runtime_makepad::menu::covers(cx, &self.ui.widget(w), e.abs));\n\
             \x20           if !inside {{\n\
             {}\
             \x20           }}\n\
             \x20       }}\n",
            zones.join(", "),
            close_lists(&lists, &[], 16)
        ),
    }))
}

#[note("targets-makepad-menus-item-click")]
pub fn item_click(doc: &Document, emitted: &[&Node], item: &Node) -> Result<Option<String>, Erro> {
    let lists = lists(doc, emitted)?;
    if lists.is_empty() {
        return Ok(None);
    }
    let f = identificador(item.name())?;
    let on_bar = doc
        .parent_of(&item.id)
        .and_then(|p| doc.node(p))
        .is_some_and(|p| p.class == "MainMenu");
    let Some(own) = lists.iter().find(|l| l.owner.id == item.id) else {
        return Ok(Some(close_lists(&lists, &[], 12)));
    };
    let body = if on_bar {
        format!(
            "            let open = self.ui.widget(id!({id})).visible();\n\
             {close}\
             \x20           if !open {{\n\
             \x20               let at = crawrad_runtime_makepad::menu::below(cx, &self.ui.widget(id!({f})));\n\
             \x20               crawrad_runtime_makepad::menu::open(cx, &self.ui.widget(id!({id})), &self.ui.widget(id!(body)), at);\n\
             \x20           }}\n",
            id = own.id,
            close = close_lists(&lists, &[], 12)
        )
    } else {
        format!(
            "{close}\
             \x20           let at = crawrad_runtime_makepad::menu::beside(cx, &self.ui.widget(id!({f})));\n\
             \x20           crawrad_runtime_makepad::menu::open(cx, &self.ui.widget(id!({id})), &self.ui.widget(id!(body)), at);\n",
            id = own.id,
            close = close_lists(&lists, &own.ancestors, 12)
        )
    };
    Ok(Some(body))
}

#[note("targets-makepad-menus-popup-open")]
pub fn popup_open(doc: &Document, emitted: &[&Node], field: &str) -> Result<Option<String>, Erro> {
    let lists = lists(doc, emitted)?;
    let mut arms = Vec::new();
    for l in lists.iter().filter(|l| l.owner.class == "PopupMenu") {
        let on_popup = match super::layout::handler(l.owner, "on_popup") {
            Some(h) => format!(
                "                            self.form.{}();\n",
                identificador(&h)?
            ),
            None => String::new(),
        };
        arms.push((
            literal(&identificador(l.owner.name())?),
            format!(
                "{on_popup}\
                 \x20                           crawrad_runtime_makepad::menu::open(cx, &self.ui.widget(id!({})), &self.ui.widget(id!(body)), hit.at);\n",
                l.id
            ),
        ));
    }
    if arms.is_empty() {
        return Ok(None);
    }
    let open = if let [(name, body)] = arms.as_slice() {
        format!(
            "                        if self.form.{field}.popup_menu == {name} {{\n\
             {body}\
             \x20                       }}\n"
        )
    } else {
        let mut m =
            format!("                        match self.form.{field}.popup_menu.as_str() {{\n");
        for (name, body) in &arms {
            m.push_str(&format!(
                "                        {name} => {{\n\
                 {body}\
                 \x20                       }}\n"
            ));
        }
        m.push_str("                        _ => {}\n                        }\n");
        m
    };
    Ok(Some(format!(
        // the button is Makepad's: the form may name a `MouseButton` of its own
        "if hit.button.is_some_and(|b| b.is_secondary()) {{\n\
         {}\
         {open}\
         \x20                   }}",
        close_lists(&lists, &[], 24)
    )))
}
