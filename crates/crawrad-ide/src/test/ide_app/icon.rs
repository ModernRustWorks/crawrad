//! The gate over the icons of the Makepad shell.
//!
//! Two families of defect are locked down here, and both have happened in this
//! house:
//!
//! 1. **A glyph the font does not have becomes an empty box.** Nothing panics,
//!    nothing is logged; the screen shows a rectangle. This test reads the
//!    `cmap` of the `codicon.ttf` in the repository and charges for every code
//!    point.
//! 2. **A catalogue class with no drawing.** The Toolbox and the Structure draw
//!    what the catalogue has; a new class with no icon comes out with a gap.
//!    Here that fails.
//!
//! And a third one: the strip texture has one cell per icon, and the shader
//! contains no `sdf.` drawing of its own — data is the single source of truth.

use std::collections::{BTreeMap, BTreeSet, HashSet};

use crawrad_registry::Registry;

use super::icon::{Chrome, ClassIcon};

/// The theme of the Makepad shell, as text — the icon shader lives there.
const THEME: &str = super::tests::TEMA_COMPLETO;

/// The bytes of Codicons, the SAME ones the DSL loads through
/// `crate://self/assets/fontes/codicon.ttf`.
const CODICONS: &[u8] = include_bytes!("../../../assets/fontes/codicon.ttf");

/// The code points Codicons covers, read from the `cmap` of the file.
///
/// Thirty lines of `cmap` format 4 — the only format this font publishes — and
/// they turn into an automatic check a measurement that used to be done by hand
/// and written down in a comment.
pub(crate) fn codicons_code_points() -> BTreeSet<u32> {
    codicons_glyph_ids().into_keys().collect()
}

fn u16_at(o: usize) -> u16 {
    u16::from_be_bytes([CODICONS[o], CODICONS[o + 1]])
}

/// Where the table `tag` starts in the font.
fn table(tag: &[u8; 4]) -> usize {
    let u32_at = |o: usize| {
        u32::from_be_bytes([
            CODICONS[o],
            CODICONS[o + 1],
            CODICONS[o + 2],
            CODICONS[o + 3],
        ])
    };
    (0..u16_at(4) as usize)
        .map(|i| 12 + 16 * i)
        .find(|&o| &CODICONS[o..o + 4] == tag)
        .map(|o| u32_at(o + 8) as usize)
        .unwrap_or_else(|| panic!("Codicons has no `{}`", String::from_utf8_lossy(tag)))
}

/// Each code point with the glyph it draws (`cmap` format 4).
fn codicons_glyph_ids() -> BTreeMap<u32, u16> {
    let u32_at = |o: usize| {
        u32::from_be_bytes([
            CODICONS[o],
            CODICONS[o + 1],
            CODICONS[o + 2],
            CODICONS[o + 3],
        ])
    };
    let cmap = table(b"cmap");

    let mut out = BTreeMap::new();
    for i in 0..u16_at(cmap + 2) as usize {
        let sub = cmap + u32_at(cmap + 8 + 8 * i) as usize;
        if u16_at(sub) != 4 {
            continue;
        }
        let segments = u16_at(sub + 6) as usize / 2;
        let ends = sub + 14;
        let starts = ends + segments * 2 + 2;
        let deltas = starts + segments * 2;
        let ranges = deltas + segments * 2;
        for s in 0..segments {
            let (end, start) = (u16_at(ends + s * 2), u16_at(starts + s * 2));
            if start == 0xFFFF {
                continue;
            }
            let delta = u16_at(deltas + s * 2);
            let range = u16_at(ranges + s * 2) as usize;
            for c in start..=end {
                let g = if range == 0 {
                    c.wrapping_add(delta)
                } else {
                    let o = ranges + s * 2 + range + (c - start) as usize * 2;
                    match u16_at(o) {
                        0 => 0,
                        g => g.wrapping_add(delta),
                    }
                };
                if g != 0 {
                    out.insert(c as u32, g);
                }
            }
        }
    }
    out
}

