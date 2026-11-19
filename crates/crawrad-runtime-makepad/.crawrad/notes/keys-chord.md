---
id: keys-chord
kind: doc
target_file: src/keys.rs
line_hint: 76
author: 
created: 
title: "Chord"
tags: []
signature: "pub struct Chord"
---

The modifiers requested by a chord.

`primary` represents `Ctrl` (or `Cmd` on macOS), matching Makepad's
`KeyModifiers::is_primary`.

# Examples

```rust
use crawrad_runtime_makepad::keys::Chord;

let chord = Chord::default();
assert!(!chord.primary);
```
