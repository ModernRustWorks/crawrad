//! Picture atlas for the designer canvas.
//!
//! Makepad paints pictures through textured quads. Each draw call binds one
//! texture; to keep the stack order intact and draw all pictures in a single
//! layer ([`crate::canvas::Layer::Picture`]), all decoded pictures are packed
//! into a single atlas texture.

use std::collections::HashMap;

use makepad_widgets::image_cache::ImageBuffer;
use makepad_widgets::*;

use crate::picture::{Kind, fingerprint, kind_of};
use crawrad_note::note;

#[note("canvas-atlas-atlas-side")]
pub const ATLAS_SIDE: usize = 2048;

#[note("canvas-atlas-max-picture-side")]
pub const MAX_PICTURE_SIDE: usize = 512;

#[note("canvas-atlas-atlasentry")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AtlasEntry {
    /// Texture coordinate rectangle: `[x, y, w, h]` in `0.0..1.0`.
    pub uv: [f32; 4],
    /// Original pixel width of the picture before downscaling.
    pub original_width: usize,
    /// Original pixel height of the picture before downscaling.
    pub original_height: usize,
}

#[note("canvas-atlas-pictureatlas")]
#[derive(Default)]
pub struct PictureAtlas {
    /// Allocated with the first picture: a form without one, and every test
    /// `Probe`, keep the 16 MB they would never use.
    buffer: Vec<u32>,
    map: HashMap<u64, AtlasEntry>,
    cursor_x: usize,
    cursor_y: usize,
    shelf_height: usize,
    dirty: bool,
    texture: Option<Texture>,
}

impl PictureAtlas {
    #[note("canvas-atlas-pictureatlas-new")]
    pub fn new() -> Self {
        Self::default()
    }

    #[note("canvas-atlas-pictureatlas-clear")]
    pub fn clear(&mut self) {
        self.buffer.fill(0);
        self.map.clear();
        self.cursor_x = 0;
        self.cursor_y = 0;
        self.shelf_height = 0;
        self.dirty = true;
    }

    #[note("canvas-atlas-pictureatlas-texture")]
    pub fn texture(&self) -> Option<&Texture> {
        self.texture.as_ref()
    }

    #[note("canvas-atlas-pictureatlas-pack-shelf")]
    fn pack_shelf(&mut self, w: usize, h: usize) -> Option<(usize, usize)> {
        if w > ATLAS_SIDE || h > ATLAS_SIDE {
            return None;
        }
        if self.cursor_x + w > ATLAS_SIDE {
            self.cursor_y += self.shelf_height;
            self.cursor_x = 0;
            self.shelf_height = 0;
        }
        if self.cursor_y + h > ATLAS_SIDE {
            return None;
        }
        let pos = (self.cursor_x, self.cursor_y);
        self.cursor_x += w;
        self.shelf_height = self.shelf_height.max(h);
        Some(pos)
    }

    #[note("canvas-atlas-pictureatlas-insert-image")]
    pub fn insert_image(
        &mut self,
        print: u64,
        orig_w: usize,
        orig_h: usize,
        data: &[u32],
    ) -> [f32; 4] {
        let (w, h) = downscale_size(orig_w, orig_h, MAX_PICTURE_SIDE);
        let downscaled = if w == orig_w && h == orig_h {
            None
        } else {
            Some(downscale_box_filter(data, orig_w, orig_h, w, h))
        };
        let pixels: &[u32] = downscaled.as_deref().unwrap_or(data);

        if self.buffer.is_empty() {
            self.buffer = vec![0u32; ATLAS_SIDE * ATLAS_SIDE];
        }
        let (alloc_x, alloc_y) = match self.pack_shelf(w, h) {
            Some(pos) => pos,
            None => {
                self.clear();
                self.pack_shelf(w, h)
                    .expect("downscaled picture must fit in empty atlas")
            }
        };

        for row in 0..h {
            let dst_offset = (alloc_y + row) * ATLAS_SIDE + alloc_x;
            let src_offset = row * w;
            self.buffer[dst_offset..dst_offset + w]
                .copy_from_slice(&pixels[src_offset..src_offset + w]);
        }
        self.dirty = true;

        let uv = [
            alloc_x as f32 / ATLAS_SIDE as f32,
            alloc_y as f32 / ATLAS_SIDE as f32,
            w as f32 / ATLAS_SIDE as f32,
            h as f32 / ATLAS_SIDE as f32,
        ];
        self.map.insert(
            print,
            AtlasEntry {
                uv,
                original_width: orig_w,
                original_height: orig_h,
            },
        );
        uv
    }

    #[note("canvas-atlas-pictureatlas-get-or-insert")]
    pub fn get_or_insert(&mut self, bytes: &[u8]) -> Option<AtlasEntry> {
        if bytes.is_empty() {
            return None;
        }
        let print = fingerprint(bytes);
        if let Some(entry) = self.map.get(&print) {
            return Some(*entry);
        }
        let kind = kind_of(bytes)?;
        let img = match kind {
            Kind::Png => ImageBuffer::from_png(bytes).ok()?,
            Kind::Jpeg => ImageBuffer::from_jpg(bytes).ok()?,
        };
        if img.width == 0 || img.height == 0 || img.data.len() < img.width * img.height {
            return None;
        }
        let uv = self.insert_image(print, img.width, img.height, &img.data);
        Some(AtlasEntry {
            uv,
            original_width: img.width,
            original_height: img.height,
        })
    }

