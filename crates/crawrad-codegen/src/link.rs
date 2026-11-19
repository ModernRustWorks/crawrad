//! The other side of the link: **finding** the `fn` that the generator wrote.
//!
//! [`stubs`](crate::rxp) knows how to *create* a handler. This module knows how to tell
//! *where it is* — and that is the half that makes double-clicking a button place the
//! cursor inside `on_click`, the reason the editor exists
//! (ADR 0015).
//!
//! ## Why searching for `"fn <name>("` is not enough
//!
//! Because the file belongs to the user, and the user writes code. The same sequence
//! appears in `// TODO: call fn btn_okClick(`, inside a message
//! string, in a SQL `r#"..."#`. And `self.btn_okClick();` is a call, not a
//! definition. A naive `find` finds the first occurrence and the cursor lands
//! in a comment — the defect is silent and the user blames the editor.
//!
//! The solution is a state scanner (outside / text / raw text / line
//! comment / block comment) that clears everything that is not code before
//! searching. It is not tree-sitter and does not intend to be: ADR 0015 defers the tree
//! until `TextEdit` is replaced. It is the same machine the editor highlighter needs,
//! so it pays for itself twice.
//!
//! ## What this module does not do
//!
//! Does not rename, **does not decide what to delete**, and does not guess. If the user
//! renamed the `fn` by hand, the link broke — and the ADR states, in no uncertain terms,
//! that the remedy is real refactoring, not guessing.
//!
//! The "does not delete" caveat changed form on 07/09/2026, and makes all
//! the difference: this module now knows how to **measure** a handler
//! ([`extensao_do_handler`]) and report if anyone else **cites** it
//! ([`citado_fora`]) — which is what `rxp` pruning needs in order to work. But
//! what decides remains the other side: here there is not a single line that inspects
//! the document or the stub text. A scanner that also judges is
//! a scanner that deletes by mistake.

use std::ops::Range;
use std::str;

use crawrad_ir::Node;
use crawrad_note::note;
use crawrad_registry::ComponentDesc;

#[note("link-local")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Local {
    pub linha: usize,
    pub coluna: usize,
    /// The byte offset within the passed `fonte`.
    pub byte: usize,
}

pub type Location = Local;

#[note("link-localizar-handler")]
pub fn localizar_handler(fonte: &str, nome: &str) -> Option<Local> {
    if nome.is_empty() {
        return None;
    }
    definicoes(fonte)
        .into_iter()
        .find(|(n, _)| n == nome)
        .map(|(_, byte)| local_de(fonte, byte))
}

#[note("link-handlers-do-arquivo")]
pub fn handlers_do_arquivo(fonte: &str) -> Vec<(String, Local)> {
    definicoes(fonte)
        .into_iter()
        .map(|(nome, byte)| (nome, local_de(fonte, byte)))
        .collect()
}

#[note("link-locate-handler")]
pub fn locate_handler(fonte: &str, nome: &str) -> Option<Local> {
    localizar_handler(fonte, nome)
}

pub fn file_handlers(fonte: &str) -> Vec<(String, Local)> {
    handlers_do_arquivo(fonte)
}

pub fn handler_extent(fonte: &str, byte: usize) -> Option<Extensao> {
    extensao_do_handler(fonte, byte)
}

pub fn typing_point(fonte: &str, nome: &str) -> Option<Local> {
    ponto_de_digitacao(fonte, nome)
}

