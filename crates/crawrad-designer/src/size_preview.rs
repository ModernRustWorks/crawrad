//! Pure model for the Sizes preview tab.

use crate::layout;
use crawrad_doc::Doc;
use crawrad_ir::{Document, NodeId, Rect, Value};
use crawrad_registry::Registry;

/// Target platform presets for the sizes tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizePlatform {
    Desktop,
    Mobile,
    Web,
}

/// A size preset with a display name and viewport dimensions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SizePreset {
    pub name: String,
    pub width: u32,
    pub height: u32,
}

/// Preset sizes for a platform.
pub fn presets(platform: SizePlatform) -> Vec<SizePreset> {
    match platform {
        SizePlatform::Desktop => vec![
            SizePreset {
                name: "Desktop".to_string(),
                width: 1280,
                height: 800,
            },
            SizePreset {
                name: "Tablet".to_string(),
                width: 820,
                height: 1180,
            },
            SizePreset {
                name: "Phone".to_string(),
                width: 390,
                height: 844,
            },
            SizePreset {
                name: "Laptop".to_string(),
                width: 1366,
                height: 768,
            },
            SizePreset {
                name: "Full HD".to_string(),
                width: 1920,
                height: 1080,
            },
        ],
        SizePlatform::Mobile => vec![
            SizePreset {
                name: "Phone".to_string(),
                width: 390,
                height: 844,
            },
            SizePreset {
                name: "Tablet".to_string(),
                width: 820,
                height: 1180,
            },
            SizePreset {
                name: "Desktop".to_string(),
                width: 1280,
                height: 800,
            },
            SizePreset {
                name: "Large phone".to_string(),
                width: 430,
                height: 932,
            },
            SizePreset {
                name: "Tablet landscape".to_string(),
                width: 1180,
                height: 820,
            },
        ],
        SizePlatform::Web => vec![
            SizePreset {
                name: "1440".to_string(),
                width: 1440,
                height: 900,
            },
            SizePreset {
                name: "1024".to_string(),
                width: 1024,
                height: 768,
            },
            SizePreset {
                name: "390".to_string(),
                width: 390,
                height: 844,
            },
            SizePreset {
                name: "768".to_string(),
                width: 768,
                height: 1024,
            },
        ],
    }
}

/// Initial visibility of presets for a platform.
pub fn presets_shown(platform: SizePlatform) -> Vec<bool> {
    match platform {
        SizePlatform::Desktop => vec![true, true, true, false, false],
        SizePlatform::Mobile => vec![true, true, false, false, false],
        SizePlatform::Web => vec![true, true, true, false],
    }
}

/// Lays out a form at a specific viewport size.
///
/// Creates a clone of `doc` whose root rect is `width`×`height`, with
/// anchors and then `layout::apply` (Align) applied, in that order.
/// The input document is never changed.
pub fn frame(doc: &Document, width: f32, height: f32) -> Document {
    let mut framed = doc.clone();
    let (design_w, design_h) = doc
        .node(&doc.root)
        .and_then(|n| n.rect())
        .map(|r| (r.w, r.h))
        .unwrap_or((width, height));

    let root = framed.root.clone();
    if let Some(root_node) = framed.node_mut(&root) {
        let r = root_node
            .rect()
            .unwrap_or(Rect::new(0.0, 0.0, width, height));
        root_node.set_rect(Rect::new(r.x, r.y, width, height));
    }

    let mut live = Doc::new(framed);
    let dw = width - design_w;
    let dh = height - design_h;
    if let Some(tx) = layout::apply_anchors(live.ir(), dw, dh) {
        let _ = live.apply(tx);
    }
    if let Some(tx) = layout::apply(live.ir()) {
        let _ = live.apply(tx);
    }
    live.ir().clone()
}

/// The frame edge where a control overflows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edge {
    Right,
    Bottom,
}

/// Information about a control overflowing a frame edge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Overflow {
    pub node: NodeId,
    pub edge: Edge,
    pub px: u32,
}

/// Finds all direct root children that overflow the frame boundaries.
pub fn overflows(framed: &Document, reg: &Registry) -> Vec<Overflow> {
    let Some(root_node) = framed.node(&framed.root) else {
        return Vec::new();
    };
    let Some(root_rect) = root_node.rect() else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for id in &root_node.children {
        let Some(node) = framed.node(id) else {
            continue;
        };
        let is_visual = reg.get(&node.class).is_some_and(|d| d.visual);
        if !is_visual {
            continue;
        }
        let Some(rect) = node.rect() else {
            continue;
        };
        if matches!(node.props.get("visible"), Some(Value::Bool(false))) {
            continue;
        }
        let past_right = rect.x + rect.w - root_rect.w;
        if past_right > 0.0 {
            let px = past_right.ceil() as u32;
            out.push(Overflow {
                node: id.clone(),
                edge: Edge::Right,
                px,
            });
        }
        let past_bottom = rect.y + rect.h - root_rect.h;
        if past_bottom > 0.0 {
            let px = past_bottom.ceil() as u32;
            out.push(Overflow {
                node: id.clone(),
                edge: Edge::Bottom,
                px,
            });
        }
    }
    out
}

