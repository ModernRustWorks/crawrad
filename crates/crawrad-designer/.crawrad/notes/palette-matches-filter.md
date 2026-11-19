---
id: palette-matches-filter
kind: doc
target_file: src/palette.rs
line_hint: 115
author: 
created: 
title: "matches_filter"
tags: []
signature: "fn matches_filter (d : & ComponentDesc , query : & str) -> bool"
---

Returns whether the component matches the search query.

Does the component enter the list with this search?

The rule is that of egui's Toolbox, word for word
(`crates/crawrad-ide/src/paleta.rs:476-480`): substring in any position
(not prefix), case-insensitive, matching both the **display name** and the
**class**. Both because today they are identical across the entire catalog and will
diverge at the first entry that gets a label with a space ("Check Box").

The `query` arrives already trimmed and in lower case, from [`rows`].
