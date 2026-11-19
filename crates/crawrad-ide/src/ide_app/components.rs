//! **THE COMPONENT SUITE in the shell** — install, and view what is installed.
//!
//! This is part of `impl IdeApp`: the shell remains SINGLE, and the child module
//! accesses its private fields.
//!
//! # The shell only PROMPTS and DISPLAYS
//!
//! Verification and copying are handled by `crate::instalacao`, which is headless and tested.
//! Here lives the folder picker and the status message — and this division is what allows
//! `component.install` to be called by an agent without a file picker,
//! which is step §10 of
//! ADR 0033.
//!
//! # Refusals now APPEAR, and this is a bugfix
//!
//! The documentation in `instalados.rs` stated, from the beginning, that a refused manifest
//! *"becomes a line that the shell displays"*. **The shell displayed nothing**: the
//! messages were concatenated and discarded. A component disappearing without explanation is
//! the worst thing that gateway can do, and was exactly what happened.

use super::*;
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-components-ideapp-instalar-componente")]
    pub(super) fn instalar_componente(&mut self, cx: &mut Cx, folder: Option<PathBuf>) {
        let pick = || {
            rfd::FileDialog::new()
                .set_title(ESCOLHER_PACOTE)
                .pick_folder()
        };
        let Some(origem) = folder.or_else(pick) else {
            return;
        };
        let veredito = crawrad_project::installation::install_now(&origem);
        self.status = Some(veredito.phrase());
        self.reload_registry(cx);
    }

    #[note("ide-app-components-ideapp-show-installed")]
    pub(super) fn show_installed(&mut self, cx: &mut Cx) {
        self.status = Some(crawrad_project::installed::summary());
        self.refresh(cx);
    }

    #[note("ide-app-components-ideapp-reload-registry")]
    pub(super) fn reload_registry(&mut self, cx: &mut Cx) {
        self.designer.registry = Some(crawrad_project::installed::ide_registry());
        self.refresh(cx);
    }
}

#[note("ide-app-components-escolher-pacote")]
pub(super) const ESCOLHER_PACOTE: &str = "Pick the component package folder";

#[cfg(test)]
#[path = "../test/shell_components/test.rs"]
mod test;
