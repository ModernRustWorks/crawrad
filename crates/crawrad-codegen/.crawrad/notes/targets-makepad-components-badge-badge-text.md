---
id: targets-makepad-components-badge-badge-text
kind: doc
target_file: src/targets/makepad/components/badge.rs
line_hint: 246
author: 
created: 
title: "badge_text"
tags: []
signature: "pub (crate) fn badge_text (caption : & str , count : i64 , max_count : i64) -> Option < String >"
---

Determines the text displayed by a badge.

What a badge reads: the count (capped as "99+") when there is one, else
its caption; `None` hides it. The generator's copy of
`crawrad_runtime_makepad::badge::badge_text` (the codegen does not depend on
the runtime); a test keeps the two equal.
