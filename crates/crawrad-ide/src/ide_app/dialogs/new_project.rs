//! New Project and the creation progress — the two windows of one gesture; the
//! draw/handle pair of each dialog lives together (who changes one changes both).

use super::super::*;
use crawrad_project::workspace_edit::WorkspaceStep as Step;
use crawrad_runtime_makepad::card::CrawradFormCardWidgetRefExt;
use crawrad_runtime_makepad::check_list::CrawradCheckListBoxWidgetRefExt;
use crawrad_runtime_makepad::chips::CrawradChipsWidgetRefExt;
use crawrad_runtime_makepad::drive;
use crawrad_runtime_makepad::list::CrawradListBoxWidgetRefExt;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub const NP_TITULO: &str = "New Project";
pub const NP_ESCOLHER: &str = "Create Project";
pub const ITEMS_CANCEL: &str = "Cancel";

#[derive(Debug, Clone)]
pub struct NovoProjetoDados {
    pub categoria_filtro: usize, // 0: Favorites, 1: Desktop, 2: Console, 3: WASM, 4: Mobile, 5: Library, 6: Embedded, 7: Examples, 8: More
    pub template_selecionado: usize, // 0..=7
    pub busca: String,
    pub framework: crawrad_codegen::AlvoGerador,
    pub nome: String,
    pub pasta: Option<PathBuf>,
    pub git_init: bool,
    pub favoritos: HashSet<usize>,
    pub opt_example: bool,
    pub opt_livereload: bool,
    pub opt_menu: bool,
    pub opt_tracing: bool,
    pub fonte_templates: usize, // 0: Built-in Templates, 1: Custom Templates
    /// Add Project: the workspace the new project joins; `None` = New Project.
    pub add_to: Option<PathBuf>,
}

pub use crawrad_project::project_template::TEMPLATES;
use crawrad_project::project_template::structure_preview;

#[note("ide-app-dialogs-new-project-np-categories")]
const NP_CATEGORIES: [&str; 9] = [
    "⭐  Favorites",
    "💻  Desktop",
    ">_  Console",
    "🌐  WebAssembly",
    "📱  Mobile",
    "📦  Library",
    "⚙  Embedded",
    "📖  Examples",
    "•••  More Templates",
];

#[note("ide-app-dialogs-new-project-np-id")]
fn np_id(name: String) -> [LiveId; 1] {
    [LiveId::from_str(&name)]
}

/// CrawRAD's own installation folder, when it can be known.
///
/// `None` is the honest answer when the executable's path cannot be read: an
/// unknown install folder must not make every path look unwritable.
fn install_folder() -> Option<PathBuf> {
    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(Path::to_path_buf))
}

