//! The line marks the extensions painted, by extension and file, with
//! a revision the window watches to know when to draw them again.

use std::collections::BTreeMap;
use std::path::Path;

use crawrad_extension_api::marks::{LineMark, SetLineMarksParams};
use crawrad_note::note;

type Waker = Box<dyn Fn() + Send>;

#[note("line-marks-linemarkstore")]
#[derive(Default)]
pub struct LineMarkStore {
    /// (extension id, file key) → that extension's marks on that file.
    by: BTreeMap<(String, String), Vec<LineMark>>,
    revision: u64,
    /// Wakes the window: the marks arrive on an extension's reader thread,
    /// and an idle window would not look until something else woke it.
    waker: Option<Waker>,
}

#[note("line-marks-file-key")]
pub fn file_key(file: &str) -> String {
    let key = file.replace('\\', "/");
    if cfg!(windows) {
        key.to_lowercase()
    } else {
        key
    }
}

impl LineMarkStore {
    #[note("line-marks-linemarkstore-set-waker")]
    pub fn set_waker(&mut self, wake: impl Fn() + Send + 'static) {
        self.waker = Some(Box::new(wake));
    }

    #[note("line-marks-linemarkstore-wake")]
    pub fn wake(&self) {
        if let Some(wake) = &self.waker {
            wake();
        }
    }

    fn changed(&mut self) {
        self.revision += 1;
        if let Some(wake) = &self.waker {
            wake();
        }
    }

    #[note("line-marks-linemarkstore-set")]
    pub fn set(&mut self, extension: &str, params: SetLineMarksParams) {
        let key = (extension.to_string(), file_key(&params.file));
        if params.marks.is_empty() {
            self.by.remove(&key);
        } else {
            self.by.insert(key, params.marks);
        }
        self.changed();
    }

    #[note("line-marks-linemarkstore-forget")]
    pub fn forget(&mut self, extension: &str) {
        let before = self.by.len();
        self.by.retain(|(ext, _), _| ext != extension);
        if self.by.len() != before {
            self.changed();
        }
    }

    #[note("line-marks-linemarkstore-revision")]
    pub fn revision(&self) -> u64 {
        self.revision
    }

    #[note("line-marks-linemarkstore-on")]
    pub fn on(&self, file: &Path) -> Vec<(String, LineMark)> {
        let key = file_key(&file.to_string_lossy());
        self.by
            .iter()
            .filter(|((_, f), _)| *f == key)
            .flat_map(|((ext, _), marks)| marks.iter().map(move |m| (ext.clone(), m.clone())))
            .collect()
    }
}

#[cfg(test)]
#[path = "test/line_marks/test.rs"]
mod tests;
