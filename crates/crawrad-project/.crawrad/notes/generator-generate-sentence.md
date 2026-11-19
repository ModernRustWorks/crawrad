---
id: generator-generate-sentence
kind: doc
target_file: src/generator.rs
line_hint: 351
author: 
created: 
title: "generate_sentence"
tags: []
signature: "pub fn generate_sentence (saved : Option < & Path > , generation : String) -> String"
---

Formats the status bar message for a completed generation.

The whole sentence of `Generate`: what it saved by itself, and what it
generated.

# The save comes first

Because it is what happened first, and because saving without being asked
is the part that frightens the first time: it has to be where the eye
starts reading, and not at the end of a sentence that already counts three
numbers.

# And it disappears when the generation failed

It is not taste. `Run` and the double click on the canvas recognise a
refused generation by the PREFIX of the sentence ([`refused`]); putting
`saved x.rs — ` in front of a `generate: …` would make `Run` stop aborting
and launch cargo on top of a generation that did not happen. The rule lives
here, and not in the head of the caller, because "remember to use it only in
the `Ok` branch" is exactly the kind of rule the next hand forgets.

# Examples

```rust
use crawrad_project::generator::generate_sentence;

let s = generate_sentence(None, "generated".to_string());
assert_eq!(s, "generated");
```
