//! Where the form for the Makepad shell comes from, and what the interface says about
//! it.
//!
//! This module exists so that the shell does **not** need an `include_str!`.
//! Until this iteration, `ide_app.rs` embedded
//! the sample app's `form_main.crawform` at COMPILE time and the status
//! bar announced, in fixed text in the DSL, a path from
//! `C:/Users/.../Downloads` that was never opened. Two fictions for the price of
//! one: the file from disk was not loaded, and what the bar stated did not exist.
//!
//! Everything here is **pure**, with one announced exception ([`abrir`], which reads
//! disk). It is this separation that allows search order, failure messages,
//! and tree lines to be measured by tests without opening any window.

use std::path::{Path, PathBuf};

use crawrad_doc::Doc;
use crawrad_ir::{Document, NodeId};
use crawrad_note::note;

#[note("document-var-form")]
pub const VAR_FORM: &str = "CRAWRAD_FORM";

#[note("document-candidatos")]
pub const CANDIDATOS: &[&str] = &[
    "examples/calculator/calculator/src/form_main.crawform",
    "examples/contacts/contacts/src/form_main.crawform",
    "examples/contacts-edit/contacts_edit/src/form_main.crawform",
];

#[note("document-aberto")]
#[derive(Debug, Clone, PartialEq)]
pub struct Aberto {
    pub doc: Document,
    pub caminho: PathBuf,
}

#[note("document-raiz-do-repositorio")]
fn raiz_do_repositorio() -> Option<PathBuf> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
}

#[note("document-candidatos-2")]
pub fn candidatos(arg: Option<String>, var: Option<String>, bases: &[PathBuf]) -> Vec<PathBuf> {
    let pedidos: Vec<String> = match arg.or(var) {
        Some(p) => vec![p],
        None => CANDIDATOS.iter().map(|s| (*s).to_string()).collect(),
    };
    let mut fora = Vec::new();
    for pedido in pedidos {
        let p = PathBuf::from(&pedido);
        if p.is_absolute() {
            fora.push(p);
            continue;
        }
        for base in bases {
            let completo = base.join(&p);
            if !fora.contains(&completo) {
                fora.push(completo);
            }
        }
    }
    fora
}

#[note("document-carregar")]
pub fn carregar<L>(lista: &[PathBuf], ler: L) -> Result<Aberto, String>
where
    L: Fn(&Path) -> Result<String, String>,
{
    let mut tentados = Vec::new();
    for caminho in lista {
        match ler(caminho) {
            Ok(texto) => match Document::from_json(&texto) {
                Ok(doc) => {
                    return Ok(Aberto {
                        doc,
                        caminho: caminho.clone(),
                    });
                }
                Err(e) => tentados.push(format!("{}: {e}", caminho.display())),
            },
            Err(e) => tentados.push(format!("{}: {e}", caminho.display())),
        }
    }
    if tentados.is_empty() {
        return Err("no form to open: the candidate list is empty".to_string());
    }
    Err(format!("no form opened — tried {}", tentados.join(" | ")))
}

#[note("document-start-list")]
pub fn start_list(
    arg: Option<String>,
    var: Option<String>,
    last: Option<PathBuf>,
    bases: &[PathBuf],
) -> Vec<PathBuf> {
    let named = arg.is_some() || var.is_some();
    let mut list: Vec<PathBuf> = last.filter(|_| !named).into_iter().collect();
    if !named {
        // Nothing named: the last project, or the Welcome page — never the
        // repository's sample forms, which only exist on a developer's disk.
        return list;
    }
    for p in candidatos(arg, var, bases) {
        if !list.contains(&p) {
            list.push(p);
        }
    }
    list
}

#[note("document-abrir")]
pub fn abrir(last: Option<PathBuf>) -> Result<Option<Aberto>, String> {
    let mut bases = Vec::new();
    if let Ok(cwd) = std::env::current_dir() {
        bases.push(cwd);
    }
    if let Some(raiz) = raiz_do_repositorio()
        && !bases.contains(&raiz)
    {
        bases.push(raiz);
    }
    if bases.is_empty() {
        bases.push(PathBuf::from("."));
    }
    let arg = std::env::args()
        .nth(1)
        .filter(|a| !a.starts_with("--") && !Path::new(a).is_dir());
    let var = std::env::var(VAR_FORM).ok().filter(|v| !v.is_empty());
    let list = start_list(arg, var, last, &bases);
    if list.is_empty() {
        return Ok(None);
    }
    carregar(&list, |p| {
        std::fs::read_to_string(p).map_err(|e| e.to_string())
    })
    .map(Some)
}

#[note("document-file-label")]
pub fn file_label(aberto: Option<&Path>, falha: Option<&str>) -> String {
    match (aberto, falha) {
        // Nothing: the editor's breadcrumb and the window title already say
        // which file is open (the owner, 01/10/2026); the bar keeps the news.
        (Some(_), _) => String::new(),
        (None, Some(e)) => e.to_string(),
        (None, None) => "no form open".to_string(),
    }
}

