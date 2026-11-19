//! The microphone's words in the prompt and the status bar, as the window said them.

use super::*;

#[test]
fn recording_keeps_what_was_typed_before_the_note() {
    assert_eq!(recording_started("  "), "Recording audio... (speak now)");
    assert_eq!(
        recording_started(" fix this "),
        "fix this Recording audio..."
    );
    assert_eq!(
        recording_prompt("", 1.25),
        "🔴 Recording (1.2s)... Speak into the microphone and click to finish"
    );
    assert_eq!(
        recording_prompt("fix", 2.0),
        "fix [🔴 Recording (2.0s)... Speak into the microphone and click to finish]"
    );
    assert!(recording_status(3.04).starts_with("🔴 Recording audio: 3.0s"));
}

#[test]
fn the_transcription_joins_what_was_typed_and_what_was_said() {
    assert_eq!(after_transcription("typed ", "  "), "typed ");
    assert_eq!(after_transcription(" ", " hello "), "hello");
    assert_eq!(after_transcription(" fix ", " the leak "), "fix the leak");
    let mut result = TranscriptionResult {
        text: "  ".into(),
        language: "pt".into(),
        confidence: 1.0,
        duration_secs: 1.5,
    };
    assert_eq!(
        transcribed_status(&result),
        "No speech detected on the microphone."
    );
    result.text = " olá ".into();
    assert_eq!(
        transcribed_status(&result),
        "Audio transcribed (pt, 1.5s): \"olá\""
    );
}

#[test]
fn recording_dims_the_box_and_disables_send() {
    let (on, off) = (look(true), look(false));
    assert_eq!((on.disabled, off.disabled), (1.0, 0.0));
    assert_eq!(on.cursor, [0.0; 4], "no caret while recording");
    assert_eq!(off.button, [0.0, 0.48, 0.8, 1.0]);
    assert_eq!(on.text, on.button_text);
}
