---
id: explorer-actions
kind: doc
target_file: src/lib.rs
line_hint: 3849
author: 
created: 
title: "explorer_actions"
tags: []
signature: "fn explorer_actions () -> Vec < Action >"
---

THE TWO FOLDS OF THE EXPLORER — a member's node, and the legend.

# Why they exist

Before them, the Makepad shell folded a member node by mutating its own
field (`self.folds.toggle(...)`) and the legend did not fold at all.
The house rule is one line: *can an agent do this via a
tool, without a mouse?* For both the answer was no, and a capability that the
shell solves internally is a capability that never reaches the protocol.

# The group is `Explorer`, not `View`

`View` is the group that the bar menu draws (`ide_app/menu.rs`), and what
is open inside a panel is not a window menu item: it would be the
third line of a menu that already lists the six panels and `Reset Layout`.
The group here is the id prefix, which is how the MCP groups tools — and
no menu draws it, which is the right answer for a gesture that only exists
inside the panel.

# The predicate is `has_document`, not `|_| true`

The tree root is **derived from the open form**
(`crawrad-ide/src/ide_app/explorer.rs`, `group_root_of`): without a document the
panel says `No folder open.` and there is no node to fold. The predicate gives
the grey for free, and prevents the protocol from asking for a fold on a tree
that does not exist.
