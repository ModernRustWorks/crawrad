---
id: write-file
kind: doc
target_file: src/lib.rs
line_hint: 463
author: 
created: 
title: "gravar"
tags: []
signature: "fn gravar (destino : & Path , conteudo : & str) -> Result < () , Erro >"
---

Writes a file of the generated project in pure UTF-8 without BOM.

Writes a file of the generated project. **Pure UTF-8, without BOM.**

The BOM was once here, and for a real reason: UTF-8 without BOM is indistinguishable
from Windows-1252 for guessers, and a VS Code tab once showed
`formulÃ¡rio`. It is no use removing accents from OUR comments — the user's
caption goes into the code, and a "Salvar Configurações" button puts accents
in the literal.

**This reasoning has been superseded since 01/09/2026, and the change was measured.**
`rustc` ignores the BOM; rust-analyzer does not. It emits `expected an item`
at offset 0, and because of that the `#![allow(...)]` on the next line is no longer
in the position of an INNER attribute: it becomes a loose `#` and dies on
`expected L_BRACK`. Without the `allow`, `mod formPrincipal;` in `main.rs`
starts reporting `non_snake_case`, and so do the designer fields. An example
project opened in VS Code with ~30 false errors and 8 warnings because of
three bytes. With the BOM removed, the SAME project gives zero diagnostics.

The accent remains correct: the file is valid UTF-8, and that is how any
modern editor reads it by default. Whoever has `files.autoGuessEncoding`
turned on for a file whose first accent falls late may still see
`formulÃ¡rio` — that is the price, and it belongs to the editor, not the file.
