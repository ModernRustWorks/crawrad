pub mod action_list;
pub mod badge;
pub mod bevel;
pub mod bit_btn;
pub mod breadcrumb;
pub mod button;
pub mod card;
pub mod category_panel;
pub mod check_list_box;
pub mod checkbox;
pub mod chips;
pub mod codegen;
pub mod color_box;
pub mod combobox;
pub mod date_time_picker;
pub mod drawer;
pub mod edit;
pub mod grid;
pub mod group_box;
pub mod image;
pub mod image_list;
pub mod label;
pub mod link_label;
pub mod list_box;
pub mod list_view;
pub mod main_menu;
pub mod mask_edit;
pub mod memo;
pub mod message_dialog;
pub mod multi_select;
pub mod open_dialog;
pub mod page_control;
pub mod pagination;
pub mod paint_box;
pub mod panel;
pub mod popup_menu;
pub mod progress_bar;
pub mod radio_button;
pub mod radio_group;
pub mod save_dialog;
pub mod scroll_bar;
pub mod scroll_box;
pub mod shape;
pub mod speed_button;
pub mod spin_edit;
pub mod splitter;
pub mod status_bar;
pub mod toggle;

pub mod timer;
pub mod tool_bar;
pub mod track_bar;
pub mod tree_view;

pub use codegen::EguiComponentCodegen;

use crate::{Erro, literal};
use crawrad_ir::{Document, Node};
use crawrad_note::note;

pub struct ContextoEmissao<'a> {
    pub doc: &'a Document,
    pub node: &'a Node,
    pub nome: &'a str,
    pub radios: &'a [(crawrad_ir::NodeId, String)],
    /// **THE REGISTRY** — and through it a third-party class becomes code.
    pub reg: &'a crawrad_registry::Registry,
}

pub struct ResultadoComponente {
    pub campo: String,
    pub padrao: String,
    pub constante: String,
    pub corpo: String,
    pub tipo_usado: &'static str,
    pub precisa_timer: bool,
    pub precisa_item: bool,
    /// **The shared sections that this component's body CALLS.**
    pub compartilhado: Compartilhado,
}

#[note("targets-egui-components-compartilhado")]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Compartilhado {
    /// Calls `declared_dashed`.
    pub tracejado: bool,
    /// Calls `declared_check`.
    pub marca: bool,
    /// Calls `fraction`.
    pub fracao: bool,
}

impl Compartilhado {
    #[note("targets-egui-components-compartilhado-com")]
    pub fn com(self, outro: Self) -> Self {
        Self {
            tracejado: self.tracejado || outro.tracejado,
            marca: self.marca || outro.marca,
            fracao: self.fracao || outro.fracao,
        }
    }
}

static REGISTRY: &[&'static dyn EguiComponentCodegen] = &[
    &action_list::ActionListCodegen,
    &action_list::ActionCodegen,
    &button::ButtonCodegen,
    &checkbox::CheckBoxCodegen,
    &chips::ChipsCodegen,
    &breadcrumb::BreadcrumbCodegen,
    &pagination::PaginationCodegen,
    &multi_select::MultiSelectCodegen,
    &check_list_box::CheckListBoxCodegen,
    &color_box::ColorBoxCodegen,
    &combobox::ComboBoxCodegen,
    &date_time_picker::DateTimePickerCodegen,
    &edit::EditCodegen,
    &mask_edit::MaskEditCodegen,
    &group_box::GroupBoxCodegen,
    &image::ImageCodegen,
    &image_list::ImageListCodegen,
    &label::LabelCodegen,
    &list_box::ListBoxCodegen,
    &list_view::ListViewCodegen,
    &list_view::ListColumnCodegen,
    &grid::GridCodegen,
    &memo::MemoCodegen,
    &page_control::PageControlCodegen,
    &page_control::TabSheetCodegen,
    &panel::PanelCodegen,
    &radio_button::RadioButtonCodegen,
    &radio_group::RadioGroupCodegen,
    &scroll_bar::ScrollBarCodegen,
    &scroll_box::ScrollBoxCodegen,
    &timer::TimerCodegen,
    &tree_view::TreeViewCodegen,
    &bit_btn::BitBtnCodegen,
    &speed_button::SpeedButtonCodegen,
    &spin_edit::SpinEditCodegen,
    &toggle::ToggleCodegen,
    &link_label::LinkLabelCodegen,
    &bevel::BevelCodegen,
    &badge::BadgeCodegen,
    &card::CardCodegen,
    &drawer::DrawerCodegen,
    &category_panel::CategoryPanelCodegen,
    &main_menu::MainMenuCodegen,
    &main_menu::MenuItemCodegen,
    &popup_menu::PopupMenuCodegen,
    &tool_bar::ToolBarCodegen,
    &status_bar::StatusBarCodegen,
    &status_bar::StatusPanelCodegen,
    &progress_bar::ProgressBarCodegen,
    &track_bar::TrackBarCodegen,
    &paint_box::PaintBoxCodegen,
    &shape::ShapeCodegen,
    &open_dialog::OpenDialogCodegen,
    &save_dialog::SaveDialogCodegen,
    &message_dialog::MessageDialogCodegen,
    &splitter::SplitterCodegen,
];

