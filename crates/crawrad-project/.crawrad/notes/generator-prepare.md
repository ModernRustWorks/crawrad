---
id: generator-prepare
kind: doc
target_file: src/generator.rs
line_hint: 281
author: 
created: 
title: "prepare"
tags: []
signature: "pub fn prepare (destination : & Path , pending : & mut dyn PendingEdits , ir : & Document , package : & str ,) -> Result < Plan , String >"
---

Prepares a generation plan by flushing editor edits and evaluating disk state.

**Write what is in the editor, and ONLY THEN ask the disk what to
generate.**

The order is the whole function, and it is issue #49. The pre-pass
[`crawrad_codegen::vinculos_vencidos`] reads the **disk**; what the user has
just typed lives in the editor's buffer. Without this write, `Generate` and
`Run` judge a file older than the screen.

# What was measured, and it is worse than losing the typing

With a freshly created stub on disk, one line typed into the buffer and not
saved, the old `Generate` produced, in this order:

```text
expired       -> ["btn_okClick"]        (the pre-pass judges an empty disk)
pruned        -> ["btn_okClick"]        (the `fn` leaves the user's file)
the next Ctrl+S -> Err(MudouNoDisco)
```

The last line is the one that was not in the issue. The dirty buffer is
still on screen — which is why the review of PR #48 called the finding
non-blocking — but it becomes **impossible to write**: saving is refused
because the file changed underneath (the pruning changed it), and reloading
is refused because the buffer is dirty. Both refusals are right on their
own, and together they close the door.

# Is writing in silence on Generate/Run the right thing?

It is what a classic RAD IDE does when compiling, and it is the same statement the
owner used for `Ctrl+S` generating: *"if I am saving, I am agreeing with
everything that is there"*. Nobody presses `Run` to see the code from
before they typed. What must not be missing is the bar saying what was
written — and it does, through [`saved_sentence`].

# And when the write is refused

`Err`, and the generation **does not happen**. Generating on top would make
the pre-pass judge a disk the user does not control, which is exactly the
defect above. The sentence already carries the [`FAILURE_PREFIX`], because
to the caller this is a generation that did not work out — and it is how
`Run` knows not to launch cargo.

# Errors

Returns an error string with [`FAILURE_PREFIX`] if flushing pending edits fails.

# Examples

```rust
use std::path::Path;
use crawrad_ir::Document;
use crawrad_project::generator::prepare;

let doc = Document::new("MainForm", "Form");
let plan = prepare(Path::new("."), &mut (), &doc, "app");
assert!(plan.is_ok());
```
