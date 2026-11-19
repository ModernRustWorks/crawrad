//! Formal trait definition for egui component code generation.

use super::{ContextoEmissao, ResultadoComponente};
use crate::Erro;

#[crawrad_note::note("targets-egui-components-codegen-eguicomponentcodegen")]
pub trait EguiComponentCodegen: Send + Sync {
    #[crawrad_note::note("targets-egui-components-codegen-eguicomponentcodegen-class")]
    fn class(&self) -> &'static str;

    #[crawrad_note::note("targets-egui-components-codegen-eguicomponentcodegen-emit")]
    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro>;

    #[crawrad_note::note("targets-egui-components-codegen-eguicomponentcodegen-state-text")]
    fn state_text(&self) -> Option<StateText> {
        None
    }

    #[crawrad_note::note("targets-egui-components-codegen-eguicomponentcodegen-envolve-filhos")]
    fn envolve_filhos(&self) -> bool {
        false
    }

    #[crawrad_note::note("targets-egui-components-codegen-eguicomponentcodegen-rola")]
    fn rola(&self) -> bool {
        false
    }

    #[crawrad_note::note("targets-egui-components-codegen-eguicomponentcodegen-corpo-fecha")]
    fn corpo_fecha(&self, _ctx: &ContextoEmissao) -> Result<String, Erro> {
        Ok(String::new())
    }

    #[crawrad_note::note("targets-egui-components-codegen-eguicomponentcodegen-form-strip")]
    fn form_strip(&self) -> bool {
        false
    }
}

#[crawrad_note::note("targets-egui-components-codegen-statetext")]
pub struct StateText {
    pub head: &'static str,
    pub tail: &'static str,
}
