---
id: installed-refusal-notice
kind: doc
target_file: src/installed.rs
line_hint: 117
author: 
created: 
title: "refusal_notice"
tags: []
signature: "pub fn refusal_notice () -> Option < String >"
---

Opening notice for the status bar when installed packages were refused.

**The opening notice**, when a package did not load: `None` is the normal
case — a bar that talks with nothing to say is a bar nobody reads when
there is. It cannot wait for the user to look: whoever installed a
component opens the IDE, sees no new palette entry, and has nowhere to start.

# Examples

```rust
use crawrad_project::installed::refusal_notice;

let notice = refusal_notice();
if let Some(msg) = notice {
    assert!(msg.contains("refused"));
}
```