#[note("targets-egui-components-for-class-name")]
pub fn for_class_name(name: &str) -> Option<&'static dyn EguiComponentCodegen> {
    REGISTRY.iter().copied().find(|c| c.class() == name)
}

#[note("targets-egui-components-classes")]
pub fn classes() -> Vec<&'static str> {
    REGISTRY.iter().map(|c| c.class()).collect()
}

pub fn emitir_componente(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let effective = ctx.reg.effective_class(&ctx.node.class);
    if let Some(comp) = for_class_name(effective) {
        let mut res = comp.emit(ctx)?;
        if let Some(desc) = ctx.reg.get(&ctx.node.class)
            && desc.inherits.is_some()
            && !desc.crate_module.is_empty()
        {
            let ext_type = format!("{}::{}", desc.crate_module, desc.class);
            let ext_init = crate::targets::makepad::components::third_party_literal(
                ctx.reg, ctx.doc, ctx.node, desc,
            );
            let field = ctx.nome;
            res.campo
                .push_str(&format!("    pub {field}_ext: {ext_type},\n"));
            res.padrao
                .push_str(&format!("            {field}_ext: {ext_init},\n"));
        }
        return Ok(res);
    }
    // **Having no handler does not mean unknown.** A third-party class states which
    // crate its type comes from, and that is enough for it to become code — ADR 0033
    // §4. `TODO` is reserved for classes that do not even specify that.
    if let Some(desc) = ctx.reg.get(&ctx.node.class)
        && !desc.crate_module.is_empty()
    {
        return Ok(emitir_de_terceiro(
            ctx,
            &ctx.node.class,
            desc.crate_module,
            desc,
        ));
    }
    emitir_desconhecido(ctx, &ctx.node.class)
}

#[note("targets-egui-components-envolve-filhos")]
pub fn envolve_filhos(classe: &str) -> bool {
    for_class_name(classe).is_some_and(|c| c.envolve_filhos())
}

#[note("targets-egui-components-rola")]
pub fn rola(classe: &str) -> bool {
    for_class_name(classe).is_some_and(|c| c.rola())
}

#[note("targets-egui-components-form-strip")]
pub fn form_strip(class: &str) -> bool {
    for_class_name(class).is_some_and(|c| c.form_strip())
}

#[note("targets-egui-components-corpo-fecha")]
pub fn corpo_fecha(ctx: &ContextoEmissao) -> Result<String, Erro> {
    let effective = ctx.reg.effective_class(&ctx.node.class);
    match for_class_name(effective) {
        Some(comp) => comp.corpo_fecha(ctx),
        None => Ok(String::new()),
    }
}

