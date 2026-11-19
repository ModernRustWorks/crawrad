---
id: workspace-steps-for
kind: doc
target_file: src/workspace.rs
line_hint: 1438
author: 
created: 
title: "steps_for"
tags: []
signature: "pub fn steps_for (project_type : ProjectType , with_form : bool) -> Vec < Step >"
---

The creation plan: the steps that will happen, **in the order they happen**.

Pure, and `with_form` enters as a parameter instead of being deduced here:
it is the caller's `form.is_some()`, and **not** `project_type.with_form()`.
The difference is real — it is the `Option<Document>` that decides whether `.crawform`
is saved, and asking the type here would put a step into the plan that the
caller never told to run, leaving the dialog waiting for a step that never starts.

The order is that of `create_project`, and the test suite verifies this by running
real `cargo` and comparing what it ANNOUNCES with what it DOES.

# Examples

```rust
use crawrad_project::workspace::{steps_for, ProjectType, Step};

let steps = steps_for(ProjectType::Gui, true);
assert_eq!(steps, vec![Step::Package, Step::Dependency, Step::Code]);
```
