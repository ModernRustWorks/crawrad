---
id: workspace-recent-limit
kind: doc
target_file: src/workspace.rs
line_hint: 61
author: 
created: 
title: "RECENT_LIMIT"
tags: []
signature: "const RECENT_LIMIT : usize = 10"
---

How many recent folders are retained.

Ten, not fifty: the menu is a list read from top to bottom, and a list that
does not fit on screen stops being a shortcut.
