---
id: workspace-color-fields
kind: doc
target_file: src/workspace.rs
line_hint: 928
author: 
created: 
title: "COLOR_FIELDS"
tags: []
signature: "pub const COLOR_FIELDS : [(& str , & str) ; 15] = [(\"kw\" , \"Keywords\") , (\"type\" , \"Types\") , (\"fn\" , \"Functions\") , (\"id\" , \"Identifiers\") , (\"num\" , \"Numbers\") , (\"str\" , \"Strings\") , (\"com\" , \"Comments\") , (\"delim\" , \"Delimiters\") , (\"bg\" , \"Background\") , (\"diag_err\" , \"Error\") , (\"diag_warn\" , \"Warning\") , (\"diag_info\" , \"Info\") , (\"diag_err_band\" , \"Error band\") , (\"diag_warn_band\" , \"Warning band\") , (\"diag_info_band\" , \"Info band\") ,]"
---

The fifteen Color Scheme fields, in [`Preferences::color_fields`] order.

The fifteen Color Scheme fields, in [`Preferences::color_fields`] order:
the stem of their controls (`opt_cs_<stem>_val`, `_err`, `_sw`, `_more`)
and the name the Color dialog's title shows.

# Examples

```rust
use crawrad_project::workspace::COLOR_FIELDS;

assert_eq!(COLOR_FIELDS.len(), 15);
assert_eq!(COLOR_FIELDS[0].0, "kw");
```
