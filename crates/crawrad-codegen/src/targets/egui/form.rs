//! Assembling the `<form>.designer.rs` file for egui / eframe.

use super::componentes::{ContextoEmissao, ResultadoComponente, emitir_componente, form_strip};
use super::desenho::trecho;
use super::layout::{ancoras, desenha_por_si, pai_de};
use crate::{Erro, identificador, numero};
use crawrad_ir::{Document, Node};
use crawrad_note::note;
use std::collections::BTreeSet;

pub fn ui_rs(
    doc: &Document,
    reg: &crawrad_registry::Registry,
    raiz: &Node,
    tipo: &str,
    emitidos: &[&Node],
) -> Result<String, Erro> {
    ui_rs_with_base(doc, reg, raiz, tipo, emitidos, &[], false)
}

#[note("targets-egui-form-reported-events")]
pub(crate) fn reported_events(
    reg: &crawrad_registry::Registry,
    base: &Document,
    above: &[Document],
) -> Result<Vec<crate::events::WiredEvent>, Erro> {
    let root = base.node(&base.root).ok_or(Erro::SemRaiz)?;
    let form_name = identificador(root.name())?;
    let type_name = crate::tipo_do_form(&form_name);
    let emitted: Vec<&Node> = base
        .in_paint_order()
        .iter()
        .filter_map(|id| base.node(id))
        .filter(|n| super::layout::emitido_no(base, n))
        .collect();
    let emitted: Vec<&Node> = match crate::resolve_bases(above) {
        Some(top) => emitted
            .into_iter()
            .filter(|n| !base.is_inherited(&n.id, &top))
            .collect(),
        None => emitted,
    };
    let (_, reported) = ui_rs_inner(base, reg, root, &type_name, &emitted, above, true)?;
    Ok(reported)
}

#[note("targets-egui-form-ui-rs-with-base")]
pub(crate) fn ui_rs_with_base(
    doc: &Document,
    reg: &crawrad_registry::Registry,
    raiz: &Node,
    tipo: &str,
    emitidos: &[&Node],
    bases: &[Document],
    is_base: bool,
) -> Result<String, Erro> {
    Ok(ui_rs_inner(doc, reg, raiz, tipo, emitidos, bases, is_base)?.0)
}

