---
id: session-audiosessionstatus
kind: doc
target_file: src/session.rs
line_hint: 14
author: 
created: 
title: "AudioSessionStatus"
tags: []
signature: "pub enum AudioSessionStatus"
---

Status of the audio recording/transcription session.

# Examples

```rust
use crawrad_audio::AudioSessionStatus;

let status = AudioSessionStatus::Idle;
assert_eq!(status, AudioSessionStatus::Idle);
```
