---
id: session-audiosession-stop-and-transcribe
kind: doc
target_file: src/session.rs
line_hint: 67
author: 
created: 
title: "AudioSession::stop_and_transcribe"
tags: []
signature: "pub fn stop_and_transcribe < T : SpeechTranscriber + 'static > (& mut self , transcriber : T , config : TranscriptionConfig ,) -> Result < () , AudioError >"
---

Stops recording and starts asynchronous background transcription.

# Examples

```rust
use crawrad_audio::{AudioSession, MockAudioRecorder, TranscriptionConfig};
use crawrad_audio::transcriber::MockTranscriber;

let rec = MockAudioRecorder::new(vec![0.0; 1600]);
let mut session = AudioSession::new(rec, || {});
session.start_recording().unwrap();
let mock_tr = MockTranscriber::new("text", "en");
session.stop_and_transcribe(mock_tr, TranscriptionConfig::default()).unwrap();
```

# Errors

Returns [`AudioError`](crate::AudioError) if stopping the recorder or spawning the worker fails.
