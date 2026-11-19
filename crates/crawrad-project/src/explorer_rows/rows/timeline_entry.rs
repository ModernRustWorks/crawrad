//! A local history entry in the Timeline section.

use crawrad_actions::Args;
use crawrad_ir::Value;

use super::super::row::{Activation, ExplorerRow};
use crate::explorer::ACTION_OPEN_TIMELINE_ENTRY;
use crawrad_note::note;

#[note("explorer-rows-rows-timeline-entry-timelineentry")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimelineEntry {
    index: usize,
    text: String,
}

impl TimelineEntry {
    #[note("explorer-rows-rows-timeline-entry-timelineentry-new")]
    pub fn new(index: usize, text: impl Into<String>) -> Self {
        Self {
            index,
            text: text.into(),
        }
    }
}

impl ExplorerRow for TimelineEntry {
    fn template(&self) -> &'static str {
        "File"
    }

    fn text(&self) -> &str {
        &self.text
    }

    fn indent(&self) -> usize {
        super::CHILD
    }

    fn activation(&self) -> Activation {
        Activation::Run(
            ACTION_OPEN_TIMELINE_ENTRY,
            Args::one("index", Value::Int(self.index as i64)),
        )
    }
}