fn ui_rs_inner(
    doc: &Document,
    reg: &crawrad_registry::Registry,
    raiz: &Node,
    tipo: &str,
    emitidos: &[&Node],
    bases: &[Document],
    is_base: bool,
) -> Result<(String, Vec<crate::events::WiredEvent>), Erro> {
    let mut campos = String::new();
    let mut padroes = String::new();
    let mut constantes = String::new();
    let mut corpo = String::new();
    // the MainMenu's bar goes last, over every control, as the
    // designer and the Makepad target draw it
    let mut menu_bars = String::new();
    let mut precisa_item = false;
    let mut precisa_timer = false;
    let mut precisa_ancora = false;
    let mut precisa_ui = false;
    let mut compartilhado = super::componentes::Compartilhado::default();
    let mut usados: BTreeSet<&'static str> = BTreeSet::new();

    let radios: Vec<(crawrad_ir::NodeId, String)> = emitidos
        .iter()
        .filter(|n| n.class == "RadioButton")
        .map(|n| Ok((pai_de(doc, n), identificador(n.name())?)))
        .collect::<Result<_, Erro>>()?;

    let r = raiz
        .rect()
        .unwrap_or(crawrad_ir::Rect::new(0.0, 0.0, 640.0, 420.0));

    // THE FORM'S OWN EVENTS, and they are 21 of the 22 that the `Form`
    // class provides — the reason, and the only one left out, are written
    // out in full in `form_events`.
    let mut form_events_out = super::form_events::form_events(raiz)?;
    if emitidos.iter().any(|n| super::control_keys::wants(n)) {
        let extra = super::form_events::control_key_items(&form_events_out);
        form_events_out.items.push_str(&extra);
    }
    // the controls' pointer, focus and keyboard events the widgets do not
    // report themselves, routed after the components (`control_events`)
    let drawn: Vec<&Node> = emitidos
        .iter()
        .filter(|n| desenha_por_si(reg, &n.class) && !form_strip(reg.effective_class(&n.class)))
        .copied()
        .collect();
    let control_events = super::control_events::plan(reg, &drawn, |n| {
        let name = identificador(n.name())?;
        let ctx = ContextoEmissao {
            reg,
            doc,
            node: n,
            nome: &name,
            radios: &radios,
        };
        Ok(emitir_componente(&ctx)?.corpo)
    })?;
    if let Some(ev) = &control_events {
        let (after, button, modifiers, key) = ev.after();
        let conversions =
            super::form_events::conversion_items(&form_events_out, button, modifiers, key);
        form_events_out.items.push_str(&ev.items());
        form_events_out.items.push_str(&conversions);
        form_events_out.before.push_str(&ev.before());
        form_events_out.after = after + &form_events_out.after;
        let (field, default) = ev.field();
        form_events_out.field.push_str(field);
        form_events_out.default.push_str(default);
    }
    // Tab walks the controls in TabOrder, not in drawing order
    let tab_order = super::tab_order::plan(reg, doc, &drawn);
    if let Some(t) = &tab_order {
        form_events_out.items.push_str(&t.items());
        form_events_out.before.push_str(t.before());
        form_events_out.after.push_str(t.after());
        let (field, default) = t.field();
        form_events_out.field.push_str(field);
        form_events_out.default.push_str(default);
    }
    if !form_events_out.empty() {
        precisa_ui = true;
    }

    let mut emitidos_ordenados = emitidos.to_vec();
    emitidos_ordenados.sort_by_key(|n| {
        if n.class == "Badge"
            && n.props.get("associate").is_some_and(|v| match v {
                crawrad_ir::Value::Ref(s) | crawrad_ir::Value::Str(s) => !s.is_empty(),
                _ => false,
            })
        {
            1
        } else {
            0
        }
    });

    for n in &emitidos_ordenados {
        // **THE CHILD OF A WRAPPER DOES NOT GO HERE**: it goes inside the parent's
        // block, right below. Emitting it here would also draw it twice — and the
        // second time, as a sibling, over the box.
        if envolvido_por_conteiner(doc, &n.id) {
            continue;
        }
        let nome = identificador(n.name())?;
        let rect = doc
            .rect_in_form(&n.id)
            .unwrap_or(crawrad_ir::Rect::new(0.0, 0.0, 80.0, 24.0));

        let guarda = super::layout::visibility_guard(reg, doc, &n.id)?;
        corpo.push_str(&guarda.abre);
        // what a Splitter's drag adds to this control's literal box
        let split_terms = crate::targets::splitters::terms(doc, n, "self.");

        if desenha_por_si(reg, &n.class) {
            precisa_ancora = true;
            precisa_ui = true;
            let is_associated = doc.nodes.values().any(|other| {
                other.class == "Badge"
                    && other.props.get("associate").is_some_and(|v| match v {
                        crawrad_ir::Value::Ref(s) | crawrad_ir::Value::Str(s) => {
                            s == &n.id || s == n.name()
                        }
                        _ => false,
                    })
            });
            let r_decl = if is_associated {
                format!(
                    "let r_{nome} = anchor(origin, egui::vec2({}{}, {}{}), egui::vec2({}{}, {}{}), {}, grown);\n        let r = r_{nome};",
                    numero(rect.x),
                    split_terms[0],
                    numero(rect.y),
                    split_terms[1],
                    numero(rect.w),
                    split_terms[2],
                    numero(rect.h),
                    split_terms[3],
                    ancoras(n)
                )
            } else {
                format!(
                    "let r = anchor(origin, egui::vec2({}{}, {}{}), egui::vec2({}{}, {}{}), {}, grown);",
                    numero(rect.x),
                    split_terms[0],
                    numero(rect.y),
                    split_terms[1],
                    numero(rect.w),
                    split_terms[2],
                    numero(rect.h),
                    split_terms[3],
                    ancoras(n)
                )
            };
            corpo.push_str(&format!(
                "\n        // {} — {}\n        {r_decl}\n",
                nome, n.class,
            ));
        } else {
            corpo.push_str(&format!("\n        // {} — {}\n", nome, n.class));
        }

        let ctx = ContextoEmissao {
            reg,
            doc,
            node: n,
            nome: &nome,
            radios: &radios,
        };

        let res = emitir_componente(&ctx)?;
        compartilhado = compartilhado.com(res.compartilhado);
        usados.insert(res.tipo_usado);
        campos.push_str(&res.campo);
        padroes.push_str(&res.padrao);
        // a constant several controls share (the standard colours) goes
        // out once
        if !constantes.contains(&res.constante) {
            constantes.push_str(&res.constante);
        }
        corpo.push_str(&look_abre(reg, n, &nome));
        if form_strip(reg.effective_class(&n.class)) {
            menu_bars.push_str(&res.corpo);
        } else {
            corpo.push_str(&res.corpo);
            if let Some(ev) = &control_events {
                corpo.push_str(&ev.rect_line(n, &nome));
            }
            if let Some(t) = &tab_order {
                corpo.push_str(&t.stop_line(n, &nome));
                corpo.push_str(&t.link_line(reg, n, &nome));
            }
        }
        corpo.push_str(&tooltip_de(reg, n, &nome));
        corpo.push_str(&cursor_de(reg, n, &nome));
        corpo.push_str(&over_de(reg, doc, n, &nome));
        corpo.push_str(&popup_de(reg, doc, n, &nome)?);
        if res.precisa_timer {
            precisa_timer = true;
            precisa_ui = true;
        }
        if res.precisa_item {
            precisa_item = true;
        }

        // THE CHILDREN, INSIDE the block that this component opened.
        if super::componentes::envolve_filhos(reg.effective_class(&n.class)) {
            let dentro = filhos_desenhaveis(doc, n, emitidos);
            // **THE ORIGIN IS ONLY CREATED IF SOMEONE USES IT.** An empty
            // `ScrollBox` with these two lines causes `unused_variable` in the
            // user's project — and they cannot fix it because `.designer.rs` is
            // rewritten on the next generation. The `gate_sem_avisos` gate caught
            // exactly this, in the "one of each class" case.
            //
            // They live here, rather than in the container emitter, because this
            // loop is what knows whether there is a DRAWABLE child: the emitted
            // list has already filtered out what does not go to the screen.
            corpo.push_str(if dentro.is_empty() {
                // Without children, the closure's `ui` is unused by anyone — and
                // an unused closure parameter is a warning in the user's
                // project, just like the variable.
                SEM_FILHOS
            } else {
                PROLOGO_DOS_FILHOS
            });
            for f in dentro {
                let res_f = emitir_filho(
                    doc,
                    reg,
                    f,
                    &radios,
                    control_events.as_ref(),
                    tab_order.as_ref(),
                )?;
                compartilhado = compartilhado.com(res_f.compartilhado);
                usados.insert(res_f.tipo_usado);
                campos.push_str(&res_f.campo);
                padroes.push_str(&res_f.padrao);
                constantes.push_str(&res_f.constante);
                corpo.push_str(&res_f.corpo);
                precisa_item |= res_f.precisa_item;
                if res_f.precisa_timer {
                    precisa_timer = true;
                    precisa_ui = true;
                }
            }
            corpo.push_str(&super::componentes::corpo_fecha(&ctx)?);
        }

        corpo.push_str(&look_fecha(reg, n, &nome));
        corpo.push_str(&guarda.fecha);
    }
    corpo.push_str(&menu_bars);

    // AFTER the components, not before: the form's event state field is
    // not a component at all, and placing it at the end of the struct keeps
    // the order of fields matching the order the user drew them.
    // the form's font size and colour, for ParentFont / ParentColor
    constantes.push_str(&crate::targets::shared::look_consts(doc, reg, emitidos));
    campos.push_str(&form_events_out.field);
    padroes.push_str(&form_events_out.default);

    let bound_controls = crate::targets::data_binding::collect_bound_controls(doc, emitidos)?;
    let dataset_actions = crate::targets::data_binding::collect_dataset_actions(doc, emitidos)?;
    let (db_campos, db_padroes) =
        crate::targets::data_binding::egui_fields_all(&bound_controls, &dataset_actions);
    campos.push_str(&db_campos);
    padroes.push_str(&db_padroes);
    let dataset_grids = crate::targets::data_binding::dataset_grids(
        doc,
        emitidos,
        &crate::targets::data_binding::active_datasets(&bound_controls, &dataset_actions),
    )?;
    campos.push_str(&crate::targets::data_binding::dataset_grid_fields(
        &dataset_grids,
        "",
    ));
    for g in &dataset_grids {
        padroes.push_str(&format!(
            "            {n}_ds_seen: None,\n            {n}_rec_seen: None,\n",
            n = g.grid_name
        ));
    }
    if !bound_controls.is_empty() || !dataset_actions.is_empty() {
        precisa_ui = true;
    }

    let tipo_item = if precisa_item { ITEM_E_ARVORE } else { "" };
    let temporizador = if precisa_timer {
        trecho("temporizador")
    } else {
        String::new()
    };
    // The piece comes from the box; the egui wrapper stays here. See `PECA_ANCHORS`.
    let ancoragem = if precisa_ancora {
        format!("{}\n{ANCORAGEM_EGUI}", PECA_ANCHORS.replace("\r\n", "\n"))
    } else {
        String::new()
    };
    let medidas = if precisa_ancora {
        "        let origin = ui.max_rect().min;\n\
         \x20       // How much the form grew since design size. This is what the\n\
         \x20       // anchors spend.\n\
         \x20       let grown = ui.max_rect().size() - egui::vec2(Self::SIZE[0], Self::SIZE[1]);\n"
    } else {
        ""
    };
    // THE SCROLL BAR IS VISIBLE AT REST, and this is the bug the owner noted:
    //
    // > *"the bar only appears if I move the mouse over it. Imagine a user: glancing
    // > at it, not seeing the scrollbar, how will they know there is something below?"*
    //
    // egui's `ScrollStyle::solid()` starts with `dormant_handle_opacity: 0.0`
    // (`egui-0.36.2/src/style.rs:610`) — the bar EXISTS and is invisible as long
    // as no one touches it. In a classic RAD IDE it is there, solid, the entire time there is
    // content to scroll; that is how the user knows there is content beyond the window.
    let rolagem = if usados.iter().any(|c| super::componentes::rola(c)) {
        precisa_ui = true;
        TEMA_DA_BARRA
    } else {
        ""
    };
    if usados.contains("MessageDialog") {
        precisa_ui = true;
    }
    let own_wired_events = crate::events::collect_wired_events(reg, raiz, emitidos)?;
    let has_wired = !own_wired_events.is_empty();

    let base_wired_events = match bases.split_first() {
        Some((b, above)) => reported_events(reg, b, above)?,
        None => Vec::new(),
    };
    let base_has_wired = !base_wired_events.is_empty();

    let ui_sem_uso = if precisa_ui || base_has_wired || is_base {
        ""
    } else {
        "        // Nothing on this form draws anything yet — drop a component\n\
         \x20       // on it in the designer and this fills in.\n\
         \x20       let _ = ui;\n"
    };
    let (base_campo, base_padrao, base_show, base_deref, base_use) = match doc.inherits() {
        Some(base_name) => {
            let base_id = identificador(base_name)?;
            let base_type = crate::tipo_do_form(&base_id);
            let show_call = if base_has_wired {
                crate::events::base_dispatch(
                    doc,
                    &base_type,
                    &base_wired_events,
                    "self",
                    "self.base",
                    "self.base.show_reporting(ui)",
                )?
            } else {
                "        self.base.show(ui);\n".to_string()
            };
            // the `Fired` type is named by an override's match arm, and by
            // the `Base` variant of a base that reports its base's events up
            let base_use = if crate::events::overrides_base(doc, &base_wired_events)
                || (is_base && base_has_wired)
            {
                format!("use crate::{base_id}_designer::{{{base_type}, {base_type}Fired}};\n")
            } else {
                format!("use crate::{base_id}_designer::{base_type};\n")
            };
            (
                format!("    pub base: {base_type},\n"),
                format!("            base: {base_type}::default(),\n"),
                show_call,
                format!(
                    "\nimpl std::ops::Deref for {tipo} {{\n\
                     \x20   type Target = {base_type};\n\
                     \x20   fn deref(&self) -> &Self::Target {{\n\
                     \x20       &self.base\n\
                     \x20   }}\n\
                     }}\n\
                     \nimpl std::ops::DerefMut for {tipo} {{\n\
                     \x20   fn deref_mut(&mut self) -> &mut Self::Target {{\n\
                     \x20       &mut self.base\n\
                     \x20   }}\n\
                     }}\n"
                ),
                base_use,
            )
        }
        None => (
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        ),
    };
    // a derived form's base is `Base::default()` too: with nothing of its own
    // the impl would be the derivable one clippy refuses
    let (derive_padrao, impl_padrao) = if padroes.is_empty() {
        ("#[derive(Default)]\n", String::new())
    } else {
        (
            "",
            format!(
                "\nimpl Default for {tipo} {{\n\
                 \x20   fn default() -> Self {{\n\
                 \x20       Self {{\n\
                 {base_padrao}{padroes}\x20       }}\n\
                 \x20   }}\n\
                 }}\n"
            ),
        )
    };
    let abas = if usados.contains("PageControl") {
        trecho("abas")
    } else {
        String::new()
    };
    // the designer's own bevel lines, card and category header
    let bevel = format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        if usados.contains("Bevel") {
            trecho("bevel")
        } else {
            String::new()
        },
        if usados.contains("Badge") {
            trecho("badge")
        } else {
            String::new()
        },
        if usados.contains("Card") {
            trecho("card")
        } else {
            String::new()
        },
        if usados.contains("Drawer") {
            trecho("drawer")
        } else {
            String::new()
        },
        if usados.contains("CategoryPanel") {
            trecho("category")
        } else {
            String::new()
        },
        if usados.contains("Chips") {
            trecho("chips")
        } else {
            String::new()
        },
        if usados.contains("Breadcrumb") {
            trecho("breadcrumb")
        } else {
            String::new()
        },
        if usados.contains("Pagination") {
            trecho("pagination")
        } else {
            String::new()
        },
        if usados.contains("MultiSelect") {
            trecho("multi_select")
        } else {
            String::new()
        },
        if usados.contains("CheckListBox") {
            trecho("check_list_box")
        } else {
            String::new()
        },
        if usados.contains("ListView") {
            trecho("list_view")
        } else {
            String::new()
        },
        if usados.contains("MessageDialog") {
            trecho("message_dialog")
        } else {
            String::new()
        },
        // the bar the designer draws, and not an `egui::Slider`
        if usados.contains("ScrollBar") {
            trecho("scroll_bar")
        } else {
            String::new()
        },
        // the vertical bar and the bar in blocks
        if usados.contains("ProgressBar") {
            trecho("progress")
        } else {
            String::new()
        },
        // the edit's CharCase
        if usados.contains("Edit") || usados.contains("MaskEdit") {
            trecho("char_case")
        } else {
            String::new()
        },
        // the MaskEdit's mask, on every change; DateTimePicker
        if usados.contains("MaskEdit") || usados.contains("DateTimePicker") {
            trecho("mask_edit")
        } else {
            String::new()
        },
    );
    // **THE DECLARED DRAWING PIECES.** Each one is added only if the body calls it:
    // a function nobody calls is `dead_code` in a file rewritten by the next
    // generation, and warning there warns someone who cannot fix it.
    let declarado = format!(
        "{}{}{}",
        if compartilhado.fracao {
            super::desenho::trecho_da_geometria("fraction")
        } else {
            String::new()
        },
        if compartilhado.tracejado {
            trecho("declarado_tracejado")
        } else {
            String::new()
        },
        if compartilhado.marca {
            trecho("declarado_marca")
        } else {
            String::new()
        },
    );
    // the RadioGroup draws the same frame
    let moldura = if usados.contains("GroupBox") || usados.contains("RadioGroup") {
        MOLDURA_GROUPBOX
    } else {
        ""
    };
    let tipo_strings = if usados.contains("ListBox")
        || usados.contains("ComboBox")
        || usados.contains("Memo")
        || usados.contains("RadioGroup")
        || usados.contains("Chips")
        || usados.contains("Breadcrumb")
        || usados.contains("MultiSelect")
        || usados.contains("CheckListBox")
    {
        std::borrow::Cow::Owned(TIPO_STRINGS.replace("\r\n", "\n"))
    } else {
        std::borrow::Cow::Borrowed("")
    };
    // **THE BOX PIECES this form needs**, and the question is that of the
    // DESCRIPTOR: who declares a property of this type? The line above
    // asks by CLASS NAME, which is the wrong question — no third-party
    // component enters a list written here, and it is precisely the one that can
    // declare a `kind` that no native component uses.
    let mut pecas = crate::targets::compartilhado::texto_das_pecas(doc, reg);
    let pecas_set = crate::targets::compartilhado::pecas_necessarias(doc, reg);
    // a class declaring a `Shortcut` (an Action, a MenuItem — or a third
    // party's) needs the piece; egui has no `FormBase` to lend it
    if pecas_set.contains("shortcut") {
        pecas.push_str(crate::targets::compartilhado::PECA_SHORTCUT);
        pecas.push('\n');
        pecas = crate::targets::compartilhado::unique_uses(&pecas);
    }
    let mut excl = Vec::new();
    if pecas_set.contains("shortcut") {
        excl.push("Key");
    }
    // Only the types named by THIS form's handlers: a type nobody
    // cites is `dead_code` in the generated binary, and the designer is rewritten
    // whole — warning there warns someone who cannot fix it. The measured reason
    // is in `crate::events`.
    let tipos_eventos = crate::events::payload_types_excluding(
        reg,
        &crate::events::wired_handlers(raiz, emitidos)?,
        &excl,
    );
    if corpo.contains("cursor_icon_of(") {
        pecas.push_str(CURSOR_ICON_FN);
    }
    let cursor_do_form = form_cursor(raiz);
    let (hint_abre, hint_do_form) = form_hint(raiz, &corpo);
    let estados: String = usados.iter().map(|t| estado(t, reg)).collect();
    let arquivo = format!("{}.rs", raiz.name());

    let mut antes_dos_componentes = form_events_out.before;
    antes_dos_componentes.push_str(&crate::targets::data_binding::egui_before_components_all(
        &bound_controls,
        &dataset_actions,
    ));
    antes_dos_componentes.push_str(&crate::targets::data_binding::egui_dataset_grid_follow(
        &dataset_grids,
    ));
    let mut depois_dos_componentes = form_events_out.after;
    depois_dos_componentes.push_str(&crate::targets::data_binding::egui_after_components(
        &bound_controls,
    ));

    let fundo = match raiz.props.get("color") {
        Some(crawrad_ir::Value::Color(c)) if c.a > 0 => format!(
            "        ui.painter().rect_filled(ui.max_rect(), 0.0, egui::Color32::from_rgba_unmultiplied({}, {}, {}, {}));\n",
            c.r, c.g, c.b, c.a
        ),
        _ => String::new(),
    };

    let (fired_enum, metodos_show, dispatch_metodo, reported) = if is_base
        && (has_wired || base_has_wired)
    {
        let mut rewritten = Vec::new();
        let mut used = std::collections::BTreeSet::new();
        // the three pieces of `show` that hold handler calls, in drawing order
        let mut report = |code: &str| {
            crate::events::rewrite_statement_calls(
                code,
                tipo,
                &own_wired_events,
                "self.",
                reg,
                &mut used,
                &mut rewritten,
            )
        };
        let antes_reporting = report(&antes_dos_componentes);
        let body_reporting = report(&corpo);
        let depois_reporting = report(&depois_dos_componentes);
        if !rewritten.is_empty() || base_has_wired {
            // a base that inherits reports its base's events up too,
            // wrapped, so a form further down can still override them
            let base_reporting = match base_has_wired {
                true => format!(
                    "        for fired in self.base.show_reporting(ui) {{\n\
                     \x20           crawrad_fired.push({tipo}Fired::Base(fired));\n\
                     \x20       }}\n"
                ),
                false => base_show.clone(),
            };
            let reporting = format!(
                "    /// Draws the form and returns events that fired during this frame.\n\
                 \x20   pub fn show_reporting(&mut self, ui: &mut egui::Ui) -> Vec<{tipo}Fired> {{\n\
                 \x20       let mut crawrad_fired = Vec::new();\n\
                 {base_reporting}\
                 {fundo}\
                 {rolagem}\
                 {medidas}\
                 {ui_sem_uso}\
                 {antes_reporting}\
                 {hint_abre}\
                 {body_reporting}\
                 {cursor_do_form}\
                 {hint_do_form}\
                 {depois_reporting}\
                 \x20       crawrad_fired\n\
                 \x20   }}\n\
                 \n\
                 \x20   /// Draws the form. Positions are the designer ones, relative to the\n\
                 \x20   /// corner of the `Ui` this is given.\n\
                 \x20   pub fn show(&mut self, ui: &mut egui::Ui) {{\n\
                 \x20       let crawrad_fired = self.show_reporting(ui);\n\
                 \x20       let crawrad_had_fired = !crawrad_fired.is_empty();\n\
                 \x20       for fired in crawrad_fired {{\n\
                 \x20           self.dispatch(fired);\n\
                 \x20       }}\n\
                 \x20       if crawrad_had_fired {{\n\
                 \x20           ui.ctx().request_repaint();\n\
                 \x20       }}\n\
                 \x20   }}\n"
            );
            let (base_variant, base_arm, nested) = match doc.inherits() {
                Some(b) if base_has_wired => {
                    let base_id = identificador(b)?;
                    let base_type = crate::tipo_do_form(&base_id);
                    let path = format!("crate::{base_id}_designer::{base_type}");
                    (
                        Some(base_type.clone()),
                        crate::events::base_arm(doc, tipo, &base_type, &base_wired_events)?,
                        crate::events::nest_base_events(doc, &path, &base_wired_events),
                    )
                }
                _ => (None, String::new(), Vec::new()),
            };
            (
                crate::events::emit_fired_enum(tipo, &rewritten, base_variant.as_deref()),
                reporting,
                crate::events::emit_dispatch(tipo, &rewritten, &base_arm),
                rewritten.into_iter().chain(nested).collect(),
            )
        } else {
            let show = format!(
                "    /// Draws the form. Positions are the designer ones, relative to the\n\
                 \x20   /// corner of the `Ui` this is given.\n\
                 \x20   pub fn show(&mut self, ui: &mut egui::Ui) {{\n\
                 {base_show}\
                 {fundo}\
                 {rolagem}\
                 {medidas}\
                 {ui_sem_uso}\
                 {antes_dos_componentes}\
                 {hint_abre}\
                 {corpo}\
                 {cursor_do_form}\
                 {hint_do_form}\
                 {depois_dos_componentes}\
                 \x20   }}\n"
            );
            (String::new(), show, String::new(), Vec::new())
        }
    } else {
        let show = format!(
            "    /// Draws the form. Positions are the designer ones, relative to the\n\
             \x20   /// corner of the `Ui` this is given.\n\
             \x20   pub fn show(&mut self, ui: &mut egui::Ui) {{\n\
             {base_show}\
             {fundo}\
             {rolagem}\
             {medidas}\
             {ui_sem_uso}\
             {antes_dos_componentes}\
             {hint_abre}\
             {corpo}\
             {cursor_do_form}\
             {hint_do_form}\
             {depois_dos_componentes}\
             \x20   }}\n"
        );
        (String::new(), show, String::new(), Vec::new())
    };

    let apply_language_method = if super::has_locales() {
        let base_doc = crate::resolve_bases(bases);
        let mut lines = String::new();
        if doc.inherits().is_some() {
            lines.push_str("        self.base.apply_language(lang);\n");
        }
        let form_name = identificador(raiz.name())?;
        let node_ids = std::iter::once(doc.root.clone()).chain(doc.in_paint_order());
        for node_id in node_ids {
            let Some(node) = doc.node(&node_id) else {
                continue;
            };
            if node.id == doc.root || node.class == "Form" {
                continue;
            }
            if let Some(ref b) = base_doc
                && doc.is_inherited(&node.id, b)
            {
                continue;
            }
            let Some(desc) = reg.get(&node.class) else {
                continue;
            };
            for &prop in crawrad_ir::TRANSLATABLE {
                if desc.prop(prop).is_none() {
                    continue;
                }
                if let Some(crawrad_ir::Value::Str(text)) = node.props.get(prop)
                    && !text.is_empty()
                {
                    let Ok(node_name) = identificador(node.name()) else {
                        continue;
                    };
                    let fields = crate::targets::egui::state::fields_of(desc);
                    let Some((field_name, _)) = fields.iter().find(|(name, _)| name == prop) else {
                        continue;
                    };
                    let key = crawrad_ir::text_key(&form_name, &node.id, prop);
                    lines.push_str(&format!(
                        "        self.{node_name}.{field_name} = crate::locales::text(lang, {key:?}).unwrap_or({text:?}).to_string();\n"
                    ));
                }
            }
        }
        let param = if lines.is_empty() { "_lang" } else { "lang" };
        format!(
            "    /// Puts this form's texts in `lang`; a text with no translation stays in English.\n\
             \x20   pub fn apply_language(&mut self, {param}: &str) {{\n\
             {lines}\
             \x20   }}\n\
             \n\
             \x20   /// Sets the active language for this form.\n\
             \x20   pub fn set_language(&mut self, lang: &str) {{\n\
             \x20       self.apply_language(lang);\n\
             \x20   }}\n"
        )
    } else {
        String::new()
    };

    let hooks_use = if doc
        .nodes
        .values()
        .any(|n| reg.get(&n.class).and_then(|d| d.inherits) == Some("SpinEdit"))
    {
        "use crawrad_runtime::SpinEditHooks;\n"
    } else {
        ""
    };

    let code = format!(
        "//! GENERATED BY CrawRAD from the form `{}`.\n\
         //!\n\
         //! **DO NOT EDIT THIS FILE** — it is the form you drew, and CrawRAD\n\
         //! rewrites it WHOLE on every generation. Same idea as\n\
         //! `Form1.Designer.cs` in WinForms: the designer code lives apart, and\n\
         //! you do not touch it.\n\
         //!\n\
         //! YOUR code goes in `{arquivo}`, which CrawRAD does not regenerate.\n\
         \n\
         use eframe::egui;\n\
         {hooks_use}\
         {base_use}\
         {estados}\
         {ancoragem}\
         {moldura}\
         {abas}\
         {bevel}\
         {declarado}\
         {tipo_item}\
         {tipo_strings}\
         {pecas}\
         {tipos_eventos}\
         {itens_de_evento}\
         {temporizador}\
         \n\
         {constantes}\
         {fired_enum}\
         {derive_padrao}\
         pub struct {tipo} {{\n\
         {base_campo}{campos}}}\n\
         {impl_padrao}\
         \n\
         impl {tipo} {{\n\
         \x20   pub const SIZE: [f32; 2] = [{w}, {h}];\n\
         \n\
         \x20   /// The form, ready to show: the designer values first, your\n\
         \x20   /// `OnCreate` last.\n\
         \x20   ///\n\
         \x20   /// That order is the contract, and it is the same one a classic\n\
         \x20   /// RAD IDE gives you: `OnCreate` runs AFTER the design-time values are\n\
         \x20   /// in place, so whatever you set in it wins.\n\
         \x20   pub fn new() -> Self {{\n\
         {criacao}\x20   }}\n\
         \n\
{metodos_show}\
{dispatch_metodo}\
{apply_language_method}\
         }}\n\
         {base_deref}",
        raiz.name(),
        criacao = form_events_out.new_body,
        itens_de_evento = form_events_out.items,
        w = numero(r.w),
        h = numero(r.h),
    );
    Ok((
        crate::targets::shared::with_show_hint_const(doc, code),
        reported,
    ))
}

