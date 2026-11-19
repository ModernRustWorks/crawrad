//! The class icon strip texture for the Makepad shell.
//!
//! Uploads the strip `crawrad_project::icon_strip` rasterizes on the CPU as one
//! horizontal texture, cached by DPI and by the set of third-party icons.

use crawrad_ir::icon::IconStep;
use crawrad_note::note;
use crawrad_project::icon_strip::{IconCells, extra_icons, strip_pixels};
use makepad_widgets::*;

#[derive(Default)]
pub(crate) struct ClassIconStrip {
    side: u32,
    cells: IconCells,
    texture: Option<Texture>,
}

impl ClassIconStrip {
    #[note("ide-app-class-icon-strip-classiconstrip-texture")]
    pub(crate) fn texture(
        &mut self,
        cx: &mut Cx,
        dpi: f64,
        extra: Vec<(String, Vec<IconStep>)>,
    ) -> Texture {
        let side = ((crawrad_ir::icon::GRID as f64) * dpi).round().max(1.0) as u32;
        if side != self.side || self.cells.extra() != extra.as_slice() || self.texture.is_none() {
            let (width, height, buf) = strip_pixels(side, &extra);
            let texture = Texture::new_with_format(
                cx,
                TextureFormat::VecBGRAu8_32 {
                    width,
                    height,
                    data: Some(buf),
                    updated: TextureUpdated::Full,
                },
            );
            self.side = side;
            self.cells = IconCells::new(extra);
            self.texture = Some(texture);
        }
        self.texture.clone().expect("texture was initialized above")
    }

    #[note("ide-app-class-icon-strip-classiconstrip-cells")]
    pub(crate) fn cells(&self) -> &IconCells {
        &self.cells
    }
}

impl super::IdeApp {
    #[note("ide-app-class-icon-strip-ideapp-class-icon-texture")]
    pub(super) fn class_icon_texture(&mut self, cx: &mut Cx) -> Texture {
        let window = self.window_id(cx);
        let dpi = cx.windows[window].window_geom.dpi_factor;
        let dpi = if dpi > 0.0 { dpi } else { 1.0 };
        let extra = self
            .designer
            .registry
            .as_deref()
            .map(extra_icons)
            .unwrap_or_default();
        self.class_icon_strip.texture(cx, dpi, extra)
    }
}
