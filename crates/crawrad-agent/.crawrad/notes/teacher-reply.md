---
id: teacher-reply
kind: doc
target_file: src/teacher.rs
line_hint: 236
author: 
created: 
title: "Reply"
tags: []
signature: "pub enum Reply"
---

Result of a teacher command: prompt or direct chat response.

What a teacher command does: send a prompt to the agent, or say something
in the chat without one.

# Examples

```rust
use crawrad_agent::teacher::Reply;

let reply = Reply::Say("Hello".to_string());
match reply {
    Reply::Say(text) => assert_eq!(text, "Hello"),
    Reply::Prompt(_) => unreachable!(),
}
```
