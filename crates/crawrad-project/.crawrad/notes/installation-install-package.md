---
id: installation-install-package
kind: doc
target_file: src/installation.rs
line_hint: 171
author: 
created: 
title: "install_package"
tags: []
signature: "pub fn install_package (origin : & Path , base : & Path , already_installed : & Registry) -> Verdict"
---

**INSTALL.** Verifies, copies, and returns the verdict.

`base` is the installation's `components/` folder — chosen by the shell, and
that is why it enters as a parameter: the test installs into a temporary
directory and measures the same function the IDE runs.

# Examples

```rust
use std::path::Path;
use crawrad_registry::Registry;
use crawrad_project::installation::{install_package, Verdict};

let reg = Registry::builtin();
let verdict = install_package(Path::new("missing_dir"), Path::new("target/tmp"), &reg);
assert!(matches!(verdict, Verdict::Refused(_)));
```