#[note("targets-egui-form-prologo-dos-filhos")]
const PROLOGO_DOS_FILHOS: &str = "\
\x20                               // **THE CHILDREN GO IN HERE**, and that is what makes\n\
\x20                               // this container contain. `origin` is the CONTENT\n\
\x20                               // origin — it moves as the bar scrolls — and the\n\
\x20                               // children are placed relative to it, exactly as they\n\
\x20                               // are stored.\n\
\x20                               //\n\
\x20                               // `grown` is zero: what grows with the window is the\n\
\x20                               // BOX, through its own anchors. A child in here\n\
\x20                               // anchors to the box, and the box already took it\n\
\x20                               // along.\n\
\x20                               let origin = ui.min_rect().min;\n\
\x20                               let grown = egui::Vec2::ZERO;\n";

#[note("targets-egui-form-tema-da-barra")]
const TEMA_DA_BARRA: &str = "\
\x20       // The scroll bar stays VISIBLE while there is something to scroll.\n\
\x20       //\n\
\x20       // egui fades the handle out when nobody is touching it\n\
\x20       // (`ScrollStyle::solid()` starts at `dormant_handle_opacity: 0.0`).\n\
\x20       // That hides the one hint that there is more content below: you have\n\
\x20       // to already know, and move the mouse there, to find out.\n\
\x20       //\n\
\x20       // A classic RAD IDE keeps it on screen, and so does this.\n\
\x20       ui.style_mut().spacing.scroll = egui::style::ScrollStyle {\n\
\x20           dormant_background_opacity: 1.0,\n\
\x20           dormant_handle_opacity: 1.0,\n\
\x20           active_background_opacity: 1.0,\n\
\x20           active_handle_opacity: 1.0,\n\
\x20           ..egui::style::ScrollStyle::solid()\n\
\x20       };\n";

