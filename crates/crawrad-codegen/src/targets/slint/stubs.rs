//! Generation and maintenance of the user's file (`src/<form>.rs`) for Slint.

use crate::{Adicao, Ancora, Arquivo, Dono, Erro, identificador};
use crawrad_ir::{Document, Node};
use std::path::PathBuf;

pub const ANCHOR_MARK: &str = "\n    // New event handlers go here.\n";
const STUB_BODY: &str = "        \n";

fn event_comment(component: &str, event: &str) -> String {
    format!("/// `{component}.{event}`")
}

fn stub_block(
    reg: &crawrad_registry::Registry,
    class: &str,
    component: &str,
    event: &str,
    fn_name: &str,
    base_handler: Option<&str>,
) -> String {
    let params = crate::events::method_params(reg, class, event);
    let body = match base_handler {
        Some(base_fn) => {
            let args = crate::events::call_args_from_params(&params);
            format!("        self.base.{base_fn}({args});\n")
        }
        None => STUB_BODY.to_string(),
    };
    format!(
        "\n    {}\n    pub fn {fn_name}({params}) {{\n{body}    }}\n",
        event_comment(component, event)
    )
}

pub fn stubs(
    reg: &crawrad_registry::Registry,
    form: &str,
    type_name: &str,
    root: &Node,
    emitted: &[&Node],
) -> Result<Arquivo, Erro> {
    stubs_with_base(reg, form, type_name, root, emitted, None)
}

pub(crate) fn stubs_with_base(
    reg: &crawrad_registry::Registry,
    form: &str,
    type_name: &str,
    root: &Node,
    emitted: &[&Node],
    inheritance: Option<(&Document, &Document)>,
) -> Result<Arquivo, Erro> {
    let mut body = String::new();
    let mut additions = Vec::new();

    // Iterate through all explicitly wired event handlers on the form and emitted controls.
    for h in crate::events::wired_handlers(root, emitted)? {
        let block = stub_block(reg, &h.node.class, h.node.name(), h.event, &h.name, None);
        body.push_str(&block);
        additions.push(Adicao {
            nome_da_fn: h.name.clone(),
            texto: format!("\nimpl {type_name} {{{block}}}\n"),
            ancora: Some(Ancora {
                marca: ANCHOR_MARK.to_string(),
                texto: block.clone(),
            }),
        });
    }

    // Process visual inheritance overrides if an ancestor document is provided.
    if let Some((base_doc, doc)) = inheritance {
        for (id, derived_node) in &doc.nodes {
            // Only process nodes that originate in the base document.
            if !doc.is_inherited(id, base_doc) {
                continue;
            }
            let base_node = base_doc.node(id);
            // Effective class resolved from ancestor node if possible.
            let class = base_node
                .map(|b| b.class.as_str())
                .filter(|c| !c.is_empty())
                .unwrap_or(derived_node.class.as_str());

            for (event, target) in &derived_node.events {
                if target.trim().is_empty() {
                    continue;
                }
                // Target handler function name in user's struct.
                let fn_name = identificador(target)?;
                let base_h = base_node.and_then(|b| {
                    b.events
                        .iter()
                        .find(|(k, _)| k.eq_ignore_ascii_case(event))
                        .map(|(_, v)| v.as_str())
                });
                let block = stub_block(reg, class, derived_node.name(), event, &fn_name, base_h);
                body.push_str(&block);
                additions.push(Adicao {
                    nome_da_fn: fn_name.clone(),
                    texto: format!("\nimpl {type_name} {{{block}}}\n"),
                    ancora: Some(Ancora {
                        marca: ANCHOR_MARK.to_string(),
                        texto: block.clone(),
                    }),
                });
            }
        }
    }

    let default_body = format!(
        "// User event handler implementations for Slint form {type_name}.\n\
         \n\
         #[derive(Default)]\n\
         pub struct {type_name};\n\
         \n\
         impl {type_name} {{\n\
         {ANCHOR_MARK}{body}}}\n"
    );

    Ok(Arquivo {
        caminho: PathBuf::from(format!("src/{form}.rs")),
        conteudo: default_body,
        dono: Dono::Usuario,
        adicoes: additions,
    })
}
