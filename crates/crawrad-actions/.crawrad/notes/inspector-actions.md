---
id: inspector-actions
kind: doc
target_file: src/lib.rs
line_hint: 3571
author: 
created: 
title: "inspector_actions"
tags: []
signature: "fn inspector_actions () -> Vec < Action >"
---

Actions controlling window openings and folding in the Object Inspector.

THE THREE OPENINGS OF THE OBJECT INSPECTOR — a property's window, its
closing, and the folding of a sub-row group.

# Why they exist

Three of the eleven `PropKind` do not fit on a single line: `Multiline`, `Collection` and
the visual picker of `Align`. The only way to reach them is a button that
**opens something** — and, until now, opening was the shell manipulating its own
fields (`ide.open_items_editor(&id)`, in the egui shell). The house rule is
one line: *can an agent do this via a tool, without a mouse?* The
answer was no, and a capability that the shell solves internally is a capability
that never reaches the protocol.

# The group is `Inspector`, and no bar menu draws it

By the same argument as `explorer_actions`: what is open inside a
panel is not a window menu item.

# What they do NOT decide

**Which** window opens. That is a table in the shell
(`crawrad-ide/src/inspector.rs:959-969`), because the registry is DATA and a
descriptor that carried the name of a dialog would put the UI inside the
data (ADR 0005). And they do not decide what the window writes: everything coming out of it
returns via `component.set_property`.