#[note("targets-egui-form-sem-filhos")]
const SEM_FILHOS: &str = "\
\x20                               // Nothing has been dropped in here yet — drop a\n\
\x20                               // component on it in the designer and this fills in.\n\
\x20                               let _ = ui;\n";

#[note("targets-egui-form-envolvido-por-conteiner")]
fn envolvido_por_conteiner(doc: &Document, id: &str) -> bool {
    doc.ancestors(id)
        .iter()
        .filter(|a| **a != doc.root)
        .filter_map(|a| doc.node(a))
        .any(|p| super::componentes::envolve_filhos(&p.class))
}

#[note("targets-egui-form-filhos-desenhaveis")]
fn filhos_desenhaveis<'a>(doc: &Document, n: &Node, emitidos: &[&'a Node]) -> Vec<&'a Node> {
    n.children
        .iter()
        .filter(|c| doc.node(c).is_some())
        .filter_map(|c| emitidos.iter().find(|e| e.id == *c).copied())
        .collect()
}

#[note("targets-egui-form-emitir-filho")]
fn emitir_filho(
    doc: &Document,
    reg: &crawrad_registry::Registry,
    n: &Node,
    radios: &[(crawrad_ir::NodeId, String)],
    control_events: Option<&super::control_events::ControlEvents>,
    tab_order: Option<&super::tab_order::TabOrder>,
) -> Result<ResultadoComponente, Erro> {
    let nome = identificador(n.name())?;
    let rect = n
        .rect()
        .unwrap_or(crawrad_ir::Rect::new(0.0, 0.0, 80.0, 24.0));
    // what a Splitter's drag adds to this control's literal box
    let split_terms = crate::targets::splitters::terms(doc, n, "self.");
    let mut abre = String::new();
    if desenha_por_si(reg, &n.class) {
        let is_associated = doc.nodes.values().any(|other| {
            other.class == "Badge"
                && other.props.get("associate").is_some_and(|v| match v {
                    crawrad_ir::Value::Ref(s) | crawrad_ir::Value::Str(s) => {
                        s == &n.id || s == n.name()
                    }
                    _ => false,
                })
        });
        let r_decl = if is_associated {
            format!(
                "let r_{nome} = anchor(origin, egui::vec2({}{}, {}{}), egui::vec2({}{}, {}{}), {}, grown);\n        let r = r_{nome};",
                numero(rect.x),
                split_terms[0],
                numero(rect.y),
                split_terms[1],
                numero(rect.w),
                split_terms[2],
                numero(rect.h),
                split_terms[3],
                ancoras(n)
            )
        } else {
            format!(
                "let r = anchor(origin, egui::vec2({}{}, {}{}), egui::vec2({}{}, {}{}), {}, grown);",
                numero(rect.x),
                split_terms[0],
                numero(rect.y),
                split_terms[1],
                numero(rect.w),
                split_terms[2],
                numero(rect.h),
                split_terms[3],
                ancoras(n)
            )
        };
        abre.push_str(&format!(
            "\n        // {} — {}\n        {r_decl}\n",
            nome, n.class,
        ));
    } else {
        abre.push_str(&format!("\n        // {} — {}\n", nome, n.class));
    }
    let ctx = ContextoEmissao {
        reg,
        doc,
        node: n,
        nome: &nome,
        radios,
    };
    let mut res = emitir_componente(&ctx)?;
    res.corpo = format!(
        "{abre}{}{}{}{}{}",
        look_abre(reg, n, &nome),
        res.corpo,
        control_events.map_or(String::new(), |ev| ev.rect_line(n, &nome))
            + &tab_order.map_or(String::new(), |t| {
                t.stop_line(n, &nome) + &t.link_line(reg, n, &nome)
            }),
        tooltip_de(reg, n, &nome)
            + &cursor_de(reg, n, &nome)
            + &over_de(reg, doc, n, &nome)
            + &popup_de(reg, doc, n, &nome)?,
        look_fecha(reg, n, &nome)
    );
    Ok(res)
}

