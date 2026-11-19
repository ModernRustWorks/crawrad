use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crawrad_extension_api::contribution::{
    ActionContribution, CardContribution, MenuContribution, ProjectTemplateContribution,
    ToolContribution,
};
use crawrad_extension_api::error::ExtensionError;
use crawrad_extension_api::manifest::ExtensionManifest;
use crawrad_extension_api::services::{DesignerService, EditorService, ProjectService};
use crawrad_extension_api::traits::{CommandHandler, ExtensionContext, IdeExtension, MessageLevel};

use crawrad_extension_api::ui::UiEventKind;

use super::cards::ExtensionCardManager;
use super::process::ProcessExtensionHost;
use crate::services::{HostDesignerService, HostEditorService, HostProjectService};
use crawrad_note::note;

#[note("host-shellextensioncontext")]
#[derive(Default)]
pub struct ShellExtensionContext {
    pub workspace_path: Option<String>,
    pub messages: Arc<Mutex<Vec<(MessageLevel, String)>>>,
    pub commands: Arc<Mutex<BTreeMap<String, CommandHandler>>>,
    pub editor: Option<HostEditorService>,
    pub project: Option<HostProjectService>,
    pub designer: Option<HostDesignerService>,
}

impl ShellExtensionContext {
    pub fn new(workspace_path: Option<PathBuf>) -> Self {
        let ws_str = workspace_path
            .as_ref()
            .map(|p| p.to_string_lossy().to_string());
        Self {
            workspace_path: ws_str,
            messages: Arc::new(Mutex::new(Vec::new())),
            commands: Arc::new(Mutex::new(BTreeMap::new())),
            editor: Some(HostEditorService::new()),
            project: Some(HostProjectService::new(workspace_path)),
            designer: Some(HostDesignerService::new()),
        }
    }

    pub fn take_messages(&self) -> Vec<(MessageLevel, String)> {
        if let Ok(mut guard) = self.messages.lock() {
            std::mem::take(&mut *guard)
        } else {
            Vec::new()
        }
    }
}

impl ExtensionContext for ShellExtensionContext {
    fn register_command(
        &mut self,
        action_id: &str,
        handler: CommandHandler,
    ) -> Result<(), ExtensionError> {
        let mut guard = self
            .commands
            .lock()
            .map_err(|_| ExtensionError::Internal("Mutex poisoned in register_command".into()))?;
        guard.insert(action_id.to_string(), handler);
        Ok(())
    }

    fn show_message(&self, level: MessageLevel, text: &str) {
        if let Ok(mut guard) = self.messages.lock() {
            guard.push((level, text.to_string()));
        }
    }

    fn active_workspace_path(&self) -> Option<String> {
        self.workspace_path.clone()
    }

    fn editor(&self) -> Option<&dyn EditorService> {
        self.editor.as_ref().map(|e| e as &dyn EditorService)
    }

    fn editor_mut(&mut self) -> Option<&mut dyn EditorService> {
        self.editor.as_mut().map(|e| e as &mut dyn EditorService)
    }

    fn project(&self) -> Option<&dyn ProjectService> {
        self.project.as_ref().map(|p| p as &dyn ProjectService)
    }

    fn project_mut(&mut self) -> Option<&mut dyn ProjectService> {
        self.project.as_mut().map(|p| p as &mut dyn ProjectService)
    }

    fn designer(&self) -> Option<&dyn DesignerService> {
        self.designer.as_ref().map(|d| d as &dyn DesignerService)
    }

    fn designer_mut(&mut self) -> Option<&mut dyn DesignerService> {
        self.designer
            .as_mut()
            .map(|d| d as &mut dyn DesignerService)
    }
}

#[note("host-shellextensionhost")]
pub struct ShellExtensionHost {
    pub manifests: Vec<ExtensionManifest>,
    pub extensions: Vec<Box<dyn IdeExtension>>,
    pub context: ShellExtensionContext,
    pub process_host: Arc<Mutex<ProcessExtensionHost>>,
    pub card_manager: Arc<Mutex<ExtensionCardManager>>,
}

impl ShellExtensionHost {
    pub fn new(workspace_path: Option<PathBuf>) -> Self {
        Self::new_with_user_dir(workspace_path, crate::install::user_extensions_dir())
    }

    pub fn new_with_user_dir(workspace_path: Option<PathBuf>, user_dir: PathBuf) -> Self {
        let card_manager = Arc::new(Mutex::new(ExtensionCardManager::new()));
        let mut process_host =
            ProcessExtensionHost::new(workspace_path.clone(), card_manager.clone());
        process_host.user_dir = Some(user_dir.clone());
        let process_host = Arc::new(Mutex::new(process_host));
        let mut host = Self {
            manifests: Vec::new(),
            extensions: Vec::new(),
            context: ShellExtensionContext::new(workspace_path.clone()),
            process_host,
            card_manager,
        };
        if let Some(ref ws) = workspace_path {
            host.load_workspace_extensions_from(ws, &user_dir);
        }
        host
    }
}

