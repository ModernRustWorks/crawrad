//! Global Memory dialog of the AI Assistant (Ref 03: `03_modal_memory.png`).
//!
//! Allows viewing and editing the user's global instructions shared
//! with the chat and AI agents, displaying dynamic character and token counts.

use super::super::*;
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-dialogs-memory-ideapp-open-memory")]
    pub(in super::super) fn open_memory(&mut self, cx: &mut Cx) {
        let path = crawrad_agent::AefosMemory::default_path();
        let mem = crawrad_agent::AefosMemory::load_from_file(&path).unwrap_or_default();
        self.memoria_aberta = true;
        self.memoria_texto = mem.instructions;
        let dlg = self.ui.view(id!(memory_dialog.dlg));
        dlg.text_input(id!(mem_input))
            .set_text(cx, &self.memoria_texto);
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-memory-ideapp-draw-memory")]
    pub(in super::super) fn draw_memory(&mut self, cx: &mut Cx) {
        self.show_veil(cx, id!(memory_dialog), self.memoria_aberta);
        if !self.memoria_aberta {
            return;
        }

        let chars = self.memoria_texto.chars().count();
        let tokens = chars.div_ceil(4);
        let contador = format!("{chars} characters · {tokens} tokens");

        let dlg = self.ui.view(id!(memory_dialog.dlg));
        let input = dlg.text_input(id!(mem_input));
        if input.text() != self.memoria_texto {
            input.set_text(cx, &self.memoria_texto);
        }
        dlg.label(id!(mem_counter)).set_text(cx, &contador);
        dlg.button(id!(mem_save)).set_text(cx, "Save");
    }

    #[note("ide-app-dialogs-memory-ideapp-atender-memory")]
    pub(in super::super) fn atender_memory(&mut self, cx: &mut Cx, actions: &Actions) {
        if !self.memoria_aberta {
            return;
        }
        let dlg = self.ui.view(id!(memory_dialog.dlg));
        let mem_inp = dlg.text_input(id!(mem_input));

        if let Some(txt) = mem_inp.changed(actions) {
            self.memoria_texto = txt;
            self.draw_memory(cx);
            self.ui.redraw(cx);
        }

        if dlg.button(id!(mem_cancel)).clicked(actions) {
            self.memoria_aberta = false;
            self.refresh(cx);
        } else if dlg.button(id!(mem_save)).clicked(actions) {
            let input_text = mem_inp.text();
            self.memoria_texto = input_text.clone();
            let mem = crawrad_agent::AefosMemory::new(&input_text);
            if let Err(e) = mem.save() {
                self.display_chat_response(cx, &format!("🔴 Failed to save global memory: {e}"));
            } else {
                self.memoria_aberta = false;
                self.display_chat_response(cx, "🧠 Global memory instructions saved successfully.");
                self.sincronizar(cx);
            }
            self.ui.redraw(cx);
        }
    }
}