    #[note("canvas-atlas-pictureatlas-uv-of")]
    pub fn uv_of(&mut self, bytes: &[u8]) -> Option<[f32; 4]> {
        self.get_or_insert(bytes).map(|e| e.uv)
    }

    #[note("canvas-atlas-pictureatlas-size-of")]
    pub fn size_of(&mut self, bytes: &[u8]) -> Option<(usize, usize)> {
        self.get_or_insert(bytes)
            .map(|e| (e.original_width, e.original_height))
    }

    #[note("canvas-atlas-pictureatlas-upload")]
    pub fn upload(&mut self, cx: &mut Cx) -> bool {
        if !self.dirty || self.buffer.is_empty() {
            return false;
        }
        if let Some(texture) = &self.texture {
            texture.put_back_vec_u32(cx, self.buffer.clone(), None);
        } else {
            let texture = Texture::new_with_format(
                cx,
                TextureFormat::VecBGRAu8_32 {
                    width: ATLAS_SIDE,
                    height: ATLAS_SIDE,
                    data: Some(self.buffer.clone()),
                    updated: TextureUpdated::Full,
                },
            );
            self.texture = Some(texture);
        }
        self.dirty = false;
        true
    }
}

#[note("canvas-atlas-downscale-size")]
pub fn downscale_size(w: usize, h: usize, max_side: usize) -> (usize, usize) {
    if w == 0 || h == 0 || max_side == 0 {
        return (0, 0);
    }
    if w <= max_side && h <= max_side {
        return (w, h);
    }
    if w >= h {
        let new_w = max_side;
        let new_h = (((h as f64) * (max_side as f64) / (w as f64)).round() as usize).max(1);
        (new_w, new_h)
    } else {
        let new_h = max_side;
        let new_w = (((w as f64) * (max_side as f64) / (h as f64)).round() as usize).max(1);
        (new_w, new_h)
    }
}

#[note("canvas-atlas-downscale-box-filter")]
pub fn downscale_box_filter(
    src: &[u32],
    src_w: usize,
    src_h: usize,
    dst_w: usize,
    dst_h: usize,
) -> Vec<u32> {
    if dst_w == 0 || dst_h == 0 {
        return Vec::new();
    }
    if src_w == dst_w && src_h == dst_h {
        return src.to_vec();
    }
    let mut dst = vec![0u32; dst_w * dst_h];
    for dy in 0..dst_h {
        let sy0 = (dy * src_h) / dst_h;
        let sy1 = (((dy + 1) * src_h) / dst_h).max(sy0 + 1).min(src_h);
        for dx in 0..dst_w {
            let sx0 = (dx * src_w) / dst_w;
            let sx1 = (((dx + 1) * src_w) / dst_w).max(sx0 + 1).min(src_w);

            let mut sum_b = 0u64;
            let mut sum_g = 0u64;
            let mut sum_r = 0u64;
            let mut sum_a = 0u64;
            // Never zero: every destination pixel covers at least one source one.
            let count = ((sy1 - sy0) * (sx1 - sx0)) as u64;

            for sy in sy0..sy1 {
                let row_offset = sy * src_w;
                for sx in sx0..sx1 {
                    let pixel = src[row_offset + sx];
                    sum_b += (pixel & 0xFF) as u64;
                    sum_g += ((pixel >> 8) & 0xFF) as u64;
                    sum_r += ((pixel >> 16) & 0xFF) as u64;
                    sum_a += ((pixel >> 24) & 0xFF) as u64;
                }
            }

            let avg_b = (sum_b / count) as u32;
            let avg_g = (sum_g / count) as u32;
            let avg_r = (sum_r / count) as u32;
            let avg_a = (sum_a / count) as u32;
            dst[dy * dst_w + dx] = (avg_a << 24) | (avg_r << 16) | (avg_g << 8) | avg_b;
        }
    }
    dst
}

#[note("canvas-atlas-fit-rect")]
pub fn fit_rect(
    control: Rect,
    image_w: f64,
    image_h: f64,
    stretch: bool,
    proportional: bool,
    center: bool,
) -> Rect {
    if image_w <= 0.0 || image_h <= 0.0 || control.size.x <= 0.0 || control.size.y <= 0.0 {
        return Rect {
            pos: control.pos,
            size: dvec2(0.0, 0.0),
        };
    }
    let (w, h) = if proportional {
        let aspect = image_w / image_h;
        let control_aspect = control.size.x / control.size.y;
        if aspect > control_aspect {
            (control.size.x, control.size.x / aspect)
        } else {
            (control.size.y * aspect, control.size.y)
        }
    } else if stretch {
        (control.size.x, control.size.y)
    } else {
        (image_w, image_h)
    };

    let (x, y) = if center {
        (
            control.pos.x + (control.size.x - w) * 0.5,
            control.pos.y + (control.size.y - h) * 0.5,
        )
    } else {
        (control.pos.x, control.pos.y)
    };

    Rect {
        pos: dvec2(x, y),
        size: dvec2(w, h),
    }
}

#[cfg(test)]
#[path = "../test/atlas/test.rs"]
mod tests;
