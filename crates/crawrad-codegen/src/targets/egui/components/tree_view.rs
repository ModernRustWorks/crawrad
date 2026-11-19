use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador, literal};

pub struct TreeViewCodegen;

impl EguiComponentCodegen for TreeViewCodegen {
    fn class(&self) -> &'static str {
        "TreeView"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_TREEVIEW)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emitir(ctx)
    }

    #[crawrad_note::note("targets-egui-components-tree-view-treeviewcodegen-rola")]
    fn rola(&self) -> bool {
        true
    }
}

pub fn emitir(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let nome = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("TreeView").expect("TreeView descriptor");

    // A node with children announces the change before it happens
    let mut announce = String::new();
    for (event, when) in [("on_expanding", "is_some"), ("on_collapsing", "is_none")] {
        if let Some(h) = handler(n, event) {
            announce.push_str(&format!(
                "                if at.{when}() {{\n                    self.{}();\n                }}\n",
                identificador(&h)?
            ));
        }
    }
    // `OnChanging` first: setting `allow_change` to false keeps the old row
    let take = match handler(n, "on_changing") {
        Some(h) => format!(
            "                    let mut allow_change = true;\n\
             \x20                   self.{}(&mut allow_change);\n\
             \x20                   if allow_change {{\n\
             \x20                       self.{nome}.selection = path;\n\
             \x20                   }}\n\
             \x20                   allow_change\n",
            identificador(&h)?
        ),
        None => format!(
            "                    self.{nome}.selection = path;\n\
             \x20                   true\n"
        ),
    };
    // `OnExpanded` / `OnCollapsed`: after the fold has happened
    let after = |event: &str| -> Result<String, Erro> {
        Ok(match handler(n, event) {
            Some(h) => format!("                        self.{}();\n", identificador(&h)?),
            None => String::new(),
        })
    };
    let expanded = after("on_expanded")?;
    let collapsed = after("on_collapsed")?;
    let mut depois = format!(
        "            let changed = match picked {{\n\
         \x20               Some(path) => {{\n\
         {take}\
         \x20               }}\n\
         \x20               None => false,\n\
         \x20           }};\n\
         \x20           if let Some(path) = toggle {{\n\
         \x20               let at = self.{nome}.collapsed.iter().position(|p| *p == path);\n\
         {announce}\
         \x20               match at {{\n\
         \x20                   Some(i) => {{\n\
         \x20                       self.{nome}.collapsed.remove(i);\n\
         {expanded}\
         \x20                   }}\n\
         \x20                   None => {{\n\
         \x20                       self.{nome}.collapsed.push(path);\n\
         {collapsed}\
         \x20                   }}\n\
         \x20               }}\n\
         \x20           }}\n"
    );
    depois.push_str(&match handler(n, "on_change") {
        Some(h) => format!(
            "            if changed {{\n                self.{}();\n            }}\n",
            identificador(&h)?
        ),
        None => "            let _ = changed;\n".to_string(),
    });
    let corpo = format!(
        "        if self.{nome}.visible {{\n\
         \x20           let is_enabled = self.{nome}.enabled;\n\
         \x20           // its own Color, or the theme's\n\
         \x20           let c = self.{nome}.color;\n\
         \x20           let fill = (!self.{nome}.parent_color && c.a > 0)\n\
         \x20               .then(|| egui::Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a));\n\
         \x20           let (picked, toggle) = ui\n\
         \x20               .add_enabled_ui(is_enabled, |ui| {{\n\
         \x20                   tree(ui, {}, r, &self.{nome}.items, &self.{nome}.selection, &self.{nome}.collapsed, fill)\n\
         \x20               }})\n\
         \x20               .inner;\n\
         {depois}\
         \x20       }}\n",
        literal(nome)
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {nome}: TreeView,\n"),
        padrao: format!(
            "            {nome}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo,
        tipo_usado: "TreeView",
        precisa_timer: false,
        precisa_item: true,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_TREEVIEW: StateText = StateText {
    head: r#"
/// A `TreeView` on the form. `selection` is the path to the chosen node:
/// `[0, 2]` is the third child of the first item. Empty when nothing is
/// selected.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
