---
id: workspace-default-project-parent
kind: doc
target_file: src/workspace.rs
line_hint: 1896
author: 
created: 
title: "default_project_parent"
tags: []
signature: "pub fn default_project_parent (recent : Option < & Path >, home : Option < & Path >, cwd : Option < & Path >, app_dir : Option < & Path >) -> Option < PathBuf >"
---

Where the New Project dialog points when it opens.

It has to point somewhere a project can actually be written. The old default was
the process working directory, which is right when the IDE came from an ordinary
installer and **wrong** when it came from the Microsoft Store: there the working
directory IS the package's own `...\WindowsApps\<package>` folder, which Windows
keeps read-only with locked ACLs. The dialog opened pointing at the one place a
project can never live, and the person only found out after pressing Create.

The order is the useful one:

1. beside the project last opened, because a person keeps their projects together;
2. a `CrawRAD-Projects` folder under the home folder;
3. the working directory, but only when it is not inside the install folder.

Every input is a parameter, so the choice is tested without touching the real
environment — the caller reads the environment, this decides.

# Examples

```rust
use std::path::Path;
use crawrad_project::workspace::default_project_parent;

// The install folder is never offered, even when it is the working directory.
let install = Path::new("C:/Program Files/WindowsApps/GUARANA.CrawRAD_0.1.1.0");
let home = Path::new("C:/Users/someone");
assert_eq!(
    default_project_parent(None, Some(home), Some(install), Some(install)),
    Some(home.join("CrawRAD-Projects"))
);

// With nowhere writable it answers nothing, rather than a path that cannot work.
assert_eq!(
    default_project_parent(None, None, Some(install), Some(install)),
    None
);
```
