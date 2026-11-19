//! The drawing for what will **never** have a face: the non-visual badge.
//!
//! An `ImageList`, an `ActionList`, a `Timer` or an `Action` have no appearance
//! in the application — in a classic RAD IDE they live on the designer form as a little
//! square with the class initials and the name underneath, and that is all one
//! sees. This file is that drawing.
//!
//! # Why it is NOT [`super::generic`]
//!
//! Both draw "a component with no painting of its own", and there the
//! similarity ends:
//!
//! | | what it is | how it shows |
//! |---|---|---|
//! | [`super::generic`] | a class that entered the catalog and has no file here yet — **our defect, temporary** | dashed rectangle, which SHOUTS that a file is missing |
//! | this one | a component that by nature has no screen | filled badge, which is its DEFINITIVE drawing |
//!
//! While the non-visual fell through to `generic`, the `ActionList` showed up
//! as an empty dashed box reading `ActionList` — indistinguishable from a hole
//! in the adapter. The owner caught it by looking: *"it was filled and the name
//! centered"*.
//!
//! # Why one shared module, and not four identical drawings
//!
//! The four paint **the same thing**; what differs between them is only the
//! class, and the class already comes out of `classe()`. The house rule does
//! not ask for one file per class for its own sake — it asks that no central
//! place decide by class. There is no decision here: `image_list.rs` and the
//! other three hold their class name and nothing else, and [`super`]'s catalog
//! still knows no class name at all.
//!
//! The contract with the RXP is at `crawrad-runtime-egui/src/lib.rs:2385-2407`
//! (`fn selo_nao_visual`) and `:1785-1800` (the name, in `Align2::CENTER_TOP`).

use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene};

#[crawrad_note::note("components-non-visual-initials")]
pub fn initials(class: &str) -> String {
    class.chars().filter(|c| c.is_uppercase()).collect()
}

#[crawrad_note::note("components-non-visual-icon-box")]
pub fn icon_box(badge: Rect, scale: f64) -> Rect {
    let side = (16.0 * scale).min(badge.size.x).min(badge.size.y).round();
    Rect {
        pos: dvec2(
            (badge.pos.x + (badge.size.x - side) * 0.5).round(),
            (badge.pos.y + (badge.size.y - side) * 0.5).round(),
        ),
        size: dvec2(side, side),
    }
}

#[crawrad_note::note("components-non-visual-badge-box")]
pub fn badge_box(r: Rect) -> Rect {
    r
}

#[crawrad_note::note("components-non-visual-name-box")]
pub fn name_box(badge: Rect, scale: f64) -> Rect {
    let gap = (2.0 * scale).max(1.0);
    let height = (12.0 * scale).max(6.0);
    let width = (badge.size.x * 3.0).max(24.0);
    Rect {
        pos: dvec2(
            badge.pos.x + badge.size.x * 0.5 - width * 0.5,
            badge.pos.y + badge.size.y + gap,
        ),
        size: dvec2(width, height),
    }
}

#[crawrad_note::note("components-non-visual-badge")]
pub struct Badge {
    class: &'static str,
    strip: Option<FormStrip>,
}

#[crawrad_note::note("components-non-visual-declared-class")]
pub const DECLARED_CLASS: &str = "crawrad::DeclaredNonVisual";

#[crawrad_note::note("components-non-visual-declared")]
pub static DECLARED: Badge = Badge::for_class(DECLARED_CLASS);

#[crawrad_note::note("components-non-visual-formstrip")]
pub type FormStrip = fn(&mut Brush, &MakepadCtl, crawrad_ir::Rect, &Scene) -> bool;

impl Badge {
    #[crawrad_note::note("components-non-visual-badge-for-class")]
    pub const fn for_class(class: &'static str) -> Self {
        Self { class, strip: None }
    }

    #[crawrad_note::note("components-non-visual-badge-with-form-strip")]
    pub const fn with_form_strip(self, strip: FormStrip) -> Self {
        Self {
            strip: Some(strip),
            ..self
        }
    }
}

impl ComponentDrawing for Badge {
    fn class_name(&self) -> &'static str {
        self.class
    }

    fn draw_form_strip(
        &self,
        p: &mut Brush,
        c: &MakepadCtl,
        form: crawrad_ir::Rect,
        scene: &Scene,
    ) -> bool {
        self.strip.is_some_and(|strip| strip(p, c, form, scene))
    }

    #[crawrad_note::note("components-non-visual-badge-visual")]
    fn visual(&self) -> bool {
        false
    }

    #[crawrad_note::note("components-non-visual-badge-clips-to-own-box")]
    fn clips_to_own_box(&self) -> bool {
        false
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        // `selo`/`escrever_no_selo`, and NOT `caixa`/`escrever`: the non-visual
        // is a designer overlay and always sits **in front of** every visual
        // component. The layer is [`crate::canvas::DrawSelo`]'s, emitted after
        // everyone else's — see the ordering note in the canvas DSL.
        let badge = badge_box(r);
        p.badge(
            badge,
            MAKEPAD_NAO_VISUAL_FUNDO,
            MAKEPAD_NAO_VISUAL_BORDA,
            3.0,
        );
        // The class icon — the very drawing of its palette tile; the initials
        // only for a class that has none.
        let icon = icon_box(badge, p.medida(1.0));
        if !p.badge_icon(icon, &c.icon, MAKEPAD_NAO_VISUAL_TEXTO) {
            p.write_in_badge(
                badge,
                &initials(&c.declared_class),
                MAKEPAD_NAO_VISUAL_TEXTO,
                true,
            );
        }

        // The name tells apart two `ActionList`s that are the same badge — it
        // is what a classic RAD IDE shows under the icon.
        let name = name_box(badge, p.medida(1.0));
        p.write_in_badge(name, &c.name, MAKEPAD_NOME_NAO_VISUAL, true);
    }
}

#[cfg(test)]
#[path = "../test/non_visual/test.rs"]
mod tests;
