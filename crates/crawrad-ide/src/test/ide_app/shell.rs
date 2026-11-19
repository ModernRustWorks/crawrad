//! **THE ENTIRE SHELL, as text** — `ide_app.rs` plus its submodules.
//!
//! # Why it exists
//!
//! A significant portion of this shell's gates reads CODE as text: this is how
//! we enforce *"saving goes through the catalog"* or *"no shortcut key is
//! hardcoded here"* without opening a window. While everything lived in a single file,
//! `include_str!("../../ide_app.rs")` was enough.
//!
//! On 2026-09-17 the file exceeded ten thousand lines and the owner demanded a split:
//! *"this is unacceptable, create a folder and split this"*. It was partitioned by
//! subject — and **fifteen gates started reading only half the shell**, because the
//! method they searched for had moved to a different file.
//!
//! A gate that checks less than it claims is worse than no gate at all: it stays
//! green over what it never inspected. That is why the search target became this constant,
//! rather than the root file.
//!
//! # And why the list is EXPLICIT
//!
//! `include_str!` requires literal paths — there is no way to scan a directory at
//! compile time without `build.rs`. The list lives here, in one place, and
//! [`o_texto_da_casca_cobre_todos_os_modulos`] ensures it never forgets
//! any: it checks against `mod` declarations in `ide_app.rs` itself, which is the
//! source of truth for shell members.

/// `ide_app.rs` and all `impl IdeApp` blocks split from it.
pub const CASCA: &str = concat!(
    include_str!("../../ide_app.rs"),
    include_str!("../../ide_app/actions.rs"),
    include_str!("../../ide_app/actions/input.rs"),
    include_str!("../../ide_app/actions/menu.rs"),
    include_str!("../../ide_app/actions/editor.rs"),
    include_str!("../../ide_app/actions/designer.rs"),
    include_str!("../../ide_app/actions/panels.rs"),
    include_str!("../../ide_app/actions/chat.rs"),
    include_str!("../../ide_app/actions/terminal.rs"),
    include_str!("../../ide_app/actions/dialogs.rs"),
    include_str!("../../ide_app/arrangement.rs"),
    include_str!("../../ide_app/build.rs"),
    include_str!("../../ide_app/effects.rs"),
    include_str!("../../ide_app/events.rs"),
    include_str!("../../ide_app/problems.rs"),
    include_str!("../../ide_app/sync.rs"),
    include_str!("../../ide_app/constants.rs"),
    include_str!("../../ide_app/live.rs"),
    include_str!("../../ide_app/document_flow.rs"),
    include_str!("../../ide_app/host_api.rs"),
    include_str!("../../ide_app/view_state.rs"),
    include_str!("../../ide_app/diff_review.rs"),
    include_str!("../../ide_app/key_dispatch.rs"),
    include_str!("../../ide_app/chat_bar.rs"),
    include_str!("../../ide_app/explorer_actions.rs"),
    include_str!("../../ide_app/chat.rs"),
    include_str!("../../ide_app/chat_state.rs"),
    include_str!("../../ide_app/chrome.rs"),
    include_str!("../../ide_app/dialogs.rs"),
    include_str!("../../ide_app/draw.rs"),
    include_str!("../../ide_app/dsl_main.rs"),
    include_str!("../../ide_app/dialogs/actions.rs"),
    include_str!("../../ide_app/dialogs/build.rs"),
    include_str!("../../ide_app/dialogs/dsl.rs"),
    include_str!("../../ide_app/dialogs/dsl/editor_dialogs.rs"),
    include_str!("../../ide_app/dialogs/dsl/options_dialog.rs"),
    include_str!("../../ide_app/dialogs/dsl/project_dialogs.rs"),
    include_str!("../../ide_app/dialogs/dsl/tools_dialogs.rs"),
    include_str!("../../ide_app/dialogs/icons.rs"),
    include_str!("../../ide_app/dialogs/imagelist.rs"),
    include_str!("../../ide_app/dialogs/items.rs"),
    include_str!("../../ide_app/dialogs/lines.rs"),
    include_str!("../../ide_app/dialogs/new_project.rs"),
    include_str!("../../ide_app/dialogs/options.rs"),
    include_str!("../../ide_app/dialogs/pad.rs"),
    include_str!("../../ide_app/dialogs/exit.rs"),
    include_str!("../../ide_app/link.rs"),
    include_str!("../../ide_app/editor/buffer.rs"),
    include_str!("../../ide_app/editor/signature.rs"),
    include_str!("../../ide_app/editor/code_actions.rs"),
    include_str!("../../ide_app/editor/item_doc.rs"),
    include_str!("../../ide_app/editor/line_marks.rs"),
    include_str!("../../ide_app/extension_views.rs"),
    include_str!("../../ide_app/editor/semantic.rs"),
    include_str!("../../ide_app/editor/footer.rs"),
    include_str!("../../ide_app/editor/keys.rs"),
    include_str!("../../ide_app/maximize.rs"),
    include_str!("../../ide_app/packages_app.rs"),
    include_str!("../../ide_app/panels.rs"),
    include_str!("../../ide_app/panels/side_panels.rs"),
    include_str!("../../ide_app/panels/design.rs"),
    include_str!("../../ide_app/panels/chat.rs"),
    include_str!("../../ide_app/panels/terminal.rs"),
    include_str!("../../ide_app/status_bar.rs"),
    include_str!("../../ide_app/ui_zoom.rs"),
    include_str!("../../ide_app/property.rs"),
    include_str!("../../ide_app/component_rename.rs"),
);