#[note("targets-egui-form-has-hint")]
fn has_hint(reg: &crawrad_registry::Registry, class: &str) -> bool {
    class != "TabSheet"
        && reg
            .get(class)
            .is_some_and(|d| d.props.iter().any(|p| p.key == "hint"))
}

#[note("targets-egui-form-has-prop")]
fn has_prop(reg: &crawrad_registry::Registry, class: &str, key: &str) -> bool {
    reg.get(class)
        .is_some_and(|d| d.props.iter().any(|p| p.key == key))
}

#[note("targets-egui-form-look-abre")]
fn look_abre(reg: &crawrad_registry::Registry, n: &Node, nome: &str) -> String {
    let mut s = String::new();
    if has_prop(reg, &n.class, "font") {
        s.push_str(&format!(
            "        let look_saved_{nome} = ui.style().clone();\n\
             \x20       {{\n\
             \x20           let (size, color) = if self.{nome}.parent_font {{ (CRAWRAD_FORM_FONT_SIZE, CRAWRAD_FORM_FONT_COLOR) }} else {{ (self.{nome}.font.size, self.{nome}.font.color) }};\n\
             \x20           if size > 0.0 {{\n\
             \x20               ui.style_mut().override_font_id = Some(egui::FontId::proportional(size));\n\
             \x20           }}\n\
             \x20           if color[3] > 0 {{\n\
             \x20               ui.visuals_mut().override_text_color = Some(egui::Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]));\n\
             \x20           }}\n\
             \x20       }}\n"
        ));
    }
    // ParentColor on paints nothing: whatever is behind (the form, or the
    // panel the control sits on) shows through, as in the classic RAD IDE
    if has_prop(reg, &n.class, "color") && desenha_por_si(reg, &n.class) {
        s.push_str(&format!(
            "        if !self.{nome}.parent_color && self.{nome}.color.a > 0 {{\n\
             \x20           let c = self.{nome}.color;\n\
             \x20           ui.painter().rect_filled(r, 0.0, egui::Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a));\n\
             \x20       }}\n"
        ));
    }
    s
}

