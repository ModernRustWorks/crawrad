//! **The property map**: which property the user sets in the Object
//! Inspector reaches the generated code.
//!
//! For every class that emits code and every property it declares, the
//! property is set to a value other than its default and both targets
//! generate the project again. What the probe records, per target:
//!
//! - `ignored`: not one byte of the generated project changed — the value
//!   the user set in the Object Inspector is dropped;
//! - `emitted`: the project changed — the value reaches the code (whether
//!   the widget then honours it is what the visual check answers);
//! - `field`: the class's generated struct has a field of that name, which
//!   a handler can read and set at run time.
//!
//! A property the probe cannot change by itself (a reference, a rect, a
//! collection…) is recorded as `not_probed`, with the reason.
//!
//! The gate: every pair still `ignored` is written in `debt.txt` (one
//! `Class.key target` per line), and the list only shrinks — a pair newly
//! lost fails, and so does a pair fixed but still listed.
//!
//! With `CRAWRAD_PROPERTY_MAP` set, the whole map is also written there as JSON
//! (the source of the visual table the owner approved).

use crate::{Dono, Gerador};
use crawrad_ir::{Color, Document, Edges, Rect, Value};
use crawrad_registry::{PropDesc, PropKind, Registry};
use std::collections::BTreeMap;

/// A form the size of the `Blank form` template.
fn form(name: &str) -> Document {
    let mut d = Document::new(name, "Form");
    let root = d.node_mut(name).expect("the root just made");
    root.set_rect(Rect::new(0.0, 0.0, 640.0, 420.0));
    root.set_name(name);
    root.set_text("Form1");
    d
}

/// What `key` needs around it to mean anything, set on both sides of the
/// comparison: in the classic, `Margins` only move a control that is aligned
/// with `AlignWithMargins` on.
///
/// These two keys do all their work through the rect the designer's layout
/// computes, so their documents are laid out first. `align` and `anchors` are
/// not: what they add over the rect is the behaviour when the window resizes,
/// and that only reaches the app if the code carries them.
fn context(key: &str) -> Vec<(&'static str, Value)> {
    let top = ("align", Value::Enum("Top".into()));
    match key {
        "margins" => vec![top, ("align_with_margins", Value::Bool(true))],
        "align_with_margins" => vec![top],
        _ => Vec::new(),
    }
}

/// Keys the form gives its children (`ParentFont`, `ParentShowHint`): on a
/// form with nothing on it they have no one to reach, so the `Form` probe
/// carries a button with a hint of its own.
const INHERITED_BY_CHILDREN: &[&str] = &["font", "show_hint"];

/// The keys that place a control, or a container's group, in the Tab sequence.
const TAB_KEYS: &[&str] = &["tab_order", "tab_stop"];

/// Keys a `ToolBar` spends on its buttons: the designer's layout gives each
/// button on the bar `ButtonWidth` x `ButtonHeight`, and both targets draw
/// that box. On an empty bar they size nothing, so the probe puts a
/// `SpeedButton` on it and lays the form out.
const SIZES_ITS_BUTTONS: &[&str] = &["button_width", "button_height"];

