---
id: ide-app-audiosessioninstance
kind: doc
target_file: src/ide_app.rs
line_hint: 308
author: 
created: 
title: "AudioSessionInstance"
tags: []
signature: "pub type AudioSessionInstance = crawrad_audio :: AudioSession < crawrad_audio :: CpalAudioRecorder >"
---

Concrete instance of the audio coordinator for the CrawRAD shell.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let _session: Option<AudioSessionInstance> = None;
assert!(_session.is_none());
```