#[note("targets-egui-form-look-fecha")]
fn look_fecha(reg: &crawrad_registry::Registry, n: &Node, nome: &str) -> String {
    if has_prop(reg, &n.class, "font") {
        format!("        ui.set_style(look_saved_{nome});\n")
    } else {
        String::new()
    }
}

#[note("targets-egui-form-tooltip-de")]
fn tooltip_de(reg: &crawrad_registry::Registry, n: &Node, nome: &str) -> String {
    if !has_hint(reg, &n.class) || !desenha_por_si(reg, &n.class) {
        return String::new();
    }
    // no hint while a menu is open: it would show over the menu
    format!(
        "        if CRAWRAD_FORM_SHOW_HINT && self.{nome}.visible && self.{nome}.show_hint && !self.{nome}.hint.is_empty() && ui.rect_contains_pointer(r) && !egui::Popup::is_any_open(ui.ctx()) {{\n\
         \x20           egui::Tooltip::always_open(ui.ctx().clone(), ui.layer_id(), egui::Id::new((\"hint\", \"{nome}\")), egui::PopupAnchor::Pointer)\n\
         \x20               .show(|ui| ui.label(self.{nome}.hint.as_str()));\n\
         \x20       }}\n"
    )
}

#[note("targets-egui-form-cursor-de")]
fn cursor_de(reg: &crawrad_registry::Registry, n: &Node, nome: &str) -> String {
    if !has_hint(reg, &n.class) || !desenha_por_si(reg, &n.class) {
        return String::new();
    }
    format!(
        "        if self.{nome}.visible && self.{nome}.enabled && self.{nome}.cursor != Cursor::Default && ui.rect_contains_pointer(r) {{\n\
         \x20           ui.ctx().set_cursor_icon(cursor_icon_of(self.{nome}.cursor));\n\
         \x20       }}\n"
    )
}