#[note("targets-egui-components-emitir-de-terceiro")]
fn emitir_de_terceiro(
    ctx: &ContextoEmissao,
    classe: &str,
    modulo: &str,
    desc: &crawrad_registry::ComponentDesc,
) -> ResultadoComponente {
    let nome = ctx.nome;
    let tipo = format!("{modulo}::{classe}");

    let mut ajustes = String::new();
    for prop in &desc.props {
        if crate::emission::why_not_set(prop.key, prop.category).is_some() {
            continue;
        }
        let valor = ctx.node.props.get(prop.key).unwrap_or(&prop.default);
        // **The REFERENCE becomes the NAME of the referenced component**, and does not disappear.
        let lit = if let crawrad_ir::Value::Ref(id) = valor {
            crate::targets::compartilhado::literal_de_referencia(ctx.doc, id)
        } else if let Some(lit) = crate::targets::compartilhado::literal_da_peca(valor) {
            // A boxed part: `Edges`, `Color`. The type travels alongside, emitted
            // by `texto_das_pecas`.
            lit
        } else {
            let Some(lit) = literal_de(valor) else {
                continue;
            };
            lit
        };
        ajustes.push_str(&format!("        self.{nome}.{} = {lit};\n", prop.key));
    }

    // **DRAWING, AFTER adjustments.** In that order because painting reads the
    // fields written just above — the gauge draws the bar
    // from the `value` set by the designer, in the same frame.
    let ligacoes =
        crate::targets::compartilhado::ligacoes_do_declarado(ctx.node).unwrap_or_default();
    let pintura = crate::targets::egui::declarado::pintar(nome, desc, &ligacoes);
    let compartilhado = pintura
        .as_ref()
        .map_or_else(Compartilhado::default, |p| Compartilhado {
            tracejado: p.usa_tracejado,
            marca: p.usa_marca,
            fracao: p.usa_fracao,
        });
    let desenho = pintura.map(|p| p.corpo).unwrap_or_default();

    ResultadoComponente {
        campo: format!("    pub {nome}: {tipo},\n"),
        padrao: format!("            {nome}: {tipo}::default(),\n"),
        constante: String::new(),
        // The adjustment runs once per frame and is idempotent. A non-visual stops
        // here: it has no drawing, which is why adjustments stay outside
        // of any `if visible`.
        corpo: format!("{ajustes}{desenho}"),
        tipo_usado: "",
        precisa_timer: false,
        precisa_item: false,
        compartilhado,
    }
}

#[note("targets-egui-components-literal-de")]
fn literal_de(v: &crawrad_ir::Value) -> Option<String> {
    use crawrad_ir::Value;
    match v {
        Value::Bool(b) => Some(b.to_string()),
        Value::Int(i) => Some(i.to_string()),
        Value::Float(f) => Some(format!("{f:.1}")),
        Value::Str(s) | Value::Enum(s) => Some(format!("{}.to_string()", literal(s))),
        _ => None,
    }
}

fn emitir_desconhecido(ctx: &ContextoEmissao, outra: &str) -> Result<ResultadoComponente, Erro> {
    let nome = ctx.nome;
    let n = ctx.node;
    let popup = match n.props.get("popup_menu") {
        Some(crawrad_ir::Value::Ref(m)) => ctx
            .doc
            .node(m)
            .filter(|m| m.class == "PopupMenu")
            .and_then(|m| crate::identificador(m.name()).ok())
            .unwrap_or_default(),
        _ => String::new(),
    };
    let cursor = match n.props.get("cursor") {
        Some(crawrad_ir::Value::Cursor(c)) => *c,
        _ => crawrad_ir::Cursor::Default,
    };
    let hint = match n.props.get("hint") {
        Some(crawrad_ir::Value::Str(h)) => h.as_str(),
        _ => "",
    };
    let show = !matches!(
        n.props.get("show_hint"),
        Some(crawrad_ir::Value::Bool(false))
    );

    let corpo = if super::layout::desenha_por_si(ctx.reg, outra) {
        format!(
            "        if self.{nome}.visible {{\n\
             \x20           // TODO(CrawRAD): class `{outra}` has no egui generation yet.\n\
             \x20           ui.put(r, egui::Label::new({}));\n\
             \x20       }}\n",
            literal(&format!("[{outra}]"))
        )
    } else {
        format!(
            "        // TODO(CrawRAD): class `{outra}` has no egui generation yet.\n\
             \x20       let _ = &self.{nome};\n"
        )
    };

    Ok(ResultadoComponente {
        campo: format!("    pub {nome}: Widget,\n"),
        padrao: format!(
            "            {nome}: Widget {{ visible: {}, enabled: {}, hint: {}.to_string(), show_hint: {show}, popup_menu: {}.to_string(), cursor: Cursor::{cursor:?} }},\n",
            super::layout::visivel(n),
            super::layout::habilitado(n),
            literal(hint),
            literal(&popup),
        ),
        constante: String::new(),
        corpo,
        tipo_usado: "Widget",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}
