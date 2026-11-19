//! The Build Manager: targets, Debug/Release profile, the Cargo command (Build,
//! Check, Test, Clippy, Clean), extra flags, environment variables and a live output console.

use super::super::*;
use crawrad_runtime_makepad::check_list::CrawradCheckListBoxWidgetRefExt;
use crawrad_runtime_makepad::chips::CrawradChipsWidgetRefExt;
use crawrad_runtime_makepad::progress_card::CrawradProgressCardWidgetRefExt;

use crawrad_note::note;
pub use crawrad_project::build_settings::{
    BuildMode, BuildPage, BuildTab, CargoCommand, TargetPlatform,
};

fn apply_rail_button_look(cx: &mut Cx, btn: &ButtonRef, active: bool) {
    // theme/tokens.rs: CRAWRAD_BOTAO_PRIMARIO(_HOVER/_BORDA) vs CRAWRAD_CONTROLE / CRAWRAD_BORDA_CONTROLE / CRAWRAD_TEXTO_PRIMARIO
    let (bg, hov, bdr, txt) = if active {
        (
            vec4(0.871, 0.353, 0.169, 1.0),
            vec4(0.918, 0.388, 0.157, 1.0),
            vec4(0.722, 0.239, 0.086, 1.0),
            vec4(1.0, 1.0, 1.0, 1.0),
        )
    } else {
        let c = vec4(0.169, 0.169, 0.176, 1.0);
        (
            c,
            c,
            vec4(0.302, 0.310, 0.322, 1.0),
            vec4(0.906, 0.906, 0.918, 1.0),
        )
    };
    btn.apply_over(cx, live! {
        draw_bg: { color: (bg), color_hover: (hov), border_color_1: (bdr), border_color_2: (bdr) }
        draw_text: { color: (txt) }
    });
}