/// Every chrome role has a glyph, and the glyph exists in the font.
#[test]
fn every_chrome_role_has_a_glyph_in_codicons() {
    let table = codicons_code_points();
    assert!(
        table.len() > 500,
        "the `cmap` reader only found {} code points; it is broken, and an \
         oracle that lies is worse than no oracle",
        table.len()
    );
    for role in Chrome::ALL {
        let g = role.glyph();
        assert_eq!(
            g.chars().count(),
            1,
            "the icon of `{}` has more than one character. An icon font has no \
             space: what is left is an empty box.",
            role.codicon_name()
        );
        let cp = g.chars().next().unwrap() as u32;
        assert!(
            table.contains(&cp),
            "`{}` points at U+{cp:04X}, which Codicons does NOT have. On screen \
             that is an empty box, and nothing complains.",
            role.codicon_name()
        );
    }
}

/// The name the font gives each glyph (`post` format 2: an index per glyph,
/// past the 258 standard names into the font's own Pascal strings).
fn codicons_glyph_names() -> Vec<String> {
    let post = table(b"post");
    assert_eq!(u16_at(post), 2, "Codicons publishes `post` format 2");
    let glyphs = u16_at(post + 32) as usize;
    let indexes: Vec<usize> = (0..glyphs)
        .map(|i| u16_at(post + 34 + 2 * i) as usize)
        .collect();
    let mut own = Vec::new();
    let mut o = post + 34 + 2 * glyphs;
    while o < CODICONS.len() && own.len() < indexes.iter().filter(|&&i| i >= 258).count() {
        let len = CODICONS[o] as usize;
        own.push(String::from_utf8_lossy(&CODICONS[o + 1..o + 1 + len]).into_owned());
        o += 1 + len;
    }
    indexes
        .iter()
        .map(|&i| {
            i.checked_sub(258)
                .and_then(|k| own.get(k).cloned())
                .unwrap_or_default()
        })
        .collect()
}

/// Every chrome role draws the Codicons glyph it NAMES. A code point that
/// exists is not enough: Save All pointed at `screen-full`, the zoom cell
/// at `symbol-string` (it read "abc" in the status bar), and the gate above
/// passed, since the font has both.
#[test]
fn every_chrome_role_draws_the_glyph_it_names() {
    let ids = codicons_glyph_ids();
    let names = codicons_glyph_names();
    assert_eq!(
        names.get(ids[&0xEB4B] as usize).map(String::as_str),
        Some("save"),
        "the `post` reader is broken; an oracle that lies is worse than none"
    );
    for role in Chrome::ALL {
        let cp = role.glyph().chars().next().unwrap() as u32;
        let drawn = ids
            .get(&cp)
            .and_then(|&g| names.get(g as usize))
            .map(String::as_str);
        assert_eq!(
            drawn,
            Some(role.codicon_name()),
            "`{}` points at U+{cp:04X}, which draws `{}`",
            role.codicon_name(),
            drawn.unwrap_or("nothing")
        );
    }
}

/// No two roles share the same drawing.
///
/// The bar and the rail sit side by side; two buttons with the same glyph make
/// a bar nobody can read.
#[test]
fn no_chrome_role_repeats_another_ones_drawing() {
    let glyphs: HashSet<&str> = Chrome::ALL.iter().map(|c| c.glyph()).collect();
    assert_eq!(
        glyphs.len(),
        Chrome::ALL.len(),
        "two chrome roles have the same glyph"
    );
    let names: HashSet<&str> = Chrome::ALL.iter().map(|c| c.codicon_name()).collect();
    assert_eq!(
        names.len(),
        Chrome::ALL.len(),
        "two chrome roles claim to be the same Codicons drawing"
    );
}

/// Codicons does **not** touch the text range, and that is why it can sit in
/// front of a text face in a single font family.
///
/// This is the measured argument that took Font Awesome out of here: it maps
/// the whole latin alphabet (`U+0041`..`U+005A`, `U+0061`..`U+007A`), so in a
/// per-glyph fallback chain
/// (`makepad-draw-1.0.0/src/text/shaper.rs:80-99`) it would eat every letter.
#[test]
fn codicons_does_not_touch_the_text_range() {
    let table = codicons_code_points();
    let low: Vec<u32> = table.iter().copied().filter(|c| *c < 0x100).collect();
    assert!(
        low.is_empty(),
        "Codicons now maps code below U+0100 ({low:?}); it can no longer sit in \
         front of text in the same family"
    );
}

