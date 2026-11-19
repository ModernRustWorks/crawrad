//! Generation and maintenance of the user's file (`src/<form>.rs`) for Makepad.

use crate::{Adicao, Ancora, Arquivo, Dono, Erro, identificador};
use crawrad_ir::{Document, Node};
use crawrad_note::note;
use std::path::PathBuf;

pub const MARCA: &str = "\n    // New event handlers go here.\n";
const CORPO_DO_STUB: &str = "        \n";

fn marca_de_evento(componente: &str, evento: &str) -> String {
    format!("/// `{componente}.{evento}`")
}

fn bloco_do_stub(
    reg: &crawrad_registry::Registry,
    classe: &str,
    componente: &str,
    evento: &str,
    f: &str,
    base_handler: Option<&str>,
) -> String {
    // The parameters are the catalogue's, never this file's. A `&mut self`
    // carved in here compiles — nothing calls the stub — and only shows up
    // when the user writes the body and the parameter is not there.
    let params = crate::events::method_params(reg, classe, evento);
    let body = match base_handler {
        Some(base_f) => {
            let args = crate::events::call_args_from_params(&params);
            format!("        self.base.{base_f}({args});\n")
        }
        None => CORPO_DO_STUB.to_string(),
    };
    format!(
        "\n    {}\n    pub fn {f}({params}) {{\n{body}    }}\n",
        marca_de_evento(componente, evento)
    )
}

pub fn stubs(
    reg: &crawrad_registry::Registry,
    form: &str,
    tipo: &str,
    raiz: &Node,
    emitidos: &[&Node],
) -> Result<Arquivo, Erro> {
    stubs_with_base(reg, form, tipo, raiz, emitidos, None)
}

#[note("targets-makepad-stubs-stubs-with-base")]
pub(crate) fn stubs_with_base(
    reg: &crawrad_registry::Registry,
    form: &str,
    tipo: &str,
    raiz: &Node,
    emitidos: &[&Node],
    inheritance: Option<(&Document, &Document)>,
) -> Result<Arquivo, Erro> {
    let mut corpo = String::new();
    let mut adicoes = Vec::new();

    // Same reason as the egui twin: which handlers this document gives the
    // user's file is a question for `crate::events`, because the designer
    // answers it too — it is what decides which payload types it emits.
    for h in crate::events::wired_handlers(raiz, emitidos)? {
        let block = bloco_do_stub(reg, &h.node.class, h.node.name(), h.event, &h.name, None);
        corpo.push_str(&block);
        adicoes.push(Adicao {
            nome_da_fn: h.name.clone(),
            texto: format!("\nimpl {tipo} {{{block}}}\n"),
            ancora: Some(Ancora {
                marca: MARCA.to_string(),
                texto: block.clone(),
            }),
        });
    }

    if let Some((base_doc, doc)) = inheritance {
        for (id, derived_node) in &doc.nodes {
            if !doc.is_inherited(id, base_doc) {
                continue;
            }
            let base_node = base_doc.node(id);
            let class = base_node
                .map(|b| b.class.as_str())
                .filter(|c| !c.is_empty())
                .unwrap_or(derived_node.class.as_str());
            for (evento, alvo) in &derived_node.events {
                if alvo.trim().is_empty() {
                    continue;
                }
                let f = identificador(alvo)?;
                let base_h = base_node.and_then(|b| {
                    b.events
                        .iter()
                        .find(|(k, _)| k.eq_ignore_ascii_case(evento))
                        .map(|(_, v)| v.as_str())
                });
                let block = bloco_do_stub(reg, class, derived_node.name(), evento, &f, base_h);
                corpo.push_str(&block);
                adicoes.push(Adicao {
                    nome_da_fn: f.clone(),
                    texto: format!("\nimpl {tipo} {{{block}}}\n"),
                    ancora: Some(Ancora {
                        marca: MARCA.to_string(),
                        texto: block.clone(),
                    }),
                });
            }
        }
    }

    // A single glob, for the same reason detailed in the egui twin
    // (`targets/egui/stubs.rs`): the `impl` consumes the form type in every
    // file, so the `use` is never unused and no
    // `#[allow(unused_imports)]` is needed — and it brings along the payload
    // types that a handler signature names without a path.
    Ok(Arquivo {
        caminho: PathBuf::from(format!("src/{form}.rs")),
        conteudo: format!(
            "//! **This file is yours.** CrawRAD does not regenerate it, and it\n\
             //! never removes code you wrote.\n\
             //!\n\
             //! It does write to the file, though — better you know than be\n\
             //! surprised:\n\
             //!\n\
             //! - wiring a new event in the Object Inspector inserts the\n\
             //!   handler at the marker below, leaving the rest alone;\n\
             //! - generating takes empty event handlers back out, and\n\
             //!   saving generates — so saving takes them out too. It does\n\
             //!   not matter whether the event is still wired: the wiring\n\
             //!   is dropped in the same step, so the form still compiles.\n\
             //!   This only ever reaches a handler that is still the empty\n\
             //!   one CrawRAD wrote, under the very name CrawRAD gave it.\n\
             //!   Type one character into the body, even a comment; rename\n\
             //!   the function; or write a line of your own right above it,\n\
             //!   such as a doc comment or an attribute — any of these and\n\
             //!   it is yours: it stays for good.\n\
             //!   Methods you wrote yourself are never looked at, whatever\n\
             //!   you name them and whatever you write above them;\n\
             //! - renaming the form moves this file along with it and\n\
             //!   replaces the form's type name throughout — in your code\n\
             //!   AND in your comments — because otherwise it would not\n\
             //!   compile.\n\
             \n\
             // One glob, on purpose: it brings in the form's type, which\n\
             // the `impl` below is written on, and the event payload types\n\
             // (`KeyEvent`, `MouseEvent`) that a handler signature names\n\
             // with no path.\n\
             use crate::{form}_designer::*;\n\
             \n\
             impl {tipo} {{{corpo}{MARCA}}}\n"
        ),
        dono: Dono::Usuario,
        adicoes,
    })
}
