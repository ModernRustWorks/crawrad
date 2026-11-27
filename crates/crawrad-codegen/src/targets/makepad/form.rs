//! Makepad native `<form>.designer.rs` code generation.

use super::actions::gerar_despacho_acoes;
use super::components;
use super::drawing;
use super::layout::{has_widget, is_inside_container, text, visibility_expression};
use crate::{Erro, identificador};
use crawrad_ir::{Document, Node};
use crawrad_note::note;

pub fn form_designer_rs(
    doc: &Document,
    reg: &crawrad_registry::Registry,
    root: &Node,
    form_type: &str,
    emitted: &[&Node],
) -> Result<String, Erro> {
    form_designer_rs_with_base(doc, reg, root, form_type, emitted, &[], false)
}

#[note("targets-makepad-form-reported-events")]
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
        .filter(|n| {
            let effective = reg.effective_class(&n.class);
            super::CLASSES.contains(&effective) || components::terceiro(reg, &n.class).is_some()
        })
        .collect();
    let emitted: Vec<&Node> = match crate::resolve_bases(above) {
        Some(top) => emitted
            .into_iter()
            .filter(|n| !base.is_inherited(&n.id, &top))
            .collect(),
        None => emitted,
    };
    let (_, reported) = form_designer_rs_inner(base, reg, root, &type_name, &emitted, above, true)?;
    Ok(reported)
}

#[note("targets-makepad-form-form-designer-rs-with-base")]
pub(crate) fn form_designer_rs_with_base(
    doc: &Document,
    reg: &crawrad_registry::Registry,
    root: &Node,
    form_type: &str,
    emitted: &[&Node],
    bases: &[Document],
    is_base: bool,
) -> Result<String, Erro> {
    Ok(form_designer_rs_inner(doc, reg, root, form_type, emitted, bases, is_base)?.0)
}

