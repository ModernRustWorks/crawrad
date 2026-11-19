use super::codegen::MakepadComponentCodegen;
use crate::identificador;
use crate::targets::makepad::layout;
use crate::targets::makepad::state;
use crawrad_ir::{Document, Node};

pub struct BadgeCodegen;

impl MakepadComponentCodegen for BadgeCodegen {
    fn class(&self) -> &'static str {
        "Badge"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("Badge").expect("Badge desc");
        let body = state::fields_of(desc)
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct Badge {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("Badge").expect("Badge desc");
        let body = state::inits_of(node, desc)
            .into_iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("Badge {{ {body} }}")
    }

    fn emit_sync(&self, doc: &Document, node: &Node, field: &str) -> String {
        let rect = doc
            .rect_in_form(&node.id)
            .unwrap_or(crawrad_ir::Rect::new(0.0, 0.0, 56.0, 20.0));
        let (x, y, w, h) = (rect.x, rect.y, rect.w, rect.h);

        let assoc_ref = match node.props.get("associate") {
            Some(crawrad_ir::Value::Ref(s)) | Some(crawrad_ir::Value::Str(s)) if !s.is_empty() => {
                Some(s.as_str())
            }
            _ => None,
        };
        let assoc_target = assoc_ref.and_then(|t| {
            doc.node(t)
                .or_else(|| doc.nodes.values().find(|nd| nd.name() == t))
        });

        let place_code = if let Some(target) = assoc_target {
            let assoc_field = identificador(target.name()).unwrap_or_default();
            let tr = doc
                .rect_in_form(&target.id)
                .unwrap_or(crawrad_ir::Rect::new(0.0, 0.0, 80.0, 24.0));
            let split = crate::targets::splitters::terms(doc, target, "self.form.");
            let target_id = &target.id;
            format!(
                "        let w = if badge_str.chars().count() == 1 {{ {h:.1} }} else {{ {w:.1} }};\n\
                 \x20       if !self.form.{field}.associate.is_empty() && (self.form.{field}.associate == \"{assoc_field}\" || self.form.{field}.associate == \"{target_id}\") {{\n\
                 \x20           let bx = {:.1}{} + {:.1}{} - (w * 0.5);\n\
                 \x20           let by = {:.1}{} - ({h:.1} * 0.5);\n\
                 \x20           crawrad_runtime_makepad::split::place(cx, &self.ui.widget(id!({field})), bx, by, w, {h:.1});\n\
                 \x20       }} else {{\n\
                 \x20           crawrad_runtime_makepad::split::place(cx, &self.ui.widget(id!({field})), {x:.1}, {y:.1}, w, {h:.1});\n\
                 \x20       }}\n",
                tr.x, split[0], tr.w, split[2], tr.y, split[1]
            )
        } else {
            format!(
                "        let w = if badge_str.chars().count() == 1 {{ {h:.1} }} else {{ {w:.1} }};\n\
                 \x20       let bx = {x:.1};\n\
                 \x20       let by = {y:.1};\n\
                 \x20       if !self.form.{field}.associate.is_empty() {{\n\
                 \x20           crawrad_runtime_makepad::split::place(cx, &self.ui.widget(id!({field})), bx - (w * 0.5), by - ({h:.1} * 0.5), w, {h:.1});\n\
                 \x20       }} else {{\n\
                 \x20           crawrad_runtime_makepad::split::place(cx, &self.ui.widget(id!({field})), bx, by, w, {h:.1});\n\
                 \x20       }}\n"
            )
        };

        format!(
            "        // Badge caption, count, kind colors, and pill styling synced from state\n\
             \x20       if let Some(badge_str) = crawrad_runtime_makepad::badge::badge_text(&self.form.{field}.text, self.form.{field}.count, self.form.{field}.max_count) {{\n\
             \x20           self.ui.label(id!({field}.label)).set_text(cx, &badge_str);\n\
             \x20           self.ui.view(id!({field})).set_visible(cx, self.form.{field}.visible);\n\
             \x20           let effective_kind = if self.form.{field}.count > 0 && (self.form.{field}.kind == \"bkPrimary\" || self.form.{field}.kind.is_empty()) {{\n\
             \x20               \"bkDanger\"\n\
             \x20           }} else {{\n\
             \x20               &self.form.{field}.kind\n\
             \x20           }};\n\
             \x20           crawrad_runtime_makepad::badge::apply(\n\
             \x20               cx,\n\
             \x20               &self.ui.widget(id!({field})),\n\
             \x20               effective_kind,\n\
             \x20               self.form.{field}.outline,\n\
             \x20               self.form.{field}.enabled,\n\
             \x20           );\n\
             {place_code}\
             \x20       }} else {{\n\
             \x20           self.ui.view(id!({field})).set_visible(cx, false);\n\
             \x20       }}\n"
        )
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        let mut code = String::new();
        if let Some(h) = layout::handler(node, "on_click") {
            let h_name = identificador(&h)?;
            code.push_str(&format!(
                "        if self.ui.view(id!({field})).finger_up(actions).is_some() {{\n\
                 \x20           self.form.{h_name}();\n\
                 \x20       }}\n"
            ));
        }
        Ok(code)
    }

