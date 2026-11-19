//! The shell's fixed words and numbers: dialog texts, button tables, layout metrics. Cut out of `ide_app.rs`.

use super::*;
use crate::items_editor::ItemsCommand;
pub(crate) use crawrad_actions::action_list::{ACT_CLOSE, EDITANDO};
pub(crate) use crawrad_actions::items_editor::{
    ITEMS_APPLY, ITEMS_CANCEL, ITEMS_KEY, ITEMS_NOTA, ITEMS_OK,
};
pub(crate) use crawrad_designer::image_list_model::{
    CLASSE_IMAGE_LIST, IML_COLUNAS, IML_CONTA, IML_KEY, IML_KEY_A, IML_KEY_L, IML_NOTA,
    IML_SEM_IMAGEM, IML_TAMANHO,
};
use crawrad_note::note;
pub(crate) use crawrad_project::icons::IC_NOTA;
pub(crate) use crawrad_project::options::{
    OPT_GRADE_MAX, OPT_GRADE_MIN, OPT_GRADE_NOTA, OPT_ICONES, OPT_SNAP, OPT_TEMA_NOTA,
    OPT_VER_GRADE, OPT_ZOOM_NOTA,
};
pub(crate) use crawrad_project::templates::{CRIANDO_JA, NP_SEM_MODELO};
pub(crate) use crawrad_project::workspace::{QUIT_PERGUNTA, QUIT_SAVE};

#[note("ide-app-constants-align-largura")]
pub(super) const ALIGN_LARGURA: f64 = 168.0;
pub(super) const ALIGN_ALTURA: f64 = 120.0;

#[note("ide-app-constants-recuo-sub")]
pub(crate) const RECUO_SUB: f64 = 14.0;

#[note("ide-app-constants-zonepath")]
pub(super) type ZonePath = fn() -> &'static [LiveId];

#[note("ide-app-constants-align-zones")]
pub(super) const ALIGN_ZONES: [(ZonePath, &str); 5] = [
    (|| id!(align_pad.align_paper.zone_top), "Top"),
    (|| id!(align_pad.align_paper.zone_bottom), "Bottom"),
    (|| id!(align_pad.align_paper.align_middle.zone_left), "Left"),
    (
        || id!(align_pad.align_paper.align_middle.zone_right),
        "Right",
    ),
    (
        || id!(align_pad.align_paper.align_middle.zone_client),
        "Client",
    ),
];

#[note("ide-app-constants-novo-campos")]
pub(super) const NOVO_CAMPOS: [(ZonePath, &str); 3] = [
    (|| id!(novo_nome), "Name"),
    (|| id!(novo_simbolo), "Symbol"),
    (|| id!(novo_categoria), "Category"),
];

#[note("ide-app-constants-iml-botoes")]
pub(super) const IML_BOTOES: [(ZonePath, &str); 4] = [
    (|| id!(iml_add), "Add..."),
    (|| id!(iml_replace), "Replace..."),
    (|| id!(iml_delete), "Delete"),
    (|| id!(iml_clear), "Clear"),
];

#[note("ide-app-constants-iml-celulas")]
pub(crate) const IML_CELULAS: [ZonePath; IML_COLUNAS] = [
    || id!(c0),
    || id!(c1),
    || id!(c2),
    || id!(c3),
    || id!(c4),
    || id!(c5),
];

#[note("ide-app-constants-act-botoes")]
pub(super) const ACT_BOTOES: [ZonePath; 6] = [
    || id!(act_new),
    || id!(act_standard),
    || id!(act_delete),
    || id!(act_up),
    || id!(act_down),
    || id!(act_fields),
];

#[note("ide-app-constants-items-botoes")]
pub(super) const ITEMS_BOTOES: [(ZonePath, &str, ItemsCommand); 5] = [
    (|| id!(btn_new_item), "New Item", ItemsCommand::NewItem),
    (
        || id!(btn_new_subitem),
        "New SubItem",
        ItemsCommand::NewSubItem,
    ),
    (|| id!(btn_delete), "Delete", ItemsCommand::Delete),
    (|| id!(btn_move_up), "Move Up", ItemsCommand::MoveUp),
    (|| id!(btn_move_down), "Move Down", ItemsCommand::MoveDown),
];

#[note("ide-app-constants-viewport")]
pub(super) const VIEWPORT: (i32, i32) = (1200, 800);

#[note("ide-app-constants-recuo-base")]
pub(super) const RECUO_BASE: f64 = 4.0;

#[note("ide-app-constants-recuo-por-nivel")]
pub(super) const RECUO_POR_NIVEL: f64 = 14.0;

#[note("ide-app-constants-menu-row-height")]
pub(super) const MENU_ROW_HEIGHT: f64 = 22.0;

#[note("ide-app-constants-menu-width")]
pub(super) const MENU_WIDTH: f64 = 260.0;

#[note("ide-app-constants-menu-separator-height")]
pub(super) const MENU_SEPARATOR_HEIGHT: f64 = 7.0;

#[note("ide-app-constants-menu-padding")]
pub(super) const MENU_PADDING: f64 = 4.0;

#[note("ide-app-constants-no-icon")]
pub(super) const NO_ICON: f64 = -1.0;
