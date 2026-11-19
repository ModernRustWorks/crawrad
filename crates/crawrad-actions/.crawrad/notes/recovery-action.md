---
id: recovery-action
kind: doc
target_file: src/lib.rs
line_hint: 1308
author: 
created: 
title: "recovery_action"
tags: []
signature: "fn recovery_action (id : & 'static str , label : & 'static str , hint : & 'static str , execute : fn (& Context) -> Effect ,) -> Action"
---

Builds an action over recovery copies.

**`Save` is valid when there is ANYTHING to save** — the form or the
file open in the editor.

It replaced [`has_document`] which was right while the only savable
thing was the form. Ever since the code editor began editing, a
project without a form — which ADR 0017 permits, and Explorer opens —
had `Ctrl+S` greyed out over a file full of modifications.
An action over the recovery copies (#555): `files` names them by their
paths in the project, and none means every copy on offer.
