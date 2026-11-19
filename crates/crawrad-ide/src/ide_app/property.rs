//! Object Inspector WRITING: from what was typed to the document.
//!
//! Everything goes through the action catalogue (ADR 0005). What lives here is the return
//! path — which node the edit affects, when it commits, and what the screen shows
//! afterward.
//!
//! It is part of `impl IdeApp`, not a new type: the shell remains
//! ONE, and the child module accesses its private fields.

use super::*;
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-property-ideapp-write-property")]
    pub(super) fn write_property(
        &mut self,
        cx: &mut Cx,
        alvos: Vec<NodeId>,
        key: &str,
        valor: Value,
    ) {
        if alvos.is_empty() {
            return;
        }
        if key == "name" {
            let Some(no) = alvos.first().cloned() else {
                return;
            };
            let Some(new_name) = valor.as_str().map(str::to_string) else {
                return;
            };
            self.start_component_rename(cx, no, new_name);
        } else {
            let args = crawrad_actions::Args::texts("nodes", alvos)
                .and("key", Value::Str(key.to_string()))
                .and("value", valor);
            self.execute_with(cx, "component.set_property", args);
        }
        self.redraw_panel_list(cx, Panel::Inspector, id!(inspector_panel.inspector_list));
    }

    #[note("ide-app-property-ideapp-alvos-da-edicao")]
    pub(super) fn alvos_da_edicao(&self, no: &NodeId) -> Vec<NodeId> {
        let sel: Vec<NodeId> = self
            .host
            .doc()
            .map(|d| d.selection().to_vec())
            .unwrap_or_default();
        if sel.contains(no) {
            sel
        } else {
            vec![no.clone()]
        }
    }

    #[note("ide-app-property-ideapp-confirmar-edicao")]
    pub(super) fn confirmar_edicao(&mut self, cx: &mut Cx) {
        let Some(e) = self.edicao.take() else { return };
        let Some(editor) = self.editor_da_linha(e.linha) else {
            return;
        };
        let resultado = editor.gesture(e.slot, inspector::Gesture::Typed(&e.texto));
        self.aplicar_resultado(cx, resultado, Some(&e.no), e.key);
    }

    #[note("ide-app-property-ideapp-editor-da-linha")]
    pub(super) fn editor_da_linha(&self, linha: usize) -> Option<inspector::Editor> {
        match self.linhas_do_inspector().into_iter().nth(linha) {
            Some(inspector::Row::Prop(p)) => Some(p.editor),
            _ => None,
        }
    }

    #[note("ide-app-property-ideapp-aplicar-resultado")]
    pub(super) fn aplicar_resultado(
        &mut self,
        cx: &mut Cx,
        resultado: inspector::Outcome,
        no: Option<&NodeId>,
        key: &str,
    ) {
        match resultado {
            inspector::Outcome::Set(valor) => {
                let alvos = match no {
                    Some(n) => self.alvos_da_edicao(n),
                    None => self
                        .host
                        .doc()
                        .map(|d| d.selection().to_vec())
                        .unwrap_or_default(),
                };
                self.write_property(cx, alvos, key, valor);
            }
            inspector::Outcome::Run(id, args) => {
                self.execute_with(cx, id, args);
                self.refresh(cx);
            }
            inspector::Outcome::Refuse(porque) => {
                self.status = Some(porque);
                self.refresh(cx);
            }
            inspector::Outcome::Ignore => {}
        }
    }

    #[note("ide-app-property-ideapp-confirm-event")]
    pub(super) fn confirm_event(&mut self, cx: &mut Cx) {
        let Some(e) = self.edicao_de_evento.take() else {
            return;
        };
        let ir = self.host.doc().map(|d| d.ir());
        let (action, args) = crawrad_actions::handler_edit(ir, &e.no, e.evento, &e.texto);
        self.execute_with(cx, action, args);
        self.refresh(cx);
    }

    #[note("ide-app-property-ideapp-escrever-linha")]
    pub(super) fn escrever_linha(
        cx: &mut Cx2d,
        item: &WidgetRef,
        novo: bool,
        indice: usize,
        linha: &inspector::Row,
        rascunho: Option<&Edicao>,
        rascunho_evento: Option<&EventEdit>,
        enums: &mut CacheDeOpcoes,
        mark: Option<bool>,
    ) {
        if matches!(linha, inspector::Row::Prop(_) | inspector::Row::Event(_)) {
            let area = item.widget(id!(value)).area();
            let corrente = f64::from(u8::from(area != Area::Empty && cx.has_key_focus(area)));
            let listra = match linha {
                inspector::Row::Prop(p) => f64::from(u8::from(p.striped)),
                _ => 0.0,
            };
            item.apply_over(
                cx,
                live! { draw_bg: { corrente: (corrente), listra: (listra) } },
            );
        }
        let p = match linha {
            inspector::Row::Category { label, open } => {
                item.label(id!(face.marca)).set_text(
                    cx,
                    if *open {
                        inspector::icon::FOLD_OPEN
                    } else {
                        inspector::icon::FOLD_CLOSED
                    },
                );
                item.label(id!(face.nome)).set_text(cx, label);
                return;
            }
            inspector::Row::Notice(t) => {
                item.set_text(cx, t);
                return;
            }
            inspector::Row::InheritedFrom(form) => {
                item.label(id!(form)).set_text(cx, form);
                return;
            }
            inspector::Row::Event(e) => {
                item.label(id!(name)).set_text(cx, e.label);
                if let (Some(base), Some(form)) = (&e.base_handler, &e.base_form) {
                    let calls = format!("calls inherited: {form}::{base}");
                    item.label(id!(calls)).set_text(cx, &calls);
                }
                if e.is_inherited() {
                    item.label(id!(value.handler)).set_text(cx, &e.handler);
                    return;
                }
                let campo = item.text_input(id!(value));
                let alvo = match rascunho_evento {
                    Some(r) if r.evento == e.event => r.texto.clone(),
                    _ => e.handler.clone(),
                };
                if campo.text() != alvo {
                    campo.set_text(cx, &alvo);
                }
                return;
            }
            inspector::Row::Prop(p) => p,
        };
        // A property behind a finding gets a ▲ and the finding's colour;
        // a reused row gets its template's colour back
        let (name_text, name_color) = match (mark, p.depth > 0) {
            (Some(error), _) => (
                format!("{} \u{25b2}", p.label),
                if error { MARK_ERROR } else { MARK_WARNING },
            ),
            (None, true) => (p.label.clone(), SUB_NAME),
            (None, false) => (p.label.clone(), PROP_NAME),
        };
        item.label(id!(name)).set_text(cx, &name_text);
        let recuo = p.depth as f64 * RECUO_SUB;
        item.label(id!(name)).apply_over(
            cx,
            live! {
                padding: { left: (recuo) },
                draw_text: { color: (name_color) },
            },
        );

        let mut clear_shown = false;
        for celula in p.editor.cells() {
            let caminho = slot_path(celula.slot());
            match celula {
                inspector::Cell::Field { slot, text } => {
                    let campo = item.text_input(caminho);
                    let alvo = match rascunho {
                        Some(e) if e.linha == indice && e.slot == slot => e.texto.clone(),
                        _ => text,
                    };
                    if campo.text() != alvo {
                        campo.set_text(cx, &alvo);
                    }
                }
                inspector::Cell::Check { on, .. } => {
                    let caixa = item.check_box(caminho);
                    if caixa.active(cx) != on {
                        caixa.set_active(cx, on);
                    }
                }
                inspector::Cell::Choice {
                    options, current, ..
                } => {
                    let combo = item.drop_down(caminho);
                    if novo || enums.get(&indice) != Some(&options) {
                        combo.set_labels(cx, options.clone());
                        enums.insert(indice, options);
                    }
                    if let Some(i) = current {
                        combo.set_selected_item(cx, i);
                    }
                }
                inspector::Cell::Caption { text, .. } => {
                    item.label(caminho).set_text(cx, &text);
                }
                inspector::Cell::Press { slot, text } => {
                    clear_shown |= slot == inspector::Slot::Clear && !text.is_empty();
                    item.button(caminho).set_text(cx, &text);
                }
                inspector::Cell::Swatch { rgba, .. } => {
                    let cor = vec4(
                        f32::from(rgba[0]) / 255.0,
                        f32::from(rgba[1]) / 255.0,
                        f32::from(rgba[2]) / 255.0,
                        f32::from(rgba[3]) / 255.0,
                    );
                    item.view(caminho)
                        .apply_over(cx, live! { draw_bg: { color: (cor) } });
                }
            }
        }
        let clear = item.widget(slot_path(inspector::Slot::Clear));
        if clear.visible() != clear_shown {
            clear.set_visible(cx, clear_shown);
        }
    }
}

/// The Inspector's name colours, as `vec4` for `apply_over`: a row
/// behind an error or a warning, and the templates' own — `CRAWRAD_ACENTO_TEXTO`
/// (#ff9c6a) and `CRAWRAD_TEXTO_SECUNDARIO` (#b7b7ba) in `theme/tokens.rs`.
const MARK_ERROR: Vec4 = Vec4 {
    x: 0.941,
    y: 0.639,
    z: 0.612,
    w: 1.0,
};
const MARK_WARNING: Vec4 = Vec4 {
    x: 0.902,
    y: 0.769,
    z: 0.431,
    w: 1.0,
};
const PROP_NAME: Vec4 = Vec4 {
    x: 1.0,
    y: 0.612,
    z: 0.416,
    w: 1.0,
};
const SUB_NAME: Vec4 = Vec4 {
    x: 0.718,
    y: 0.718,
    z: 0.729,
    w: 1.0,
};
