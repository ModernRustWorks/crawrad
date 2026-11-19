//! **Color Scheme preview widget**: a small read-only code sample
//! painted with given colours (the Options Color Scheme preview).

use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;

    pub DrawPreviewRect = {{DrawPreviewRect}} {
        fn pixel(self) -> vec4 {
            return self.color;
        }
    }

    pub CrawradSchemePreviewBase = {{CrawradSchemePreview}} {}

    pub CrawradSchemePreview = <CrawradSchemePreviewBase> {
        width: Fill, height: 128.0,
        draw_text: {
            text_style: <THEME_FONT_CODE> { font_size: 10.0 }
        }
    }
}

#[crawrad_note::note("scheme-preview-previewrow")]
#[derive(Clone, Debug, PartialEq)]
pub struct PreviewRow {
    /// The line number string displayed in the gutter.
    pub number: String,
    /// RGBA color of the line number.
    pub number_color: [f32; 4],
    /// The line's background band color across the full row, if any.
    pub band: Option<[f32; 4]>,
    /// Syntax-highlighted code spans: `(text, color)`.
    pub spans: Vec<(String, [f32; 4])>,
}

impl PreviewRow {
    #[crawrad_note::note("scheme-preview-previewrow-new")]
    pub fn new(
        number: impl Into<String>,
        number_color: [f32; 4],
        band: Option<[f32; 4]>,
        spans: Vec<(String, [f32; 4])>,
    ) -> Self {
        Self {
            number: number.into(),
            number_color,
            band,
            spans,
        }
    }
}

#[crawrad_note::note("scheme-preview-drawpreviewrect")]
#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawPreviewRect {
    #[deref]
    pub draw_super: DrawQuad,
    /// Fill color for the rectangle.
    #[live]
    pub color: Vec4,
}

#[crawrad_note::note("scheme-preview-crawradschemepreview")]
#[derive(Live, LiveHook, Widget)]
pub struct CrawradSchemePreview {
    #[walk]
    walk: Walk,
    #[redraw]
    #[live]
    draw_rect: DrawPreviewRect,
    #[live]
    draw_text: DrawText,
    #[rust]
    area: Area,
    #[rust]
    background: [f32; 4],
    #[rust]
    rows: Vec<PreviewRow>,
}

const ROW_HEIGHT: f64 = 20.0;
const TOP_PADDING: f64 = 4.0;
const GUTTER_WIDTH: f64 = 30.0;
const GUTTER_GAP: f64 = 8.0;

impl Widget for CrawradSchemePreview {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        if rect.size.x <= 0.0 || rect.size.y <= 0.0 {
            return DrawStep::done();
        }

        // Background over the entire walk
        self.draw_rect.color = vec4(
            self.background[0],
            self.background[1],
            self.background[2],
            self.background[3],
        );
        self.draw_rect.draw_abs(cx, rect);

        let font_size = if self.draw_text.text_style.font_size > 0.0 {
            self.draw_text.text_style.font_size
        } else {
            10.0
        };
        let font_px = crawrad_component::text_line::font_px(font_size);
        let text_top = f64::from(crawrad_component::text_line::cap_centred_top(
            ROW_HEIGHT as f32,
            font_px,
        ));

        let char_w = {
            let measured = self
                .draw_text
                .layout(cx, 0.0, 0.0, None, false, Align::default(), "0")
                .size_in_lpxs
                .width as f64;
            if measured > 0.0 { measured } else { 7.96 }
        };

        for (i, row) in self.rows.iter().enumerate() {
            let row_y = rect.pos.y + TOP_PADDING + (i as f64) * ROW_HEIGHT;
            if row_y >= rect.pos.y + rect.size.y {
                break;
            }

            // Draw line band across full width if present
            if let Some(band_color) = row.band {
                self.draw_rect.color =
                    vec4(band_color[0], band_color[1], band_color[2], band_color[3]);
                let band_rect = Rect {
                    pos: dvec2(rect.pos.x, row_y),
                    size: dvec2(rect.size.x, ROW_HEIGHT),
                };
                self.draw_rect.draw_abs(cx, band_rect);
            }

            // Right-aligned number in gutter
            let num_w = {
                let measured = self
                    .draw_text
                    .layout(cx, 0.0, 0.0, None, false, Align::default(), &row.number)
                    .size_in_lpxs
                    .width as f64;
                if measured > 0.0 {
                    measured
                } else {
                    row.number.chars().count() as f64 * char_w
                }
            };
            let num_x = (rect.pos.x + GUTTER_WIDTH - num_w).max(rect.pos.x);
            self.draw_text.color = vec4(
                row.number_color[0],
                row.number_color[1],
                row.number_color[2],
                row.number_color[3],
            );
            self.draw_text
                .draw_abs(cx, dvec2(num_x, row_y + text_top), &row.number);

            // Spans drawn one after another
            let mut span_x = rect.pos.x + GUTTER_WIDTH + GUTTER_GAP;
            for (text, color) in &row.spans {
                if text.is_empty() {
                    continue;
                }
                self.draw_text.color = vec4(color[0], color[1], color[2], color[3]);
                self.draw_text
                    .draw_abs(cx, dvec2(span_x, row_y + text_top), text);
                let span_w = {
                    let measured = self
                        .draw_text
                        .layout(cx, 0.0, 0.0, None, false, Align::default(), text)
                        .size_in_lpxs
                        .width as f64;
                    if measured > 0.0 {
                        measured
                    } else {
                        text.chars().count() as f64 * char_w
                    }
                };
                span_x += span_w;
            }
        }

        DrawStep::done()
    }
}

impl CrawradSchemePreviewRef {
    #[crawrad_note::note("scheme-preview-crawradschemepreviewref-set")]
    pub fn set(&self, cx: &mut Cx, background: [f32; 4], rows: Vec<PreviewRow>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.background = background;
            inner.rows = rows;
            inner.redraw(cx);
        }
    }
}
