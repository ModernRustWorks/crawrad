---
id: relatorio-frase-da-poda
kind: doc
target_file: src/lib.rs
line_hint: 777
author: 
created: 
title: "Relatorio::frase_da_poda"
tags: []
signature: "pub fn frase_da_poda (& self) -> Option < String >"
---

What the status bar appends when pruning occurred, **in English**.

# Examples

```rust
use crawrad_codegen::Relatorio;

let mut relatorio = Relatorio::default();
assert_eq!(relatorio.frase_da_poda(), None);
relatorio.podados.push("btn_click".to_string());
assert_eq!(relatorio.frase_da_poda(), Some("1 empty handler removed".to_string()));
```

`None` when none occurred — in which case the status bar does not mention it, which is the case for
almost every generation.

It exists because **silent deletion is what is alarming in a classic RAD IDE**. There, the
handler disappears upon saving and nothing on screen indicates it disappeared; anyone unfamiliar with the
behavior thinks they lost their method. The phrase is intentionally short:
it shares the bar with the rest of the report, and what it needs to say
is *how many*.

The word is `empty`, rather than `unused` or `orphan`, because it is the only one of the
three that describes what the user would see upon opening the file: every
handler removed here has an empty body. `unused` would be a lie — an
empty handler that something calls is **not** removed.
