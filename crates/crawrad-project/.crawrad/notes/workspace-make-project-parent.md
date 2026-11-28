---
id: workspace-make-project-parent
kind: doc
target_file: src/workspace.rs
line_hint: 1945
author: 
created: 
title: "make_project_parent"
tags: []
signature: "pub fn make_project_parent (folder : & Path , app_dir : Option < & Path >) -> Result < () , Refusal >"
---

Makes the folder a project is about to be written into, or says why it cannot.

`Create` is a promise to create, so a folder that does not exist yet is created
here instead of being answered with a file picker.

The picker used to be the silent fallback for any path that was not a directory.
That is how editing the drive letter of the default location — which made the path
point nowhere — turned the Create button into a folder chooser, saying nothing
about why. A button named Create that opens a chooser reads as mis-wired, and the
folder icon beside the field is already the chooser.

`app_dir` is CrawRAD's own install folder when it is known. A path inside it gets
its own phrase, because no amount of retrying will make that folder writable and
the person deserves to be told that rather than handed an OS error code.

# Errors

Returns a `Refusal` whose phrase says what is wrong and that nothing was written:

- the path exists but is a **file**, not a folder;
- the path is **inside CrawRAD's own install folder**, which Windows keeps
  read-only — this one is named explicitly, because retrying cannot help;
- the folder **could not be created**, with the operating system's own reason
  carried in the phrase.

# Examples

```rust
use std::path::Path;
use crawrad_project::workspace::make_project_parent;

// A path inside the install folder is refused by name, and nothing is created.
let install = Path::new("C:/Program Files/WindowsApps/GUARANA.CrawRAD_0.1.1.0");
let refusal = make_project_parent(&install.join("my_app"), Some(install))
    .expect_err("the install folder is read-only");
assert!(refusal.phrase.contains("read-only"));
assert!(refusal.phrase.contains("Nothing was written"));
```
