---
id: installed-ide-catalog
kind: doc
target_file: src/installed.rs
line_hint: 86
author: 
created: 
title: "ide_catalog"
tags: []
signature: "pub fn ide_catalog () -> & 'static InstalledPackages"
---

**THE IDE'S CATALOGUE FOR THIS SESSION** — built once.

Once, and not on every query: scanning disk on every frame would be expensive and,
worse, would cause the palette to mutate under the user while dragging.

**An install reads it again, and only an install** (#779):
[`reload_ide_catalog`] runs after `installation::install_now` copied a package,
and the shell then points its designer at the new catalog — a component is in
the palette at once, no restart. Between installs it stays the same, so nothing
mutates under a drag.

# Examples

```rust
use crawrad_project::installed::ide_catalog;

let cat = ide_catalog();
assert!(cat.catalog.all().count() > 0);
```
