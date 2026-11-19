---
id: componenteffect
kind: doc
target_file: src/lib.rs
line_hint: 966
author: 
created: 
title: "ComponentEffect"
tags: []
signature: "pub enum ComponentEffect"
---

**WHAT TO DO WITH THE COMPONENT SUITE** — `Component` of the classic RAD IDE.

A family separate from [`FileEffect`] by the criterion that already separates all others:
that one touches the user's project — saves, generates, runs —, and this does not touch the
project. It touches what the **IDE** knows how to draw, which is the catalogue.

An agent reading `Effect` to know what will happen reads the family and
already knows: it does not write to the document and does not touch the open project; it writes to the
installation components folder.

# The action installs nothing — it says to ASK

It is the same discipline as [`FileEffect::NewProject`]: the action declares, the shell
asks with its dialog, and whoever checks and copies is
`crawrad-ide::instalacao`, which is headless and has tests.

It is what makes `component.install` callable by an agent without needing
a folder picker — the agent passes `path`, the person gets the picker — step §10 of
ADR 0033.

# Examples

```rust
use crawrad_actions::ComponentEffect;

use std::path::PathBuf;

let effect = ComponentEffect::Install(Some(PathBuf::from("components/dataengine-crawrad")));
assert_ne!(effect, ComponentEffect::Install(None));
```
