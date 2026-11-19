---
id: creation-outcome-on-screen
kind: doc
target_file: src/creation.rs
line_hint: 196
author: 
created: 
title: "Outcome::on_screen"
tags: []
signature: "pub fn on_screen (& self) -> Option < (String , Option < PathBuf >) >"
---

Returns the phrase and optional open folder shown on screen.

What the progress dialog shows for this outcome: the **phrase**, and the
folder for the `Open` button when there is one to offer.

`None` means *there is no dialog*: [`Outcome::Opened`] clears `creating`
entirely in `Ide::settle_creation`, and that is what makes the window disappear.

Lives here, and not in a `match` inside rendering, for a measured reason: a
`match` inside `show_viewport_immediate` can only be measured by launching the IDE — and
what this PR had to fix was precisely a missing arm.

# Examples

```rust
use std::path::PathBuf;
use crawrad_project::creation::Outcome;

let out = Outcome::Opened(PathBuf::from("."));
assert_eq!(out.on_screen(), None);
```
