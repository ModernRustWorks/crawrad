//! **Generated** by `tools/gen_dialog_table.py` from the shell's dialog DSL:
//! what is inside each dialog `view.get_dialog` reads. Do not edit;
//! run the generator, then `cargo fmt`. `every_generated_dialog_matches_its_dsl`
//! fails when
//! this table and the DSL drift apart.

use super::dialogs::{ControlKind, ControlSpec, DialogSpec};
use crawrad_note::note;

const fn one(name: &'static str, kind: ControlKind) -> ControlSpec {
    ControlSpec {
        name,
        kind,
        count: 1,
        text_of: "",
    }
}

#[note("dialogs-table-items")]
pub const ITEMS: DialogSpec = DialogSpec {
    name: "items",
    root: ["items_dialog", "dlg"],
    controls: &[
        one("items_title", ControlKind::Label),
        one("items_owner", ControlKind::Label),
        one("btn_new_item", ControlKind::Button),
        one("btn_new_subitem", ControlKind::Button),
        one("btn_delete", ControlKind::Button),
        one("btn_move_up", ControlKind::Button),
        one("btn_move_down", ControlKind::Button),
        one("text_label", ControlKind::Label),
        one("item_text", ControlKind::Text),
        one("image_label", ControlKind::Label),
        one("item_image", ControlKind::Text),
        one("item_expanded", ControlKind::Check),
        one("props_note", ControlKind::Label),
        one("items_ok", ControlKind::Button),
        one("items_cancel", ControlKind::Button),
        one("items_apply", ControlKind::Button),
    ],
};

#[note("dialogs-table-lines")]
pub const LINES: DialogSpec = DialogSpec {
    name: "lines",
    root: ["lines_dialog", "dlg"],
    controls: &[
        one("lines_title", ControlKind::Label),
        one("lines_owner", ControlKind::Label),
        one("lines_text", ControlKind::Text),
        one("lines_count", ControlKind::Label),
        one("lines_ok", ControlKind::Button),
        one("lines_cancel", ControlKind::Button),
        one("lines_apply", ControlKind::Button),
    ],
};

#[note("dialogs-table-rename")]
pub const RENAME: DialogSpec = DialogSpec {
    name: "rename",
    root: ["rename_dialog", "dlg"],
    controls: &[
        one("rename_title", ControlKind::Label),
        one("rename_what", ControlKind::Label),
        one("rename_input", ControlKind::Text),
        one("rename_preview", ControlKind::Label),
        one("rename_ok", ControlKind::Button),
        one("rename_cancel", ControlKind::Button),
    ],
};

#[note("dialogs-table-component-rename")]
pub const COMPONENT_RENAME: DialogSpec = DialogSpec {
    name: "component_rename",
    root: ["component_rename_dialog", "dlg"],
    controls: &[
        one("comp_rename_title", ControlKind::Label),
        one("check_comp", ControlKind::Check),
        one("detail_comp", ControlKind::Label),
        one("check_handlers", ControlKind::Check),
        one("detail_handlers", ControlKind::Label),
        one("check_refs", ControlKind::Check),
        one("detail_refs", ControlKind::Label),
        one("comp_rename_footer_note", ControlKind::Label),
        one("comp_rename_all", ControlKind::Button),
        one("comp_rename_cancel", ControlKind::Button),
    ],
};

#[note("dialogs-table-actions")]
pub const ACTIONS: DialogSpec = DialogSpec {
    name: "actions",
    root: ["actions_dialog", "dlg"],
    controls: &[
        one("act_back", ControlKind::Button),
        one("crumb_prev", ControlKind::Label),
        one("actions_title", ControlKind::Label),
        one("act_new", ControlKind::Button),
        one("act_standard", ControlKind::Button),
        one("act_delete", ControlKind::Button),
        one("act_up", ControlKind::Button),
        one("act_down", ControlKind::Button),
        one("act_fields", ControlKind::Button),
        one("act_footer_note", ControlKind::Label),
        one("act_close", ControlKind::Button),
    ],
};

#[note("dialogs-table-standard")]
pub const STANDARD: DialogSpec = DialogSpec {
    name: "standard",
    root: ["standard_dialog", "dlg"],
    controls: &[
        one("std_title", ControlKind::Label),
        one("marca", ControlKind::Check),
        one("nome", ControlKind::Label),
        one("std_ok", ControlKind::Button),
        one("std_cancel", ControlKind::Button),
    ],
};