impl IdeApp {
    #[note("ide-app-dialogs-new-project-ideapp-open-new-project")]
    pub(in super::super) fn open_new_project(&mut self, cx: &mut Cx, add_to: Option<PathBuf>) {
        if self.criando.is_some() {
            self.status = Some(CRIANDO_JA.into());
            self.sincronizar(cx);
            return;
        }

        // The dialog must open pointing somewhere a project can be written. Taking
        // the working directory unconditionally was right for an installer build and
        // wrong for the Store one, where that directory IS the package's read-only
        // install folder. `default_project_parent` decides; this only reads the
        // environment for it.
        let pasta_padrao = crawrad_project::workspace::default_project_parent(
            self.welcome.first().map(PathBuf::as_path),
            std::env::var("USERPROFILE")
                .or_else(|_| std::env::var("HOME"))
                .ok()
                .map(PathBuf::from)
                .as_deref(),
            std::env::current_dir().ok().as_deref(),
            install_folder().as_deref(),
        );
        let mut favs = HashSet::new();
        favs.insert(0); // GUI Application favorited
        favs.insert(5); // Multi-window Desktop

        self.novo_projeto = Some(NovoProjetoDados {
            categoria_filtro: 1, // Desktop active by default as in the reference
            template_selecionado: 0,
            busca: String::new(),
            framework: crawrad_codegen::AlvoGerador::Makepad,
            nome: "my_app".to_string(),
            pasta: add_to.clone().or(pasta_padrao),
            git_init: true,
            favoritos: favs,
            opt_example: true,
            opt_livereload: true,
            opt_menu: false,
            opt_tracing: false,
            fonte_templates: 0,
            add_to,
        });
        self.np_recusa = None;
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-new-project-ideapp-open-dialog-spec")]
    pub(crate) fn open_dialog_spec(&self) -> Option<crawrad_mcp::dialogs::DialogSpec> {
        // the open dialog is the one whose root its draw made visible
        let shown = |d: &crawrad_mcp::dialogs::DialogSpec| {
            self.ui.widget(&[LiveId::from_str(d.root[0])]).visible()
        };
        crawrad_mcp::dialogs::all().into_iter().find(shown)
    }

    #[note("ide-app-dialogs-new-project-ideapp-operate-dialog")]
    pub(crate) fn operate_dialog(
        &mut self,
        cx: &mut Cx,
        spec: &crawrad_mcp::dialogs::DialogSpec,
        op: &crawrad_mcp::dialog_tool::DialogOp,
    ) -> crawrad_mcp::run_tool::Answer {
        use crawrad_mcp::dialog_tool::DialogOp;
        let root = self.ui.widget(&spec.root.map(LiveId::from_str));
        let path = [LiveId::from_str(op.control())];
        let driven = match op {
            DialogOp::Press { .. } => drive::press(cx, &root, &path),
            DialogOp::Toggle { .. } => drive::toggle(cx, &root, &path),
            DialogOp::SetText { value, .. } => drive::type_text(cx, &root, &path, value),
            DialogOp::SetChecked { value, .. } => drive::set_checked(cx, &root, &path, *value),
            DialogOp::Click { .. } => drive::click_card(cx, &root, &path),
            DialogOp::Pick { label, .. } => drive::pick(cx, &root, &path, label),
        };
        self.sincronizar(cx);
        crawrad_mcp::dialog_tool::done(op, spec.name, driven)
    }

    /// Esc on an open dialog presses its Cancel button, or its Close when it has none.
    pub(crate) fn press_dialog_cancel(&mut self, cx: &mut Cx) {
        let spec = self.open_dialog_spec();
        if let Some((spec, name)) = spec.and_then(|s| Some((s, s.cancel_control()?))) {
            let root = self.ui.widget(&spec.root.map(LiveId::from_str));
            let _ = drive::press(cx, &root, &[LiveId::from_str(name)]);
            self.sincronizar(cx);
        }
    }

    #[note("ide-app-dialogs-new-project-ideapp-dialog-json")]
    pub(crate) fn dialog_json(&self, cx: &Cx) -> serde_json::Value {
        use crawrad_mcp::dialogs::{ControlKind, ControlState};
        let Some(spec) = self.open_dialog_spec() else {
            return serde_json::json!({ "open": null });
        };
        let root = self.ui.widget(&spec.root.map(LiveId::from_str));
        let fw = drive::list_rows(cx, &root, "np_fw_list");
        // New Project's categories, always on screen
        let cats = (spec.name == "new_project").then(|| drive::list_rows(cx, &root, "np_cat_list"));
        // the Options page tree, or New Project's palette ListBox if on screen
        let listed = spec.name == "new_project" && root.view(id!(np_fw_section)).visible();
        // resolved now: a rect read while drawing is not final yet
        let own = (spec.name == crawrad_mcp::dialogs::OPTIONS.name).then(|| {
            let rows = self.dialog_rows.iter();
            rows.map(|(t, a)| (t.clone(), a.rect(cx)))
                .collect::<Vec<_>>()
        });
        // the Build dialog's palette Chips: each chip, then its `×` as "× <name>"
        let chips = (spec.name == "build").then(|| {
            root.widget(id!(bld_chips))
                .as_crawrad_chips()
                .drawn_chips(cx)
        });
        // the Build dialog's palette CheckListBox: each target's row
        let targets = (spec.name == "build").then(|| {
            root.widget(id!(bld_targets))
                .as_crawrad_check_list_box()
                .drawn_rows(cx)
        });
        let chips = chips.unwrap_or_default().into_iter().flat_map(|(t, c, x)| {
            std::iter::once((t.clone(), c)).chain(x.map(|x| (format!("× {t}"), x)))
        });
        let rows: Vec<_> = own
            .unwrap_or_default()
            .into_iter()
            .chain(cats.unwrap_or_default())
            .chain(fw.into_iter().filter(|_| listed))
            .chain(targets.unwrap_or_default())
            .chain(chips)
            .map(|(t, r)| (t, xywh(r)))
            .collect();
        crawrad_mcp::dialogs::describe(&spec, &rows, |c| {
            let id = [LiveId::from_str(&c.name)];
            let w = root.widget(&id);
            let r = w.area().rect(cx);
            let label = c
                .text_of
                .as_ref()
                .map(|l| root.label(&[LiveId::from_str(l)]).text());
            let checked = (c.kind == ControlKind::Check).then(|| root.check_box(&id).active(cx));
            let text = label
                .or_else(|| drive::drop_text(&w))
                .unwrap_or_else(|| w.text());
            ControlState {
                items: drive::drop_items(&w),
                text,
                rect: xywh(r),
                visible: w.visible(),
                checked,
            }
        })
    }

    #[note("ide-app-dialogs-new-project-ideapp-workspace-command")]
    pub(in super::super) fn workspace_command(
        &mut self,
        cx: &mut Cx,
        c: crawrad_actions::FileEffect,
    ) {
        let pick = || rfd::FileDialog::new().pick_folder();
        match crawrad_project::workspace_edit::run(self.raiz().as_deref(), c, pick) {
            Step::OpenProject(p) => self.open_created_project(cx, &p),
            Step::AskInDialog(root) => self.open_new_project(cx, Some(root)),
            Step::Say(s) => self.status = Some(s).filter(|s| !s.is_empty()),
        }
        self.sincronizar(cx);
    }

    #[note("ide-app-dialogs-new-project-ideapp-draw-new-project")]
    pub(in super::super) fn draw_new_project(&mut self, cx: &mut Cx) {
        let aberto = self.novo_projeto.is_some();
        self.show_veil(cx, id!(newproject_dialog), aberto);
        let Some(dados) = self.novo_projeto.clone() else {
            return;
        };
        let dlg = self.ui.view(id!(newproject_dialog.dlg));
        let title = dados.add_to.as_ref().map_or(NP_TITULO, |_| "Add Project");
        dlg.label(id!(np_title)).set_text(cx, title);
        dlg.button(id!(np_escolher)).set_text(cx, NP_ESCOLHER);
        dlg.button(id!(np_cancel)).set_text(cx, ITEMS_CANCEL);

        // Title Bar: Search
        let campo_busca = dlg.text_input(id!(np_busca));
        campo_busca.apply_over(cx, live! { empty_text: ("Search templates...") });
        if campo_busca.text() != dados.busca {
            campo_busca.set_text(cx, &dados.busca);
        }

        // Column 1: the categories, the palette ListBox with each count
        let cat_counts = [
            dados.favoritos.len().to_string(),
            "4".to_string(),
            "2".to_string(),
            "2".to_string(),
            "2".to_string(),
            "2".to_string(),
            "1".to_string(),
            "6".to_string(),
            "3".to_string(),
        ];
        let names: Vec<String> = NP_CATEGORIES.iter().map(|n| n.to_string()).collect();
        let cats = dlg.widget(id!(np_cat_list)).as_crawrad_list_box();
        cats.set_items(cx, &names);
        cats.set_details(cx, &cat_counts);
        cats.set_selected(cx, Some(dados.categoria_filtro));

        // Column 2: Template Cards (Scrollable gallery)

        let q = dados.busca.trim().to_lowercase();

        for (idx, def) in TEMPLATES.iter().enumerate() {
            let part = |name: &str| np_id(format!("np_c{idx}_{name}"));
            let card_id = np_id(format!("np_card_{idx}"));
            let (icon_id, title_id, fav_id) = (part("icon"), part("title"), part("fav"));
            let (desc_id, plat_id) = (part("desc"), part("plat"));
            let match_cat = match dados.categoria_filtro {
                0 => dados.favoritos.contains(&idx),
                1 => def.category_idx == 1 || idx == 1 || idx == 6, // Desktop (4)
                2 => def.category_idx == 2 || idx == 7,             // Console (2)
                3 => def.category_idx == 3 || idx == 0,             // WASM (2)
                4 => def.category_idx == 4 || idx == 7,             // Mobile (2)
                5 => def.category_idx == 5,                         // Library (2)
                6 => def.category_idx == 2,                         // Embedded (1)
                7 => def.category_idx == 7 || (idx != 4 && idx != 3), // Examples (6)
                _ => idx >= 5,                                      // More Templates (3)
            };
            let match_busca = q.is_empty()
                || def.name.to_lowercase().contains(&q)
                || def.subtitle.to_lowercase().contains(&q)
                || def.description.to_lowercase().contains(&q)
                || def.platforms_badge.to_lowercase().contains(&q);

            let visivel = match_cat && match_busca;
            dlg.widget(&card_id).set_visible(cx, visivel);

            if visivel {
                let is_sel = idx == dados.template_selecionado;
                let cor_card = if is_sel {
                    vec4(0.125, 0.173, 0.259, 1.0)
                } else {
                    vec4(0.094, 0.133, 0.196, 1.0)
                };
                let cor_borda = if is_sel {
                    vec4(0.871, 0.353, 0.169, 1.0)
                } else {
                    vec4(0.137, 0.184, 0.259, 1.0)
                };
                let card = dlg.widget(&card_id).as_crawrad_form_card();
                card.set_colors(cx, cor_card, cor_borda);

                dlg.label(&icon_id).set_text(cx, def.icon);
                dlg.label(&title_id).set_text(cx, def.name);
                let is_fav = dados.favoritos.contains(&idx);
                dlg.button(&fav_id)
                    .set_text(cx, if is_fav { "★" } else { "☆" });
                dlg.label(&desc_id).set_text(cx, def.subtitle);
                dlg.label(&plat_id).set_text(cx, def.platforms_badge);
            }
        }

        // Column 3: Details Panel
        let tpl = &TEMPLATES[dados.template_selecionado.min(TEMPLATES.len() - 1)];
        dlg.label(id!(np_det_icon)).set_text(cx, tpl.icon);
        dlg.label(id!(np_det_title)).set_text(cx, tpl.name);
        let is_tpl_fav = dados.favoritos.contains(&dados.template_selecionado);
        dlg.label(id!(np_det_star))
            .set_text(cx, if is_tpl_fav { "★" } else { "☆" });
        dlg.label(id!(np_det_sub)).set_text(cx, tpl.subtitle);
        dlg.label(id!(np_det_desc)).set_text(cx, tpl.description);
        dlg.label(id!(np_det_plat))
            .set_text(cx, tpl.platforms_detail);

        let with_fw = tpl.supports_framework;
        let chosen = crawrad_project::frameworks::of(dados.framework);
        let (fw_name, fw_dep) = (chosen.name, chosen.crate_dep);
        dlg.view(id!(np_fw_section)).set_visible(cx, with_fw);
        dlg.view(id!(np_fw_none)).set_visible(cx, !with_fw);
        // The palette's own ListBox lists the catalogue
        let list = crawrad_project::frameworks::CATALOGUE;
        let names: Vec<String> = list
            .iter()
            .map(|f| match crawrad_project::frameworks::badge(f) {
                "" => f.name.to_string(),
                b => format!("{}  · {b}", f.name),
            })
            .collect();
        let lb = dlg.widget(id!(np_fw_list)).as_crawrad_list_box();
        dlg.widget(id!(np_fw_list)).set_visible(cx, with_fw);
        lb.set_items(cx, &names);
        let at = list.iter().position(|f| f.target == Some(dados.framework));
        lb.set_selected(cx, at);
        let h = 26.0 * list.len() as f64;
        dlg.widget(id!(np_fw_list))
            .apply_over(cx, live! { height: (h) });
        dlg.label(id!(np_fw_det_summary))
            .set_text(cx, chosen.summary);
        dlg.label(id!(np_fw_det_platforms))
            .set_text(cx, chosen.platforms);
        dlg.label(id!(np_fw_det_crate))
            .set_text(cx, &format!("Cargo.toml: {}", chosen.crate_dep));
        if with_fw {
            dlg.label(id!(np_det_sub))
                .set_text(cx, &tpl.subtitle.replace("Makepad", fw_name));
            dlg.label(id!(np_det_desc))
                .set_text(cx, &tpl.description.replace("Makepad", fw_name));
            dlg.button(id!(np_escolher))
                .set_text(cx, &format!("Create {fw_name} project"));
        }
        // live-reload is Makepad's own
        dlg.check_box(id!(np_opt_livereload)).set_visible(
            cx,
            with_fw && dados.framework == crawrad_codegen::AlvoGerador::Makepad,
        );

        let nome_exibicao = crawrad_project::project_template::name_or_default(&dados.nome);
        let mut tree = structure_preview(tpl, &nome_exibicao, dados.add_to.as_deref());
        if with_fw {
            // on the project's own manifest, the last one (the first is the workspace's)
            if let Some(at) = tree.rfind("Cargo.toml") {
                tree.insert_str(at + "Cargo.toml".len(), &format!("      {fw_dep}"));
            }
        }
        dlg.label(id!(np_det_tree)).set_text(cx, &tree);

        dlg.check_box(id!(np_opt_example))
            .set_active(cx, dados.opt_example);
        dlg.check_box(id!(np_opt_livereload))
            .set_active(cx, dados.opt_livereload);
        dlg.check_box(id!(np_opt_menu))
            .set_active(cx, dados.opt_menu);
        dlg.check_box(id!(np_opt_tracing))
            .set_active(cx, dados.opt_tracing);

        // Bottom Configuration Panel
        let campo_nome = dlg.text_input(id!(np_nome));
        campo_nome.apply_over(cx, live! { empty_text: ("my_app") });
        if campo_nome.text() != dados.nome {
            campo_nome.set_text(cx, &dados.nome);
        }

        let pasta_str = dados
            .pasta
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        let campo_loc = dlg.text_input(id!(np_loc));
        if campo_loc.text() != pasta_str {
            campo_loc.set_text(cx, &pasta_str);
        }

        dlg.check_box(id!(np_git_init))
            .set_active(cx, dados.git_init);

        let tpl_source_lbl = if dados.fonte_templates == 1 {
            "Custom Templates ▾"
        } else {
            "Built-in Templates ▾"
        };
        dlg.button(id!(np_tpl_source)).set_text(cx, tpl_source_lbl);

        // Validation and refusal
        let recusa = crawrad_project::workspace::refusal_typed_name(&nome_exibicao);
        let frase = match (&recusa, self.np_recusa.as_deref()) {
            (None, Some(anterior)) => anterior.to_string(),
            (Some(f), _) => f.clone(),
            (None, None) => {
                if let Some(p) = &dados.pasta {
                    format!(
                        "Will create project `{}` inside `{}`",
                        nome_exibicao,
                        p.display()
                    )
                } else {
                    format!(
                        "Will create project `{}` (Select destination folder)",
                        nome_exibicao
                    )
                }
            }
        };
        dlg.label(id!(np_recusa)).set_text(cx, &frase);
        Self::habilitar(cx, &dlg.widget(id!(np_escolher)), recusa.is_none());
    }

    #[note("ide-app-dialogs-new-project-ideapp-atender-novo-projeto")]
    pub(in super::super) fn atender_novo_projeto(&mut self, cx: &mut Cx, actions: &Actions) {
        let Some(mut dados) = self.novo_projeto.clone() else {
            return;
        };
        let dlg = self.ui.view(id!(newproject_dialog.dlg));
        let mut mudou = false;

        // Close via [X] in title bar
        if dlg.button(id!(np_close)).clicked(actions) {
            self.novo_projeto = None;
            self.refresh(cx);
            return;
        }

        // Search
        if let Some(t) = dlg.text_input(id!(np_busca)).changed(actions) {
            dados.busca = t;
            mudou = true;
        }

        // a category row of the palette ListBox
        let picked = dlg
            .widget(id!(np_cat_list))
            .as_crawrad_list_box()
            .selected(actions);
        if let Some(i) = picked.filter(|&i| i != dados.categoria_filtro) {
            dados.categoria_filtro = i;
            mudou = true;
        }

        // Favorite stars on cards
        for i in 0..TEMPLATES.len() {
            if dlg.button(&np_id(format!("np_c{i}_fav"))).clicked(actions) {
                if dados.favoritos.contains(&i) {
                    dados.favoritos.remove(&i);
                } else {
                    dados.favoritos.insert(i);
                }
                mudou = true;
            }
        }

        // a template card's click (the palette's Card)
        for i in 0..TEMPLATES.len() {
            let id = np_id(format!("np_card_{i}"));
            if dlg.widget(&id).as_crawrad_form_card().clicked(actions)
                && dados.template_selecionado != i
            {
                dados.template_selecionado = i;
                mudou = true;
            }
        }

        // A row of the palette ListBox; a planned entry is not selectable
        let picked = dlg
            .widget(id!(np_fw_list))
            .as_crawrad_list_box()
            .selected(actions);
        if let Some(fw) = picked.and_then(|i| crawrad_project::frameworks::CATALOGUE.get(i)?.target)
        {
            mudou |= dados.framework != fw;
            dados.framework = fw;
        }

        // Template option checkboxes
        if let Some(v) = dlg.check_box(id!(np_opt_example)).changed(actions) {
            dados.opt_example = v;
            mudou = true;
        }
        if let Some(v) = dlg.check_box(id!(np_opt_livereload)).changed(actions) {
            dados.opt_livereload = v;
            mudou = true;
        }
        if let Some(v) = dlg.check_box(id!(np_opt_menu)).changed(actions) {
            dados.opt_menu = v;
            mudou = true;
        }
        if let Some(v) = dlg.check_box(id!(np_opt_tracing)).changed(actions) {
            dados.opt_tracing = v;
            mudou = true;
        }

        // Name
        if let Some(t) = dlg.text_input(id!(np_nome)).changed(actions) {
            dados.nome = t;
            self.np_recusa = None;
            mudou = true;
        }

        // Manual location
        if let Some(t) = dlg.text_input(id!(np_loc)).changed(actions) {
            let path = if t.trim().is_empty() {
                None
            } else {
                Some(PathBuf::from(t.trim()))
            };
            dados.pasta = path;
            mudou = true;
        }

        // Browse folder
        if dlg.button(id!(np_browse)).clicked(actions)
            && let Some(pasta) = rfd::FileDialog::new()
                .set_title("Select Project Location")
                .pick_folder()
        {
            dados.pasta = Some(pasta);
            mudou = true;
        }

        // Git init
        if let Some(v) = dlg.check_box(id!(np_git_init)).changed(actions) {
            dados.git_init = v;
            mudou = true;
        }

        // Template source switch
        if dlg.button(id!(np_tpl_source)).clicked(actions) {
            dados.fonte_templates = (dados.fonte_templates + 1) % 2;
            if dados.fonte_templates == 1 {
                dados.categoria_filtro = 7; // Examples / Custom
                dados.template_selecionado = 7;
            } else {
                dados.categoria_filtro = 1; // Desktop
                dados.template_selecionado = 0;
            }
            mudou = true;
        }

        // Cancel
        if dlg.button(id!(np_cancel)).clicked(actions) {
            self.novo_projeto = None;
            self.refresh(cx);
            return;
        }

        // Create Project
        let pelo_enter = dlg.text_input(id!(np_nome)).returned(actions).is_some();
        if dlg.button(id!(np_escolher)).clicked(actions) || pelo_enter {
            self.execute_project_creation(cx, &dados);
            return;
        }

        if mudou {
            self.novo_projeto = Some(dados);
            self.refresh(cx);
        }
    }

    #[note("ide-app-dialogs-new-project-ideapp-execute-project-creation")]
    pub(in super::super) fn execute_project_creation(
        &mut self,
        cx: &mut Cx,
        dados: &NovoProjetoDados,
    ) {
        let nome = crawrad_project::project_template::name_or_default(&dados.nome);
        if let Some(frase) = crawrad_project::workspace::refusal_typed_name(&nome) {
            self.np_recusa = Some(frase);
            self.refresh(cx);
            return;
        }

        // `Create` creates. A folder that does not exist yet is made here; it used
        // to fall through to a file picker, so editing the drive letter of the
        // default location turned Create into a folder chooser with nothing said
        // about why. Choosing is what the folder button beside the field is for.
        let Some(pasta) = dados.pasta.clone() else {
            self.np_recusa = Some(crawrad_project::templates::NP_SEM_PASTA.into());
            self.refresh(cx);
            return;
        };
        if let Err(refusal) =
            crawrad_project::workspace::make_project_parent(&pasta, install_folder().as_deref())
        {
            self.np_recusa = Some(refusal.phrase);
            self.refresh(cx);
            return;
        }

        crawrad_project::generator::set_active_target(dados.framework);

        let Some(tpl) = TEMPLATES.get(dados.template_selecionado) else {
            self.np_recusa = Some(NP_SEM_MODELO.into());
            self.sincronizar(cx);
            return;
        };
        let tipo = tpl.base_type;

        let (tx, rx) = std::sync::mpsc::channel();
        let raiz = pasta.join(&nome);
        let git_init = dados.git_init;
        let framework = dados.framework;

        let tpl_id = tpl.id;
        let nome_da_thread = nome.to_string();
        let pasta_thread = pasta.clone();
        let add_to = dados.add_to.clone();

        self.creation_tasks.spawn("new-project", move |reporter| {
            let relatar = |p: crawrad_project::creation::CreationStep| {
                let _ = tx.send(crawrad_project::creation::CreationProgress::Running(p));
                crawrad_os::tasks::wake();
            };
            relatar(crawrad_project::creation::CreationStep::Started(
                crawrad_project::workspace::Step::Package,
            ));

            let created = crawrad_project::project_template::create_or_add(
                add_to.as_deref(),
                &pasta_thread,
                &nome_da_thread,
                tpl_id,
                git_init,
                framework,
            );
            let veredito = match created {
                Ok(r) => {
                    relatar(crawrad_project::creation::CreationStep::Started(
                        crawrad_project::workspace::Step::Code,
                    ));
                    Ok(crawrad_project::creation::CreatedProject {
                        root: r,
                        warning: None,
                    })
                }
                Err(e) => Err(crawrad_project::workspace::Refusal {
                    phrase: e,
                    open_instead: false,
                }),
            };

            let _ = tx.send(crawrad_project::creation::CreationProgress::Done(veredito));
            crawrad_os::tasks::wake();
            reporter.done(());
        });

        self.novo_projeto = None;
        self.criando = Some(crawrad_project::creation::Creation {
            rx,
            project_type: tipo,
            root: raiz,
            plan: Vec::new(),
            completed: 0,
            current: None,
            since: std::time::Instant::now(),
            outcome: None,
            show: true,
        });
        self.status = Some(format!("creating {} project with cargo…", tipo.label()));
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-new-project-ideapp-bombear-criacao")]
    pub(in super::super) fn bombear_criacao(&mut self, cx: &mut Cx) {
        let _ = self.creation_tasks.drain();
        let Some(c) = self.criando.as_mut() else {
            return;
        };
        let mut acabou = None;
        while let Ok(a) = c.rx.try_recv() {
            match a {
                crawrad_project::creation::CreationProgress::Running(p) => c.advance(p),
                crawrad_project::creation::CreationProgress::Done(v) => acabou = Some(v),
            }
        }
        if let Some(veredito) = acabou {
            let raiz = c.root.clone();
            let desfecho = crawrad_project::creation::creation_outcome(&raiz, veredito, Ok(()));
            if let Some(abrir) = desfecho.root_to_open().map(std::path::Path::to_path_buf) {
                self.open_created_project(cx, &abrir);
            }
            if let Some(frase) = desfecho.phrase() {
                self.status = Some(frase.to_string());
            }
            let fica = self.criando.as_mut().is_some_and(|c| c.settle(desfecho));
            if !fica {
                self.criando = None;
            }
        }
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-new-project-ideapp-open-created-project")]
    pub fn open_created_project(&mut self, cx: &mut Cx, raiz: &std::path::Path) {
        let (registry, notice) = crawrad_project::installed::open_project(raiz);
        self.designer.registry = Some(registry);
        self.close_current_project(cx);
        let doc = crawrad_project::workspace::documents_in(raiz)
            .into_iter()
            .next();
        match doc {
            Some(d) => {
                self.abrir_formulario(cx, &d);
                self.show_center(cx, self.centro_das_abas());
            }
            None => {
                // No form in it: the form open before must not stay the document.
                self.host.set_doc(None);
                let main_rs = raiz.join("src").join("main.rs");
                let lib_rs = raiz.join("src").join("lib.rs");
                if main_rs.exists() {
                    self.host.set_active_file(Some(main_rs.clone()));
                    self.abrir_no_editor(cx, main_rs);
                } else if lib_rs.exists() {
                    self.host.set_active_file(Some(lib_rs.clone()));
                    self.abrir_no_editor(cx, lib_rs);
                } else {
                    self.host.set_active_file(Some(raiz.to_path_buf()));
                }
                self.status = Some(format!("{} is ready", raiz.display()));
            }
        }
        self.status = crawrad_project::installed::with_notice(self.status.take(), notice);
        self.restore_desktop(cx, raiz, false);
        let instance = std::env::var("CRAWRAD_INSTANCE").unwrap_or_else(|_| "default".to_string());
        crate::ide_app::mcp_host::sync_workspace_mcp_config(raiz, &instance);
        self.refresh_notes(cx);
        self.redraw_panel_list(cx, Panel::Explorer, id!(explorer_body.explorer_list));
    }

    #[note("ide-app-dialogs-new-project-ideapp-draw-creation")]
    pub(in super::super) fn draw_creation(&mut self, cx: &mut Cx) {
        let aberto = self.criando.as_ref().is_some_and(|c| c.show);
        self.show_veil(cx, id!(creating_dialog), aberto);
        let Some(c) = self.criando.as_ref() else {
            return;
        };
        if !c.show {
            return;
        }
        let onde = c.root.display().to_string();
        let passo = c.phrase();
        let tempo = format!("{:.0}s", c.since.elapsed().as_secs_f32());
        let desfecho = c
            .outcome
            .as_ref()
            .and_then(crawrad_project::creation::Outcome::on_screen)
            .map(|(frase, _)| frase)
            .unwrap_or_default();
        let dlg = self.ui.view(id!(creating_dialog.dlg));
        dlg.label(id!(cr_onde)).set_text(cx, &onde);
        dlg.label(id!(cr_passo)).set_text(cx, &passo);
        dlg.label(id!(cr_tempo)).set_text(cx, &tempo);
        dlg.label(id!(cr_desfecho)).set_text(cx, &desfecho);
    }

    #[note("ide-app-dialogs-new-project-ideapp-atender-criacao")]
    pub(in super::super) fn atender_criacao(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.criando.is_none() {
            return;
        }
        if self
            .ui
            .view(id!(creating_dialog.dlg))
            .button(id!(cr_fechar))
            .clicked(actions)
        {
            let terminou = self.criando.as_ref().is_some_and(|c| c.outcome.is_some());
            if terminou {
                self.criando = None;
            } else if let Some(c) = self.criando.as_mut() {
                c.show = false;
            }
            self.refresh(cx);
        }
    }
}

use crawrad_note::note;
pub use crawrad_project::project_template::create_project_template;