impl Default for ShellExtensionHost {
    fn default() -> Self {
        Self::new(None)
    }
}

impl ShellExtensionHost {
    #[note("host-shellextensionhost-scan-extensions")]
    pub fn scan_extensions(workspace_root: &Path, user_dir: &Path) -> Vec<ExtensionManifest> {
        let mut discovered = Vec::new();

        if let Ok(project_exts) = crate::install::read_project_extensions(workspace_root) {
            for ext_ref in project_exts.extensions {
                let folder = user_dir.join(format!("{}@{}", ext_ref.id, ext_ref.version));
                if folder.is_dir()
                    && let Ok(manifest) = crate::install::read_manifest(&folder)
                    && manifest.id == ext_ref.id
                {
                    discovered.push(manifest);
                }
            }
        }

        let legacy_dir = workspace_root.join(".crawrad").join("extensions");
        if legacy_dir.is_dir()
            && let Ok(entries) = std::fs::read_dir(&legacy_dir)
        {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir()
                    && let Ok(manifest) = crate::install::read_manifest(&path)
                    && !discovered.iter().any(|m| m.id == manifest.id)
                {
                    discovered.push(manifest);
                }
            }
        }

        discovered
    }

    /// [`Self::load_workspace_extensions_from`] this user's extension store.
    pub fn load_workspace_extensions(&mut self, workspace_root: &Path) {
        self.load_workspace_extensions_from(workspace_root, &crate::install::user_extensions_dir());
    }

    #[note("host-shellextensionhost-load-workspace-extensions")]
    pub fn load_workspace_extensions_from(&mut self, workspace_root: &Path, user_dir: &Path) {
        // a relative `entrypoint` is found under this root
        if let Ok(mut ph) = self.process_host.lock() {
            ph.workspace_root = Some(workspace_root.to_path_buf());
            ph.user_dir = Some(user_dir.to_path_buf());
        }
        let scanned = Self::scan_extensions(workspace_root, user_dir);
        for manifest in scanned {
            let _ = self.register_manifest(manifest);
        }
    }

    #[note("host-shellextensionhost-register-manifest")]
    pub fn register_manifest(&mut self, manifest: ExtensionManifest) -> Result<(), ExtensionError> {
        manifest.validate()?;
        if self.manifests.iter().any(|m| m.id == manifest.id) {
            return Err(ExtensionError::Duplicate(manifest.id));
        }
        if let Ok(mut cm) = self.card_manager.lock() {
            cm.register_manifest_cards(&manifest);
        }
        if manifest.is_external()
            && let Ok(mut ph) = self.process_host.lock()
        {
            let _ = ph.register_manifest(manifest.clone());
        }
        self.manifests.push(manifest);
        Ok(())
    }

    #[note("host-shellextensionhost-register-extension")]
    pub fn register_extension(
        &mut self,
        mut ext: Box<dyn IdeExtension>,
    ) -> Result<(), ExtensionError> {
        let ext_id = ext.id().to_string();
        if self.extensions.iter().any(|e| e.id() == ext_id) {
            return Err(ExtensionError::Duplicate(ext_id));
        }
        ext.activate(&mut self.context)?;
        self.extensions.push(ext);
        Ok(())
    }

    #[note("host-shellextensionhost-line-marks")]
    pub fn line_marks(&self) -> Option<Arc<Mutex<crate::line_marks::LineMarkStore>>> {
        self.process_host.lock().ok().map(|p| p.line_marks.clone())
    }

    #[note("host-shellextensionhost-views")]
    pub fn views(&self) -> Option<Arc<Mutex<crate::views::ViewStore>>> {
        self.process_host.lock().ok().map(|p| p.views.clone())
    }

    #[note("host-shellextensionhost-view-item-clicked")]
    pub fn view_item_clicked(
        &self,
        extension: &str,
        view_id: &str,
        item_id: &str,
    ) -> Result<(), ExtensionError> {
        let mut host = self
            .process_host
            .lock()
            .map_err(|_| ExtensionError::Internal("Mutex poisoned in view_item_clicked".into()))?;
        host.view_item_clicked(extension, view_id, item_id)
    }

    /// Hands the extensions the file in the editor and its text.
    pub fn sync_editor_state(&mut self, file: Option<PathBuf>, text: &str) {
        if let Some(ref mut ed) = self.context.editor {
            ed.active_file = file.clone();
            if let Some(p) = file {
                ed.set_buffer(&p, text.to_string());
            }
        }
    }

    /// Hands the extensions the form in the designer and its selected names.
    pub fn sync_designer_state(&mut self, form_name: Option<String>, selected: Vec<String>) {
        if let Some(ref mut dsg) = self.context.designer {
            dsg.active_form_name = form_name;
            dsg.selected = selected;
        }
    }

    #[note("host-shellextensionhost-line-mark-clicked")]
    pub fn line_mark_clicked(
        &self,
        extension: &str,
        id: &str,
        file: &str,
    ) -> Result<(), ExtensionError> {
        let mut host = self
            .process_host
            .lock()
            .map_err(|_| ExtensionError::Internal("Mutex poisoned in line_mark_clicked".into()))?;
        host.line_mark_clicked(extension, id, file)
    }

    #[note("host-shellextensionhost-execute-command")]
    pub fn execute_command(&self, action_id: &str) -> Result<(), ExtensionError> {
        // 1. In-process command handler
        let guard =
            self.context.commands.lock().map_err(|_| {
                ExtensionError::Internal("Mutex poisoned in execute_command".into())
            })?;

        if let Some(handler) = guard.get(action_id) {
            handler();
            return Ok(());
        }
        drop(guard);

        // 2. External subprocess extension
        if let Ok(mut ph) = self.process_host.lock()
            && ph.can_handle(action_id)
        {
            ph.execute_command(action_id, None)?;
            return Ok(());
        }

        Err(ExtensionError::NotFound(action_id.to_string()))
    }

    #[note("host-shellextensionhost-execute-command-with-args")]
    pub fn execute_command_with_args(
        &self,
        action_id: &str,
        args: Option<serde_json::Value>,
    ) -> Result<Option<serde_json::Value>, ExtensionError> {
        // 1. In-process command handler
        let guard =
            self.context.commands.lock().map_err(|_| {
                ExtensionError::Internal("Mutex poisoned in execute_command".into())
            })?;

        if let Some(handler) = guard.get(action_id) {
            handler();
            return Ok(None);
        }
        drop(guard);

        // 2. External subprocess extension
        if let Ok(mut ph) = self.process_host.lock()
            && ph.can_handle(action_id)
        {
            return ph.execute_command(action_id, args);
        }

        Err(ExtensionError::NotFound(action_id.to_string()))
    }

    #[note("host-shellextensionhost-take-messages")]
    pub fn take_messages(&self) -> Vec<(MessageLevel, String)> {
        let mut msgs = self.context.take_messages();
        if let Ok(ph) = self.process_host.lock() {
            msgs.extend(ph.take_messages());
        }
        msgs
    }

    #[note("host-shellextensionhost-shutdown")]
    pub fn shutdown(&self) {
        if let Ok(mut ph) = self.process_host.lock() {
            ph.shutdown_all();
        }
    }

    #[note("host-shellextensionhost-contributed-actions")]
    pub fn contributed_actions(&self) -> Vec<&ActionContribution> {
        self.manifests
            .iter()
            .flat_map(|m| &m.contributions.actions)
            .collect()
    }

    #[note("host-shellextensionhost-contributed-menus")]
    pub fn contributed_menus(&self) -> Vec<&MenuContribution> {
        self.manifests
            .iter()
            .flat_map(|m| &m.contributions.menus)
            .collect()
    }

    #[note("host-shellextensionhost-contributed-cards")]
    pub fn contributed_cards(&self) -> Vec<&CardContribution> {
        self.manifests
            .iter()
            .flat_map(|m| &m.contributions.cards)
            .collect()
    }

    #[note("host-shellextensionhost-contributed-tools")]
    pub fn contributed_tools(&self) -> Vec<&ToolContribution> {
        self.manifests
            .iter()
            .flat_map(|m| &m.contributions.tools)
            .collect()
    }

    #[note("host-shellextensionhost-contributed-templates")]
    pub fn contributed_templates(&self) -> Vec<&ProjectTemplateContribution> {
        self.manifests
            .iter()
            .flat_map(|m| &m.contributions.templates)
            .collect()
    }

    #[note("host-shellextensionhost-card-manager")]
    pub fn card_manager(&self) -> Arc<Mutex<ExtensionCardManager>> {
        self.card_manager.clone()
    }

    #[note("host-shellextensionhost-dispatch-ui-event")]
    pub fn dispatch_ui_event(
        &mut self,
        card_id: &str,
        element_id: &str,
        event: UiEventKind,
    ) -> Result<(), ExtensionError> {
        if let Ok(mut ph) = self.process_host.lock() {
            ph.send_ui_event(card_id, element_id, event)
        } else {
            Err(ExtensionError::Internal(
                "Process host mutex poisoned".into(),
            ))
        }
    }
}