impl IdeApp {
    #[note("ide-app-dialogs-build-ideapp-open-build")]
    pub(in super::super) fn open_build(&mut self, cx: &mut Cx) {
        self.build.dialog = true;
        self.build.ensure_default_targets();
        if self.build.env_vars.is_empty() {
            self.build.env_vars = vec![("RUST_LOG".into(), "info".into())];
        }
        let nome_proj = self
            .alvo_do_cargo()
            .map(|(_, p)| p)
            .unwrap_or_else(|| "my_app".into());
        self.ordenar_alvos_build();
        self.ui
            .view(id!(build_dialog.dlg))
            .label(id!(bld_title))
            .set_text(cx, &format!("Build - {nome_proj}"));
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-build-ideapp-ordenar-alvos-build")]
    pub(in super::super) fn ordenar_alvos_build(&mut self) {
        self.build.sort_targets();
    }

    #[note("ide-app-dialogs-build-ideapp-close-build")]
    pub(in super::super) fn close_build(&mut self, cx: &mut Cx) {
        self.build.dialog = false;
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-build-ideapp-draw-build")]
    pub(in super::super) fn draw_build(&mut self, cx: &mut Cx) {
        let card = self.ui.widget(id!(build_card)).as_crawrad_progress_card();
        if !self.build.dialog
            && !self.build.release_card_dismissed
            && let Some(rc) = self.build.last_release.as_ref()
        {
            card.set_visible(cx, true);
            card.set_release(
                cx,
                &rc.headline,
                &rc.size,
                rc.change.as_ref().map(|(s, g)| (s.as_str(), *g)),
                rc.mostly.as_deref(),
            );
            card.set_button_texts(cx, "See what weighs", "Dismiss");
        } else {
            let card_opt = self.build.card_now();
            let show_card = !self.build.dialog && card_opt.is_some();
            card.set_visible(cx, show_card);
            if let (true, Some(c)) = (show_card, card_opt) {
                card.set(
                    cx,
                    &c.icon,
                    &c.headline,
                    &c.detail,
                    &c.clock,
                    c.fraction,
                    c.running,
                );
                card.set_button_texts(cx, "Show output", "Cancel");
            }
        }

        self.show_veil(cx, id!(build_dialog), self.build.dialog);
        if !self.build.dialog {
            return;
        }

        let dlg = self.ui.view(id!(build_dialog.dlg));
        super::fields::paint(cx, &dlg, &self.build);
        dlg.button(id!(bld_btn_close)).set_text(cx, "Close");
        let is_opts = self.build.page == BuildPage::Options;
        let is_size = self.build.page == BuildPage::Size;
        let active_cmd = match self.build.page {
            BuildPage::Command(m) => Some(m),
            _ => None,
        };

        apply_rail_button_look(cx, &dlg.button(id!(bld_page_opts)), is_opts);
        dlg.button(id!(bld_page_opts)).set_enabled(cx, true);
        for (btn_id, mode) in [
            (id!(bld_mode_build), BuildMode::Build),
            (id!(bld_btn_check), BuildMode::Check),
            (id!(bld_btn_clean), BuildMode::Clean),
        ] {
            apply_rail_button_look(cx, &dlg.button(btn_id), active_cmd == Some(mode));
            let btn = dlg.button(btn_id);
            btn.set_enabled(cx, self.build.is_mode_enabled(mode));
            btn.set_text(cx, &self.build.rail_label(mode));
        }

        let (root_opt, package) = match self.alvo_do_cargo() {
            Some((r, p)) => (Some(r), p),
            None => (self.raiz(), "project".into()),
        };
        let history = root_opt
            .as_deref()
            .map(crawrad_project::release_size::read_history)
            .unwrap_or_default();

        let size_btn = dlg.button(id!(bld_btn_size));
        size_btn.set_visible(cx, true);
        apply_rail_button_look(cx, &size_btn, is_size);
        let size_label = if self.build.size_unseen {
            "📏 Size •"
        } else {
            "📏 Size"
        };
        size_btn.set_text(cx, size_label);

        dlg.view(id!(bld_page_options)).set_visible(cx, is_opts);
        dlg.view(id!(bld_page_command))
            .set_visible(cx, !is_opts && !is_size);
        dlg.widget(id!(bld_page_size)).set_visible(cx, is_size);

        let page_id = if is_opts {
            id!(bld_page_options)
        } else if is_size {
            id!(bld_page_size)
        } else {
            id!(bld_page_command)
        };
        dlg.widget(page_id)
            .apply_over(cx, live! { margin: { left: (self.build.page_offset()) } });

        if is_opts {
            let (names, triples, on) = TargetPlatform::check_list(&self.build.selected_targets);
            let list = dlg.widget(id!(bld_targets)).as_crawrad_check_list_box();
            list.set_items(cx, &names);
            list.set_details(cx, &triples);
            list.set_checked(cx, &on);

            let chips = dlg.widget(id!(bld_chips)).as_crawrad_chips();
            chips.set_removable(cx, true);
            chips.set_items(cx, &self.build.target_chip_labels());

            dlg.view(id!(bld_env_row_0))
                .set_visible(cx, !self.build.env_vars.is_empty());
        } else if is_size {
            let exe_name = if cfg!(windows) {
                format!("{package}.exe")
            } else {
                package.to_string()
            };
            if let Some(page) = crawrad_project::release_size::size_page(
                &history,
                &package,
                &exe_name,
                &self.build.last_release_symbols,
                &package,
            ) {
                dlg.release_size_view(id!(bld_page_size))
                    .set_page(cx, page.parts().into());
            } else {
                dlg.release_size_view(id!(bld_page_size)).set_empty(cx);
            }
        } else if let Some(mode) = active_cmd {
            let is_this_running = self
                .build
                .running
                .as_ref()
                .is_some_and(|(m, _, _)| *m == mode);
            let cancel_btn = dlg.button(id!(bld_cancel));
            cancel_btn.set_visible(cx, is_this_running);
            cancel_btn.set_text(cx, "Cancel");

            let (_, _, _, frac) = self.build.command_view_info(mode);

            let bar_view = dlg.view(id!(bld_run_bar));
            bar_view.set_visible(cx, frac.is_some());
            if let Some(f) = frac {
                let w = (bar_view.area().rect(cx).size.x * f).max(2.0);
                dlg.view(id!(bld_run_bar.bar))
                    .apply_over(cx, live! { width: (w), height: Fill });
            }

            self.update_build_console(cx);
        }
    }

    #[note("ide-app-dialogs-build-ideapp-update-build-console")]
    pub(in super::super) fn update_build_console(&mut self, cx: &mut Cx) {
        let dlg = self.ui.view(id!(build_dialog.dlg));
        let diags = || self.diagnosticos.as_deref().unwrap_or(&[]);
        let texto = match self.build.tab {
            BuildTab::Output => match self.build.page {
                BuildPage::Command(m) => {
                    self.build.run(m).map(|r| r.log_text()).unwrap_or_default()
                }
                _ => String::new(),
            },
            BuildTab::Warnings => crawrad_project::build_report::diagnostics_text(
                diags(),
                crawrad_project::diagnostics_model::Severity::Warning,
            ),
            BuildTab::Errors => crawrad_project::build_report::diagnostics_text(
                diags(),
                crawrad_project::diagnostics_model::Severity::Error,
            ),
        };
        let input = dlg.text_input(id!(bld_console_text));
        input.set_text(cx, &texto);
        input.set_is_read_only(cx, true);
    }

    #[note("ide-app-dialogs-build-ideapp-atender-build")]
    pub(in super::super) fn atender_build(&mut self, cx: &mut Cx, actions: &Actions) {
        let card = self.ui.widget(id!(build_card)).as_crawrad_progress_card();
        if !self.build.dialog
            && !self.build.release_card_dismissed
            && self.build.last_release.is_some()
        {
            if card.show_clicked(actions) {
                self.build.release_card_dismissed = true;
                self.build.switch_page(BuildPage::Size);
                self.open_build(cx);
                return;
            }
            if card.cancel_clicked(actions) {
                self.build.release_card_dismissed = true;
                card.set_visible(cx, false);
                self.refresh(cx);
                return;
            }
        } else {
            if card.show_clicked(actions) {
                self.build
                    .switch_page(BuildPage::Command(self.build.active_or_last_mode()));
                self.open_build(cx);
                return;
            }
            if card.cancel_clicked(actions) {
                if let Some((_, id, _)) = self.build.running {
                    self.build_tasks.cancel(id);
                }
                return;
            }
        }

        if !self.build.dialog {
            return;
        }

        let dlg = self.ui.view(id!(build_dialog.dlg));
        if dlg.button(id!(bld_close)).clicked(actions)
            || dlg.button(id!(bld_btn_close)).clicked(actions)
        {
            self.close_build(cx);
            return;
        }
        if dlg.button(id!(bld_cancel)).clicked(actions)
            && let Some((_, id, _)) = self.build.running
        {
            self.build_tasks.cancel(id);
        }
        if dlg.button(id!(bld_page_opts)).clicked(actions) {
            self.build.switch_page(BuildPage::Options);
            self.draw_build(cx);
        }
        if dlg.button(id!(bld_btn_size)).clicked(actions) {
            self.build.switch_page(BuildPage::Size);
            self.draw_build(cx);
        }
        if self.build.page == BuildPage::Size
            && dlg.release_size_view(id!(bld_page_size)).action(actions)
                == ReleaseSizeAction::Shrink
        {
            self.open_shrink(cx);
        }

        let mut trigger_cmd = |app: &mut Self, mode: BuildMode, cmd: Option<CargoCommand>| {
            if app.build.page != BuildPage::Command(mode) && app.build.run(mode).is_some() {
                app.build.switch_page(BuildPage::Command(mode));
                app.draw_build(cx);
            } else if let Some(c) = cmd {
                app.execute_build_action(cx, c);
            } else {
                app.rodar_projeto(cx);
            }
        };

        for (id, mode, cmd) in [
            (
                id!(bld_mode_build),
                BuildMode::Build,
                Some(CargoCommand::Build),
            ),
            (
                id!(bld_btn_check),
                BuildMode::Check,
                Some(CargoCommand::Check),
            ),
            (
                id!(bld_btn_clean),
                BuildMode::Clean,
                Some(CargoCommand::Clean),
            ),
        ] {
            if dlg.button(id).clicked(actions) {
                trigger_cmd(self, mode, cmd);
            }
        }
        if dlg.button(id!(bld_btn_help)).clicked(actions) {
            self.status =
                Some("Build Manager: ⚙ Options, 🔨 Build, ▶ Run, 📝 Check, 🧹 Clean".into());
        }

        let list = dlg.widget(id!(bld_targets)).as_crawrad_check_list_box();
        if let Some((i, on)) = list.toggled(actions) {
            TargetPlatform::set_in(&mut self.build.selected_targets, i, on);
            self.ordenar_alvos_build();
            self.draw_build(cx);
        }

        let removed = dlg
            .widget(id!(bld_chips))
            .as_crawrad_chips()
            .removed(actions);
        if let Some(i) = removed.filter(|&i| i < self.build.selected_targets.len()) {
            self.build.selected_targets.remove(i);
            self.ordenar_alvos_build();
            self.draw_build(cx);
        }

        if super::fields::take_input(cx, &dlg, actions, &mut self.build) {
            self.draw_build(cx);
        }
        if dlg.button(id!(bld_out_copy)).clicked(actions) {
            let txt = dlg.text_input(id!(bld_console_text)).text();
            if !txt.is_empty() {
                cx.copy_to_clipboard(&txt);
                self.status = Some("Console text copied to clipboard".into());
            }
        }
        if dlg.button(id!(bld_out_clear)).clicked(actions) {
            if let BuildPage::Command(m) = self.build.page
                && let Some(r) = self.build.run_mut(m)
            {
                r.clear_log();
            }
            self.update_build_console(cx);
        }
    }

    #[note("ide-app-dialogs-build-ideapp-execute-build-action")]
    pub(in super::super) fn execute_build_action(&mut self, cx: &mut Cx, cmd: CargoCommand) {
        if self.build_tasks.is_running("build") {
            self.status = Some(self.build.refuse_busy());
            return;
        }
        let Some((raiz, pacote)) = self.alvo_do_cargo() else {
            self.status = Some("no Cargo project open — nothing to build".into());
            return;
        };

        let (cargo_call, cmd_line) = self.build.cargo_call(cmd, &pacote, &raiz);
        self.status = Some(format!("Executed: {cmd_line}"));

        if (cmd == CargoCommand::Build || cmd == CargoCommand::Check) && !self.prepare_build(cx) {
            return;
        }

        let (mode, verb) = match cmd {
            CargoCommand::Build => (BuildMode::Build, "Building"),
            CargoCommand::Check => (BuildMode::Check, "Checking"),
            CargoCommand::Clean => (BuildMode::Clean, "Cleaning"),
        };
        self.build.mode = mode;
        self.build.start_run(
            mode,
            crawrad_project::build_progress::BuildProgress::new(verb, &pacote, &cmd_line),
        );
        self.build.switch_page(BuildPage::Command(mode));
        let now = std::time::Instant::now();
        let id = self.build_tasks.spawn("build", move |reporter| {
            let mut cmd = cargo_call.progress_command();
            let rep = &reporter;
            let res = crawrad_os::process::stream(
                &mut cmd,
                |p, l| rep.progress(crawrad_project::build_progress::encode_line(p, l)),
                || rep.is_cancelled(),
            );
            match res {
                Ok(outcome) => reporter.done(outcome),
                Err(err) => reporter.failed(err),
            }
        });
        self.build.running = Some((mode, id, now));
        self.build_clock = cx.start_timeout(0.1);
        self.draw_build(cx);
    }

    #[note("ide-app-dialogs-build-ideapp-execute-build-all")]
    pub(in super::super) fn execute_build_all(&mut self, cx: &mut Cx, mode: BuildMode) {
        if self.build_tasks.is_running("build") {
            self.status = Some(self.build.refuse_busy());
            return;
        }
        let Some(raiz) = self.raiz() else {
            self.status = Some("no Cargo project open — nothing to build".into());
            return;
        };
        if !self.prepare_build(cx) {
            return;
        }
        // Clean All empties the shared target: every member's exe and library.
        let (args, verb): (&[&str], _) = if mode == BuildMode::Clean {
            (&["clean"], "Cleaning")
        } else {
            (
                &["build", "--workspace", "--message-format=json"],
                "Building",
            )
        };
        let cmd_line = format!("cargo {}", args[..args.len().min(2)].join(" "));
        self.status = Some(format!("Executed: {cmd_line}"));
        self.build.mode = mode;
        self.build.start_run(
            mode,
            crawrad_project::build_progress::BuildProgress::new(verb, "workspace", &cmd_line),
        );
        self.build.switch_page(BuildPage::Command(mode));
        let cargo_call = crawrad_os::process::CargoCall::new(args.iter().copied(), &raiz);
        let now = std::time::Instant::now();
        let id = self.build_tasks.spawn("build", move |reporter| {
            let mut cmd = cargo_call.progress_command();
            let rep = &reporter;
            let res = crawrad_os::process::stream(
                &mut cmd,
                |p, l| rep.progress(crawrad_project::build_progress::encode_line(p, l)),
                || rep.is_cancelled(),
            );
            match res {
                Ok(outcome) => reporter.done(outcome),
                Err(err) => reporter.failed(err),
            }
        });
        self.build.running = Some((mode, id, now));
        self.build_clock = cx.start_timeout(0.1);
        self.draw_build(cx);
    }
}
