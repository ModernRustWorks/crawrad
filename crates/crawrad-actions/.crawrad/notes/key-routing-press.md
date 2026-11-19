---
id: key-routing-press
kind: doc
target_file: src/key_routing.rs
line_hint: 92
author: 
created: 
title: "Press"
tags: []
signature: "pub struct Press < 'a >"
---

One key press representation as read by the key router.

One key press as the router reads it, the toolkit's own types left in the
shell: whether a catalog chord (`"Ctrl+Shift+B"`) is this key, which arrow
it is, and the modifiers held.

# Examples

```rust
use crawrad_actions::key_routing::Press;

let press = Press {
    matches: &|chord| chord == "Ctrl+S",
    arrow: None,
    escape: false,
    shift: false,
    control: true,
    alt: false,
    logo: false,
};
assert!((press.matches)("Ctrl+S"));
```