/// Every component the Toolbox offers has an icon.
#[test]
fn every_palette_class_has_an_icon() {
    let reg = Registry::builtin();
    let mut missing = Vec::new();
    for (_, items) in reg.palette(crawrad_runtime_makepad::ALVO) {
        for d in items {
            if ClassIcon::of_class(d.class).is_none() {
                missing.push(d.class);
            }
        }
    }
    assert!(
        missing.is_empty(),
        "a catalogue class with no drawing: the row shows a gap where the icon \
         goes:\n  {}",
        missing.join("\n  ")
    );
}

/// The class-to-icon pair round-trips.
#[test]
fn the_class_and_the_icon_are_one_source() {
    for i in ClassIcon::ALL {
        assert_eq!(
            ClassIcon::of_class(i.class()),
            Some(*i),
            "`{}` does not come back through `of_class`",
            i.class()
        );
    }
    let classes: HashSet<&str> = ClassIcon::ALL.iter().map(|i| i.class()).collect();
    assert_eq!(
        classes.len(),
        ClassIcon::ALL.len(),
        "two icon variants claim to draw the same class"
    );
}

/// The `kind` is unique and is an index into `ALL`.
///
/// It is the value the shader compares against; two equal ones would make two
/// components come out with the same drawing, and one out of range would paint
/// nothing.
#[test]
fn the_shader_kind_is_unique_and_in_range() {
    let mut seen = Vec::new();
    for i in ClassIcon::ALL {
        let k = i.kind();
        assert!(
            k >= 0.0 && k < ClassIcon::ALL.len() as f64,
            "`{}` has kind {k}, out of range",
            i.class()
        );
        assert!(
            !seen.contains(&(k as i64)),
            "`{}` repeats the kind {k} of another icon",
            i.class()
        );
        seen.push(k as i64);
    }
}

#[test]
fn the_grid_and_the_stroke_are_one_for_all() {
    assert!(
        THEME.contains("pub CRAWRAD_ICONE_GRADE = 16.0"),
        "the icon grid is no longer 16"
    );
    assert!(
        THEME.contains("pub CRAWRAD_ICONE_TRACO = 1.25"),
        "the stroke weight is no longer 1.25"
    );
}

/// The body of the `CrawradClassIcon` `fn pixel`, as text.
///
/// The boundaries are names the DSL itself declares; if one of them disappears
/// the test fails saying so, instead of passing while reading the wrong part of
/// the file.
fn shader_block() -> &'static str {
    let start = THEME
        .find("pub CrawradClassIcon = ")
        .expect("`CrawradClassIcon` left the theme; revisit the boundary of this test");
    let rest = &THEME[start..];
    let end = rest
        .find("pub CrawradIconRow = ")
        .expect("`CrawradIconRow` left the theme; revisit the boundary of this test");
    &rest[..end]
}

#[test]
fn the_strip_has_one_cell_per_icon() {
    let needle = format!("pub CRAWRAD_ICONE_QUANTIDADE = {}.0", ClassIcon::ALL.len());
    assert!(
        THEME.contains(&needle),
        "theme does not define `CRAWRAD_ICONE_QUANTIDADE = {}.0`",
        ClassIcon::ALL.len()
    );
}

#[test]
fn the_stroke_constant_is_the_token() {
    let needle = format!(
        "pub CRAWRAD_ICONE_TRACO = {}",
        crawrad_project::icon_strip::ICON_STROKE
    );
    assert!(
        THEME.contains(&needle),
        "the theme stroke token does not match ICON_STROKE"
    );
}

#[test]
fn every_icon_paints_something_in_its_own_cell() {
    let side = 16u32;
    let count = ClassIcon::ALL.len();
    let cell = (side * crawrad_project::icon_strip::OVERSAMPLE) as usize;
    let (width, height, buf) = crawrad_project::icon_strip::strip_pixels(side, &[]);
    assert_eq!(width, count * cell, "strip width mismatch");
    assert_eq!(height, cell, "strip height mismatch");
    assert_eq!(buf.len(), width * height, "strip buffer size mismatch");

    for (k, icon) in ClassIcon::ALL.iter().enumerate() {
        let painted = (0..cell).any(|y| (0..cell).any(|x| buf[y * width + k * cell + x] != 0));
        assert!(
            painted,
            "icon `{}` (index {k}) painted nothing in its cell",
            icon.class()
        );
    }
}

#[test]
fn the_shader_has_no_drawing_of_its_own() {
    let block = shader_block();
    assert!(
        !block.contains("sdf."),
        "the `CrawradClassIcon` block still contains `sdf.` drawing calls"
    );
}

