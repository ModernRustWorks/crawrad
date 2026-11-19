//! # Static AST parser for `impl Paint for Component`
//!
//! When the CrawRAD CLI runs `crawrad manifest`, it parses Rust source files
//! directly via `syn` without compiling the project. This module analyzes
//! `impl Paint for <Component>` blocks and extracts `DrawSpec` drawing steps
//! directly from the AST method calls on `canvas`.

use syn::{Expr, ExprLit, ExprMethodCall, ExprPath, ImplItem, ItemImpl, Lit, Stmt, Type};

use crate::render::json_string;
use crate::spec::DrawSpec;

/// Parses an `impl Paint for <Component>` block and extracts drawing specs.
///
/// # Examples
///
/// ```rust,ignore
/// // private module canvas_ast: not exposed in crate root
/// use syn::parse_quote;
///
/// let item: syn::ItemImpl = parse_quote!(impl Paint for MyButton {});
/// let res = parse_paint_impl(&item);
/// ```
///
/// # Errors
///
/// Returns a [`syn::Error`] if AST traversal or argument parsing encounters invalid constructs.
pub fn parse_paint_impl(item_impl: &ItemImpl) -> syn::Result<Option<(String, Vec<DrawSpec>)>> {
    // 1. Verify that this is an `impl Paint for ...` block
    let Some((neg, path, _)) = &item_impl.trait_ else {
        return Ok(None);
    };
    if neg.is_some() {
        return Ok(None);
    }
    let is_paint = path.segments.last().is_some_and(|seg| seg.ident == "Paint");
    if !is_paint {
        return Ok(None);
    }

    // 2. Identify target struct name from `self_ty`
    let Type::Path(type_path) = &*item_impl.self_ty else {
        return Ok(None);
    };
    let Some(class_ident) = type_path.path.segments.last().map(|s| &s.ident) else {
        return Ok(None);
    };
    let class_name = class_ident.to_string();

    // 3. Find `fn paint(&self, canvas: &mut Canvas)`
    let mut paint_fn = None;
    for item in &item_impl.items {
        if let ImplItem::Fn(method) = item
            && method.sig.ident == "paint"
        {
            paint_fn = Some(method);
            break;
        }
    }
    let Some(method) = paint_fn else {
        return Ok(None);
    };

    // 4. Simulate canvas commands sequentially
    let mut sim = CanvasSimulator::new();
    for stmt in &method.block.stmts {
        sim.process_stmt(stmt);
    }

    Ok(Some((class_name, sim.draws)))
}

struct CanvasSimulator {
    active_fill: Option<String>,
    active_stroke: Option<String>,
    active_line_width: f32,
    draws: Vec<DrawSpec>,
}

impl CanvasSimulator {
    fn new() -> Self {
        Self {
            active_fill: None,
            active_stroke: None,
            active_line_width: 1.0,
            draws: Vec::new(),
        }
    }

    fn process_stmt(&mut self, stmt: &Stmt) {
        if let Stmt::Expr(expr, _) = stmt {
            self.process_expr(expr);
        }
    }

    fn process_expr(&mut self, expr: &Expr) {
        if let Expr::MethodCall(mc) = expr {
            self.process_method_call(mc);
        }
    }

