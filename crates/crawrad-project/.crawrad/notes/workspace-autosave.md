---
id: workspace-autosave
kind: doc
target_file: src/workspace.rs
line_hint: 465
author: 
created: 
title: "AutoSave"
tags: []
signature: "pub enum AutoSave"
---

The GLOBAL IDE preferences — those not belonging to the open folder.

Stored in the user configuration directory for the same reason as recents: a
preference about how the IDE **opens** cannot live inside a project,
otherwise it does not exist precisely when it matters, which is before any
project is open.

The file is `key=value` so new entries arrive without migration — and the
bar for what enters here is narrow: a FOLDER preference belongs to its
`.crawrad/`, not here.
When the editor files save by themselves (#555). Only editor files: the
form saves with Save, because saving it regenerates the designer code.

# Examples

```rust
use crawrad_project::workspace::AutoSave;

assert_eq!(AutoSave::default(), AutoSave::Off);
```
