//! **What is inside each dialog, for `view.get_dialog`**.
//!
//! The MCP opens every dialog (`tools.options`, `project.build_manager`,
//! `inspector.open_editor`…), but nothing could read what is INSIDE one. The
//! owner's order is the whole IDE operable from outside, for short videos that
//! operate it: a script reads a dialog's controls with their rects and acts on
//! them with the real mouse and keyboard, which is what a video shows anyway.
//!
//! Makepad 1.0 keeps a `View`'s children private (`makepad-widgets-1.0.0/src/
//! view.rs:182`; only `child_count` is public), so a dialog cannot be walked
//! from outside: each one DECLARES its controls here, once, by widget name. A
//! row repeated N times is one entry with `{i}` in its name and a count.

use crawrad_note::note;
#[note("dialogs-controlkind")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlKind {
    /// A button: its text.
    Button,
    /// A check box: its text and whether it is ticked.
    Check,
    /// A text field: its text.
    Text,
    /// A label: its text.
    Label,
    /// A clickable row (a `View`), its text read from a label inside it.
    Row,
    /// A drop-down: its selected item's text; a click opens its list.
    Drop,
}

#[note("dialogs-controlspec")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlSpec {
    pub name: &'static str,
    pub kind: ControlKind,
    pub count: usize,
    pub text_of: &'static str,
}

const fn one(name: &'static str, kind: ControlKind) -> ControlSpec {
    ControlSpec {
        name,
        kind,
        count: 1,
        text_of: "",
    }
}

const fn rows(name: &'static str, text_of: &'static str, count: usize) -> ControlSpec {
    ControlSpec {
        name,
        kind: ControlKind::Row,
        count,
        text_of,
    }
}

const fn many(name: &'static str, kind: ControlKind, count: usize) -> ControlSpec {
    ControlSpec {
        name,
        kind,
        count,
        text_of: "",
    }
}

#[note("dialogs-control")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Control {
    pub name: String,
    pub kind: ControlKind,
    pub text_of: Option<String>,
}

#[note("dialogs-dialogspec")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DialogSpec {
    pub name: &'static str,
    /// The dialog's root widget path (`["newproject_dialog", "dlg"]`).
    pub root: [&'static str; 2],
    pub controls: &'static [ControlSpec],
}

impl DialogSpec {
    #[note("dialogs-dialogspec-expanded")]
    pub fn expanded(&self) -> Vec<Control> {
        let mut out = Vec::new();
        for c in self.controls {
            for i in 0..c.count {
                let at = |s: &str| s.replace("{i}", &i.to_string());
                let text_of = (!c.text_of.is_empty()).then(|| at(c.text_of));
                out.push(Control {
                    name: at(c.name),
                    kind: c.kind,
                    text_of,
                });
            }
        }
        out
    }
}

/// While a dialog is open, an action that would open another one is refused.
/// A dialog action is one whose label ends in an ellipsis (`Options...`).
pub fn modal_refusal(modal_open: bool, label: &str) -> Option<&'static str> {
    let opens_dialog = label.ends_with("...") || label.ends_with('\u{2026}');
    (modal_open && opens_dialog).then_some("a dialog is open: close it first")
}

/// The veil widget of every dialog: the described ones, plus Shrink and
/// Environment, which have no description.
pub fn veil_roots() -> impl Iterator<Item = &'static str> {
    let described = all().into_iter().map(|d| d.root[0]);
    described.chain(["shrink_dialog", "environment_dialog"])
}

impl DialogSpec {
    /// The button Esc presses: the Cancel, or the Close when there is none
    /// (the shortest name wins: `opt_cancel` over `opt_color_cancel`).
    pub fn cancel_control(&self) -> Option<&'static str> {
        let pick = |suffix: &str| {
            let names = self.controls.iter().map(|c| c.name);
            names
                .filter(|n| n.ends_with(suffix))
                .min_by_key(|n| n.len())
        };
        pick("_cancel").or_else(|| pick("_close"))
    }
}

