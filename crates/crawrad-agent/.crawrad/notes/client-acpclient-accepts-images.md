---
id: client-acpclient-accepts-images
kind: doc
target_file: src/client.rs
line_hint: 1053
author: 
created: 
title: "AcpClient::accepts_images"
tags: []
signature: "pub fn accepts_images (& self) -> bool"
---

Whether the agent takes image blocks in a prompt.

Read from `agentCapabilities.promptCapabilities.image` in `initialize`.
When false, pasted pictures go as their file path.

# Examples

```rust,no_run
fn send(client: &crawrad_agent::client::AcpClient) {
    if !client.accepts_images() {
        println!("images go as paths");
    }
}
```