    fn process_method_call(&mut self, mc: &ExprMethodCall) {
        let method = mc.method.to_string();
        let args: Vec<&Expr> = mc.args.iter().collect();

        match method.as_str() {
            "set_fill_style" => {
                if let Some(arg) = args.first()
                    && let Some(role) = extract_theme_role(arg)
                {
                    self.active_fill = Some(role);
                }
            }
            "set_stroke_style" => {
                if let Some(arg) = args.first()
                    && let Some(role) = extract_theme_role(arg)
                {
                    self.active_stroke = Some(role);
                }
            }
            "set_line_width" => {
                if let Some(arg) = args.first()
                    && let Some(w) = extract_float(arg)
                {
                    self.active_line_width = w;
                }
            }
            "fill_round_rect" => {
                // (x, y, w, h, radius)
                let radius = args.get(4).and_then(|e| extract_float(e)).unwrap_or(0.0);
                let fill = self
                    .active_fill
                    .clone()
                    .unwrap_or_else(|| "surface".to_string());
                let mut keys = vec![
                    ("verb".into(), json_string("box")),
                    ("fill".into(), json_string(&fill)),
                ];
                if let Some(stroke) = &self.active_stroke {
                    keys.push(("border".into(), json_string(stroke)));
                }
                keys.push(("radius".into(), format!("{radius:.1}")));
                self.draws.push(DrawSpec { keys, area: None });
            }
            "fill_rect" => {
                // (x, y, w, h)
                let fill = self
                    .active_fill
                    .clone()
                    .unwrap_or_else(|| "surface".to_string());
                let mut keys = vec![
                    ("verb".into(), json_string("box")),
                    ("fill".into(), json_string(&fill)),
                ];
                if let Some(stroke) = &self.active_stroke {
                    keys.push(("border".into(), json_string(stroke)));
                }
                self.draws.push(DrawSpec { keys, area: None });
            }
            "fill_fraction_rect" => {
                // (axis, prop_val, prop_min, prop_max)
                let axis = args
                    .first()
                    .and_then(|e| extract_axis(e))
                    .unwrap_or_else(|| "x".to_string());
                let prop_val = args
                    .get(1)
                    .and_then(|e| extract_string(e))
                    .unwrap_or_default();
                let prop_min = args
                    .get(2)
                    .and_then(|e| extract_string(e))
                    .unwrap_or_default();
                let prop_max = args
                    .get(3)
                    .and_then(|e| extract_string(e))
                    .unwrap_or_default();
                let fill = self
                    .active_fill
                    .clone()
                    .unwrap_or_else(|| "accent".to_string());

                let keys = vec![
                    ("verb".into(), json_string("box")),
                    ("fill".into(), json_string(&fill)),
                ];
                let area = Some(format!(
                    r#"{{ "fraction_of": "{prop_val}", "axis": "{axis}", "min": "{prop_min}", "max": "{prop_max}" }}"#
                ));
                self.draws.push(DrawSpec { keys, area });
            }
            "stroke_round_rect" | "stroke_rect" => {
                let color = self
                    .active_stroke
                    .clone()
                    .unwrap_or_else(|| "accent".to_string());
                let thickness = self.active_line_width;
                let keys = vec![
                    ("verb".into(), json_string("frame")),
                    ("color".into(), json_string(&color)),
                    ("thickness".into(), format!("{thickness:.1}")),
                ];
                self.draws.push(DrawSpec { keys, area: None });
            }
            "fill_text_prop" => {
                // (prop_name, x, y, w, h, align)
                let prop = args
                    .first()
                    .and_then(|e| extract_string(e))
                    .unwrap_or_default();
                let is_centered = args.get(5).is_some_and(|e| is_align_center(e));
                let color = self
                    .active_fill
                    .clone()
                    .unwrap_or_else(|| "text".to_string());

                let keys = vec![
                    ("verb".into(), json_string("text")),
                    ("prop".into(), json_string(&prop)),
                    ("color".into(), json_string(&color)),
                    (
                        "centered".into(),
                        if is_centered {
                            "true".to_string()
                        } else {
                            "false".to_string()
                        },
                    ),
                ];
                self.draws.push(DrawSpec { keys, area: None });
            }
            "fill_text_in_rect" => {
                // (text, x, y, w, h, align)
                let text = args
                    .first()
                    .and_then(|e| extract_string(e))
                    .unwrap_or_default();
                let is_centered = args.get(5).is_some_and(|e| is_align_center(e));
                let color = self
                    .active_fill
                    .clone()
                    .unwrap_or_else(|| "text".to_string());

                let keys = vec![
                    ("verb".into(), json_string("text")),
                    ("text".into(), json_string(&text)),
                    ("color".into(), json_string(&color)),
                    (
                        "centered".into(),
                        if is_centered {
                            "true".to_string()
                        } else {
                            "false".to_string()
                        },
                    ),
                ];
                self.draws.push(DrawSpec { keys, area: None });
            }
            "draw_check" => {
                let color = self
                    .active_stroke
                    .clone()
                    .unwrap_or_else(|| "accent".to_string());
                let keys = vec![
                    ("verb".into(), json_string("check")),
                    ("color".into(), json_string(&color)),
                ];
                self.draws.push(DrawSpec { keys, area: None });
            }
            _ => {}
        }
    }
}

fn extract_theme_role(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Path(ExprPath { path, .. }) => {
            let last = path.segments.last()?.ident.to_string();
            match last.as_str() {
                "Surface" => Some("surface".to_string()),
                "Border" => Some("border".to_string()),
                "Accent" => Some("accent".to_string()),
                "Text" => Some("text".to_string()),
                "TextDim" => Some("text_dim".to_string()),
                _ => None,
            }
        }
        Expr::Call(call) => call.args.first().and_then(extract_theme_role),
        Expr::Lit(ExprLit {
            lit: Lit::Str(s), ..
        }) => Some(s.value()),
        _ => None,
    }
}

fn extract_axis(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Path(ExprPath { path, .. }) => {
            let last = path.segments.last()?.ident.to_string();
            match last.as_str() {
                "X" => Some("x".to_string()),
                "Y" => Some("y".to_string()),
                _ => None,
            }
        }
        _ => None,
    }
}

fn is_align_center(expr: &Expr) -> bool {
    match expr {
        Expr::Path(ExprPath { path, .. }) => path
            .segments
            .last()
            .is_some_and(|seg| seg.ident == "Center"),
        _ => false,
    }
}

fn extract_string(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Str(s), ..
        }) => Some(s.value()),
        _ => None,
    }
}

fn extract_float(expr: &Expr) -> Option<f32> {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Float(f), ..
        }) => f.base10_parse::<f32>().ok(),
        Expr::Lit(ExprLit {
            lit: Lit::Int(i), ..
        }) => i.base10_parse::<f32>().ok(),
        _ => None,
    }
}
