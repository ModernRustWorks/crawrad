//! One row of the editor's completion list — what it shows and what it
//! inserts. Pure; the shell draws it (`ide_app/editor/complete.rs`).

use crawrad_note::note;
#[note("item-itemkind")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemKind {
    /// Component property or field (e.g. edt_nome_, text, enabled).
    Field,
    /// Method or event handler (e.g. btn_ok_click, clear, show).
    Method,
    /// Data type or component (e.g. Button, String, Option).
    Type,
    /// Language keyword (e.g. let, fn, if, match).
    Keyword,
    /// Snippet or code template (e.g. arc, box, call).
    Snippet,
    /// Not an item: the row that says rust-analyzer is still answering.
    Pending,
}

#[note("item-hex")]
const fn hex(rgb: u32) -> [f32; 4] {
    let r = ((rgb >> 16) & 0xff) as f32 / 255.0;
    let g = ((rgb >> 8) & 0xff) as f32 / 255.0;
    let b = (rgb & 0xff) as f32 / 255.0;
    [r, g, b, 1.0]
}

impl ItemKind {
    #[note("item-itemkind-badge")]
    pub fn badge(&self) -> &'static str {
        match self {
            Self::Field => "\u{eb5f}",
            Self::Method => "\u{ea8c}",
            Self::Type => "\u{eb5b}",
            Self::Keyword => "\u{eb62}",
            Self::Snippet => "\u{eb66}",
            Self::Pending => "\u{eb19}",
        }
    }

    #[note("item-itemkind-rgb")]
    const fn rgb(&self) -> u32 {
        match self {
            Self::Field => 0x7cb46b,
            Self::Method => 0xc39ac9,
            Self::Type => 0x5fb3b3,
            Self::Keyword => 0xd9a441,
            Self::Snippet | Self::Pending => 0x98989b,
        }
    }

    #[note("item-itemkind-colour")]
    pub fn colour(&self) -> [f32; 4] {
        hex(self.rgb())
    }

    #[note("item-itemkind-hex-colour")]
    pub fn hex_colour(&self) -> String {
        format!("#{:06x}", self.rgb())
    }
}

#[note("item-itemsugestao")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemSugestao {
    pub rotulo: String,
    pub kind: ItemKind,
    pub detalhe: Option<String>,
    /// What qualifies the label, `(as Clone)`: LSP `labelDetails.detail`.
    pub origin: Option<String>,
    pub insert_text: Option<String>,
    /// The server's `data` for `completionItem/resolve`.
    pub resolve: Option<serde_json::Value>,
    /// The documentation, when the server sent it with the list.
    pub doc: Option<String>,
    /// `insert_text` is a snippet, to expand before it is written.
    pub snippet: bool,
    /// The server's `filterText`: what typing is matched against. With
    /// snippets on, rust-analyzer labels a call `len()` or `soma(…)` and
    /// filters it as `len` / `soma` (LSP 3.17 *CompletionItem*).
    pub filter: Option<String>,
    /// The item is `#[deprecated]`: its label is struck through.
    pub deprecated: bool,
}

#[note("item-rowcolours")]
pub struct RowColours {
    pub bg: [f32; 4],
    pub hover: [f32; 4],
    pub text: [f32; 4],
    /// The origin and the signature beside the label.
    pub soft: [f32; 4],
}

impl RowColours {
    pub fn of(selected: bool) -> Self {
        match selected {
            true => Self {
                bg: [0.04, 0.38, 0.72, 1.0],
                hover: [0.08, 0.44, 0.80, 1.0],
                text: [1.0, 1.0, 1.0, 1.0],
                soft: [1.0, 1.0, 1.0, 1.0],
            },
            false => Self {
                bg: [0.14, 0.14, 0.16, 1.0],
                hover: [0.20, 0.20, 0.24, 1.0],
                text: [0.85, 0.85, 0.88, 1.0],
                soft: hex(0x98989b),
            },
        }
    }
}

#[note("item-columns")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Columns {
    pub icon: &'static str,
    pub label: String,
    pub origin: String,
    pub signature: String,
    pub deprecated: bool,
}