/// The class on a form (the form itself for `Form`), with `key` at `value`
/// when given, laid out as the designer does when `key` has a context;
/// `None` when the class cannot stand on a form by itself.
fn document(class: &str, key: &str, value: Option<&Value>) -> Option<Document> {
    let r = Registry::builtin();
    let mut d = form("form_main");
    let id = if class == "Form" {
        if INHERITED_BY_CHILDREN.contains(&key) {
            let mut child = r.instantiate("Button", "child1", Rect::new(8.0, 8.0, 75.0, 25.0))?;
            child.set_name("child1");
            child
                .props
                .insert("hint".into(), Value::Str("child hint".into()));
            d.add_child("form_main", child).ok()?;
        }
        "form_main".to_string()
    } else {
        let (w, h) = r.get(class)?.default_size;
        let mut n = r.instantiate(class, "probe1", Rect::new(8.0, 8.0, w, h))?;
        n.set_name("probe1");
        // A page means something only in its PageControl, which draws its
        // tab and its body: on the bare form no one reads it.
        let parent = if class == "TabSheet" {
            let (pw, ph) = r.get("PageControl")?.default_size;
            let mut pc = r.instantiate("PageControl", "pages1", Rect::new(8.0, 8.0, pw, ph))?;
            pc.set_name("pages1");
            d.add_child("form_main", pc).ok()?;
            "pages1"
        } else {
            "form_main"
        };
        d.add_child(parent, n).ok()?;
        // A container's `TabOrder` orders the group of controls inside it:
        // with nothing inside there is no group, so it gets an Edit.
        if TAB_KEYS.contains(&key) && r.get(class)?.children_area() {
            let mut child = r.instantiate("Edit", "child1", Rect::new(8.0, 8.0, 80.0, 24.0))?;
            child.set_name("child1");
            let _ = d.add_child("probe1", child);
        }
        if class == "ToolBar" && SIZES_ITS_BUTTONS.contains(&key) {
            let mut b = r.instantiate("SpeedButton", "button1", Rect::new(0.0, 0.0, 23.0, 22.0))?;
            b.set_name("button1");
            d.add_child("probe1", b).ok()?;
        }
        "probe1".to_string()
    };
    let node = d.node_mut(&id)?;
    for (k, v) in context(key) {
        node.props.insert(k.to_string(), v);
    }
    if let Some(v) = value {
        node.props.insert(key.to_string(), v.clone());
        // Turning a ScrollBar's Kind swaps its box, as the Object
        // Inspector does; the orientation is read from that shape
        if let (true, Value::Enum(kind)) = (node.class == "ScrollBar" && key == "kind", v)
            && let Some(r) = node
                .rect()
                .and_then(|r| crawrad_ir::scrolling::scroll_bar_box_for_kind(r, kind))
        {
            node.set_rect(r);
        }
    }
    d.validate().ok()?;
    let laid_out = !context(key).is_empty() || SIZES_ITS_BUTTONS.contains(&key);
    let Some(tx) = crawrad_designer::layout::apply(&d).filter(|_| laid_out) else {
        return Some(d);
    };
    let mut doc = crawrad_doc::Doc::new(d);
    doc.apply(tx).ok()?;
    Some(doc.ir().clone())
}

/// A value other than `p`'s default, or why there is none.
fn other_value(p: &PropDesc) -> Result<Value, &'static str> {
    Ok(match (&p.kind, &p.default) {
        (_, Value::Bool(b)) => Value::Bool(!b),
        (PropKind::Int { min, max }, Value::Int(d)) => {
            // past `max` the probe falls back to `min`, unless `min` IS the
            // default (NumGlyphs: 1 of 1..4), when it takes `max`
            Value::Int(if d + 7 <= *max {
                d + 7
            } else if min != d {
                *min
            } else {
                *max
            })
        }
        (_, Value::Int(d)) => Value::Int(d + 7),
        (PropKind::Float { min, max, .. }, Value::Float(d)) => {
            let v = d + (max - min).abs().min(1000.0) / 3.0;
            Value::Float(if v <= *max { v } else { *min })
        }
        (_, Value::Float(d)) => Value::Float(d + 7.0),
        (PropKind::Enum(opts), _) => {
            let now = match &p.default {
                Value::Enum(s) | Value::Str(s) => s.as_str(),
                _ => "",
            };
            let other = opts.iter().find(|o| **o != now).ok_or("one option only")?;
            Value::Enum((*other).to_string())
        }
        (PropKind::Set(_), Value::Map(m)) => {
            let mut m = m.clone();
            let (k, v) = m.iter().next().ok_or("empty set")?;
            let (k, v) = (k.clone(), !matches!(v, Value::Bool(true)));
            m.insert(k, Value::Bool(v));
            Value::Map(m)
        }
        (_, Value::Str(s)) if s != "ProbeValue7" => match p.kind {
            PropKind::String | PropKind::Multiline => Value::Str("ProbeValue7".into()),
            _ => return Err("text of a special kind"),
        },
        (_, Value::Color(c)) => Value::Color(if c.r == 18 {
            Color::rgb(200, 30, 90)
        } else {
            Color::rgb(18, 52, 86)
        }),
        (_, Value::Edges(_)) => Value::Edges(Edges {
            left: 3.0,
            top: 5.0,
            right: 7.0,
            bottom: 9.0,
        }),
        (_, Value::Font(f)) => {
            let mut f = f.clone();
            f.bold = !f.bold;
            f.size += 3.0;
            Value::Font(f)
        }
        (_, Value::Cursor(c)) => Value::Cursor(if *c == crawrad_ir::Cursor::Hand {
            crawrad_ir::Cursor::Crosshair
        } else {
            crawrad_ir::Cursor::Hand
        }),
        (_, Value::ModalResult(m)) => Value::ModalResult(if *m == crawrad_ir::ModalResult::Ok {
            crawrad_ir::ModalResult::Cancel
        } else {
            crawrad_ir::ModalResult::Ok
        }),
        (PropKind::Reference(_), _) => return Err("reference"),
        (PropKind::Rect, _) => return Err("geometry"),
        (PropKind::Collection { .. }, _) => return Err("collection"),
        _ => return Err("kind not probed"),
    })
}

