---
id: chat-attachment-paste-image
kind: doc
target_file: src/chat_attachment.rs
line_hint: 181
author: 
created: 
title: "paste_image"
tags: []
signature: "pub fn paste_image (taken : usize) -> Option < Result < ImageAttachment , String > >"
---

Ctrl+V in the chat: the clipboard's picture, saved as an attachment.

`None` when the clipboard holds text (the text paste goes on as always)
or no picture. `taken` is how many chips the prompt already has.

# Errors

When the chips are full or the picture cannot be saved.

# Examples

```rust,no_run
match crawrad_agent::paste_image(0) {
    None => println!("nothing to paste as an image"),
    Some(Ok(img)) => println!("{}", img.chip_text(0)),
    Some(Err(why)) => println!("{why}"),
}
```