#[note("item-pad")]
const PAD: f64 = 8.0;
const GAP: f64 = 8.0;

#[note("item-rowlayout")]
#[derive(Debug, Clone, PartialEq)]
pub struct RowLayout {
    pub signature: String,
    pub strike: (f64, f64),
}

impl Columns {
    #[note("item-columns-layout")]
    pub fn layout(&self, row_width: f64, mut width: impl FnMut(&str) -> f64) -> RowLayout {
        let (icon, label) = (width(self.icon), width(&self.label));
        let room = row_width - 2.0 * PAD - 3.0 * GAP - icon - label - width(&self.origin);
        let from = PAD + icon + GAP;
        RowLayout {
            signature: fit(&self.signature, room, width),
            strike: if self.deprecated {
                (from, from + label)
            } else {
                (0.0, 0.0)
            },
        }
    }
}

#[note("item-thumb")]
pub fn thumb(total: usize, rows: usize, top: usize, row_h: f64) -> (f64, f64) {
    if total <= rows {
        return (0.0, 0.0);
    }
    let track = row_h * rows as f64;
    let height = (track * rows as f64 / total as f64).max(12.0);
    (
        height,
        (track - height) * top as f64 / (total - rows) as f64,
    )
}

#[note("item-fit")]
pub fn fit(text: &str, room: f64, mut width: impl FnMut(&str) -> f64) -> String {
    if width(text) <= room {
        return text.to_string();
    }
    if width("…") > room {
        return String::new();
    }
    let mut best_prefix = "";
    for (idx, c) in text.char_indices() {
        let prefix = &text[..idx + c.len_utf8()];
        let candidate = format!("{prefix}…");
        if width(&candidate) <= room {
            best_prefix = prefix;
        } else {
            break;
        }
    }
    format!("{best_prefix}…")
}

impl ItemSugestao {
    #[note("item-itemsugestao-key")]
    pub fn key(&self) -> &str {
        self.filter.as_deref().unwrap_or(&self.rotulo)
    }

    #[note("item-itemsugestao-columns")]
    pub fn columns(&self) -> Columns {
        Columns {
            icon: self.kind.badge(),
            label: self.rotulo.clone(),
            origin: self.origin.clone().unwrap_or_default(),
            signature: self.detalhe.clone().unwrap_or_default(),
            deprecated: self.deprecated,
        }
    }
}

impl ItemSugestao {
    pub fn novo(rotulo: impl Into<String>, kind: ItemKind) -> Self {
        Self {
            rotulo: rotulo.into(),
            kind,
            detalhe: None,
            origin: None,
            insert_text: None,
            resolve: None,
            doc: None,
            snippet: false,
            filter: None,
            deprecated: false,
        }
    }

    pub fn com_detalhe(mut self, detalhe: impl Into<String>) -> Self {
        self.detalhe = Some(detalhe.into());
        self
    }

    pub fn com_insert_text(mut self, text: impl Into<String>) -> Self {
        self.insert_text = Some(text.into());
        self
    }
}

#[note("item-docpanel")]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DocPanel {
    tickets: u64,
    label: Option<String>,
    pending: Option<u64>,
    /// The documentation shown, when there is one.
    pub text: Option<String>,
}

#[note("item-docstep")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocStep {
    /// Draw the panel again: its text changed (or it went away).
    Redraw,
    /// Resolve this item (ticket, label, `data`); the panel is empty meanwhile.
    Ask(u64, String, serde_json::Value),
    /// The same item: nothing to do.
    Same,
}

impl DocPanel {
    #[note("item-docpanel-select")]
    pub fn select(&mut self, item: Option<&ItemSugestao>) -> DocStep {
        let label = item.map(|i| i.rotulo.clone());
        if label == self.label {
            return DocStep::Same;
        }
        (self.label, self.pending, self.text) = (label.clone(), None, None);
        match (item, label) {
            (Some(ItemSugestao { doc: Some(d), .. }), _) => self.text = Some(d.clone()),
            (
                Some(ItemSugestao {
                    resolve: Some(data),
                    ..
                }),
                Some(label),
            ) => {
                self.tickets += 1;
                self.pending = Some(self.tickets);
                return DocStep::Ask(self.tickets, label, data.clone());
            }
            _ => {}
        }
        DocStep::Redraw
    }

