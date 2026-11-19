---
id: clipboard-write-text
kind: doc
target_file: src/clipboard.rs
line_hint: 15
author: 
created: 
title: "write_text"
tags: []
signature: "pub fn write_text (text : & str) -> bool"
---

Puts `text` on the clipboard **now**, and says whether it did.

Makepad's `Cx::copy_to_clipboard` only queues the copy: it happens at the
end of the event pass. An MCP tool that answers "copied" before that pass
ends lies to whoever reads the clipboard next — measured on the E2E
battery (CB13), which read the old text right after `chat.copy_message`.

# Examples

```rust
use crawrad_os::clipboard::{read_text, write_text};

if write_text("CrawRAD") {
    assert_eq!(read_text().as_deref(), Some("CrawRAD"));
}
```
