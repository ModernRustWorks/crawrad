---
id: transcriber-mocktranscriber
kind: doc
target_file: src/transcriber.rs
line_hint: 51
author: 
created: 
title: "MockTranscriber"
tags: []
signature: "pub struct MockTranscriber"
---

Mock transcriber for unit tests and headless environments.

# Examples

```rust
use crawrad_audio::transcriber::MockTranscriber;

let mock = MockTranscriber::new("hello", "en");
assert_eq!(mock.response_text, "hello");
```
