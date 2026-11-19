---
id: generator-report-sentence
kind: doc
target_file: src/generator.rs
line_hint: 370
author: 
created: 
title: "report_sentence"
tags: []
signature: "pub fn report_sentence (target : & str , destination : & Path , r : & crawrad_codegen :: Relatorio) -> String"
---

What the bar says after a generation that worked.

A function, and not a `format!` buried in the shell, because building an
IDE in a test asks for a window and a video context — written in there, the
sentence would only be measurable by opening the program, and that is how
the pruning line went months without being written.

The report says what was **kept**, and that matters: it is the visible proof
that the user's code was not touched.

And it says what was **pruned**, which is the part that was missing.
Deleting in silence is what frightens in a classic RAD IDE: the handler leaves on save
and nothing on screen says it left. The sentence already existed
(`Relatorio::frase_da_poda`) and nobody read it — the user switched an event
off, the handler left his file, and the bar talked about something else. In
a video that looks like the IDE eating code; and now that `Ctrl+S`
generates, it can prune without anyone having pressed `Generate`.

# Examples

```rust
use std::path::Path;
use crawrad_codegen::Relatorio;
use crawrad_project::generator::report_sentence;

let rep = Relatorio::default();
let s = report_sentence("Makepad", Path::new("."), &rep);
assert!(s.contains("generated for Makepad"));
```