    pub fn pending(&self) -> Option<u64> {
        self.pending
    }

    #[note("item-docpanel-answered")]
    pub fn answered(&mut self, ticket: u64, doc: Option<String>) -> bool {
        let awaited = self.pending == Some(ticket);
        if awaited {
            (self.pending, self.text) = (None, doc);
        }
        awaited
    }
}

#[note("item-from-lsp")]
pub fn from_lsp(mut itens: Vec<crawrad_lsp::CompletionItem>) -> Vec<ItemSugestao> {
    use crawrad_lsp::CompletionItemKind as K;
    itens.sort_by(|a, b| a.sort_key().cmp(b.sort_key()));
    itens
        .into_iter()
        .map(|item| {
            let kind = match item.kind {
                Some(K::Field | K::Property) => ItemKind::Field,
                Some(K::Method | K::Function) => ItemKind::Method,
                Some(K::Class | K::Struct | K::Enum | K::Interface | K::TypeParameter) => {
                    ItemKind::Type
                }
                Some(K::Keyword) => ItemKind::Keyword,
                Some(K::Snippet) => ItemKind::Snippet,
                _ => ItemKind::Field,
            };
            let mut sug = ItemSugestao::novo(item.label.clone(), kind);
            sug.origin = item
                .label_details
                .as_ref()
                .and_then(|d| d.detail.as_deref())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string);
            sug.resolve = item.data.clone();
            sug.snippet = item.insert_text_format == Some(2);
            sug.filter = item.filter_text.clone();
            sug.deprecated = item.is_deprecated();
            sug.doc = item
                .documentation
                .as_ref()
                .map(|d| crawrad_lsp::hover::plain(d.as_str()));
            let sug = sug.com_insert_text(item.text_to_insert());
            match item.signature() {
                Some(detail) => sug.com_detalhe(detail),
                None => sug,
            }
        })
        .collect()
}

#[note("item-filtered")]
pub fn filtered(all: &[ItemSugestao], typed: &str) -> Vec<ItemSugestao> {
    let items: Vec<&ItemSugestao> = all.iter().filter(|i| i.kind != ItemKind::Pending).collect();
    let ranked: Vec<ItemSugestao> = crate::typing::rank(items.iter().map(|i| i.key()), typed)
        .into_iter()
        .map(|i| items[i].clone())
        .collect();
    if ranked.is_empty() {
        return all
            .iter()
            .filter(|i| i.kind == ItemKind::Pending)
            .cloned()
            .collect();
    }
    ranked
}

#[note("item-known-detail")]
pub fn known_detail(name: &str) -> Option<&'static str> {
    match name {
        "to_string_formatted" => Some("fn(&self) -> String"),
        "bold" => Some("fn(self, bool) -> Font"),
        "italic" => Some("fn(self, bool) -> Font"),
        "strike_out" => Some("fn(self, bool) -> Font"),
        "underline" => Some("fn(self, bool) -> Font"),
        "with_size" => Some("fn(self, f32) -> Font"),
        "clone" => Some("fn(&self) -> Self"),
        "clone_from" => Some("fn(&mut self, &Self)"),
        "clone_into" => Some("fn(&self, &mut Self)"),
        "to_string" => Some("fn(&self) -> String"),
        "is_empty" => Some("fn(&self) -> bool"),
        "len" => Some("fn(&self) -> usize"),
        _ => None,
    }
}

#[note("item-prefix-first")]
pub fn prefix_first(items: &mut [ItemSugestao], typed: &str) {
    items.sort_by(|a, b| {
        let starts = |i: &ItemSugestao| i.rotulo.starts_with(typed);
        starts(b)
            .cmp(&starts(a))
            .then_with(|| a.rotulo.cmp(&b.rotulo))
    });
}

