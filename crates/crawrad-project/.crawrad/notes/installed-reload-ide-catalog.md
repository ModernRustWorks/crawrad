---
id: installed-reload-ide-catalog
kind: doc
target_file: src/installed.rs
line_hint: 95
author: 
created: 
title: "reload_ide_catalog"
tags: []
signature: "pub fn reload_ide_catalog () -> & 'static InstalledPackages"
---

Reads the installed packages again, so an install is used with no restart.

The new catalog is LEAKED, and on purpose: every caller holds
`&'static InstalledPackages`, one catalog is a few KB, and an install is
rare. Freeing the old one would leave those references dangling.

# Examples

```
use crawrad_project::installed::reload_ide_catalog;

let catalog = reload_ide_catalog();
assert!(catalog.catalog.all().count() > 0);
```