#[note("dialogs-controlstate")]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ControlState {
    pub text: String,
    pub rect: [f64; 4],
    pub visible: bool,
    /// A check box's tick; `None` for any other kind.
    pub checked: Option<bool>,
    /// A drop-down's items, in order; empty for any other kind.
    pub items: Vec<String>,
}

#[note("dialogs-describe")]
pub fn describe(
    spec: &DialogSpec,
    rows: &[(String, [f64; 4])],
    read: impl Fn(&Control) -> ControlState,
) -> serde_json::Value {
    let controls: Vec<_> = spec
        .expanded()
        .iter()
        .map(|c| {
            let s = read(c);
            let kind = format!("{:?}", c.kind).to_lowercase();
            let mut state = serde_json::json!({"name": c.name, "kind": kind, "text": s.text, "rect": s.rect, "visible": s.visible, "checked": s.checked});
            if !s.items.is_empty() {
                state["items"] = serde_json::json!(s.items);
            }
            state
        })
        .collect();
    // the host passes only the rows drawn in THIS dialog (the Options page
    // tree, a palette ListBox like New Project's framework list)
    let rows: Vec<_> = rows
        .iter()
        .map(|(text, rect)| serde_json::json!({"text": text, "rect": rect}))
        .collect();
    serde_json::json!({ "open": spec.name, "controls": controls, "rows": rows })
}

#[note("dialogs-new-project")]
pub const NEW_PROJECT: DialogSpec = DialogSpec {
    name: "new_project",
    root: ["newproject_dialog", "dlg"],
    controls: &[
        one("np_title", ControlKind::Label),
        one("np_busca", ControlKind::Text),
        one("np_close", ControlKind::Button),
        one("np_cat_list", ControlKind::Label),
        rows("np_card_{i}", "np_c{i}_title", 8),
        many("np_c{i}_fav", ControlKind::Button, 8),
        one("np_det_title", ControlKind::Label),
        one("np_det_tree", ControlKind::Label),
        // The UI framework cards (a template with a window only)
        one("np_fw_list", ControlKind::Label),
        one("np_fw_det_summary", ControlKind::Label),
        one("np_fw_det_platforms", ControlKind::Label),
        one("np_fw_det_crate", ControlKind::Label),
        one("np_opt_example", ControlKind::Check),
        one("np_opt_livereload", ControlKind::Check),
        one("np_opt_menu", ControlKind::Check),
        one("np_opt_tracing", ControlKind::Check),
        one("np_nome", ControlKind::Text),
        one("np_loc", ControlKind::Text),
        one("np_browse", ControlKind::Button),
        one("np_git_init", ControlKind::Check),
        one("np_tpl_source", ControlKind::Button),
        one("np_escolher", ControlKind::Button),
        one("np_cancel", ControlKind::Button),
        one("np_recusa", ControlKind::Label),
    ],
};

#[note("dialogs-all")]
pub fn all() -> Vec<DialogSpec> {
    let (over, rest): (Vec<_>, Vec<_>) = crate::dialogs_table::GENERATED
        .into_iter()
        .partition(|d| d.name == crate::dialogs_table::COLOR.name);
    let mut all = over;
    all.extend([NEW_PROJECT, BUILD, OPTIONS]);
    all.extend(rest);
    all
}

