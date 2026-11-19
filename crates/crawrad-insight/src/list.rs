//! The completion list's state: open or not, what it shows, what it waits
//! for. Pure — no window — so it lives here; the Makepad shell draws it.

use crate::item::ItemSugestao;
use crawrad_note::note;

impl EditorComplete {
    #[note("list-editorcomplete-to-json")]
    pub fn to_json(&self) -> serde_json::Value {
        let items: Vec<serde_json::Value> = self
            .sugestoes
            .iter()
            .map(|i| {
                serde_json::json!({
                    "label": i.rotulo,
                    "kind": format!("{:?}", i.kind),
                    "detail": i.detalhe,
                    "origin": i.origin,
                    "colour": i.kind.hex_colour(),
                })
            })
            .collect();
        serde_json::json!({
            "open": self.aberto,
            "waiting_for_rust_analyzer": self.esperando_lsp,
            "selected": self.selecionado,
            "top": self.top,
            "at": self.area,
            "word_at": self.word_at,
            "items": items,
            "doc": self.doc.text,
        })
    }
}

#[note("list-editorcomplete")]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EditorComplete {
    pub aberto: bool,
    pub prefixo: String,
    pub start_byte: usize,
    pub sugestoes: Vec<ItemSugestao>,
    pub selecionado: usize,
    pub ignorar_proximo_text_input: bool,
    pub pos_x: f64,
    pub pos_y: f64,
    pub linha_abertura: usize,
    /// The request this list waits on; an answer for another is dropped.
    pub ticket: u64,
    /// rust-analyzer was asked and has not answered yet.
    pub esperando_lsp: bool,
    /// The component's own fields, kept to head the list when the server's
    /// items arrive.
    pub rad: Vec<ItemSugestao>,
    /// Everything the list can show; `sugestoes` is this, filtered by what
    /// is typed (`crate::typing`).
    pub todos: Vec<ItemSugestao>,
    /// rust-analyzer said its list was partial: ask again as the user types.
    pub incompleto: bool,
    /// Why the next request is made (LSP `CompletionContext`), and whether
    /// the box waits for the server's items before it shows.
    pub trigger: crawrad_lsp::complete::Trigger,
    pub quiet: bool,
    /// A quiet request is out and the list may still open when it answers.
    pub quiet_pending: bool,
    /// The first row shown, and the selection it was last drawn for.
    pub top: usize,
    pub top_sel: usize,
    /// Where the box was last drawn, in window coordinates.
    pub area: [f64; 4],
    /// The start of the word the list completes, in window coordinates —
    /// what its left edge is aligned to.
    pub word_at: [f64; 2],
    /// The selected item's documentation beside the list.
    pub doc: crate::item::DocPanel,
}

impl EditorComplete {
    #[note("list-editorcomplete-coletar-itens")]
    pub fn coletar_itens(prefixo: &str, texto: &str, base: Option<&str>) -> Vec<ItemSugestao> {
        crate::item::local_items(prefixo, texto, base)
    }

    #[note("list-editorcomplete-coletar-sugestoes")]
    pub fn coletar_sugestoes(prefixo: &str, texto_documento: &str) -> Vec<String> {
        Self::coletar_itens(prefixo, texto_documento, None)
            .into_iter()
            .map(|it| it.rotulo)
            .collect()
    }
}