#[note("dialogs-table-imagelist")]
pub const IMAGELIST: DialogSpec = DialogSpec {
    name: "imagelist",
    root: ["imagelist_dialog", "dlg"],
    controls: &[
        one("iml_title", ControlKind::Label),
        one("iml_add", ControlKind::Button),
        one("iml_replace", ControlKind::Button),
        one("iml_delete", ControlKind::Button),
        one("iml_clear", ControlKind::Button),
        one("iml_size_label", ControlKind::Label),
        one("iml_size", ControlKind::Drop),
        one("iml_count", ControlKind::Label),
        one("iml_index", ControlKind::Label),
        one("iml_name", ControlKind::Label),
        one("iml_size_of", ControlKind::Label),
        one("iml_previa", ControlKind::Label),
        one("iml_nota", ControlKind::Label),
        one("iml_ok", ControlKind::Button),
        one("iml_cancel", ControlKind::Button),
        one("iml_apply", ControlKind::Button),
    ],
};

#[note("dialogs-table-addicon")]
pub const ADDICON: DialogSpec = DialogSpec {
    name: "addicon",
    root: ["addicon_dialog", "dlg"],
    controls: &[
        one("add_title", ControlKind::Label),
        one("novo_nome", ControlKind::Text),
        one("novo_simbolo", ControlKind::Text),
        one("novo_categoria", ControlKind::Text),
        one("novo_add", ControlKind::Button),
        one("add_ok", ControlKind::Button),
        one("add_close", ControlKind::Button),
    ],
};

#[note("dialogs-table-quit")]
pub const QUIT: DialogSpec = DialogSpec {
    name: "quit",
    root: ["quit_dialog", "dlg"],
    controls: &[
        one("quit_title", ControlKind::Label),
        one("quit_icone", ControlKind::Label),
        one("quit_pergunta", ControlKind::Label),
        one("quit_arquivo", ControlKind::Label),
        one("quit_aviso", ControlKind::Label),
        one("quit_save", ControlKind::Button),
        one("quit_discard", ControlKind::Button),
        one("quit_cancel", ControlKind::Button),
    ],
};

#[note("dialogs-table-recover")]
pub const RECOVER: DialogSpec = DialogSpec {
    name: "recover",
    root: ["recover_dialog", "dlg"],
    controls: &[
        one("marca", ControlKind::Check),
        one("file", ControlKind::Label),
        one("copy", ControlKind::Label),
        one("disk", ControlKind::Label),
        one("rec_recover", ControlKind::Button),
        one("rec_discard", ControlKind::Button),
        one("rec_later", ControlKind::Button),
    ],
};

#[note("dialogs-table-project-options")]
pub const PROJECT_OPTIONS: DialogSpec = DialogSpec {
    name: "project_options",
    root: ["project_options_dialog", "dlg"],
    controls: &[
        one("popt_title", ControlKind::Label),
        one("popt_close", ControlKind::Button),
        one("popt_page_app", ControlKind::Row),
        one("popt_page_log", ControlKind::Row),
        one("popt_title_input", ControlKind::Text),
        one("popt_load_btn", ControlKind::Button),
        one("popt_default_btn", ControlKind::Button),
        one("popt_caption_lbl", ControlKind::Label),
        one("plog_enabled", ControlKind::Check),
        one("plog_name", ControlKind::Text),
        one("plog_m0", ControlKind::Button),
        one("plog_m1", ControlKind::Button),
        one("plog_m2", ControlKind::Button),
        one("plog_m3", ControlKind::Button),
        one("plog_m4", ControlKind::Button),
        one("plog_today", ControlKind::Label),
        one("plog_f0", ControlKind::Button),
        one("plog_f1", ControlKind::Button),
        one("plog_user_dir", ControlKind::Label),
        one("plog_f2", ControlKind::Button),
        one("plog_custom", ControlKind::Text),
        one("plog_browse", ControlKind::Button),
        one("plog_debug", ControlKind::Drop),
        one("plog_release", ControlKind::Drop),
        one("plog_new", ControlKind::Drop),
        one("plog_keep", ControlKind::Text),
        one("plog_sample", ControlKind::Button),
        one("popt_error_lbl", ControlKind::Label),
        one("popt_ok", ControlKind::Button),
        one("popt_cancel", ControlKind::Button),
    ],
};

