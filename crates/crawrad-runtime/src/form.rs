//! Base form infrastructure inspired by the classic RAD `TForm`.
//!
//! Provides the foundational window and form lifecycle properties:
//! title, dimensions, modal state, key preview, and active control focus.

use crawrad_component::pieces::ModalResult;
use crawrad_ir::{Document, Rect, Value};
use crawrad_note::note;

#[note("form-formbase")]
#[derive(Debug, Clone, PartialEq)]
pub struct FormBase {
    pub title: String,
    pub width: f32,
    pub height: f32,
    pub key_preview: bool,
    pub modal_result: ModalResult,
    pub active_control: Option<String>,
}

impl Default for FormBase {
    fn default() -> Self {
        Self {
            title: "Form".to_string(),
            width: 640.0,
            height: 480.0,
            key_preview: false,
            modal_result: ModalResult::None,
            active_control: None,
        }
    }
}

impl FormBase {
    #[note("form-formbase-new")]
    pub fn new(title: impl Into<String>, width: f32, height: f32) -> Self {
        Self {
            title: title.into(),
            width,
            height,
            ..Default::default()
        }
    }

    #[note("form-formbase-from-doc")]
    pub fn from_doc(doc: &Document) -> Self {
        let Some(root) = doc.node(&doc.root) else {
            return Self::default();
        };
        let title = match root.props.get("text") {
            Some(Value::Str(s)) => s.clone(),
            _ => root.name().to_string(),
        };
        let rect = root.rect().unwrap_or(Rect::new(0.0, 0.0, 640.0, 480.0));
        let key_preview = match root.props.get("key_preview") {
            Some(Value::Bool(b)) => *b,
            _ => false,
        };
        Self {
            title,
            width: rect.w,
            height: rect.h,
            key_preview,
            modal_result: ModalResult::None,
            active_control: None,
        }
    }

    #[note("form-formbase-close")]
    pub fn close(&mut self) {
        if self.modal_result == ModalResult::None {
            self.modal_result = ModalResult::Cancel;
        }
    }

    #[note("form-formbase-close-with")]
    pub fn close_with(&mut self, result: ModalResult) {
        self.modal_result = result;
    }
}

#[note("form-form")]
pub trait Form {
    #[note("form-form-base")]
    fn base(&self) -> &FormBase;

    #[note("form-form-base-mut")]
    fn base_mut(&mut self) -> &mut FormBase;

    #[note("form-form-title")]
    fn title(&self) -> &str {
        &self.base().title
    }

    #[note("form-form-width")]
    fn width(&self) -> f32 {
        self.base().width
    }

    #[note("form-form-height")]
    fn height(&self) -> f32 {
        self.base().height
    }

    #[note("form-form-modal-result")]
    fn modal_result(&self) -> ModalResult {
        self.base().modal_result
    }

    #[note("form-form-close")]
    fn close(&mut self) {
        self.base_mut().close();
    }
}

impl Form for FormBase {
    fn base(&self) -> &FormBase {
        self
    }
    fn base_mut(&mut self) -> &mut FormBase {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FormTest {
        base: FormBase,
    }

    impl Form for FormTest {
        fn base(&self) -> &FormBase {
            &self.base
        }
        fn base_mut(&mut self) -> &mut FormBase {
            &mut self.base
        }
    }

    #[test]
    fn form_base_defaults_and_lifecycle() {
        let mut f = FormTest {
            base: FormBase::new("MainForm", 800.0, 600.0),
        };
        assert_eq!(f.title(), "MainForm");
        assert_eq!(f.width(), 800.0);
        assert_eq!(f.height(), 600.0);
        assert_eq!(f.modal_result(), ModalResult::None);

        f.close();
        assert_eq!(f.modal_result(), ModalResult::Cancel);

        f.base_mut().close_with(ModalResult::Ok);
        assert_eq!(f.modal_result(), ModalResult::Ok);
    }
}
