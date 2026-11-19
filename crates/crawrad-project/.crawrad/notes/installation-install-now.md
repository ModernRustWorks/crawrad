---
id: installation-install-now
kind: doc
target_file: src/installation.rs
line_hint: 240
author: 
created: 
title: "install_now"
tags: []
signature: "pub fn install_now (origin : & Path) -> Verdict"
---

Installs `origin` where the IDE installs, and reloads the catalog.

The one door the menu and the MCP share (#779): the package is checked
against the catalog in use, copied into [`installation_dir`], and the
catalog is read again, so its components are in the palette at once.

# Examples

```no_run
use crawrad_project::installation::install_now;

let verdict = install_now(std::path::Path::new("components/gauge-crawrad"));
println!("{}", verdict.phrase());
```