#[note("dialogs-options")]
pub const OPTIONS: DialogSpec = DialogSpec {
    name: "options",
    root: ["options_dialog", "dlg"],
    controls: &[
        one("opt_title", ControlKind::Label),
        one("opt_close", ControlKind::Button),
        one("opt_search_input", ControlKind::Text),
        one("opt_search_clear", ControlKind::Button),
        one("opt_scope_user", ControlKind::Button),
        one("opt_scope_workspace", ControlKind::Button),
        one("opt_cu_font_size", ControlKind::Drop),
        one("opt_cu_line_numbers", ControlKind::Check),
        one("opt_cu_save_build", ControlKind::Check),
        one("opt_cu_ver_grade", ControlKind::Check),
        one("opt_cu_snap", ControlKind::Check),
        one("opt_cu_ai_provider", ControlKind::Drop),
        one("opt_sr_font_size", ControlKind::Drop),
        one("opt_sr_line_numbers", ControlKind::Check),
        one("opt_sr_save_build", ControlKind::Check),
        one("opt_sr_ver_grade", ControlKind::Check),
        one("opt_sr_snap", ControlKind::Check),
        one("opt_sr_ai_provider", ControlKind::Drop),
        one("opt_sr_keymap_scheme", ControlKind::Drop),
        one("opt_ver_grade", ControlKind::Check),
        one("opt_snap", ControlKind::Check),
        one("opt_grade_x", ControlKind::Text),
        one("opt_grade_menos", ControlKind::Button),
        one("opt_grade_mais", ControlKind::Button),
        one("opt_icones", ControlKind::Button),
        one("opt_keymap_scheme", ControlKind::Drop),
        one("opt_ai_model", ControlKind::Drop),
        one("opt_ai_effort", ControlKind::Drop),
        one("opt_ai_status_text", ControlKind::Label),
        one("opt_ai_status_hint", ControlKind::Label),
        one("opt_ai_active_cmd", ControlKind::Label),
        one("opt_reg_feedback", ControlKind::Label),
        one("opt_ai_fix_btn", ControlKind::Button),
        one("opt_ai_change_btn", ControlKind::Button),
        one("opt_ai_adv_btn", ControlKind::Button),
        one("opt_ai_test_btn", ControlKind::Button),
        one("opt_ai_term_btn", ControlKind::Button),
        one("opt_ai_auto_approve", ControlKind::Check),
        one("opt_ai_native_tools", ControlKind::Check),
        one("opt_ai_back_btn", ControlKind::Button),
        one("opt_reg_link", ControlKind::Button),
        one("opt_reg_search", ControlKind::Text),
        one("opt_reg_clear", ControlKind::Button),
        one("opt_ai_provider", ControlKind::Drop),
        one("opt_ai_endpoint", ControlKind::Text),
        one("opt_editor_font_size", ControlKind::Drop),
        one("opt_editor_tab_size", ControlKind::Drop),
        one("opt_editor_line_numbers", ControlKind::Check),
        one("opt_editor_inlay_hints", ControlKind::Check),
        one("opt_cs_preset", ControlKind::Drop),
        one("opt_cs_reset_btn", ControlKind::Button),
        one("opt_cs_kw_val", ControlKind::Text),
        one("opt_cs_type_val", ControlKind::Text),
        one("opt_cs_fn_val", ControlKind::Text),
        one("opt_cs_id_val", ControlKind::Text),
        one("opt_cs_num_val", ControlKind::Text),
        one("opt_cs_str_val", ControlKind::Text),
        one("opt_cs_com_val", ControlKind::Text),
        one("opt_cs_delim_val", ControlKind::Text),
        one("opt_cs_bg_val", ControlKind::Text),
        one("opt_cs_diag_err_val", ControlKind::Text),
        one("opt_cs_diag_warn_val", ControlKind::Text),
        one("opt_cs_diag_info_val", ControlKind::Text),
        one("opt_cs_diag_err_band_val", ControlKind::Text),
        one("opt_cs_diag_warn_band_val", ControlKind::Text),
        one("opt_cs_diag_info_band_val", ControlKind::Text),
        one("opt_cs_kw_more", ControlKind::Button),
        one("opt_cs_type_more", ControlKind::Button),
        one("opt_cs_fn_more", ControlKind::Button),
        one("opt_cs_id_more", ControlKind::Button),
        one("opt_cs_num_more", ControlKind::Button),
        one("opt_cs_str_more", ControlKind::Button),
        one("opt_cs_com_more", ControlKind::Button),
        one("opt_cs_delim_more", ControlKind::Button),
        one("opt_cs_bg_more", ControlKind::Button),
        one("opt_cs_diag_err_more", ControlKind::Button),
        one("opt_cs_diag_warn_more", ControlKind::Button),
        one("opt_cs_diag_info_more", ControlKind::Button),
        one("opt_cs_diag_err_band_more", ControlKind::Button),
        one("opt_cs_diag_warn_band_more", ControlKind::Button),
        one("opt_cs_diag_info_band_more", ControlKind::Button),
        one("opt_cs_kw_err", ControlKind::Label),
        one("opt_cs_type_err", ControlKind::Label),
        one("opt_cs_fn_err", ControlKind::Label),
        one("opt_cs_id_err", ControlKind::Label),
        one("opt_cs_num_err", ControlKind::Label),
        one("opt_cs_str_err", ControlKind::Label),
        one("opt_cs_com_err", ControlKind::Label),
        one("opt_cs_delim_err", ControlKind::Label),
        one("opt_cs_bg_err", ControlKind::Label),
        one("opt_cs_diag_err_err", ControlKind::Label),
        one("opt_cs_diag_warn_err", ControlKind::Label),
        one("opt_cs_diag_info_err", ControlKind::Label),
        one("opt_cs_diag_err_band_err", ControlKind::Label),
        one("opt_cs_diag_warn_band_err", ControlKind::Label),
        one("opt_cs_diag_info_band_err", ControlKind::Label),
        one("opt_default_dir_input", ControlKind::Text),
        one("opt_default_dir_btn", ControlKind::Button),
        one("opt_history_enabled", ControlKind::Check),
        one("opt_history_limit", ControlKind::Drop),
        one("opt_recover", ControlKind::Check),
        one("opt_recover_minutes", ControlKind::Drop),
        one("opt_history_merge", ControlKind::Drop),
        one("opt_history_max", ControlKind::Drop),
        one("opt_reopen_last", ControlKind::Check),
        one("opt_auto_save", ControlKind::Drop),
        one("opt_save_desktop", ControlKind::Check),
        one("opt_save_build", ControlKind::Check),
        one("opt_profile", ControlKind::Drop),
        one("opt_profile_save_as", ControlKind::Button),
        one("opt_profile_delete", ControlKind::Button),
        one("opt_profile_default", ControlKind::Button),
        one("opt_profile_name", ControlKind::Text),
        one("opt_profile_name_ok", ControlKind::Button),
        one("opt_profile_name_cancel", ControlKind::Button),
        one("opt_save", ControlKind::Button),
        one("opt_cancel", ControlKind::Button),
    ],
};