#[note("targets-egui-form-cursor-icon-fn")]
const CURSOR_ICON_FN: &str = "
/// The egui pointer of a `Cursor` (#667).
fn cursor_icon_of(cursor: Cursor) -> egui::CursorIcon {
    match cursor {
        Cursor::Default | Cursor::Arrow => egui::CursorIcon::Default,
        Cursor::Hand => egui::CursorIcon::PointingHand,
        Cursor::Text => egui::CursorIcon::Text,
        Cursor::Crosshair => egui::CursorIcon::Crosshair,
        Cursor::Move => egui::CursorIcon::Move,
        Cursor::Wait => egui::CursorIcon::Wait,
        Cursor::NotAllowed => egui::CursorIcon::NotAllowed,
        Cursor::ResizeNs => egui::CursorIcon::ResizeVertical,
        Cursor::ResizeEw => egui::CursorIcon::ResizeHorizontal,
        Cursor::ResizeNesw => egui::CursorIcon::ResizeNeSw,
        Cursor::ResizeNwse => egui::CursorIcon::ResizeNwSe,
    }
}
";

#[note("targets-egui-form-form-cursor")]
fn form_cursor(root: &Node) -> String {
    let icon = match root.props.get("cursor") {
        Some(crawrad_ir::Value::Cursor(c)) => match c {
            crawrad_ir::Cursor::Default | crawrad_ir::Cursor::Arrow => return String::new(),
            crawrad_ir::Cursor::Hand => "PointingHand",
            crawrad_ir::Cursor::Text => "Text",
            crawrad_ir::Cursor::Crosshair => "Crosshair",
            crawrad_ir::Cursor::Move => "Move",
            crawrad_ir::Cursor::Wait => "Wait",
            crawrad_ir::Cursor::NotAllowed => "NotAllowed",
            crawrad_ir::Cursor::ResizeNs => "ResizeVertical",
            crawrad_ir::Cursor::ResizeEw => "ResizeHorizontal",
            crawrad_ir::Cursor::ResizeNesw => "ResizeNeSw",
            crawrad_ir::Cursor::ResizeNwse => "ResizeNwSe",
        },
        _ => return String::new(),
    };
    format!(
        "        // the form's Cursor, where no control showed its own\n\
         \x20       if ui.rect_contains_pointer(ui.max_rect()) && ui.ctx().output(|o| o.cursor_icon) == egui::CursorIcon::Default {{\n\
         \x20           ui.ctx().set_cursor_icon(egui::CursorIcon::{icon});\n\
         \x20       }}\n"
    )
}

#[note("targets-egui-form-over-de")]
fn over_de(reg: &crawrad_registry::Registry, doc: &Document, n: &Node, nome: &str) -> String {
    let wanted = doc
        .node(&doc.root)
        .and_then(crate::targets::shared::form_hint_text)
        .is_some();
    if !wanted || !has_hint(reg, &n.class) || !desenha_por_si(reg, &n.class) {
        return String::new();
    }
    format!("        crawrad_over_control |= self.{nome}.visible && ui.rect_contains_pointer(r);\n")
}

#[note("targets-egui-form-form-hint")]
fn form_hint(root: &Node, corpo: &str) -> (String, String) {
    let Some(hint) = crate::targets::shared::form_hint_text(root) else {
        return (String::new(), String::new());
    };
    let flagged = corpo.contains("crawrad_over_control |=");
    let open = if flagged {
        "        let mut crawrad_over_control = false;\n".to_string()
    } else {
        String::new()
    };
    let over = if flagged {
        "!crawrad_over_control && "
    } else {
        ""
    };
    let close = format!(
        "        // the form's Hint, over its empty body\n\
         \x20       if {over}ui.rect_contains_pointer(ui.max_rect()) && !egui::Popup::is_any_open(ui.ctx()) {{\n\
         \x20           egui::Tooltip::always_open(ui.ctx().clone(), ui.layer_id(), egui::Id::new((\"hint\", {id})), egui::PopupAnchor::Pointer)\n\
         \x20               .show(|ui| ui.label({text}));\n\
         \x20       }}\n",
        id = crate::literal(root.name()),
        text = crate::literal(hint),
    );
    (open, close)
}

#[note("targets-egui-form-popup-de")]
fn popup_de(
    reg: &crawrad_registry::Registry,
    doc: &Document,
    n: &Node,
    nome: &str,
) -> Result<String, crate::Erro> {
    if !has_hint(reg, &n.class) || !desenha_por_si(reg, &n.class) {
        return Ok(String::new());
    }
    super::components::popup_menu::popup_trigger(doc, nome)
}

fn estado(classe: &str, reg: &crawrad_registry::Registry) -> String {
    if classe == "Widget" {
        return ESTADO_WIDGET.to_string();
    }
    let parts = super::components::for_class_name(classe).and_then(|c| c.state_text());
    let (Some(parts), Some(desc)) = (parts, reg.get(classe)) else {
        return String::new();
    };
    format!(
        "{}{}{}",
        parts.head,
        super::state::struct_of(desc),
        parts.tail
    )
}

const MOLDURA_GROUPBOX: &str = r#"
/// Draws a group box frame with a gap for its text.
///
/// No background: the parent shows through, like classic RAD `TGroupBox`. The gap
/// is drawn as two segments rather than by erasing part of a line, because the
/// colour behind the box is not known here — inside a panel it is not the form
/// colour, and the only right answer is to not need the answer.
///
/// `enabled` fades the frame and the text, the way classic RAD greys a disabled
/// `TGroupBox`. It is a parameter and not a read of some global because the
/// box is state, and state is what the form owns.
fn group_box_frame(ui: &egui::Ui, r: egui::Rect, text: &str, enabled: bool) {
    let p = ui.painter();
    let stroke = if enabled {
        ui.visuals().widgets.noninteractive.fg_stroke
    } else {
        ui.visuals().widgets.noninteractive.bg_stroke
    };
    let font = egui::FontId::proportional(12.0);
    let color = if enabled {
        ui.visuals().text_color()
    } else {
        ui.visuals().weak_text_color()
    };
    let gap = if text.is_empty() {
        None
    } else {
        let g = p.layout_no_wrap(text.to_owned(), font.clone(), color);
        Some((r.min.x + 8.0, g.size().x, g.size().y))
    };
    let top = gap.map_or(r.min.y, |(_, _, h)| r.min.y + h / 2.0);
    match gap {
        Some((x, w, _)) => {
            p.line_segment([egui::pos2(r.min.x, top), egui::pos2(x - 4.0, top)], stroke);
            p.line_segment(
                [
                    egui::pos2((x + w + 4.0).min(r.max.x), top),
                    egui::pos2(r.max.x, top),
                ],
                stroke,
            );
        }
        None => {
            p.line_segment([egui::pos2(r.min.x, top), egui::pos2(r.max.x, top)], stroke);
        }
    }
    p.line_segment([egui::pos2(r.min.x, top), egui::pos2(r.min.x, r.max.y)], stroke);
    p.line_segment([egui::pos2(r.max.x, top), egui::pos2(r.max.x, r.max.y)], stroke);
    p.line_segment(
        [egui::pos2(r.min.x, r.max.y), egui::pos2(r.max.x, r.max.y)],
        stroke,
    );
    if let Some((x, _, h)) = gap {
        p.text(
            egui::pos2(x, r.min.y + h / 2.0),
            egui::Align2::LEFT_CENTER,
            text,
            font,
            color,
        );
    }
}
"#;

