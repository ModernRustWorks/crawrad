---
id: installed-ide-registry
kind: doc
target_file: src/installed.rs
line_hint: 104
author: 
created: 
title: "ide_registry"
tags: []
signature: "pub fn ide_registry () -> std :: rc :: Rc < Registry >"
---

A designer's own copy of the catalog in use, as it is now.

# Examples

```
use crawrad_project::installed::ide_registry;

assert!(ide_registry().all().count() > 0);
```