#[note("dialogs-build")]
pub const BUILD: DialogSpec = DialogSpec {
    name: "build",
    root: ["build_dialog", "dlg"],
    controls: &[
        one("bld_title", ControlKind::Label),
        one("bld_close", ControlKind::Button),
        one("bld_mode_build", ControlKind::Button),
        one("bld_profile_btn", ControlKind::Button),
        one("bld_btn_check", ControlKind::Button),
        one("bld_btn_clean", ControlKind::Button),
        one("bld_btn_size", ControlKind::Button),
        one("bld_btn_help", ControlKind::Button),
        one("bld_targets", ControlKind::Label),
        one("bld_btn_select_all", ControlKind::Button),
        one("bld_btn_deselect_all", ControlKind::Button),
        one("bld_btn_preset_desktop", ControlKind::Button),
        one("bld_btn_preset_mobile", ControlKind::Button),
        one("bld_btn_preset_web", ControlKind::Button),
        one("bld_chips", ControlKind::Label),
        one("bld_chip_empty", ControlKind::Label),
        one("bld_opt_all_features", ControlKind::Check),
        one("bld_opt_no_default", ControlKind::Check),
        one("bld_opt_clean", ControlKind::Check),
        one("bld_opt_verbose", ControlKind::Check),
        one("bld_opt_workspace", ControlKind::Check),
        one("bld_opt_messages", ControlKind::Check),
        one("bld_features_input", ControlKind::Text),
        one("bld_args_input", ControlKind::Text),
        one("bld_env_add", ControlKind::Button),
        one("bld_tab_output", ControlKind::Button),
        one("bld_tab_errors", ControlKind::Button),
        one("bld_tab_warnings", ControlKind::Button),
        one("bld_out_copy", ControlKind::Button),
        one("bld_out_clear", ControlKind::Button),
        one("bld_btn_close", ControlKind::Button),
    ],
};

#[cfg(test)]
#[path = "test/dialogs/test.rs"]
mod tests;