/// Formats the single-line explanation for one overflow.
pub fn overflow_line(framed: &Document, o: &Overflow) -> String {
    let name = framed.node(&o.node).map_or(o.node.as_str(), |n| n.name());
    let px = o.px;
    match o.edge {
        Edge::Right => {
            format!(
                "{name} runs {px} px past the right edge — anchor it right (akRight) or set Align"
            )
        }
        Edge::Bottom => {
            format!(
                "{name} runs {px} px past the bottom edge — anchor it to the bottom (akBottom) or set Align"
            )
        }
    }
}

/// Formats the single status line under a preview frame and reports if it is a warning.
pub fn frame_note(framed: &Document, reg: &Registry) -> (String, bool) {
    let ovs = overflows(framed, reg);
    if ovs.is_empty() {
        ("Fits".to_string(), false)
    } else {
        let first = overflow_line(framed, &ovs[0]);
        if ovs.len() > 1 {
            let more = ovs.len() - 1;
            (format!("{first} (and {more} more)"), true)
        } else {
            (first, true)
        }
    }
}

/// One shown size laid out: what a frame of the Sizes tab draws.
#[derive(Debug, Clone)]
pub struct FramePlan {
    pub preset: SizePreset,
    /// The form at this size (anchors and Align applied).
    pub framed: Document,
    /// The one line under the frame and whether it is a warning.
    pub note: String,
    pub warning: bool,
    /// The overflows, in paint order (a control past both edges is listed twice).
    pub overflows: Vec<Overflow>,
}

/// State of the sizes tab: presets with visibility and custom sizes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SizePreview {
    pub sizes: Vec<(SizePreset, bool)>,
    pub custom: Vec<(SizePreset, bool)>,
}

impl SizePreview {
    pub fn new(platform: SizePlatform) -> Self {
        let presets = presets(platform);
        let shown = presets_shown(platform);
        let sizes = presets.into_iter().zip(shown).collect();
        Self {
            sizes,
            custom: Vec::new(),
        }
    }

    pub fn toggle(&mut self, i: usize) {
        if let Some((_, shown)) = self.sizes.get_mut(i) {
            *shown = !*shown;
        } else if let Some((_, shown)) = self.custom.get_mut(i - self.sizes.len()) {
            *shown = !*shown;
        }
    }

    pub fn add_custom(&mut self, name: &str, w: u32, h: u32) -> Result<(), String> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if !(120..=7680).contains(&w) || !(120..=7680).contains(&h) {
            return Err("Width and height must be between 120 and 7680 px".to_string());
        }
        if self.sizes.iter().any(|(p, _)| p.name == trimmed)
            || self.custom.iter().any(|(p, _)| p.name == trimmed)
        {
            return Err(format!("A size named '{trimmed}' already exists"));
        }
        self.custom.push((
            SizePreset {
                name: trimmed.to_string(),
                width: w,
                height: h,
            },
            true,
        ));
        Ok(())
    }

    pub fn remove_custom(&mut self, i: usize) {
        if i < self.custom.len() {
            self.custom.remove(i);
        }
    }

    pub fn shown(&self) -> Vec<&SizePreset> {
        self.sizes
            .iter()
            .filter(|(_, shown)| *shown)
            .map(|(preset, _)| preset)
            .chain(
                self.custom
                    .iter()
                    .filter(|(_, shown)| *shown)
                    .map(|(preset, _)| preset),
            )
            .collect()
    }

    /// Replaces the custom sizes with the ones a project keeps (`name, width, height`).
    pub fn set_custom(&mut self, sizes: Vec<(String, u32, u32)>) {
        self.custom = sizes
            .into_iter()
            .map(|(name, width, height)| {
                (
                    SizePreset {
                        name,
                        width,
                        height,
                    },
                    true,
                )
            })
            .collect();
    }

    /// The custom sizes as the project keeps them (`name, width, height`).
    pub fn custom_tuples(&self) -> Vec<(String, u32, u32)> {
        self.custom
            .iter()
            .map(|(p, _)| (p.name.clone(), p.width, p.height))
            .collect()
    }

    /// Every shown size laid out, in order: one plan per frame.
    pub fn plans(&self, doc: &Document, reg: &Registry) -> Vec<FramePlan> {
        self.shown()
            .into_iter()
            .map(|preset| {
                let framed = frame(doc, preset.width as f32, preset.height as f32);
                let (note, warning) = frame_note(&framed, reg);
                let overflows = overflows(&framed, reg);
                FramePlan {
                    preset: preset.clone(),
                    framed,
                    note,
                    warning,
                    overflows,
                }
            })
            .collect()
    }

    /// What the `view.sizes` tool reports: every shown size laid out, its note and overflows.
    pub fn report(&self, doc: &Document, reg: &Registry) -> serde_json::Value {
        let sizes: Vec<serde_json::Value> = self
            .plans(doc, reg)
            .into_iter()
            .map(|plan| {
                let overflows: Vec<serde_json::Value> = plan
                    .overflows
                    .iter()
                    .map(|o| {
                        let edge = match o.edge {
                            Edge::Right => "right",
                            Edge::Bottom => "bottom",
                        };
                        serde_json::json!({ "control": o.node, "edge": edge, "px": o.px })
                    })
                    .collect();
                serde_json::json!({
                    "name": plan.preset.name,
                    "width": plan.preset.width,
                    "height": plan.preset.height,
                    "note": plan.note,
                    "warning": plan.warning,
                    "overflows": overflows,
                })
            })
            .collect();
        serde_json::json!({ "sizes": sizes })
    }
}

#[cfg(test)]
#[path = "test/size_preview/test.rs"]
mod tests;