fn form_designer_rs_inner(
    doc: &Document,
    reg: &crawrad_registry::Registry,
    root: &Node,
    form_type: &str,
    emitted: &[&Node],
    bases: &[Document],
    is_base: bool,
) -> Result<(String, Vec<crate::events::WiredEvent>), Erro> {
    // the whole chain above, folded: what this form inherits
    let base = crate::resolve_bases(bases);
    let base = base.as_ref();
    // THE WINDOW TITLE is the form's `Text`, as in the egui target
    // (`targets/egui/main_app.rs`, `.with_title(…)`), and as in a classic RAD IDE, where
    // `Form1.Caption` IS the title bar.
    //
    // It was left out until 2026-09-17, and the generated application opened saying
    // **"Makepad"** — the framework name, which is the default in
    // `makepad-platform-1.0.0/src/window.rs:110`. Both targets showed the
    // same form with two different titles, and the Makepad one belonged neither to the
    // user nor to the product.
    let titulo = crate::literal(&text(root));

    let rect_form = root
        .rect()
        .unwrap_or(crawrad_ir::Rect::new(0.0, 0.0, 800.0, 600.0));

    // The window IS the form. It does not grow to swallow a component left
    // outside the form's own rect: that silently broke what the Designer
    // promises — the owner saw it as a window that got bigger at run time,
    // with a margin on the right that was not in the design (the old code
    // added `+30` horizontally and `+40` vertically per stray component).
    // A component outside the form is simply not seen when the app runs,
    // exactly as in the Designer, and the canvas already marks it with a
    // numbered badge on the edge.
    let max_w = rect_form.w;
    let max_h = rect_form.h;

    let mut dsl_emitted = emitted.to_vec();
    dsl_emitted.sort_by_key(|n| {
        n.class == "Badge"
            && n.props.get("associate").is_some_and(|v| match v {
                crawrad_ir::Value::Ref(s) | crawrad_ir::Value::Str(s) => !s.is_empty(),
                _ => false,
            })
    });

    // Makepad has one widget tree, so the derived form draws the base's
    // components itself, from the resolved form (the base under the derived
    // overrides). Their state is reached through `Deref`, their clicks through
    // the base's own dispatch.
    let resolved = base
        .filter(|_| !is_base)
        .and_then(|b| doc.resolve_inheritance(b).ok());
    let inherited: Vec<&Node> = match (&resolved, base) {
        (Some(full), Some(b)) => full
            .in_paint_order()
            .iter()
            .filter_map(|id| full.node(id))
            .filter(|n| full.is_inherited(&n.id, b))
            .filter(|n| {
                super::CLASSES.contains(&n.class.as_str())
                    || components::terceiro(reg, &n.class).is_some()
            })
            .collect(),
        _ => Vec::new(),
    };
    let drawn = inherited
        .iter()
        .map(|n| (resolved.as_ref().unwrap_or(doc), *n));
    let drawn: Vec<(&Document, &Node)> = drawn
        .chain(dsl_emitted.into_iter().map(|n| (doc, n)))
        .collect();

    let mut components_dsl = String::new();
    for &(d, n) in &drawn {
        if is_inside_container(d, reg, &n.id) {
            continue;
        }
        let rect = d
            .rect_in_form(&n.id)
            .unwrap_or(crawrad_ir::Rect::new(0.0, 0.0, 80.0, 24.0));

        let comp_dsl = components::emit_dsl(d, reg, n, rect.x, rect.y, rect.w, rect.h)?;
        components_dsl.push_str(&comp_dsl);
    }

    let mut fields = String::new();
    let mut default_fields = String::new();
    for n in emitted {
        let field = identificador(n.name())?;
        // **A THIRD-PARTY TYPE COMES WRITTEN IN FULL**, with its crate
        // module: `gauge_crawrad::Gauge`. A native one remains the bare name,
        // because its type is EMITTED inside the user's project and not
        // imported from any crate — proving this is generation, not
        // disguised interpretation.
        let is_inheriting = reg.get(&n.class).and_then(|d| d.inherits).is_some();
        let class_name = if is_inheriting {
            reg.effective_class(&n.class).to_string()
        } else {
            match components::terceiro(reg, &n.class) {
                Some(d) => format!("{}::{}", d.crate_module, d.class),
                None => n.class.clone(),
            }
        };
        fields.push_str(&format!("    pub {field}: {class_name},\n"));

        let init = components::emit_default_com(reg, doc, n);
        default_fields.push_str(&format!("            {field}: {init},\n"));

        if is_inheriting && let Some(desc) = components::terceiro(reg, &n.class) {
            let ext_type = format!("{}::{}", desc.crate_module, desc.class);
            let ext_init =
                crate::targets::makepad::components::third_party_literal(reg, doc, n, desc);
            fields.push_str(&format!("    pub {field}_ext: {ext_type},\n"));
            default_fields.push_str(&format!("            {field}_ext: {ext_init},\n"));
        }
    }

    // A control linked to an Action runs it and takes its state
    let links = super::action_link::action_links(doc, emitted)?;
    let actions = gerar_despacho_acoes(reg, doc, emitted)? + &links.dispatch;

    // THE FORM'S OWN EVENTS — 21 of the 22 the `Form` class offers, including
    // all eight that carry a parameter. The why, and the one that stays out,
    // are written at length in `form_events`.
    let eventos = super::form_events::form_events(root)?;
    // Tab first, so the text box rule never sees a Tab it moved
    let text_box_enter =
        super::tab_order::tab_key_call(reg, doc, emitted)? + &text_box_enter_call(emitted)?;
    let on_create_call = eventos.on_create.clone();

    // The form's own Color, unless it is transparent (= the theme's)
    let form_color = match root.props.get("color") {
        Some(crawrad_ir::Value::Color(c)) if c.a > 0 => {
            Some(crate::targets::shared::dsl_colour(&c.to_hex()))
        }
        _ => None,
    };
    // A light form is a white page, and `thSystem` is one in the browser
    let light_page = page_light(root);
    let bg_color = form_color
        .as_deref()
        .unwrap_or(if light_page == PageLight::Always {
            "#ffffff"
        } else {
            drawing::FORM_BG_COLOR
        });
    let light_page_call = if form_color.is_none() && light_page == PageLight::InBrowser {
        "            #[cfg(target_arch = \"wasm32\")]\n\
         \x20           crawrad_runtime_makepad::look::light_page(cx, &self.ui);\n"
    } else {
        ""
    };

    let bound_controls = crate::targets::data_binding::collect_bound_controls(doc, emitted)?;
    let dataset_actions = crate::targets::data_binding::collect_dataset_actions(doc, emitted)?;

    let mut sync_lines = String::new();
    let datasets = crate::targets::data_binding::active_datasets(&bound_controls, &dataset_actions);
    sync_lines.push_str(&crate::targets::data_binding::open_datasets_code(
        &datasets,
        "self.form.",
    ));
    for a in &dataset_actions {
        if !a.has_on_update {
            let expr =
                crate::targets::data_binding::dataset_action_enabled_expr(&a.operation, "adapter");
            sync_lines.push_str(&format!(
                "        self.form.{}.enabled = self.{}_adapter.as_ref().is_some_and(|adapter| {expr});\n",
                a.action_name, a.dataset_name
            ));
        }
    }
    sync_lines.push_str(&links.pre_sync);
    sync_lines.push_str(&crate::targets::data_binding::makepad_model_to_widget(
        &bound_controls,
    ));
    let dataset_grids = crate::targets::data_binding::dataset_grids(doc, emitted, &datasets)?;
    sync_lines.push_str(&crate::targets::data_binding::makepad_dataset_grid_follow(
        &dataset_grids,
    ));

    let mut usa_fracao = false;
    let mut usa_shader = false;
    // what components keep in the `App` itself (a Timer's interval)
    let mut app = super::components::codegen::AppState::default();
    if let Some(db_app) =
        crate::targets::data_binding::makepad_app_state_all(&bound_controls, &dataset_actions)
    {
        app.uses.push_str(&db_app.uses);
        app.field.push_str(&db_app.field);
    }
    app.field
        .push_str(&crate::targets::data_binding::dataset_grid_fields(
            &dataset_grids,
            "    #[rust]\n",
        ));
    // WindowState — the window id comes with the first geometry event,
    // and the platform op maximizes or minimizes it once
    let window_op = match root.props.get("window_state") {
        Some(crawrad_ir::Value::Enum(s)) if s == "wsMaximized" => Some("MaximizeWindow"),
        Some(crawrad_ir::Value::Enum(s)) if s == "wsMinimized" => Some("MinimizeWindow"),
        _ => None,
    };
    // The form's controls follow the window by their Anchors
    app.handle
        .push_str(&super::anchors::anchors_call(reg, doc, emitted)?);
    if let Some(op) = window_op {
        app.field
            .push_str("    #[rust]\n    pub crawrad_window_placed: bool,\n");
        app.handle.push_str(&format!(
            "        if let Event::WindowGeomChange(e) = event {{\n\
             \x20           if !self.crawrad_window_placed {{\n\
             \x20               self.crawrad_window_placed = true;\n\
             \x20               cx.push_unique_platform_op(CxOsOp::{op}(e.window_id));\n\
             \x20           }}\n\
             \x20       }}\n"
        ));
    }
    // Position poScreenCenter — the window is centred on its monitor
    // once, when the first geometry event brings its id and its size
    if matches!(root.props.get("position"), Some(crawrad_ir::Value::Enum(s)) if s == "poScreenCenter")
    {
        app.field
            .push_str("    #[rust]\n    pub crawrad_window_centred: bool,\n");
        app.handle.push_str(
            "        if let Event::WindowGeomChange(e) = event {\n\
             \x20           if !self.crawrad_window_centred {\n\
             \x20               self.crawrad_window_centred = true;\n\
             \x20               crawrad_runtime_makepad::window::centre(cx, e);\n\
             \x20           }\n\
             \x20       }\n",
        );
    }
    let synced = inherited
        .iter()
        .map(|n| (resolved.as_ref().unwrap_or(doc), *n, true));
    let synced: Vec<(&Document, &Node, bool)> = synced
        .chain(emitted.iter().map(|n| (doc, *n, false)))
        .collect();
    for (doc, n, is_inherited) in synced {
        let field = identificador(n.name())?;
        if reg.get(&n.class).and_then(|d| d.inherits) == Some("SpinEdit") {
            let hooks_use = "use crawrad_runtime::SpinEditHooks;\n";
            if !app.uses.contains(hooks_use) {
                app.uses.push_str(hooks_use);
            }
        }
        // the base's runtime state (a Timer's tick) stays the base's
        if !is_inherited && let Some(part) = components::app_state(doc, n, &field)? {
            if !app.uses.contains(&part.uses) {
                app.uses.push_str(&part.uses);
            }
            app.field.push_str(&part.field);
            app.handle.push_str(&part.handle);
        }
        if has_widget(reg, &n.class) {
            let vis_expr = visibility_expression(doc, reg, n)?;
            sync_lines.push_str(&format!(
                "        self.ui.widget(id!({field})).set_visible(cx, {vis_expr});\n"
            ));
            // Dimmed and, for a Button, no clicks (the runtime knows which
            // call each widget needs)
            let effective = reg.effective_class(&n.class);
            if components::has_enabled_field(effective) {
                sync_lines.push_str(&format!(
                    "        crawrad_runtime_makepad::enabled::apply(cx, &self.ui, id!({field}), self.form.{field}.enabled);\n"
                ));
            }
            // A control a Splitter moves takes its new box each sync
            let t = crate::targets::splitters::terms(doc, n, "self.form.");
            if t.iter().any(|s| !s.is_empty()) {
                let r = n
                    .rect()
                    .unwrap_or(crawrad_ir::Rect::new(0.0, 0.0, 0.0, 0.0));
                sync_lines.push_str(&format!(
                    "        crawrad_runtime_makepad::split::place(cx, &self.ui.widget(id!({field})), {:.1}{}, {:.1}{}, {:.1}{}, {:.1}{});\n",
                    r.x, t[0], r.y, t[1], r.w, t[2], r.h, t[3]
                ));
            }
            // The font and the background, where the class has them
            sync_lines.push_str(&super::look::sync_com(doc, reg, n, &field));
            // **DECLARED DRAWING HAS A PART THAT ONLY EXISTS AT RUNTIME.**
            //
            // `live_design!` is text compiled once; a gauge bar
            // changes while the application runs. What the manifest declares became
            // a widget tree in `emit_dsl`; what depends on dynamic values becomes these
            // lines.
            let r = doc
                .rect_in_form(&n.id)
                .unwrap_or(crawrad_ir::Rect::new(0.0, 0.0, 80.0, 24.0));
            // The Shape is one Sdf2d shader in its draw_bg
            usa_shader |= n.class == "Shape";
            let has_inherits = reg.get(&n.class).and_then(|d| d.inherits).is_some();
            if !has_inherits
                && let Some(desc) = components::terceiro(reg, &n.class)
                && let Some(d) = super::declarado::desenho(&field, desc, r.w, r.h)
            {
                usa_fracao |= d.usa_fracao;
                usa_shader |= d.usa_shader;
                sync_lines.push_str(&d.sync);
            } else {
                sync_lines.push_str(&components::emit_sync_com(reg, doc, n, &field));
            }
        } else {
            sync_lines.push_str(&components::emit_sync_com(reg, doc, n, &field));
        }
    }
    // the controls' pointer, focus and keyboard events: one router for
    // every class
    let mut event_items = eventos.items.clone();
    // The form's font size and colour, for ParentFont / ParentColor
    // The inherited controls Makepad draws here read the constants too
    let looked: Vec<&Node> = inherited.iter().chain(emitted).copied().collect();
    event_items.push_str(&crate::targets::shared::look_consts(doc, reg, &looked));
    // The form's OnPaint draws on a box under every control (the
    // first child of the body, so the controls are drawn over it)
    if super::layout::handler(root, "on_paint").is_some() {
        components_dsl.insert_str(
            0,
            "                crawrad_form_paint = <CrawradPaintBox> { width: Fill, height: Fill }\n",
        );
        let uses = "use crawrad_runtime_makepad::paint::CrawradPaintBoxWidgetRefExt;\n";
        if !app.uses.contains(uses) {
            app.uses.push_str(uses);
        }
    }
    // The menus, over every control; their press-outside runs
    // before the controls' events
    components_dsl.push_str(&super::menus::dsl(doc, emitted)?);
    if let Some(menus) = super::menus::app_state(doc, emitted)? {
        app.handle.push_str(&menus.handle);
    }
    if let Some(ctl) = super::control_events::control_events(reg, doc, emitted)? {
        // The tooltip, last, so it draws over every control
        components_dsl.push_str(&ctl.dsl);
        app.uses.push_str(&ctl.app.uses);
        app.field.push_str(&ctl.app.field);
        app.handle.push_str(&ctl.app.handle);
        for (needed, item) in [
            (ctl.needs_button, super::form_events::BUTTON),
            (ctl.needs_modifiers, super::form_events::MODIFIERS),
        ] {
            if needed && !event_items.contains(item) {
                event_items.push_str(item);
            }
        }
        if ctl.needs_key && !event_items.contains("fn crawrad_key(") {
            event_items.push_str(&super::form_events::key_match());
        }
    }
    // The form's own Hint, in the same one tooltip
    let mut form_hint_after = String::new();
    if let Some(fh) = super::form_hint::form_hint(reg, doc, emitted)? {
        app.field.push_str(&fh.field);
        if !components_dsl.contains("crawrad_tooltip = <Tooltip>") {
            components_dsl.push_str(super::control_events::TOOLTIP_DSL);
        }
        form_hint_after = fh.after_widgets;
    }
    if components::grid::any_bound_grid_has_hint(doc, emitted)
        && !components_dsl.contains("crawrad_tooltip = <Tooltip>")
    {
        components_dsl.push_str(super::control_events::TOOLTIP_DSL);
    }

    let component_types = components::component_structs(doc, reg);
    // **THE SAME `fraction` AS THE CANVAS.** It is added only when someone calls it:
    // a function nobody calls is `dead_code` in a file the next
    // generation rewrites, and warning there warns someone who cannot fix it.
    // **The `Sdf2d` of the two strokes the framework has no widget to render.**
    //
    // It is the same `use` Makepad's own `CheckBox` writes
    // (`makepad-widgets-1.0.0/src/check_box.rs:11`), and without it the shader does not
    // compile — **silently**, because `live_design!` retains the text and only
    // resolves it at runtime. Measured on 2026-09-18: `Sdf2d::viewport` came out as
    // *"Call not found"* on the application's stderr, the mark simply did not
    // appear, and `cargo build` had completed cleanly.
    //
    // It is added only when someone uses it: `.designer.rs` is a file the user
    // opens and reads.
    let shaders = if usa_shader {
        "    use makepad_draw::shader::std::*;\n"
    } else {
        ""
    };
    let fracao = if usa_fracao {
        crate::targets::compartilhado::trecho_da_geometria("fraction")
    } else {
        String::new()
    };
    let pecas_set = crate::targets::compartilhado::pecas_necessarias(doc, reg);
    let mut excl = Vec::new();
    if pecas_set.contains("shortcut") {
        excl.push("Key");
    }
    // The same text the egui designer gets, byte for byte: the handler
    // signature is the catalogue's, so the types it names cannot be one
    // target's private business. Only what this form's handlers name comes
    // out — the why is written at length in `crate::events`.
    let event_types = crate::events::payload_types_excluding(
        reg,
        &crate::events::wired_handlers(root, emitted)?,
        &excl,
    );

    let (base_type, base_init) = match doc.inherits() {
        Some(base) => {
            let base_ident = identificador(base)?;
            let t = crate::tipo_do_form(&base_ident);
            (t.clone(), format!("{t}::default()"))
        }
        None => (
            "FormBase".to_string(),
            format!("FormBase::new({titulo}, {max_w:.1}, {max_h:.1})"),
        ),
    };
    let theme_link = theme_link(root);

    let own_wired_events = crate::events::collect_wired_events(reg, root, emitted)?;
    let has_wired = !own_wired_events.is_empty();

    let base_wired_events = match bases.split_first() {
        Some((b, above)) => reported_events(reg, b, above)?,
        None => Vec::new(),
    };
    let base_has_wired = !base_wired_events.is_empty();

    let base_use = match doc.inherits() {
        Some(base_name) => {
            let base_id = identificador(base_name)?;
            let base_type = crate::tipo_do_form(&base_id);
            // the `Fired` type is named by an override's match arm, and by
            // the `Base` variant of a base that reports its base's events up
            if crate::events::overrides_base(doc, &base_wired_events) || (is_base && base_has_wired)
            {
                format!("use crate::{base_id}_designer::{{{base_type}, {base_type}Fired}};\n")
            } else {
                format!("use crate::{base_id}_designer::{base_type};\n")
            }
        }
        None => String::new(),
    };

    let (fired_enum, metodos_form, base_reporting_actions, reported) = if is_base
        && (has_wired || base_has_wired)
    {
        let mut rewritten = Vec::new();
        let mut used = std::collections::BTreeSet::new();
        let reporting_actions = crate::events::rewrite_statement_calls(
            &actions,
            form_type,
            &own_wired_events,
            "self.form.",
            reg,
            &mut used,
            &mut rewritten,
        );
        if !rewritten.is_empty() || base_has_wired {
            let mut rep_act = reporting_actions.replace("self.ui.", "ui.");
            rep_act = rep_act.replace("self.form.", "self.");
            // A base that inherits reports its base's events up too,
            // wrapped, so a form further down can still override them
            if base_has_wired {
                rep_act.insert_str(
                    0,
                    &format!(
                        "        for fired in self.base.handle_actions_reporting(ui, actions) {{\n\
                         \x20           crawrad_fired.push({form_type}Fired::Base(fired));\n\
                         \x20       }}\n"
                    ),
                );
            }
            let actions_p_rep = if rep_act.trim().is_empty() {
                "_actions"
            } else {
                "actions"
            };
            let handle_actions_reporting = format!(
                "    /// Processes actions and returns events that fired during this pass.\n\
                 \x20   pub fn handle_actions_reporting(&mut self, ui: &WidgetRef, {actions_p_rep}: &Actions) -> Vec<{form_type}Fired> {{\n\
                 \x20       let mut crawrad_fired = Vec::new();\n\
                 {rep_act}\
                 \x20       crawrad_fired\n\
                 \x20   }}\n"
            );
            let base_arm = match base_has_wired {
                true => crate::events::base_arm(doc, form_type, &base_type, &base_wired_events)?,
                false => String::new(),
            };
            let dispatch = crate::events::emit_dispatch(form_type, &rewritten, &base_arm);
            let base_path = doc
                .inherits()
                .map(|b| identificador(b).map(|id| format!("crate::{id}_designer::{base_type}")))
                .transpose()?;
            let nested = match &base_path {
                Some(path) if base_has_wired => {
                    crate::events::nest_base_events(doc, path, &base_wired_events)
                }
                _ => Vec::new(),
            };
            let base_variant = base_has_wired.then_some(base_type.as_str());
            (
                crate::events::emit_fired_enum(form_type, &rewritten, base_variant),
                format!(
                    "\nimpl {form_type} {{\n\
                     {handle_actions_reporting}\
                     {dispatch}\
                     }}\n"
                ),
                true,
                rewritten.into_iter().chain(nested).collect(),
            )
        } else {
            (String::new(), String::new(), false, Vec::new())
        }
    } else {
        (String::new(), String::new(), false, Vec::new())
    };

    let base_actions = if base_has_wired {
        crate::events::base_dispatch(
            doc,
            &base_type,
            &base_wired_events,
            "self.form",
            "self.form.base",
            "self.form.base.handle_actions_reporting(&self.ui, actions)",
        )?
    } else {
        String::new()
    };

    let app_actions = if base_reporting_actions {
        // the reporting pass already brings the base's events, wrapped
        "        let crawrad_fired = self.form.handle_actions_reporting(&self.ui, actions);\n\
         \x20       for fired in crawrad_fired {\n\
         \x20           self.form.dispatch(fired);\n\
         \x20       }\n"
            .to_string()
    } else if base_has_wired {
        format!("{base_actions}{actions}")
    } else {
        actions.clone()
    };

    // A form with nothing to dispatch still has to implement `handle_actions`,
    // and the parameter it does not read is a warning the user cannot fix —
    // the designer is rewritten on the next generation. Measured on a form
    // with the key and mouse events wired and no `OnClick`: one warning, and
    // it was this one.
    let actions_param = if app_actions.trim().is_empty() {
        "_actions"
    } else {
        "actions"
    };

    // a derived form's base is `Base::default()` too: with nothing of its own
    // the impl would be the derivable one clippy refuses
    let (derive_default, impl_default) = match doc.inherits().is_some() && default_fields.is_empty()
    {
        true => ("#[derive(Default)]\n", String::new()),
        false => (
            "",
            format!(
                "impl Default for {form_type} {{\n\
                 \x20   fn default() -> Self {{\n\
                 \x20       Self {{\n\
                 \x20           base: {base_init},\n\
                 {default_fields}\
                 \x20       }}\n\
                 \x20   }}\n\
                 }}\n"
            ),
        ),
    };
    let code = format!(
        "// Generated by CrawRAD - do not edit this file directly.\n\
         //\n\
         // Native Makepad designer generated from visual form.\n\
         \n\
         use makepad_widgets::*;\n\
         use crawrad_runtime::prelude::*;\n\
         {base_use}\
         {app_uses}\
         {event_types}\
         {event_items}\
         {component_types}\n\
         {fired_enum}\
         {fracao}\
         live_design! {{\n\
         \x20   link widgets;\n\
         \x20   use link::widgets::*;\n\
         \x20   use link::theme::*;\n\
         {shaders}\
         \n\
         \x20   App = {{{{App}}}} {{\n\
         \x20       ui: <Window> {{\n\
         \x20           window: {{inner_size: vec2({max_w:.1}, {max_h:.1}), title: {titulo}}},\n\
         \x20           // THE TITLE BAR IS DRAWN BY MAKEPAD ITSELF, and it\n\
         \x20           // starts out saying \"Makepad\"\n\
         \x20           // (`makepad-widgets-1.0.0/src/window.rs:33`). Without this line the\n\
         \x20           // user's app opens with the framework's name written\n\
         \x20           // on top — and the `title:` above does not reach it: that is\n\
         \x20           // the SYSTEM title, this is the label Makepad paints.\n\
         \x20           // `margin: 0`: Makepad's label has a 100 px left margin; in a narrow\n\
         \x20           // window it leaves no room and the title is not drawn at all.\n\
         \x20           caption_bar = {{ caption_label = {{ label = {{ text: {titulo}, margin: {{left: 0}} }} }} }},\n\
         \x20           pass: {{clear_color: {bg_color}}},\n\
         \x20           body = <View> {{\n\
         \x20               flow: Overlay,\n\
         \x20               width: Fill, height: Fill,\n\
         \x20               show_bg: true,\n\
         \x20               draw_bg: {{ color: {bg_color} }},\n\
         \x20               scroll_bars: <ScrollBars> {{\n\
         \x20                   show_scroll_x: false,\n\
         \x20                   show_scroll_y: true,\n\
         \x20               }}\n\
         \n\
         {components_dsl}\
         \x20               crawrad_message_dialog = <CrawradMessageDialog> {{}}\n\
         \x20           }}\n\
         \x20       }}\n\
         \x20   }}\n\
         }}\n\
         \n\
         #[derive(Live, LiveHook)]\n\
         pub struct App {{\n\
         \x20   #[live]\n\
         \x20   pub ui: WidgetRef,\n\
         \x20   #[rust]\n\
         \x20   pub form: {form_type},\n\
         \x20   #[rust(true)]\n\
         \x20   pub initial_sync: bool,\n\
         {event_field}\
         {app_field}\
         }}\n\
         \n\
         impl App {{\n\
         \x20   pub fn sync_ui(&mut self, cx: &mut Cx) {{\n\
         {sync_lines}\
         \x20       crawrad_runtime_makepad::dialogs::sync_message_dialog(cx, &self.ui.view(id!(crawrad_message_dialog)));\n\
         \x20       self.ui.redraw(cx);\n\
         \x20   }}\n\
         }}\n\
         \n\
         {derive_default}\
         pub struct {form_type} {{\n\
         \x20   pub base: {base_type},\n\
         {fields}\
         }}\n\
         \n\
         impl std::ops::Deref for {form_type} {{\n\
         \x20   type Target = {base_type};\n\
         \x20   fn deref(&self) -> &Self::Target {{\n\
         \x20       &self.base\n\
         \x20   }}\n\
         }}\n\
         \n\
         impl std::ops::DerefMut for {form_type} {{\n\
         \x20   fn deref_mut(&mut self) -> &mut Self::Target {{\n\
         \x20       &mut self.base\n\
         \x20   }}\n\
         }}\n\
         \n\
         impl Form for {form_type} {{\n\
         \x20   fn base(&self) -> &FormBase {{\n\
         \x20       self.base.base()\n\
         \x20   }}\n\
         \x20   fn base_mut(&mut self) -> &mut FormBase {{\n\
         \x20       self.base.base_mut()\n\
         \x20   }}\n\
         }}\n\
         \n\
         {impl_default}\
         {metodos_form}\
         \n\
         impl LiveRegister for App {{\n\
         \x20   fn live_register(cx: &mut Cx) {{\n\
         \x20       makepad_widgets::live_design(cx);\n\
{theme_link}\
         \x20       crawrad_runtime_makepad::live_design(cx);\n\
         \x20   }}\n\
         }}\n\
         \n\
         impl MatchEvent for App {{\n\
         \x20   fn handle_actions(&mut self, cx: &mut Cx, {actions_param}: &Actions) {{\n\
         \x20       crawrad_runtime_makepad::dialogs::handle_dialog_actions(cx, &self.ui.view(id!(crawrad_message_dialog)), {actions_param});\n\
         {app_actions}\
         \x20       self.sync_ui(cx);\n\
         \x20   }}\n\
         }}\n\
         \n\
         impl AppMain for App {{\n\
         \x20   fn handle_event(&mut self, cx: &mut Cx, event: &Event) {{\n\
         \x20       if self.initial_sync {{\n\
         \x20           self.initial_sync = false;\n\
         {light_page_call}\
         {on_create_call}\
         \x20           self.sync_ui(cx);\n\
         \x20       }}\n\
         \x20       if crawrad_runtime_makepad::dialogs::handle_dialog_key(cx, &self.ui.view(id!(crawrad_message_dialog)), event) {{\n\
         \x20           // A key answered the dialog: run the action pass now, so a\n\
         \x20           // MessageDialog's answer reaches `modal_result`/`OnClose`\n\
         \x20           self.handle_actions(cx, &[]);\n\
         \x20           return;\n\
         \x20       }}\n\
         {text_box_enter}\
         {app_handle}\
         {event_handle}\
         \x20       self.match_event(cx, event);\n\
         \x20       self.ui.handle_event(cx, event, &mut Scope::empty());\n\
         {cursor_after}\
         {form_hint_after}\
         \x20   }}\n\
         }}\n\
         \n\
         app_main!(App);\n",
        theme_link = theme_link,
        light_page_call = light_page_call,
        event_items = event_items,
        event_field = eventos.field,
        event_handle = eventos.handle,
        app_uses = app.uses,
        app_field = app.field,
        app_handle = app.handle,
        cursor_after = super::cursor::after_widgets(reg, doc, emitted)?,
    );

    // `crawrad_runtime::prelude` carries a `KeyCode` of its own, so a
    // `KeyCode::` the file names (the key match, the Tab call, the Memo's
    // WantTabs) is ambiguous between the two globs. Makepad's is the one meant,
    // and it is imported by name — decided on the whole file, since those uses
    // come from more places than the payload types.
    let code = if code.contains("KeyCode::") {
        code.replacen(
            "use crawrad_runtime::prelude::*;\n",
            "use crawrad_runtime::prelude::*;\nuse makepad_widgets::KeyCode;\n",
            1,
        )
    } else {
        code
    };
    Ok((
        crate::targets::shared::with_show_hint_const(doc, code),
        reported,
    ))
}

