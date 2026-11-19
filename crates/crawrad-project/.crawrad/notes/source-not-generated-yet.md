---
id: source-not-generated-yet
kind: doc
target_file: src/source.rs
line_hint: 48
author: 
created: 
title: "NOT_GENERATED_YET"
tags: []
signature: "pub const NOT_GENERATED_YET : & str = \"// This form has no generated unit on disk yet.\\n\\\n     //\\n\\\n     // The unit is where your handlers live, and it is written by `Generate`\\n\\\n     // (Project menu, or the Run button, which generates before it builds).\\n\\\n     //\\n\\\n     // Alt+F12 shows the form source instead, and that one is planned live\\n\\\n     // from the document you are editing — no disk needed.\""
---

What the unit view says when the project has never been generated.

# It used to send the user to the other shell, and that was a LIE

The sentence here read *"The Makepad shell does not write to disk yet: run
`Generate` in the egui shell"*. It was true when it was written and stopped
being true when `Arquivo::Gerar` got wired (`ide_app.rs`, the `Run` round):
this shell generates, compiles and launches.

A message that outlives the gap it describes is worse than no message — it
sends the user out of the IDE to do something the IDE already does. It is
the owner's rule *what you take away, you forget*, applied to a sentence
instead of a button.

# Examples

```rust
use crawrad_project::source::NOT_GENERATED_YET;

assert!(NOT_GENERATED_YET.contains("no generated unit on disk yet"));
```