    fn emit_dsl(
        &self,
        doc: &Document,
        _reg: &crawrad_registry::Registry,
        node: &Node,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        let name = identificador(node.name())?;
        let caption_raw = layout::text(node);
        let count = match node.props.get("count") {
            Some(crawrad_ir::Value::Int(i)) => *i,
            _ => 0,
        };
        let max_count = match node.props.get("max_count") {
            Some(crawrad_ir::Value::Int(i)) => *i,
            _ => 99,
        };
        let text_opt = badge_text(&caption_raw, count, max_count);
        let text_str = text_opt.clone().unwrap_or_default();
        let actual_w = if text_str.chars().count() == 1 {
            h
        } else {
            w.max(h)
        };

        let assoc_ref = match node.props.get("associate") {
            Some(crawrad_ir::Value::Ref(s)) | Some(crawrad_ir::Value::Str(s)) if !s.is_empty() => {
                Some(s.as_str())
            }
            _ => None,
        };
        let assoc_target = assoc_ref.and_then(|t| {
            doc.node(t)
                .or_else(|| doc.nodes.values().find(|nd| nd.name() == t))
        });

        let (final_x, final_y) = if let Some(target) = assoc_target {
            let tr = doc
                .rect_in_form(&target.id)
                .unwrap_or(crawrad_ir::Rect::new(0.0, 0.0, 80.0, 24.0));
            (tr.x + tr.w - (actual_w * 0.5), tr.y - (h * 0.5))
        } else {
            (x, y)
        };

        let pos = layout::posicao(final_x, final_y);
        let caption = text_str.replace('\\', "\\\\").replace('"', "\\\"");
        let kind = match node.props.get("kind") {
            Some(crawrad_ir::Value::Enum(s)) | Some(crawrad_ir::Value::Str(s)) => s.as_str(),
            _ => "bkPrimary",
        };
        let effective_kind = if count > 0 && (kind == "bkPrimary" || kind.is_empty()) {
            "bkDanger"
        } else {
            kind
        };
        let outline = matches!(
            node.props.get("outline"),
            Some(crawrad_ir::Value::Bool(true))
        );

        // Hex colors for static DSL to avoid `vec4(..)` in live_design
        let (bg_hex, fg_hex) = match effective_kind {
            "bkDefault" => ("#383847", "#f2f2fa"),
            "bkPrimary" => ("#3a598c", "#f2f2fa"),
            "bkInfo" => ("#2e80b8", "#f2f2fa"),
            "bkWarning" => ("#d18c26", "#1a1a26"),
            "bkDanger" => ("#cc3838", "#f2f2fa"),
            _ => ("#3a598c", "#f2f2fa"),
        };

        // Makepad rounds with twice the radius (`Sdf2d::box`): a quarter of the
        // height makes the pill; half of it makes pointed ends.
        let radius = h * 0.25;

        // `border_size` is a shader uniform: run time cannot change it, so it
        // is always 1 and a filled badge's border takes the fill colour.
        let (fill_hex, border_hex, text_hex) = if outline {
            ("#00000000", bg_hex, bg_hex)
        } else {
            (bg_hex, bg_hex, fg_hex)
        };

        let is_visible = text_opt.is_some()
            && !matches!(
                node.props.get("visible"),
                Some(crawrad_ir::Value::Bool(false))
            );

        let mut dsl = format!(
            "            {name} = <RoundedView> {{\n\
             \x20               {pos}\n\
             \x20               width: {actual_w}, height: {h}\n\
             \x20               align: {{ x: 0.5, y: 0.5 }}\n\
             \x20               show_bg: true\n\
             \x20               visible: {is_visible}\n\
             \x20               draw_bg: {{\n\
             \x20                   border_radius: {radius:.1},\n\
             \x20                   color: {fill_hex},\n\
             \x20                   border_color: {border_hex},\n\
             \x20                   border_size: 1.0,\n\
             \x20               }}\n"
        );
        if layout::handler(node, "on_click").is_some() {
            dsl.push_str("                cursor: Default\n");
        }
        dsl.push_str(&format!(
            "                label = <Label> {{\n\
             \x20                   padding: 0, margin: 0\n\
             \x20                   draw_text: {{\n\
             \x20                       text_style: <THEME_FONT_REGULAR> {{ font_size: 9.0 }},\n\
             \x20                       color: {text_hex},\n\
             \x20                   }}\n\
             \x20                   text: \"{caption}\"\n\
             \x20               }}\n\
             \x20           }}\n"
        ));
        Ok(dsl)
    }
}

#[crawrad_note::note("targets-makepad-components-badge-badge-text")]
pub(crate) fn badge_text(caption: &str, count: i64, max_count: i64) -> Option<String> {
    if count > 0 {
        if count > max_count {
            Some(format!("{max_count}+"))
        } else {
            Some(count.to_string())
        }
    } else if caption.is_empty() {
        None
    } else {
        Some(caption.to_string())
    }
}

#[cfg(test)]
#[path = "../../../test/targets/makepad/badge/test.rs"]
mod test;