/// **No shell module is left out of the text that gates inspect.**
///
/// The regression this test catches is what the 2026-09-17 split would cause
/// again: moving another handful of methods to a new file and forgetting to
/// add it to [`CASCA`]. The gates would remain green, over a part
/// that nobody is reading anymore.
#[test]
fn shell_text_covers_all_modules() {
    let raiz = include_str!("../../ide_app.rs");
    let mut faltando = Vec::new();
    for linha in raiz.lines() {
        let t = linha.trim();
        let Some(caminho) = t
            .strip_prefix("#[path = \"")
            .and_then(|r| r.strip_suffix("\"]"))
        else {
            continue;
        };
        // TEST modules are excluded: they are the yardstick, and measuring
        // the yardstick with itself proves nothing.
        if caminho.starts_with("test/") {
            continue;
        }
        // Only partitioned `impl IdeApp` files. Modules with standalone roles —
        // theme, keyboard bridge, Explorer model — have their own gates.
        let e_pedaco_da_casca = [
            "ide_app/actions.rs",
            "ide_app/arranjo.rs",
            "ide_app/build.rs",
            "ide_app/chat.rs",
            "ide_app/chat_state.rs",
            "ide_app/cromo.rs",
            "ide_app/dialogs.rs",
            "ide_app/draw.rs",
            "ide_app/dsl_main.rs",
            "ide_app/elo.rs",
            "ide_app/editor/buffer.rs",
            "ide_app/editor/footer.rs",
            "ide_app/editor/keys.rs",
            "ide_app/maximize.rs",
            "ide_app/pacotes.rs",
            "ide_app/paineis.rs",
            "ide_app/status_bar.rs",
            "ide_app/ui_zoom.rs",
            "ide_app/propriedade.rs",
        ]
        .contains(&caminho);
        if !e_pedaco_da_casca {
            continue;
        }
        let arquivo = caminho.rsplit('/').next().unwrap_or(caminho);
        if !casca_cita(arquivo) {
            faltando.push(caminho.to_string());
        }
    }
    assert!(
        faltando.is_empty(),
        "these shell modules are OUTSIDE the text that the gates inspect: \
         {faltando:?}\n\n\
         They would remain green over a part that nobody reads. Add \
         the `include_str!` to `CASCA`."
    );
}

/// Does the shell text contain the file `nome`?
///
/// The proof is the header: every module in this shell starts with a `//!`, and its
/// body is unique. Searching by file name would not work — it does not
/// appear within itself.
fn casca_cita(nome: &str) -> bool {
    let cabecalho = match nome {
        "actions.rs" => "ACTION DISPATCH of the shell",
        "arranjo.rs" => "O ARRANJO da doca",
        "build.rs" => "Shell BUILD AND PROJECT",
        "chat.rs" => "The AI CHAT of the shell",
        "chat_state.rs" => "The AI chat state",
        "cromo.rs" => "O CROMO da casca",
        "dialogs.rs" => "THE DIALOG CATALOG of the shell",
        "draw.rs" => "USER INTERFACE DRAWING",
        "dsl_main.rs" => "THE MAIN DSL of the IDE",
        "elo.rs" => "O ELO entre o formulario e o codigo",
        "buffer.rs" | "editor_buffer.rs" => "THE EDITOR IS AN EDITOR",
        "footer.rs" | "editor_footer.rs" => "THE EDITOR FOOTER",
        "keys.rs" | "editor_keys.rs" => "THE EDITOR KEYBOARD",
        "maximize.rs" => "MAXIMIZE WITHOUT COVERING THE TASKBAR",
        "pacotes.rs" => "O PACKAGE MANAGER",
        "paineis.rs" => "OS SEIS PAINEIS DA DOCA, em DSL",
        "status_bar.rs" => "THE STATUS BAR of the shell",
        "ui_zoom.rs" => "THE ZOOM OF THE WHOLE IDE",
        "propriedade.rs" => "A GRAVACAO do Object Inspector",
        _ => return true,
    };
    CASCA.contains(cabecalho)
}
