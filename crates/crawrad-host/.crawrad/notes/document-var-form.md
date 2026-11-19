---
id: document-var-form
kind: doc
target_file: src/document.rs
line_hint: 21
author: 
created: 
title: "VAR_FORM"
tags: []
signature: "pub const VAR_FORM : & str = \"CRAWRAD_FORM\""
---

The environment variable pointing to a form OUTSIDE the
repository.

It lives here, and not in `ide_app.rs`, for a deliberate reason: the test
`todo_token_citado_pela_casca_existe_no_tema` scans the text of `ide_app.rs`
for every identifier that starts with `CRAWRAD_` and requires that it exists
in `ide_app/tema.rs`. `CRAWRAD_FORM` is an environment variable name, not a
theme token — written there, it would fail a test that is correct.

# Examples

```rust
use crawrad_host::document::VAR_FORM;

assert_eq!(VAR_FORM, "CRAWRAD_FORM");
```