#[note("dialogs-table-color")]
pub const COLOR: DialogSpec = DialogSpec {
    name: "color",
    root: ["color_dialog", "dlg"],
    controls: &[
        one("color_title", ControlKind::Label),
        one("color_hex", ControlKind::Text),
        one("color_alpha", ControlKind::Text),
        one("color_red", ControlKind::Text),
        one("color_green", ControlKind::Text),
        one("color_blue", ControlKind::Text),
        one("color_error", ControlKind::Label),
        one("color_ok", ControlKind::Button),
        one("color_cancel", ControlKind::Button),
    ],
};

#[note("dialogs-table-creating")]
pub const CREATING: DialogSpec = DialogSpec {
    name: "creating",
    root: ["creating_dialog", "dlg"],
    controls: &[
        one("cr_title", ControlKind::Label),
        one("cr_onde", ControlKind::Label),
        one("cr_passo", ControlKind::Label),
        one("cr_tempo", ControlKind::Label),
        one("cr_desfecho", ControlKind::Label),
        one("cr_fechar", ControlKind::Button),
    ],
};

#[note("dialogs-table-icons")]
pub const ICONS: DialogSpec = DialogSpec {
    name: "icons",
    root: ["icons_dialog", "dlg"],
    controls: &[
        one("ic_title", ControlKind::Label),
        one("ic_nota", ControlKind::Label),
        one("ic_fechar", ControlKind::Button),
    ],
};

#[note("dialogs-table-memory")]
pub const MEMORY: DialogSpec = DialogSpec {
    name: "memory",
    root: ["memory_dialog", "dlg"],
    controls: &[
        one("mem_title", ControlKind::Label),
        one("mem_subtitle", ControlKind::Label),
        one("mem_input", ControlKind::Text),
        one("mem_counter", ControlKind::Label),
        one("mem_cancel", ControlKind::Button),
        one("mem_save", ControlKind::Button),
    ],
};

#[note("dialogs-table-mcp")]
pub const MCP: DialogSpec = DialogSpec {
    name: "mcp",
    root: ["mcp_dialog", "dlg"],
    controls: &[
        one("mcp_title", ControlKind::Label),
        one("mcp_subtitle", ControlKind::Label),
        one("mcp_servers_info", ControlKind::Label),
        one("mcp_close", ControlKind::Button),
    ],
};

#[note("dialogs-table-note")]
pub const NOTE: DialogSpec = DialogSpec {
    name: "note",
    root: ["note_dialog", "dlg"],
    controls: &[
        one("note_title", ControlKind::Label),
        one("note_subtitle", ControlKind::Label),
        one("note_badge", ControlKind::Label),
        one("note_location_right", ControlKind::Label),
        one("note_item", ControlKind::Label),
        one("note_code", ControlKind::Label),
        one("note_sec_summary", ControlKind::Button),
        one("note_sec_examples", ControlKind::Button),
        one("note_sec_errors", ControlKind::Button),
        one("note_sec_panics", ControlKind::Button),
        one("note_sec_safety", ControlKind::Button),
        one("note_sec_link", ControlKind::Button),
        one("note_suggest", ControlKind::Label),
        one("note_title_input", ControlKind::Text),
        one("note_tag_input", ControlKind::Text),
        one("note_tab_edit", ControlKind::Button),
        one("note_tab_preview", ControlKind::Button),
        one("note_tab_hint", ControlKind::Label),
        one("note_content_input", ControlKind::Text),
        one("note_warning", ControlKind::Label),
        one("note_status", ControlKind::Label),
        one("note_resolve", ControlKind::Button),
        one("note_test_result", ControlKind::Label),
        one("note_test", ControlKind::Button),
        one("note_cancel", ControlKind::Button),
        one("note_save", ControlKind::Button),
    ],
};

#[note("dialogs-table-goto")]
pub const GOTO: DialogSpec = DialogSpec {
    name: "goto",
    root: ["goto_dialog", "dlg"],
    controls: &[
        one("goto_title", ControlKind::Label),
        one("goto_help", ControlKind::Label),
        one("goto_input", ControlKind::Text),
        one("goto_confirm", ControlKind::Button),
        one("goto_cancel", ControlKind::Button),
    ],
};

#[note("dialogs-table-quick-open")]
pub const QUICK_OPEN: DialogSpec = DialogSpec {
    name: "quick_open",
    root: ["quick_open_dialog", "dlg"],
    controls: &[
        one("quick_title", ControlKind::Label),
        one("quick_input", ControlKind::Text),
        one("quick_cancel", ControlKind::Button),
    ],
};