/// Every file the target writes, concatenated in path order.
fn project(g: &dyn Gerador, d: &Document) -> (String, String) {
    let mut files = g.gerar(d, "probe", &Registry::builtin()).expect("generate");
    files.sort_by(|a, b| a.caminho.cmp(&b.caminho));
    let all: String = files.iter().map(|a| a.conteudo.as_str()).collect();
    let designer = files
        .iter()
        .filter(|a| !matches!(a.dono, Dono::Usuario))
        .map(|a| a.conteudo.as_str())
        .collect();
    (all, designer)
}

/// The debt list: the pairs the targets still drop.
const DEBT: &str = include_str!("debt.txt");

#[test]
fn property_map() {
    let r = Registry::builtin();
    let targets: [(&str, Box<dyn Gerador>); 2] = [
        ("egui", Box::new(crate::GeradorRxp)),
        ("makepad", Box::new(crate::GeradorMakepad)),
    ];
    let mut rows = Vec::new();
    let mut lost = std::collections::BTreeSet::new();
    for desc in r.all().filter(|d| d.emits_code) {
        for p in desc.props.iter().filter(|p| !p.readonly && p.key != "name") {
            let mut row = BTreeMap::new();
            row.insert("class", serde_json::json!(desc.class));
            row.insert("key", serde_json::json!(p.key));
            row.insert("kind", serde_json::json!(format!("{:?}", p.kind)));
            row.insert("category", serde_json::json!(p.category.label()));
            // only the Designer reads it: the app has nothing to keep
            if crate::emission::why_not_set(p.key, p.category)
                == Some(crate::emission::NotSet::DesignTime)
            {
                row.insert("not_probed", serde_json::json!("design-time only"));
                rows.push(row);
                continue;
            }
            let value = match other_value(p) {
                Ok(v) => v,
                Err(why) => {
                    row.insert("not_probed", serde_json::json!(why));
                    rows.push(row);
                    continue;
                }
            };
            let (Some(before), Some(after)) = (
                document(desc.class, p.key, None),
                document(desc.class, p.key, Some(&value)),
            ) else {
                row.insert("not_probed", serde_json::json!("does not stand on a form"));
                rows.push(row);
                continue;
            };
            let field = p.key.replace('.', "_").to_lowercase();
            for (target, g) in &targets {
                // `makepad.*` / `egui.*`: the other target never sees it
                if !crawrad_ir::PropKey::parse(p.key).is_ok_and(|k| k.applies_to(target)) {
                    row.insert(target, serde_json::json!({ "state": "other target" }));
                    continue;
                }
                let (old, _) = project(g.as_ref(), &before);
                let (new, designer) = project(g.as_ref(), &after);
                let state = if old == new { "ignored" } else { "emitted" };
                if old == new {
                    lost.insert(format!("{}.{} {target}", desc.class, p.key));
                }
                let has_field = designer.contains(&format!("pub {field}:"));
                row.insert(
                    if *target == "egui" { "egui" } else { "makepad" },
                    serde_json::json!({ "state": state, "field": has_field }),
                );
            }
            rows.push(row);
        }
    }
    if let Ok(out) = std::env::var("CRAWRAD_PROPERTY_MAP") {
        let text = serde_json::to_string_pretty(&rows).expect("json");
        std::fs::write(&out, text).expect("write the map");
    }
    let debt: std::collections::BTreeSet<String> = DEBT
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(str::to_string)
        .collect();
    let new: Vec<_> = lost.difference(&debt).collect();
    let fixed: Vec<_> = debt.difference(&lost).collect();
    assert!(
        new.is_empty(),
        "the generated code now drops these properties (#667): {new:?}"
    );
    assert!(
        fixed.is_empty(),
        "these properties reach the code now; remove them from debt.txt: {fixed:?}"
    );
}