#[note("targets-makepad-form-theme-link")]
fn theme_link(root: &Node) -> &'static str {
    match page_light(root) {
        PageLight::Always => "        cx.link(live_id!(theme), live_id!(crawrad_theme_light));\n",
        PageLight::InBrowser => {
            "        #[cfg(target_arch = \"wasm32\")]\n\
             \x20       cx.link(live_id!(theme), live_id!(crawrad_theme_light));\n\
             \x20       #[cfg(not(target_arch = \"wasm32\"))]\n\
             \x20       cx.link(live_id!(theme), live_id!(crawrad_theme_dark));\n"
        }
        // The dark copy, not makepad's own: its code font is not the GPL one.
        PageLight::Never => "        cx.link(live_id!(theme), live_id!(crawrad_theme_dark));\n",
    }
}

/// When the form is painted light.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PageLight {
    /// `thLight`: everywhere.
    Always,
    /// `thSystem`: in the browser (a web project), where a page is white.
    InBrowser,
    /// `thDark`.
    Never,
}

fn page_light(root: &Node) -> PageLight {
    match root.props.get("theme") {
        Some(crawrad_ir::Value::Enum(s)) if s == "thLight" => PageLight::Always,
        Some(crawrad_ir::Value::Enum(s)) if s == "thDark" => PageLight::Never,
        _ => PageLight::InBrowser,
    }
}

#[note("targets-makepad-form-text-box-enter-call")]
fn text_box_enter_call(emitted: &[&Node]) -> Result<String, Erro> {
    let mut memos = Vec::new();
    let mut edits = Vec::new();
    for n in emitted {
        let enter = super::components::for_class(&n.class).map(|c| c.enter_key());
        let into = match enter {
            Some(super::components::EnterKey::BreaksLine) => &mut memos,
            Some(super::components::EnterKey::Swallowed) => &mut edits,
            _ => continue,
        };
        let field = identificador(n.name())?;
        let text = super::components::text_path(n, &field);
        into.push(format!("self.ui.widget(id!({text}))"));
    }
    if memos.is_empty() && edits.is_empty() {
        return Ok(String::new());
    }
    Ok(format!(
        "        let text_box_key = crawrad_runtime_makepad::keys::text_box_enter(cx, &[{}], &[{}], event);\n\
         \x20       let event = text_box_key.as_ref().unwrap_or(event);\n",
        memos.join(", "),
        edits.join(", ")
    ))
}

#[cfg(test)]
#[path = "../../test/text_box_enter/test.rs"]
mod text_box_enter_tests;

#[cfg(test)]
#[path = "../../test/text_input_flags/test.rs"]
mod text_input_flags_tests;
