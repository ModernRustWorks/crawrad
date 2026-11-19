---
id: explorer-model-member-files
kind: doc
target_file: src/explorer_model.rs
line_hint: 254
author: 
created: 
title: "member_files"
tags: []
signature: "pub fn member_files < 'a > (m : & MemberNode , plan : & Option < Result < Vec < Arquivo > , crawrad_codegen :: Erro > > , lines : & 'a [Line] , has_design : bool ,) -> Result < & 'a [Line] , Silence >"
---

The files that a member body can show — or why it cannot.

**It is the central decision of ADR 0016, Amendment 1, and it lives here deliberately.**
Revision measured that, with `if !m.active` loose inside rendering, deleting it
left the 157 tests green: the member without document would list the files of the
ACTIVE member's plan, with badges, as if they were its own.

What it protects, in full: owner comes from the plan, plan comes from `Generator::generate`,
and generator needs a document. A member without an open document has no plan — and
without a plan one cannot assert who owns any of its files. Showing the rows of
ANOTHER member there would be worse than making things up: it would attribute to
one project the files of another.

`Result` is not decoration. In the error branch **there is no line slice**, so whoever
renders has nothing to render even if they forget to stop — the first of the three
corrections of rule 8, in its order of preference.

# Errors

Returns [`Silence`] when the member is inactive, cannot assemble a plan,
or has no plan.

# Examples

```rust
use std::path::PathBuf;
use crawrad_project::explorer_model::{member_files, MemberNode, Silence, NO_DESIGN};

let node = MemberNode {
    folder: PathBuf::from("pkg"),
    active: false,
    is_default: false,
};
let res = member_files(&node, &None, &[], false);
assert_eq!(res, Err(Silence::Phrase(NO_DESIGN)));
```