const ESTADO_WIDGET: &str = r#"
/// A component whose class this target cannot draw yet. Holds what every
/// component has, so the form keeps compiling until the drawing arrives.
#[derive(Clone, Debug)]
pub struct Widget {
    pub visible: bool,
    pub enabled: bool,
    pub hint: String,
    pub show_hint: bool,
    pub popup_menu: String,
    /// The mouse pointer over the control (`Cursor`, #667).
    pub cursor: Cursor,
}
"#;

#[note("targets-egui-form-peca-anchors")]
const PECA_ANCHORS: &str = include_str!("../../../../crawrad-component/src/pieces/anchors.rs");

#[note("targets-egui-form-ancoragem-egui")]
const ANCORAGEM_EGUI: &str = r#"
/// Where a component ends up after the form is resized.
///
/// `origin` is the form's top-left, `pos`/`size` are what the designer stored,
/// and `grown` is how much the form grew since design time.
fn anchor(
    origin: egui::Pos2,
    pos: egui::Vec2,
    size: egui::Vec2,
    a: Anchors,
    grown: egui::Vec2,
) -> egui::Rect {
    let (x, y, w, h) = anchored(pos.x, pos.y, size.x, size.y, a, grown.x, grown.y);
    egui::Rect::from_min_size(
        origin + egui::vec2(x, y),
        egui::vec2(w.max(1.0), h.max(1.0)),
    )
}
"#;

const ITEM_E_ARVORE: &str = r#"
/// Draws the tree. Returns whether the selection changed, and the path of a
/// node with children that was clicked: the caller folds or unfolds it, after
/// telling `OnExpanding` / `OnCollapsing` — classic RAD announces the change
/// before it happens.
///
/// `selection` is the path to the selected node: `[0, 2]` is the third child
/// of the first item. Empty means nothing is selected.
///
/// `name` is what tells two trees on the same form apart — the same reason
/// `page_control` takes one, and the same reason it is not optional. `ui.id()`
/// does NOT change from one sibling to the next: a child `Ui` is born with
/// `parent.id().with("child")`, a FIXED word, so every component on the form
/// answers with the same id. Without the name here, clicking a row of the
/// second tree selects a row of the first.
///
/// **Depth is POSITION, and the marker is a shape of its own.** A row is not
/// one string: the indent is `INDENT` points of x per level and the `−` of a
/// node that has children is painted apart from the text. Folding the three
/// into a single label — `"  - Alpha"` — is what a `TreeView` is not: the text
/// of the item stops being the text of the item, selecting it would copy the
/// drawing, and the designer, which paints the marker apart, would be showing
/// a different picture from the one the app shows.
fn tree(
    ui: &mut egui::Ui,
    name: &str,
    r: egui::Rect,
    items: &[crawrad_runtime::tree::TreeNode],
    selection: &[usize],
    collapsed: &[Vec<usize>],
    fill: Option<egui::Color32>,
) -> (Option<Vec<usize>>, Option<Vec<usize>>) {
    const ROW: f32 = 18.0;
    const INDENT: f32 = 15.0;
    // The `−` of an expanded node, U+2212 — the same glyph the designer paints,
    // and not the ASCII hyphen; a folded node shows `+`.
    const EXPANDED: &str = "\u{2212}";
    const FOLDED: &str = "+";
    let visuals = ui.visuals().clone();
    // `fill`: the tree's own Color (#433); `None` is the theme's
    ui.painter()
        .rect_filled(r, 2.0, fill.unwrap_or(visuals.extreme_bg_color));
    ui.painter().rect_stroke(
        r,
        2.0,
        visuals.widgets.inactive.bg_stroke,
        egui::StrokeKind::Inside,
    );

    // the row the user picked: the form asks OnChanging before taking it
    let mut picked = None;
    let mut toggle = None;
    let mut y = r.min.y + 2.0;
    let mut stack: Vec<(&crawrad_runtime::tree::TreeNode, Vec<usize>)> = items
        .iter()
        .enumerate()
        .map(|(i, it)| (it, vec![i]))
        .rev()
        .collect();

    while let Some((item, path)) = stack.pop() {
        // The row that does not fit whole is not drawn — the component does
        // not paint outside its own box.
        if y + ROW > r.max.y {
            break;
        }
        let level = path.len() - 1;
        let x = r.min.x + 3.0 + level as f32 * INDENT;
        let row = egui::Rect::from_min_size(
            egui::pos2(r.min.x + 1.0, y),
            egui::vec2(r.width() - 2.0, ROW),
        );
        let resp = ui.interact(
            row,
            ui.id().with(("crawrad_row", name, &path)),
            egui::Sense::click(),
        );
        let is_selected = selection == path.as_slice();
        if is_selected {
            ui.painter().rect_filled(row, 2.0, visuals.selection.bg_fill);
        } else if resp.hovered() {
            ui.painter()
                .rect_filled(row, 2.0, visuals.widgets.hovered.bg_fill);
        }
        let folded = collapsed.contains(&path);
        if !item.children.is_empty() {
            ui.painter().text(
                egui::pos2(x + 5.0, y + ROW / 2.0),
                egui::Align2::CENTER_CENTER,
                if folded { FOLDED } else { EXPANDED },
                egui::FontId::proportional(12.0),
                visuals.weak_text_color(),
            );
        }
        let label = if item.symbol.is_empty() {
            item.text.clone()
        } else {
            format!("{} {}", item.symbol, item.text)
        };
        ui.painter().text(
            egui::pos2(x + 14.0, y + ROW / 2.0),
            egui::Align2::LEFT_CENTER,
            &label,
            egui::FontId::proportional(12.5),
            if is_selected {
                visuals.selection.stroke.color
            } else {
                visuals.text_color()
            },
        );
        if resp.clicked() {
            picked = Some(path.clone());
            if !item.children.is_empty() {
                toggle = Some(path.clone());
            }
        }
        y += ROW;
        if folded {
            continue;
        }
        for (i, child) in item.children.iter().enumerate().rev() {
            let mut c = path.clone();
            c.push(i);
            stack.push((child, c));
        }
    }
    (picked, toggle)
}
"#;

#[note("targets-egui-form-tipo-strings")]
const TIPO_STRINGS: &str = include_str!("../../../../crawrad-component/src/pieces/strings.rs");

#[cfg(test)]
#[path = "../../test/egui_cursor/test.rs"]
mod cursor_tests;

#[cfg(test)]
#[path = "../../test/egui_form_hint/test.rs"]
mod form_hint_tests;
