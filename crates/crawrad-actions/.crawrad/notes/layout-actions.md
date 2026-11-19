---
id: layout-actions
kind: doc
target_file: src/lib.rs
line_hint: 4326
author: 
created: 
title: "layout_actions"
tags: []
signature: "fn layout_actions () -> Vec < Action >"
---

THE LAYOUT OF PANELS — open, close, move, stack, float, restore.

# Why they are actions, and not shell state

The owner's law, 15/09/2026: *"don't forget that everything in the IDE must have actions,
so the list can later be used as tools for the MCP"*. A panel that
only opens by clicking on the rail is a panel that an agent cannot open —
and the ruler given was a single question: *can this be done via a
tool, without a mouse?*

# Why almost all require an argument, and what this does to the menu

`Reset Layout` is about the whole arrangement and asks for nothing; the others are
about **one** panel, and a menu has nowhere to type which
([`Action::requires_argument`]). So only `layout.reset` appears in the `View`
menu, and the others appear through doors that know how to collect the argument: the
rail (which knows which icon was clicked), the tab context menu (which
knows which tab) and the MCP.

This is **not** a loophole: it is the same rule that took from the `Edit` menu the
second `Standard actions...`, lit and inert.

# None of them has a predicate reading the screen

`enabled` is `|_| true` across all seven, and it is deliberate. Moving a panel to
an edge is always valid; resetting is always valid; showing a panel that is already
visible focuses it. A predicate that needed to know the current layout
would have to bring the layout into [`Context`] — and then the MCP
would need a window to ask if it can.