#[note("item-local-items")]
pub fn local_items(typed: &str, text: &str, base: Option<&str>) -> Vec<ItemSugestao> {
    let typed_lower = typed.to_lowercase();
    let mut items = Vec::new();
    let mut seen = std::collections::HashSet::new();

    // 1. After `base.`: the members used on it come first.
    if let Some(base) = base {
        let base_dot = format!("{base}.");
        for line in text.lines() {
            let mut rest = line;
            while let Some(idx) = rest.find(&base_dot) {
                let after = &rest[idx + base_dot.len()..];
                let ident_len = after
                    .find(|c: char| !c.is_alphanumeric() && c != '_')
                    .unwrap_or(after.len());
                let ident = &after[..ident_len];
                if !ident.is_empty()
                    && (typed.is_empty() || ident.to_lowercase().starts_with(&typed_lower))
                    && seen.insert(ident.to_string())
                {
                    let after_ident = &after[ident_len..].trim_start();
                    let is_method = after_ident.starts_with('(');
                    let mut item = ItemSugestao::novo(
                        ident,
                        if is_method {
                            ItemKind::Method
                        } else {
                            ItemKind::Field
                        },
                    );
                    if let Some(d) = known_detail(ident) {
                        item = item.com_detalhe(d);
                    }
                    items.push(item);
                }
                rest = &after[ident_len..];
            }
        }

        // and the `fn`s declared with `self`.
        for line in text.lines() {
            let trimmed = line.trim();
            if let Some(after_fn) = trimmed.strip_prefix("fn ")
                && let Some(paren) = after_fn.find('(')
            {
                let fn_name = after_fn[..paren].trim();
                let params = &after_fn[paren..];
                if params.contains("self")
                    && !fn_name.is_empty()
                    && (typed.is_empty() || fn_name.to_lowercase().starts_with(&typed_lower))
                    && seen.insert(fn_name.to_string())
                {
                    let mut item = ItemSugestao::novo(fn_name, ItemKind::Method);
                    if let Some(d) = known_detail(fn_name) {
                        item = item.com_detalhe(d);
                    } else {
                        item = item.com_detalhe("fn(&mut self)");
                    }
                    items.push(item);
                }
            }
        }

        // Members found: they are the whole answer.
        if !items.is_empty() {
            prefix_first(&mut items, typed);
            items.truncate(24);
            return items;
        }
    }

    // 2. The words of the file (locals, functions).
    for word in text.split(|c: char| !c.is_alphanumeric() && c != '_') {
        if word.len() >= 2
            && (typed.is_empty() || word.to_lowercase().starts_with(&typed_lower))
            && word != typed
            && seen.insert(word.to_string())
        {
            let mut item = ItemSugestao::novo(word, ItemKind::Field);
            if let Some(d) = known_detail(word) {
                item = item.com_detalhe(d);
            }
            items.push(item);
        }
    }

    // 3. Rust's keywords.
    for &kw in crate::typing::RUST_KEYWORDS {
        if (typed.is_empty() || kw.to_lowercase().starts_with(&typed_lower))
            && seen.insert(kw.to_string())
        {
            items.push(ItemSugestao::novo(kw, ItemKind::Keyword));
        }
    }

    // 4. Common types.
    for &ty in crate::typing::COMMON_TYPES {
        if (typed.is_empty() || ty.to_lowercase().starts_with(&typed_lower))
            && seen.insert(ty.to_string())
        {
            items.push(ItemSugestao::novo(ty, ItemKind::Type));
        }
    }

    // 5. The components of the catalogue, by class name.
    for desc in crawrad_registry::Registry::builtin().all() {
        if (typed.is_empty() || desc.class.to_lowercase().starts_with(&typed_lower))
            && seen.insert(desc.class.to_string())
        {
            items.push(ItemSugestao::novo(desc.class, ItemKind::Type));
        }
    }

    // What starts with the typed text first.
    prefix_first(&mut items, typed);

    items.truncate(16);
    items
}

#[cfg(test)]
#[path = "test/item/test.rs"]
mod tests;
