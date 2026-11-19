---
id: ide-app-build-ideapp-salvar
kind: doc
target_file: src/ide_app/build.rs
line_hint: 142
author: 
created: 
title: "IdeApp::salvar"
tags: []
signature: "pub (super) fn salvar (& mut self , cx : & mut Cx)"
---

Saves the form to the file it came from.

**`Ctrl+S` GENERATES**, and the decision is the owner's, with the IDE on screen
(2026-09-08): *"Ctrl+S must generate, because if I am saving, I am
agreeing with everything there both in the designer and in the editor"*.

# Why this matters, and why it was missing here

What PRUNES the empty handler is generation (ADR 0025), and it is also what
proposes UNLINKING the event that remained pointing to the pruned handler. Without
this call, the classic RAD IDE gesture — created by mistake, wrote nothing,
saved, and the block disappears along with the binding — simply did not happen
in this shell: the empty handler remained in the file and the name continued in
the Object Inspector. The owner saw it on screen on 2026-09-17.

# The ORDER is the guarantee

Disk first, generation last: `Ctrl+S` is the gesture the user trusts
most, and they cannot leave here thinking they saved when it was
prevented by a generator error. It is the same order as the egui shell, and what
decides whether to generate is the SAME function
(`crate::generator::should_generate_on_save`).

# And the saved revision is remarked AFTERWARDS

Pruning modifies the document, so the revision increments: without remarking, the title
badge would revert to `modified` in the frame following the save, and the user
would see "not saved" right after saving.

#### At `if self.save_editor_file(cx) {`

**THE EDITOR FILE COMES FIRST, AND ALONE.**

With an edited `.rs` in front, `Ctrl+S` means *save this
file* — that is what it means in VS Code and that is what it means
in a classic RAD IDE. Indirectly saving the form and generating the project
behind it would rewrite `.designer.rs` and prune handlers (ADR
0025) because of a typed line, which is not what was requested.
