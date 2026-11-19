//! What the chat's prompt box and status bar say, and how the prompt box looks,
//! while the microphone records and transcribes. Moved out of the window
//! unchanged (#478 paid the shell's line ratchet with it).

use crate::TranscriptionResult;
use crawrad_note::note;

#[note("prompt-transcribing")]
pub const TRANSCRIBING: &str = "⏳ Finishing the transcription with Whisper...";

#[note("prompt-mic-on")]
pub const MIC_ON: &str =
    "🔴 Microphone on. Speak now... (click the microphone to finish and enable Send)";

#[note("prompt-recording-started")]
pub fn recording_started(previous: &str) -> String {
    match previous.trim() {
        "" => "Recording audio... (speak now)".to_string(),
        typed => format!("{typed} Recording audio..."),
    }
}

#[note("prompt-recording-prompt")]
pub fn recording_prompt(previous: &str, secs: f32) -> String {
    let note =
        format!("🔴 Recording ({secs:.1}s)... Speak into the microphone and click to finish");
    match previous.trim() {
        "" => note,
        typed => format!("{typed} [{note}]"),
    }
}

#[note("prompt-recording-status")]
pub fn recording_status(secs: f32) -> String {
    format!(
        "🔴 Recording audio: {secs:.1}s (speak, then click the microphone to finish and enable Send)"
    )
}

#[note("prompt-after-transcription")]
pub fn after_transcription(previous: &str, said: &str) -> String {
    match (previous.trim(), said.trim()) {
        (_, "") => previous.to_string(),
        ("", said) => said.to_string(),
        (typed, said) => format!("{typed} {said}"),
    }
}

#[note("prompt-transcribed-status")]
pub fn transcribed_status(result: &TranscriptionResult) -> String {
    match result.text.trim() {
        "" => "No speech detected on the microphone.".to_string(),
        said => format!(
            "Audio transcribed ({}, {:.1}s): \"{said}\"",
            result.language, result.duration_secs
        ),
    }
}

#[note("prompt-promptlook")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PromptLook {
    pub text: [f32; 4],
    pub text_focus: [f32; 4],
    pub cursor: [f32; 4],
    pub button: [f32; 4],
    pub button_hover: [f32; 4],
    pub button_text: [f32; 4],
    /// The send button's `disabled` shader input.
    pub disabled: f64,
}

#[note("prompt-look")]
pub fn look(recording: bool) -> PromptLook {
    let (dim, white, accent) = ([0.42, 0.42, 0.43, 1.0], [1.0; 4], [0.0, 0.48, 0.8, 1.0]);
    if recording {
        let border = [0.22, 0.22, 0.26, 1.0];
        PromptLook {
            text: dim,
            text_focus: dim,
            cursor: [0.0; 4],
            button: border,
            button_hover: border,
            button_text: dim,
            disabled: 1.0,
        }
    } else {
        PromptLook {
            text: [0.91, 0.91, 0.92, 1.0],
            text_focus: white,
            cursor: accent,
            button: accent,
            button_hover: [0.12, 0.54, 0.82, 1.0],
            button_text: white,
            disabled: 0.0,
        }
    }
}

#[cfg(test)]
#[path = "test/prompt/test.rs"]
mod tests;
