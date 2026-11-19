use super::codegen::{AppState, MakepadComponentCodegen};
use crate::identificador;
use crate::targets::makepad::layout;
use crate::targets::makepad::state;
use crawrad_ir::{Document, Node};

pub struct DrawerCodegen;

impl MakepadComponentCodegen for DrawerCodegen {
    fn class(&self) -> &'static str {
        "Drawer"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("Drawer").expect("Drawer desc");
        let body = state::fields_of(desc)
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct Drawer {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("Drawer").expect("Drawer desc");
        let body = state::inits_of(node, desc)
            .into_iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("Drawer {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, field: &str) -> String {
        format!(
            "        // Drawer visibility, header, surface styling and shadow synced from state\n\
             \x20       let drawer_open = self.form.{field}.visible && self.form.{field}.opened;\n\
             \x20       self.ui.view(id!({field})).set_visible(cx, drawer_open);\n\
             \x20       self.ui.crawrad_scrim(id!({field}_scrim)).set_visible(cx, drawer_open && self.form.{field}.overlay);\n\
             \x20       self.ui.label(id!({field}.header.title)).set_text(cx, &self.form.{field}.text);\n\
             \x20       self.ui.view(id!({field}.header)).set_visible(cx, self.form.{field}.show_header);\n\
             \x20       crawrad_runtime_makepad::drawer::apply(\n\
             \x20           cx,\n\
             \x20           &self.ui.widget(id!({field})),\n\
             \x20           self.form.{field}.side.as_str(),\n\
             \x20       );\n"
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
        let on_close_call = if let Some(h) = layout::handler(node, "on_close") {
            let h_name = identificador(&h)?;
            format!("                    self.form.{h_name}();\n")
        } else {
            String::new()
        };

        // one line: the dispatch takes the `enabled` guard into it
        // (`actions::guarded`)
        code.push_str(&format!(
            "        if (self.form.{field}.overlay && self.ui.crawrad_scrim(id!({field}_scrim)).clicked(actions)) || (self.form.{field}.show_header && self.ui.button(id!({field}.header.close_btn)).clicked(actions)) {{\n\
             \x20           self.form.{field}.opened = false;\n\
             {on_close_call}\
             \x20       }}\n"
        ));
        Ok(code)
    }

    fn app_state(&self, _node: &Node, _field: &str) -> Result<Option<AppState>, crate::Erro> {
        Ok(Some(AppState {
            uses: "use crawrad_runtime_makepad::drawer::CrawradScrimWidgetRefExt;
"
            .to_string(),
            ..AppState::default()
        }))
    }

    fn emit_dsl(
        &self,
        doc: &Document,
        reg: &crawrad_registry::Registry,
        node: &Node,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        let pos = layout::posicao(x, y);
        let name = identificador(node.name())?;
        let caption = layout::text(node)
            .replace('\\', "\\\\")
            .replace('"', "\\\"");
        let side = match node.props.get("side") {
            Some(crawrad_ir::Value::Enum(s)) | Some(crawrad_ir::Value::Str(s)) => s.as_str(),
            _ => "dsLeft",
        };
        let is_right = side == "dsRight";
        let shadow_offset_x = if is_right { -4.0 } else { 4.0 };

        let header_h = 32.0;
        let row_h = header_h - 1.0;

        // One row (the title and the "×") over a divider. A right drawer puts
        // the "×" at the left, next to the form; a left one at the far right.
        let close_btn = "                       close_btn = <ButtonFlatter> {\n\
             \x20                           width: 24.0, height: 24.0\n\
             \x20                           padding: 0, margin: 0\n\
             \x20                           draw_text: {\n\
             \x20                               text_style: <THEME_FONT_REGULAR> { font_size: 11.0 },\n\
             \x20                               color: #a6adc7,\n\
             \x20                           }\n\
             \x20                           text: \"×\"\n\
             \x20                       }\n";
        let title = format!(
            "                       title = <Label> {{\n\
             \x20                           padding: 0, margin: 0\n\
             \x20                           draw_text: {{\n\
             \x20                               text_style: <THEME_FONT_BOLD> {{ font_size: 10.0 }},\n\
             \x20                               color: #f2f2fa,\n\
             \x20                           }}\n\
             \x20                           text: \"{caption}\"\n\
             \x20                       }}\n"
        );
        let row_items = if is_right {
            format!("{close_btn}{title}")
        } else {
            format!(
                "{title}\
                 \x20                      filler = <View> {{ width: Fill, height: 1.0 }}\n\
                 {close_btn}"
            )
        };
        let header_layout = format!(
            "                header = <View> {{\n\
             \x20                   width: Fill, height: {header_h:.1}\n\
             \x20                   flow: Down\n\
             \x20                   row = <View> {{\n\
             \x20                       width: Fill, height: {row_h:.1}\n\
             \x20                       flow: Right, align: {{y: 0.5}}\n\
             \x20                       padding: {{left: 6.0, right: 6.0}}\n\
             \x20                       spacing: 8.0\n\
             {row_items}\
             \x20                   }}\n\
             \x20                   divider = <View> {{\n\
             \x20                       width: Fill, height: 1.0\n\
             \x20                       show_bg: true\n\
             \x20                       draw_bg: {{ color: #6e729099 }}\n\
             \x20                   }}\n\
             \x20               }}\n"
        );

        let mut dsl = format!(
            "            {name}_scrim = <CrawradScrim> {{\n\
             \x20               width: Fill, height: Fill\n\
             \x20               visible: false\n\
             \x20           }}\n\
             \x20           {name} = <View> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h}\n\
             \x20               flow: Overlay\n\
             \x20               visible: false\n\
             \x20               shadow = <RoundedView> {{\n\
             \x20                   margin: {{left: {shadow_offset_x:.1}, top: 0.0}}\n\
             \x20                   width: Fill, height: Fill\n\
             \x20                   show_bg: true\n\
             \x20                   draw_bg: {{\n\
             \x20                       color: #00000033,\n\
             \x20                   }}\n\
             \x20               }}\n\
             \x20               surface = <RoundedView> {{\n\
             \x20                   width: Fill, height: Fill\n\
             \x20                   show_bg: true\n\
             \x20                   draw_bg: {{\n\
             \x20                       color: #2f2f45,\n\
             \x20                       border_color: #6e7290,\n\
             \x20                       border_size: 1.0,\n\
             \x20                   }}\n\
             \x20               }}\n\
             {header_layout}"
        );

        dsl.push_str(&super::emit_children(doc, reg, node, "    ")?);
        dsl.push_str("            }\n");
        Ok(dsl)
    }
}