/// **Every mark of the completion list and of the breadcrumb is in
/// Codicons** — they draw in a family where the code font falls back to
/// Codicons, and a point neither font has is an empty box (the `▫` the owner
/// saw for `⬡ ◇ ⧉`).
#[test]
fn every_completion_kind_mark_exists_in_codicons() {
    use crawrad_insight::item::ItemKind;
    let points = codicons_code_points();
    let kinds = [
        ItemKind::Field,
        ItemKind::Method,
        ItemKind::Type,
        ItemKind::Keyword,
        ItemKind::Snippet,
        ItemKind::Pending,
    ];
    for kind in kinds {
        let mark = kind.badge();
        assert!(!mark.is_empty(), "{kind:?} has no mark");
        for c in mark.chars() {
            assert!(
                points.contains(&(c as u32)),
                "{kind:?}: U+{:04X} is not in codicon.ttf",
                c as u32
            );
        }
    }
    for mark in [
        crawrad_host::breadcrumb::METHOD,
        crawrad_host::breadcrumb::TYPE,
    ] {
        let point = mark.chars().next().unwrap() as u32;
        assert!(
            points.contains(&point),
            "the breadcrumb's U+{point:04X} is not in codicon.ttf"
        );
    }
}

/// **The native icons obey the rule a third party is held to.**
///
/// `crawrad_ir::icon::validate` is what a manifest's icon must pass. A native
/// drawing that would be refused from outside would be the house keeping a
/// privilege the format denies (ADR 0035).
#[test]
fn every_native_icon_passes_the_third_party_rule() {
    for icon in ClassIcon::ALL {
        if let Err(e) = crawrad_ir::icon::validate(icon.steps()) {
            panic!(
                "the native `{}` icon would be refused in a manifest: {e}",
                icon.class()
            );
        }
    }
}

#[test]
fn a_third_party_icon_gets_its_own_cell() {
    use crawrad_ir::icon::IconStep;
    let extra = vec![(
        "ThatClass".to_string(),
        vec![
            IconStep::Circle {
                x: 8.0,
                y: 8.0,
                r: 4.0,
            },
            IconStep::Fill,
        ],
    )];
    let side = 16u32;
    let cell = (side * crawrad_project::icon_strip::OVERSAMPLE) as usize;
    let (width, height, buf) = crawrad_project::icon_strip::strip_pixels(side, &extra);
    let expected_count = ClassIcon::ALL.len() + 1;
    assert_eq!(
        width,
        expected_count * cell,
        "strip width mismatch with extra icon"
    );
    assert_eq!(height, cell, "strip height mismatch");
    assert_eq!(buf.len(), width * height, "strip buffer size mismatch");

    let k = ClassIcon::ALL.len();
    let painted = (0..cell).any(|y| (0..cell).any(|x| buf[y * width + k * cell + x] != 0));
    assert!(painted, "the extra cell painted nothing");

    let strip = crawrad_project::icon_strip::IconCells::new(extra);
    assert_eq!(strip.kind_of("ThatClass"), Some(k as f64));
    assert_eq!(strip.count(), (k + 1) as f64);
}

#[test]
fn a_native_class_keeps_its_native_icon() {
    use crawrad_ir::icon::IconStep;
    let mut reg = Registry::builtin();
    let comp = crawrad_registry::external::ExternalComponent {
        inherits: None,
        class: "Button".to_string(),
        crate_module: "custom_button".to_string(),
        desenho: Vec::new(),
        icon: vec![
            IconStep::Circle {
                x: 8.0,
                y: 8.0,
                r: 3.0,
            },
            IconStep::Fill,
        ],
        display: "Button".to_string(),
        base_name: "button".to_string(),
        palette: "Standard".to_string(),
        prefix: "btn".to_string(),
        default_size: (75.0, 25.0),
        props: Vec::new(),
        events: Vec::new(),
        targets: vec!["makepad".to_string()],
        children: crawrad_registry::external::ExternalChildren::None,
        derived_geometry: false,
        visual: true,
        emits_code: true,
        private_state: false,
        provides: Vec::new(),
        text_line: None,
        requires: Vec::new(),
    };
    reg.register(comp.into_catalogue());
    let extras = crawrad_project::icon_strip::extra_icons(&reg);
    assert!(
        extras.iter().all(|(name, _)| name != "Button"),
        "extra_icons must not include native class Button"
    );
}