#[note("dialogs-table-inherit-form")]
pub const INHERIT_FORM: DialogSpec = DialogSpec {
    name: "inherit_form",
    root: ["inherit_form_dialog", "dlg"],
    controls: &[
        one("inh_close", ControlKind::Button),
        one("inh_filter", ControlKind::Text),
        one("inh_summary", ControlKind::Label),
        one("inh_parts", ControlKind::Label),
        one("inh_name", ControlKind::Text),
        one("inh_problem", ControlKind::Label),
        one("inh_files", ControlKind::Label),
        one("inh_struct", ControlKind::Label),
        one("inh_cancel", ControlKind::Button),
        one("inh_create", ControlKind::Button),
        one("nc_pname", ControlKind::Text),
        one("nc_pty", ControlKind::Text),
        one("nc_pdefault", ControlKind::Text),
        one("nc_pcat", ControlKind::Text),
        one("nc_phint", ControlKind::Text),
        one("nc_pdel", ControlKind::Button),
    ],
};

#[note("dialogs-table-new-component")]
pub const NEW_COMPONENT: DialogSpec = DialogSpec {
    name: "new_component",
    root: ["new_component_dialog", "dlg"],
    controls: &[
        one("nc_title", ControlKind::Label),
        one("nc_step_label", ControlKind::Label),
        one("nc_close", ControlKind::Button),
        one("nc_name", ControlKind::Text),
        one("nc_base", ControlKind::Drop),
        one("nc_scope_project", ControlKind::Button),
        one("nc_scope_mine", ControlKind::Button),
        one("nc_scope_hint", ControlKind::Label),
        one("nc_add_prop", ControlKind::Button),
        one("nc_gets", ControlKind::Label),
        one("nc_error", ControlKind::Label),
        one("nc_cancel", ControlKind::Button),
        one("nc_back", ControlKind::Button),
        one("nc_next", ControlKind::Button),
        one("nc_create", ControlKind::Button),
    ],
};

#[note("dialogs-table-permission")]
pub const PERMISSION: DialogSpec = DialogSpec {
    name: "permission",
    root: ["permission_dialog", "dlg"],
    controls: &[
        one("perm_title", ControlKind::Label),
        one("perm_icone", ControlKind::Label),
        one("perm_pergunta", ControlKind::Label),
        one("perm_badge", ControlKind::Label),
        one("perm_detalhes", ControlKind::Label),
        one("perm_alvo", ControlKind::Label),
        one("perm_deny", ControlKind::Button),
        one("perm_allow_session", ControlKind::Button),
        one("perm_allow_once", ControlKind::Button),
    ],
};

#[note("dialogs-table-templates")]
pub const TEMPLATES: DialogSpec = DialogSpec {
    name: "templates",
    root: ["templates_dialog", "dlg"],
    controls: &[
        one("tpl_title", ControlKind::Label),
        one("tpl_btn_new", ControlKind::Button),
        one("tpl_btn_duplicate", ControlKind::Button),
        one("tpl_btn_import", ControlKind::Button),
        one("tpl_btn_export", ControlKind::Button),
        one("sec_label", ControlKind::Label),
        one("trigger", ControlKind::Label),
        one("description", ControlKind::Label),
        one("tpl_trigger", ControlKind::Text),
        one("tpl_desc", ControlKind::Text),
        one("tpl_body", ControlKind::Text),
        one("tpl_preview_head", ControlKind::Label),
        one("tpl_preview", ControlKind::Label),
        one("tpl_delete", ControlKind::Button),
        one("tpl_problem", ControlKind::Label),
        one("tpl_cancel", ControlKind::Button),
        one("tpl_save", ControlKind::Button),
    ],
};

#[note("dialogs-table-generated")]
pub const GENERATED: [DialogSpec; 23] = [
    ITEMS,
    LINES,
    RENAME,
    COMPONENT_RENAME,
    ACTIONS,
    STANDARD,
    IMAGELIST,
    ADDICON,
    QUIT,
    RECOVER,
    PROJECT_OPTIONS,
    COLOR,
    CREATING,
    ICONS,
    MEMORY,
    MCP,
    NOTE,
    GOTO,
    QUICK_OPEN,
    INHERIT_FORM,
    NEW_COMPONENT,
    PERMISSION,
    TEMPLATES,
];
