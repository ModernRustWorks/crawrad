---
id: explorer-rows-rows-member-member-template
kind: doc
target_file: src/explorer_rows/rows/member.rs
line_hint: 64
author: 
created: 
title: "Member::template"
tags: []
signature: "fn template (& self) -> & 'static str"
---

ONE template for open and closed.

What separates them is the GLYPH, and the glyph is written from Rust —
so a second template would be a second name for one drawing. It used to
be two (`Member` / `MemberOpen`) because the fold mark was a shader with
an `instance open`, and the only way to flip it was another template.