#[note("document-counters")]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Counters {
    /// `no selection`, `Button1 selected` or `3 selected`.
    pub selection: String,
    /// How many nodes the document has, or `—`.
    pub nodes: String,
    /// The document revision, or `—`.
    pub revision: String,
}

#[note("document-counters-2")]
pub fn counters(doc: Option<&Doc>) -> Counters {
    let Some(d) = doc else {
        return Counters {
            selection: "no selection".to_string(),
            nodes: "—".to_string(),
            revision: "—".to_string(),
        };
    };
    Counters {
        selection: match d.selection() {
            [] => "no selection".to_string(),
            // The NAME, and not the `id`: the id is an address and does not appear in the interface
            // (ADR 0008). This is what the egui tree already does (`tree.rs:229-234`).
            [um] => format!("{} selected", nome_de(d.ir(), um)),
            muitos => format!("{} selected", muitos.len()),
        },
        nodes: d.ir().nodes.len().to_string(),
        revision: d.revision().to_string(),
    }
}

#[note("document-nome-de")]
pub fn nome_de(doc: &Document, id: &str) -> String {
    doc.node(id)
        .map(|n| n.name().to_string())
        .unwrap_or_else(|| id.to_string())
}

#[note("document-separador")]
pub const SEPARADOR: &str = "  ·  ";

#[note("document-linha")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Linha {
    pub id: NodeId,
    /// Zero for root; incremented by one on each descent.
    pub nivel: usize,
    /// The node's name (ADR 0008: name in interface, id only underneath).
    pub nome: String,
    /// The catalog class, or empty when the node does not declare one.
    pub classe: String,
}

impl Linha {
    #[note("document-linha-sufixo-de-classe")]
    pub fn sufixo_de_classe(&self) -> String {
        if self.classe.is_empty() {
            return String::new();
        }
        format!("{SEPARADOR}{}", self.classe)
    }
}

#[note("document-linhas-da-arvore")]
pub fn linhas_da_arvore(doc: &Document) -> Vec<Linha> {
    let mut fora = Vec::new();
    let raiz = doc.root.clone();
    fora.push(linha(doc, &raiz, 0));
    descer(doc, &raiz, 1, &mut fora);
    fora
}

fn descer(doc: &Document, pai: &str, nivel: usize, fora: &mut Vec<Linha>) {
    // The same ceiling as `Document::descer` (`crawrad-ir/src/lib.rs:376`): a corrupted
    // document does not freeze the interface.
    if nivel > 64 {
        return;
    }
    let Some(n) = doc.node(pai) else { return };
    for id in n.children.clone() {
        fora.push(linha(doc, &id, nivel));
        descer(doc, &id, nivel + 1, fora);
    }
}

fn linha(doc: &Document, id: &str, nivel: usize) -> Linha {
    Linha {
        nome: nome_de(doc, id),
        classe: doc.node(id).map(|n| n.class.clone()).unwrap_or_default(),
        id: id.to_string(),
        nivel,
    }
}

/// The name of the first selected node, or of the root when nothing is selected;
/// a dash without a document.
pub fn active_node_name(doc: Option<&Doc>) -> String {
    let Some(d) = doc else {
        return "—".to_string();
    };
    let id = d
        .selection()
        .first()
        .cloned()
        .unwrap_or_else(|| d.ir().root.clone());
    nome_de(d.ir(), &id)
}

/// The form's name, the selected names and the node count, for the chat context.
pub fn designer_summary(doc: &Doc) -> (String, Vec<String>, usize) {
    let ir = doc.ir();
    let name = ir
        .nodes
        .get(&ir.root)
        .map(|n| n.name().to_string())
        .unwrap_or_else(|| "Form".to_string());
    let sel: Vec<String> = doc
        .selection()
        .iter()
        .filter_map(|id| ir.nodes.get(id).map(|n| n.name().to_string()))
        .collect();
    (name, sel, ir.nodes.len())
}

/// What the status bar shows, gathered by the shell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusBarInputs<'a> {
    pub errors: String,
    pub warnings: String,
    pub lsp_name: String,
    pub lsp_progress: String,
    pub target: String,
    pub project: String,
    pub message: String,
    pub counters: Counters,
    pub zoom_cell: String,
    pub framework: &'a str,
}

/// The status bar cells, left to right.
pub fn status_bar_cells(i: StatusBarInputs<'_>) -> Vec<String> {
    vec![
        i.errors,
        i.warnings,
        i.lsp_name,
        i.lsp_progress,
        i.target,
        i.project,
        i.message,
        i.counters.selection,
        format!("{} nodes", i.counters.nodes),
        format!("rev {}", i.counters.revision),
        i.zoom_cell,
        i.framework.to_string(),
    ]
}

#[cfg(test)]
#[path = "test/document/test.rs"]
mod document_test;
