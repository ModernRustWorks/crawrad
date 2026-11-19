//! Internationalization (i18n) support for CrawRAD Studio IDE.
//!
//! Provides multi-language strings with English as the primary default language,
//! and built-in support for Portuguese (PT-BR) and future languages.
//! Actions from `crawrad-actions` define their canonical English labels, and this
//! module provides localization maps.

use crawrad_note::note;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    #[default]
    English,
    Portuguese,
}

impl Language {
    #[note("i18n-language-toggle")]
    pub fn toggle(&mut self) {
        *self = match *self {
            Language::English => Language::Portuguese,
            Language::Portuguese => Language::English,
        };
    }

    #[note("i18n-language-tag")]
    pub fn tag(&self) -> &'static str {
        match self {
            Language::English => "EN",
            Language::Portuguese => "PT",
        }
    }
}

#[note("i18n-action-label")]
pub fn action_label(action_id: &str, default_label: &'static str, lang: Language) -> &'static str {
    if lang == Language::English {
        return default_label;
    }
    match action_id {
        "file.new" => "Novo",
        "file.new_component" => "Novo Componente...",
        "file.open" => "Abrir",
        "file.save" => "Salvar",
        "file.reload" => "Recarregar",
        "edit.undo" => "Desfazer",
        "edit.redo" => "Refazer",
        "edit.cut" => "Recortar",
        "edit.copy" => "Copiar",
        "edit.paste" => "Colar",
        "edit.delete" => "Excluir",
        "edit.select_all" => "Selecionar Tudo",
        "project.run" => "Executar (F9)",
        "view.toggle_code" => "Código / Design (F12)",
        _ => default_label,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiText {
    AppTitle,
    ObjectTreeHeader,
    ToolboxHeader,
    SearchComponents,
    ObjectInspectorHeader,
    PropertiesTab,
    EventsTab,
    DesignerTab,
    CodeTab,
    StatusReady,
    TargetMakepad,
    AefosConnected,
    LanguageButton,
}

pub fn ui_text(key: UiText, lang: Language) -> &'static str {
    match lang {
        Language::English => match key {
            UiText::AppTitle => "CrawRAD Studio",
            UiText::ObjectTreeHeader => "OBJECT HIERARCHY",
            UiText::ToolboxHeader => "TOOLBOX",
            UiText::SearchComponents => "Search components...",
            UiText::ObjectInspectorHeader => "OBJECT INSPECTOR",
            UiText::PropertiesTab => "Properties",
            UiText::EventsTab => "Events",
            UiText::DesignerTab => "🎨 Form1.crawform [Designer]",
            UiText::CodeTab => "🦀 Form1.rs [Code]",
            UiText::StatusReady => "Ready",
            UiText::TargetMakepad => "IDE Target: Native Makepad GPU",
            UiText::AefosConnected => "🤖 Aefos AI Connected",
            UiText::LanguageButton => "🌐 Language: EN",
        },
        Language::Portuguese => match key {
            UiText::AppTitle => "CrawRAD Studio",
            UiText::ObjectTreeHeader => "ESTRUTURA DE OBJETOS",
            UiText::ToolboxHeader => "CAIXA DE FERRAMENTAS",
            UiText::SearchComponents => "Buscar componentes...",
            UiText::ObjectInspectorHeader => "INSPETOR DE OBJETOS",
            UiText::PropertiesTab => "Propriedades",
            UiText::EventsTab => "Eventos",
            UiText::DesignerTab => "🎨 Form1.crawform [Designer]",
            UiText::CodeTab => "🦀 Form1.rs [Código]",
            UiText::StatusReady => "Pronto",
            UiText::TargetMakepad => "Alvo da IDE: Makepad GPU Nativo",
            UiText::AefosConnected => "🤖 Aefos AI Conectado",
            UiText::LanguageButton => "🌐 Idioma: PT",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_toggle() {
        let mut lang = Language::English;
        assert_eq!(lang.tag(), "EN");
        lang.toggle();
        assert_eq!(lang, Language::Portuguese);
        assert_eq!(lang.tag(), "PT");
        lang.toggle();
        assert_eq!(lang, Language::English);
    }

    #[test]
    fn test_action_labels() {
        assert_eq!(action_label("file.save", "Save", Language::English), "Save");
        assert_eq!(
            action_label("file.save", "Save", Language::Portuguese),
            "Salvar"
        );
        assert_eq!(
            action_label("project.run", "▶ Run (F9)", Language::Portuguese),
            "Executar (F9)"
        );
    }

    #[test]
    fn test_ui_text_translations() {
        assert_eq!(ui_text(UiText::ToolboxHeader, Language::English), "TOOLBOX");
        assert_eq!(
            ui_text(UiText::ToolboxHeader, Language::Portuguese),
            "CAIXA DE FERRAMENTAS"
        );
        assert_eq!(
            ui_text(UiText::LanguageButton, Language::English),
            "🌐 Language: EN"
        );
        assert_eq!(
            ui_text(UiText::LanguageButton, Language::Portuguese),
            "🌐 Idioma: PT"
        );
    }
}
