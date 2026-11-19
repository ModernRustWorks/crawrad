---
id: page-control
kind: doc
target_file: src/lib.rs
line_hint: 1389
author: 
created: 
title: "page_control"
tags: []
signature: "fn page_control (c : & Context) -> Result < NodeId , String >"
---

Resolves the PageControl node receiving a new page.

The `PageControl` receiving the new page: the **specified** one, the
**selected** one, or the **owner of the selected page** — clicking on a tab and
requesting another is natural, and is the same rule `action_list_node` applies to
`ActionList`.

Returns the refusal phrase along with it, and not an `Option`, because both ends
ask the same thing and respond in different places: `enabled`
only wants to know if there is a target, and whoever calls via protocol — where there is no selection
nor tab strip to click — needs to hear WHICH refusal occurred. With
`Option`, the three turned into the same silence.