pub fn cited_elsewhere(fonte: &str, nome: &str, dono: &Range<usize>) -> bool {
    citado_fora(fonte, nome, dono.clone())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extensao {
    /// The inner content between the braces, excluding them. This is what is checked to tell whether the
    /// body is empty.
    pub corpo: Range<usize>,
    /// The entire item: from the first character on the `fn` line to the byte right
    /// after the `}` closing the body.
    ///
    /// Starts at the **line**, not at `fn`, because indentation is part of
    /// what is removed: trimming from `pub` would leave four orphaned spaces
    /// on a line by themselves.
    pub item: Range<usize>,
}

pub type Extent = Extensao;

#[note("link-extensao-do-handler")]
pub fn extensao_do_handler(fonte: &str, byte: usize) -> Option<Extensao> {
    let mascara = mascarar(fonte);
    let codigo = str::from_utf8(&mascara).unwrap_or(fonte);
    let b = codigo.as_bytes();

    let abre = byte + codigo.get(byte..)?.find('{')?;
    let mut prof = 0usize;
    let mut i = abre;
    while i < b.len() {
        match b[i] {
            b'{' => prof += 1,
            b'}' => {
                prof -= 1;
                if prof == 0 {
                    let linha = fonte[..byte].rfind('\n').map_or(0, |p| p + 1);
                    return Some(Extensao {
                        corpo: abre + 1..i,
                        item: linha..i + 1,
                    });
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

#[note("link-ponto-de-digitacao")]
pub fn ponto_de_digitacao(fonte: &str, nome: &str) -> Option<Local> {
    let definicao = localizar_handler(fonte, nome)?;
    let corpo = extensao_do_handler(fonte, definicao.byte)?.corpo;
    let miolo = &fonte[corpo.clone()];
    // The line with `{` does not count: what remains of it is a single-line body
    // (`{ /* done */ }`), and then landing is right there, after the space.
    let (base, resto) = match miolo.find('\n') {
        Some(p) => (corpo.start + p + 1, &miolo[p + 1..]),
        None => (corpo.start, miolo),
    };
    // Stops at a bare `\n`: blank line lands on column 1, and is the worst
    // accepted case. `\t` is counted because the file belongs to the user and they
    // may have reindented everything with tabs.
    let recuo = resto
        .bytes()
        .take_while(|c| *c == b' ' || *c == b'\t')
        .count();
    Some(local_de(fonte, base + recuo))
}

#[note("link-citado-fora")]
pub fn citado_fora(fonte: &str, nome: &str, dono: Range<usize>) -> bool {
    if nome.is_empty() {
        return false;
    }
    let mascara = mascarar(fonte);
    let codigo = str::from_utf8(&mascara).unwrap_or(fonte);
    let mut i = 0;
    while let Some(passo) = codigo[i..].find(nome) {
        let ini = i + passo;
        let fim = ini + nome.len();
        i = fim;
        let colado = codigo[..ini]
            .chars()
            .next_back()
            .is_some_and(e_parte_de_ident)
            || codigo[fim..].chars().next().is_some_and(e_parte_de_ident);
        if !colado && !dono.contains(&ini) {
            return true;
        }
    }
    false
}

// ------------------------------------------------------------------ the scanner

#[note("link-definicoes")]
fn definicoes(fonte: &str) -> Vec<(String, usize)> {
    let mascara = mascarar(fonte);
    // The mask replaces **entire regions** with spaces, so the output
    // remains valid UTF-8; the fallback exists solely to avoid `unwrap`.
    let codigo = str::from_utf8(&mascara).unwrap_or(fonte);

    let mut achados = Vec::new();
    let mut i = 0;
    while let Some(passo) = codigo[i..].find("fn") {
        let kw = i + passo;
        i = kw + 2;

        // `fn` must be a whole word. Without this, a `self.fnord` field
        // or a `Confn` type becomes a declaration.
        if codigo[..kw]
            .chars()
            .next_back()
            .is_some_and(e_parte_de_ident)
        {
            continue;
        }
        if codigo[i..].chars().next().is_some_and(e_parte_de_ident) {
            continue;
        }

        // Irregular spacing is the author's problem, not ours:
        // `fn   nome (` declares just as much as `fn nome(`. And looking for the word
        // `fn` — instead of matching `pub fn` — is what makes `pub(crate) fn`,
        // `async fn`, `const fn`, and `unsafe fn` come for free.
        let resto = &codigo[i..];
        let nome_ini = i + (resto.len() - resto.trim_start().len());
        let nome: String = codigo[nome_ini..]
            .chars()
            .take_while(|c| e_parte_de_ident(*c))
            .collect();
        if nome.is_empty() {
            // `fn(u32) -> u32` is a function pointer type, not a definition.
            continue;
        }
        i = nome_ini + nome.len();
        achados.push((nome, nome_ini));
    }
    achados
}

#[note("link-mascarar")]
fn mascarar(fonte: &str) -> Vec<u8> {
    let b = fonte.as_bytes();
    let mut m = b.to_vec();
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'/' if b.get(i + 1) == Some(&b'/') => {
                let ini = i;
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
                apagar(&mut m, ini, i);
            }
            b'/' if b.get(i + 1) == Some(&b'*') => {
                let ini = i;
                i = fim_do_bloco(b, i);
                apagar(&mut m, ini, i);
            }
            b'"' => {
                let ini = i;
                i = fim_do_texto(b, i);
                apagar(&mut m, ini, i);
            }
            b'\'' => match fim_do_char(fonte, i) {
                Some(fim) => {
                    apagar(&mut m, i, fim);
                    i = fim;
                }
                // Does not close: it is a lifetime (`'a`) or label (`'volta:`), and these
                // are code. Treating them as literals would swallow up to the next
                // single quote, which could be ten lines below.
                None => i += 1,
            },
            // `r"..."`, `r#"..."#`, `br#"..."#`, `cr#"..."#`.
            b'r' | b'b' | b'c' => match fim_do_cru(fonte, i) {
                Some(fim) => {
                    apagar(&mut m, i, fim);
                    i = fim;
                }
                None => i += 1,
            },
            _ => i += 1,
        }
    }
    m
}

fn apagar(m: &mut [u8], ini: usize, fim: usize) {
    let (ini, fim) = (ini.min(m.len()), fim.min(m.len()));
    for c in &mut m[ini..fim] {
        if *c != b'\n' {
            *c = b' ';
        }
    }
}

#[note("link-fim-do-bloco")]
fn fim_do_bloco(b: &[u8], ini: usize) -> usize {
    let mut prof = 0usize;
    let mut i = ini;
    while i < b.len() {
        if b[i] == b'/' && b.get(i + 1) == Some(&b'*') {
            prof += 1;
            i += 2;
        } else if b[i] == b'*' && b.get(i + 1) == Some(&b'/') {
            prof = prof.saturating_sub(1);
            i += 2;
            if prof == 0 {
                return i;
            }
        } else {
            i += 1;
        }
    }
    b.len()
}

#[note("link-fim-do-texto")]
fn fim_do_texto(b: &[u8], ini: usize) -> usize {
    let mut i = ini + 1;
    while i < b.len() {
        match b[i] {
            // `\"` does not close, and `\\` does not escape the following quote. Skipping two
            // bytes handles both cases at once; UTF-8 continuation bytes
            // are never `"` or `\`, so the jump cannot land on a deceptive
            // position.
            b'\\' => i += 2,
            b'"' => return i + 1,
            _ => i += 1,
        }
    }
    b.len()
}

#[note("link-fim-do-cru")]
fn fim_do_cru(fonte: &str, ini: usize) -> Option<usize> {
    // An `r` adjacent to an identifier is part of it. Without this guard,
    // `contador#` and similar things would start a literal out of nowhere.
    if fonte[..ini]
        .chars()
        .next_back()
        .is_some_and(e_parte_de_ident)
    {
        return None;
    }
    let b = fonte.as_bytes();
    let mut i = ini;
    if matches!(b[i], b'b' | b'c') {
        i += 1;
    }
    if b.get(i) != Some(&b'r') {
        return None;
    }
    i += 1;
    let cerquilhas = b[i..].iter().take_while(|c| **c == b'#').count();
    i += cerquilhas;
    // Without a quote after the hashes it is not a raw literal: it is a raw
    // identifier (`r#type`), which is code like any other.
    if b.get(i) != Some(&b'"') {
        return None;
    }
    i += 1;
    // In a raw string **there are no escapes**: it only closes on a quote followed by the
    // same number of hashes. That is why it exists.
    while i < b.len() {
        if b[i] == b'"'
            && b[i + 1..]
                .iter()
                .take(cerquilhas)
                .filter(|c| **c == b'#')
                .count()
                == cerquilhas
        {
            return Some(i + 1 + cerquilhas);
        }
        i += 1;
    }
    Some(b.len())
}

#[note("link-fim-do-char")]
fn fim_do_char(fonte: &str, ini: usize) -> Option<usize> {
    let resto = fonte.get(ini + 1..)?;
    let mut chars = resto.chars();
    let primeiro = chars.next()?;

    if primeiro == '\\' {
        // Escape: `'\''`, `'\\'`, `'\n'`, `'\u{41}'`. The character immediately after
        // the backslash does not close anything, whatever it may be.
        let b = fonte.as_bytes();
        let mut i = ini + 3;
        while i < b.len() && b[i] != b'\n' {
            if b[i] == b'\'' {
                return Some(i + 1);
            }
            i += 1;
        }
        // Did not close on the line: character literals do not cross line breaks,
        // so this is broken code — and broken code is read as
        // code, not as a literal that swallows the file.
        return None;
    }

    if chars.next()? == '\'' {
        return Some(ini + 1 + primeiro.len_utf8() + 1);
    }
    None
}

fn e_parte_de_ident(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

#[note("link-local-de")]
fn local_de(fonte: &str, byte: usize) -> Local {
    let antes = &fonte[..byte];
    let linha = antes.matches('\n').count() + 1;
    let inicio = antes.rfind('\n').map_or(0, |p| p + 1);
    let prefixo = &fonte[inicio..byte];
    // `gravar()` no longer writes a BOM (see reason there), but projects generated
    // before 01/09/2026 still have one on disk, and `read_to_string` returns
    // it as a character. Counting it would shift the entire first line one column
    // to the right — and the BOM occupies no column on the screen.
    let prefixo = prefixo.strip_prefix('\u{feff}').unwrap_or(prefixo);
    Local {
        linha,
        coluna: prefixo.chars().count() + 1,
        byte,
    }
}

// ------------------------------------------------- WHICH `fn` double-click
// looks for
//
// This arrived from the shell on 16/09/2026, for the same reason that brought
// `handler_padrao` to this crate on 07/09: there came to be TWO callers. The
// egui shell asked here via `codigo.rs`; the Makepad shell does not reach
// that file — it depends on the egui text editor —, and a second copy
// would mean two ideas of which event double-click opens.
//
// And this is the place and nowhere else: what knows how to FIND the `fn` is this module, and knowing
// WHICH ONE to look for is the other half of the same question.

#[note("link-elo")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Elo {
    /// Event key in IR: `on_click`.
    pub evento: String,
    /// The `fn` in the user file: `btn_okClick`.
    pub handler: String,
    /// Did the document **already** point to this `fn`?
    ///
    /// `false` means double-click must bind the event before
    /// searching for code — which is what a classic RAD IDE does by naming `Button1Click`
    /// on its own upon the first double-click.
    pub ja_ligado: bool,
}

#[note("link-elo-do-duplo-clique")]
pub fn elo_do_duplo_clique(no: &Node, desc: &ComponentDesc) -> Option<Elo> {
    let ligado = desc.events.iter().find(|e| {
        no.events
            .get(e.name)
            .is_some_and(|alvo| !alvo.trim().is_empty())
    });
    if let Some(e) = ligado {
        return Some(Elo {
            evento: e.name.to_string(),
            handler: no.events[e.name].trim().to_string(),
            ja_ligado: true,
        });
    }
    let primeiro = desc.events.first()?;
    Some(Elo {
        evento: primeiro.name.to_string(),
        handler: crate::handler_padrao(no.name(), primeiro.name),
        ja_ligado: false,
    })
}

#[cfg(test)]
#[path = "test/link/test.rs"]
mod test;
