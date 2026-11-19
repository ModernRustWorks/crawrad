//! **The shape probe**: what every class paints on the canvas, written
//! down with no GPU.
//!
//! Making the canvas paint nothing at all used to leave every test of this
//! crate green: geometry, gestures and lines were measured, the ink was not.
//! Here every class of the catalogue is painted through the same path as a
//! frame ([`paint_controls`]) on a [`Probe`], and two things are demanded:
//!
//! - every visual class paints something, inside its own box;
//! - what it paints — how many shapes, on which layer, where, in which colour,
//!   with which text — is the shape on record in `shapes.txt`.
//!
//! The record is the regression net. When a drawing changes on purpose, run
//! the test with `CRAWRAD_BLESS_SHAPES=1`, read the diff of `shapes.txt` and
//! commit it with the change: the diff IS the review of what the screen will
//! show. It does not judge looks — that is still the owner's eye.

use std::fmt::Write as _;

use crawrad_ir::{Document, Rect, Value};
use crawrad_registry::Registry;
use crawrad_runtime::Runtime;

use crate::canvas::{Brush, Layer, Mark, Probe, paint_controls};
use crate::components::desenho_de;
use crate::{MakepadRuntime, Transformacao};

/// Where the probed control sits on the form, in design units (and on screen:
/// the probe paints at zoom 1 from the origin).
const BOX: (f32, f32, f32, f32) = (10.0, 10.0, 200.0, 120.0);

const RECORD: &str = include_str!("shapes.txt");

/// Paints a form holding one `class`, instantiated from the catalogue.
fn probe(reg: &Registry, class: &str) -> Result<Vec<Mark>, String> {
    probe_with(reg, class, &[])
}

/// [`probe`] with some properties set over the catalogue's defaults.
fn probe_with(reg: &Registry, class: &str, props: &[(&str, Value)]) -> Result<Vec<Mark>, String> {
    let mut doc = Document::new("form", "Form");
    if let Some(form) = doc.node_mut("form") {
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
    }
    let mut node = reg
        .instantiate(class, "probe", Rect::new(BOX.0, BOX.1, BOX.2, BOX.3))
        .ok_or("the catalogue does not instantiate it")?;
    for (key, value) in props {
        node.props.insert((*key).to_string(), value.clone());
    }
    doc.add_child("form", node).map_err(|e| format!("{e:?}"))?;
    let mut rt = MakepadRuntime::new();
    rt.open(&doc).map_err(|e| format!("{e:?}"))?;
    let mut probe = Probe::default();
    let mut p = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
    paint_controls(
        &mut p,
        &rt.ctls,
        &rt.ordem,
        rt.modo,
        rt.form_rect,
        rt.form_font.as_ref(),
    );
    Ok(probe.marks().to_vec())
}

fn colour(c: [f32; 4]) -> String {
    let b = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
    format!(
        "#{:02x}{:02x}{:02x}{:02x}",
        b(c[0]),
        b(c[1]),
        b(c[2]),
        b(c[3])
    )
}

/// One line per mark, in painting order.
fn record_of(class: &str, marks: &[Mark]) -> String {
    let mut out = String::new();
    for m in marks {
        let _ = write!(
            out,
            "{class} {:?} {:.1} {:.1} {:.1} {:.1} {}",
            m.layer,
            m.rect.pos.x,
            m.rect.pos.y,
            m.rect.size.x,
            m.rect.size.y,
            colour(m.color)
        );
        if let Some(b) = m.border {
            let _ = write!(out, " border {}", colour(b));
        }
        if let Some(t) = &m.text {
            let _ = write!(out, " {t:?}");
        }
        out.push('\n');
    }
    out
}

#[test]
fn every_visual_class_paints_something_inside_its_own_box() {
    let reg = Registry::builtin();
    let (x0, y0, w, h) = BOX;
    let (x1, y1) = (f64::from(x0 + w), f64::from(y0 + h));
    let mut probed = 0;
    let mut blank = Vec::new();
    let mut outside = Vec::new();
    for desc in reg.all() {
        let d = desenho_de(desc.class);
        if !d.visual() || d.drawn_by_owner().is_some() {
            continue;
        }
        let marks = match probe(&reg, desc.class) {
            Ok(m) => m,
            Err(e) => {
                blank.push(format!("{}: {e}", desc.class));
                continue;
            }
        };
        probed += 1;
        if marks.is_empty() {
            blank.push(format!("{}: painted nothing", desc.class));
            continue;
        }
        if !d.clips_to_own_box() {
            continue;
        }
        for m in &marks {
            let r = m.rect;
            let inside = r.pos.x >= f64::from(x0) - 0.01
                && r.pos.y >= f64::from(y0) - 0.01
                && r.pos.x + r.size.x <= x1 + 0.01
                && r.pos.y + r.size.y <= y1 + 0.01;
            if !inside {
                outside.push(format!("{}: {m:?}", desc.class));
            }
        }
    }
    assert!(probed >= 20, "only {probed} visual classes were probed");
    assert!(
        blank.is_empty(),
        "a visual class left the canvas blank: {blank:#?}"
    );
    assert!(
        outside.is_empty(),
        "a class painted outside its own box: {outside:#?}"
    );
}

#[test]
fn the_shapes_on_record_are_the_shapes_painted() {
    let reg = Registry::builtin();
    let mut now = String::new();
    for desc in reg.all() {
        if let Ok(marks) = probe(&reg, desc.class) {
            now.push_str(&record_of(desc.class, &marks));
        }
    }
    if std::env::var_os("CRAWRAD_BLESS_SHAPES").is_some() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/test/probe/shapes.txt");
        std::fs::write(path, &now).expect("write the record");
        return;
    }
    let record = RECORD.replace("\r\n", "\n");
    if record == now {
        return;
    }
    // Name the first class whose shapes moved, with both versions of it.
    let class_of = |l: &str| l.split(' ').next().unwrap_or("").to_string();
    let (recorded, painted): (Vec<&str>, Vec<&str>) =
        (record.lines().collect(), now.lines().collect());
    let at = (0..recorded.len().max(painted.len()))
        .find(|&i| recorded.get(i) != painted.get(i))
        .unwrap_or(0);
    let first = class_of(recorded.get(at).or(painted.get(at)).copied().unwrap_or(""));
    let of = |text: &str| -> String {
        text.lines()
            .filter(|l| class_of(l) == first)
            .collect::<Vec<_>>()
            .join("\n")
    };
    panic!(
        "the canvas paints `{first}` differently from the record.\n\
         recorded:\n{}\n\
         painted now:\n{}\n\
         If the change is on purpose, run with CRAWRAD_BLESS_SHAPES=1 and commit \
         `src/test/probe/shapes.txt` with it.",
        of(&record),
        of(&now)
    );
}

/// The Grid's design-time cells are written on the canvas, each in its
/// own cell.
#[test]
fn the_grid_writes_its_design_time_cells() {
    let marks = probe_with(
        &Registry::builtin(),
        "Grid",
        &[("cell_text", Value::Str("\tName\nAna".into()))],
    )
    .expect("paints");
    let at = |text: &str| {
        marks
            .iter()
            .find(|m| m.text.as_deref() == Some(text))
            .map(|m| m.rect)
    };
    let (Some(name), Some(ana)) = (at("Name"), at("Ana")) else {
        panic!("both texts are written: {marks:?}");
    };
    assert!(
        name.pos.x > ana.pos.x && name.pos.y < ana.pos.y,
        "Name is right of and above Ana: {name:?} {ana:?}"
    );
    // the fixed cell's fill is under the title, not on a layer above the text
    let r = name;
    let covered = marks.iter().any(|m| {
        m.layer == Layer::Stroke
            && m.rect.pos.x <= r.pos.x
            && m.rect.pos.y <= r.pos.y
            && m.rect.pos.x + m.rect.size.x >= r.pos.x + r.size.x
            && m.rect.pos.y + m.rect.size.y >= r.pos.y + r.size.y
    });
    assert!(!covered, "the title is painted over: {marks:?}");
}
