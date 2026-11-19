use crawrad_extension_api::error::ExtensionError;
use crawrad_extension_api::services::{
    ComponentInfo, DesignerService, EditorSelection, EditorService, FormInfo, ProjectService,
    TextEdit,
};
use crawrad_note::note;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[note("services-apply-text-edit")]
pub fn apply_text_edit(content: &str, edit: &TextEdit) -> Result<String, ExtensionError> {
    let range = &edit.range;
    if range.start_line == 0 || range.start_col == 0 || range.end_line == 0 || range.end_col == 0 {
        return Err(ExtensionError::ExecutionFailed(
            "TextRange line and column numbers must be 1-based (> 0)".into(),
        ));
    }
    if range.start_line > range.end_line
        || (range.start_line == range.end_line && range.start_col > range.end_col)
    {
        return Err(ExtensionError::ExecutionFailed(format!(
            "Invalid range: start ({}:{}) > end ({}:{})",
            range.start_line, range.start_col, range.end_line, range.end_col
        )));
    }

    let lines: Vec<&str> = content.split('\n').collect();
    let num_lines = lines.len();

    if range.start_line > num_lines || range.end_line > num_lines {
        return Err(ExtensionError::ExecutionFailed(format!(
            "Target line out of bounds: range end line {} > total lines {}",
            range.end_line, num_lines
        )));
    }

    // Convert (line, col) 1-based to byte index
    fn line_col_to_byte_offset(
        lines: &[&str],
        line: usize,
        col: usize,
    ) -> Result<usize, ExtensionError> {
        let mut offset = 0;
        for (idx, l) in lines.iter().enumerate() {
            let cur_line = idx + 1;
            if cur_line == line {
                // col is 1-based UTF-8 character or byte index
                let line_len = l.len();
                let col_offset = col - 1;
                if col_offset > line_len {
                    return Err(ExtensionError::ExecutionFailed(format!(
                        "Column {} out of bounds on line {} (length {})",
                        col, line, line_len
                    )));
                }
                return Ok(offset + col_offset);
            }
            offset += l.len() + 1; // +1 for '\n'
        }
        Ok(offset)
    }

    let start_byte = line_col_to_byte_offset(&lines, range.start_line, range.start_col)?;
    let end_byte = line_col_to_byte_offset(&lines, range.end_line, range.end_col)?;

    if start_byte > content.len() || end_byte > content.len() || start_byte > end_byte {
        return Err(ExtensionError::ExecutionFailed(
            "Calculated byte offsets are invalid".into(),
        ));
    }

    let mut result =
        String::with_capacity(content.len() - (end_byte - start_byte) + edit.new_text.len());
    result.push_str(&content[..start_byte]);
    result.push_str(&edit.new_text);
    result.push_str(&content[end_byte..]);

    Ok(result)
}

#[note("services-hosteditorservice")]
#[derive(Debug, Clone, Default)]
pub struct HostEditorService {
    pub active_file: Option<PathBuf>,
    pub buffers: BTreeMap<PathBuf, String>,
    pub active_selection: Option<EditorSelection>,
    pub open_requests: Vec<(PathBuf, Option<(usize, usize)>)>,
}

impl HostEditorService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_buffer(&mut self, path: impl AsRef<Path>, content: String) {
        self.buffers.insert(path.as_ref().to_path_buf(), content);
    }
}

impl EditorService for HostEditorService {
    fn active_file(&self) -> Option<String> {
        self.active_file
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
    }

    fn open_file(
        &mut self,
        path: &str,
        position: Option<(usize, usize)>,
    ) -> Result<(), ExtensionError> {
        let p = PathBuf::from(path);
        self.open_requests.push((p.clone(), position));
        self.active_file = Some(p);
        Ok(())
    }

    fn get_content(&self, path: &str) -> Result<String, ExtensionError> {
        let p = PathBuf::from(path);
        if let Some(buf) = self.buffers.get(&p) {
            return Ok(buf.clone());
        }
        if p.exists() {
            std::fs::read_to_string(&p).map_err(|e| ExtensionError::Io(e.to_string()))
        } else {
            Err(ExtensionError::NotFound(path.into()))
        }
    }

    fn get_lines(
        &self,
        path: &str,
        start_line: usize,
        end_line: usize,
    ) -> Result<Vec<String>, ExtensionError> {
        if start_line == 0 || end_line < start_line {
            return Err(ExtensionError::ExecutionFailed(
                "Invalid line range requested".into(),
            ));
        }
        let content = self.get_content(path)?;
        let lines: Vec<&str> = content.split('\n').collect();
        if start_line > lines.len() {
            return Ok(Vec::new());
        }
        let end = end_line.min(lines.len());
        Ok(lines[(start_line - 1)..end]
            .iter()
            .map(|s| s.trim_end_matches('\r').to_string())
            .collect())
    }

    fn apply_edit(&mut self, path: &str, edit: TextEdit) -> Result<(), ExtensionError> {
        let current = self.get_content(path)?;
        let updated = apply_text_edit(&current, &edit)?;
        self.buffers.insert(PathBuf::from(path), updated);
        Ok(())
    }

    fn active_selection(&self) -> Option<EditorSelection> {
        self.active_selection.clone()
    }
}

#[note("services-hostprojectservice")]
#[derive(Debug, Clone, Default)]
pub struct HostProjectService {
    pub root_path: Option<PathBuf>,
    pub project_name: Option<String>,
    pub files: Vec<PathBuf>,
    pub forms: Vec<FormInfo>,
    pub build_requested: bool,
}

impl HostProjectService {
    pub fn new(root_path: Option<PathBuf>) -> Self {
        let mut svc = Self {
            root_path: root_path.clone(),
            project_name: None,
            files: Vec::new(),
            forms: Vec::new(),
            build_requested: false,
        };
        if let Some(ref root) = root_path {
            svc.scan_project_files(root);
        }
        svc
    }

    pub fn scan_project_files(&mut self, root: &Path) {
        if !root.is_dir() {
            return;
        }
        self.files.clear();
        self.forms.clear();

        // Detect project name from Cargo.toml or directory name
        let cargo_toml = root.join("Cargo.toml");
        if cargo_toml.exists()
            && let Ok(content) = std::fs::read_to_string(&cargo_toml)
            && let Ok(parsed) = content.parse::<toml::Table>()
            && let Some(pkg) = parsed.get("package").and_then(|p| p.as_table())
            && let Some(name) = pkg.get("name").and_then(|n| n.as_str())
        {
            self.project_name = Some(name.to_string());
        }
        if self.project_name.is_none() {
            self.project_name = root.file_name().map(|n| n.to_string_lossy().to_string());
        }

        // Walk directory
        let mut stack = vec![root.to_path_buf()];
        while let Some(dir) = stack.pop() {
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let file_name = entry.file_name();
                    let name_str = file_name.to_string_lossy();
                    if name_str.starts_with('.') || name_str == "target" {
                        continue;
                    }
                    if path.is_dir() {
                        stack.push(path);
                    } else if path.is_file() {
                        self.files.push(path.clone());
                        if let Some(ext) = path.extension()
                            && ext == "crawform"
                        {
                            let stem = path.file_stem().unwrap_or_default().to_string_lossy();
                            let companion_rs = path.with_extension("rs");
                            self.forms.push(FormInfo {
                                name: stem.to_string(),
                                form_file: path.to_string_lossy().to_string(),
                                code_file: companion_rs.to_string_lossy().to_string(),
                            });
                        }
                    }
                }
            }
        }
    }
}

impl ProjectService for HostProjectService {
    fn active_project_path(&self) -> Option<String> {
        self.root_path
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
    }

    fn active_project_name(&self) -> Option<String> {
        self.project_name.clone()
    }

    fn list_project_files(&self) -> Result<Vec<String>, ExtensionError> {
        Ok(self
            .files
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect())
    }

    fn list_forms(&self) -> Result<Vec<FormInfo>, ExtensionError> {
        Ok(self.forms.clone())
    }

    fn add_file(&mut self, relative_or_abs_path: &str) -> Result<(), ExtensionError> {
        let p = if let Some(ref root) = self.root_path {
            root.join(relative_or_abs_path)
        } else {
            PathBuf::from(relative_or_abs_path)
        };
        if !self.files.contains(&p) {
            self.files.push(p);
        }
        Ok(())
    }

    fn request_build(&mut self) -> Result<(), ExtensionError> {
        self.build_requested = true;
        Ok(())
    }
}

#[note("services-hostdesignerservice")]
#[derive(Debug, Clone, Default)]
pub struct HostDesignerService {
    pub active_form_name: Option<String>,
    pub components: Vec<ComponentInfo>,
    pub properties: BTreeMap<(String, String), String>,
    pub selected: Vec<String>,
    pub property_changes: Vec<(String, String, String)>,
    pub deleted_components: Vec<String>,
}

impl HostDesignerService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_component(&mut self, info: ComponentInfo) {
        if let Some(pos) = self.components.iter().position(|c| c.name == info.name) {
            self.components[pos] = info;
        } else {
            self.components.push(info);
        }
    }
}

impl DesignerService for HostDesignerService {
    fn active_form_name(&self) -> Option<String> {
        self.active_form_name.clone()
    }

    fn list_components(&self) -> Result<Vec<ComponentInfo>, ExtensionError> {
        Ok(self.components.clone())
    }

    fn get_property(
        &self,
        component_name: &str,
        property_key: &str,
    ) -> Result<Option<String>, ExtensionError> {
        Ok(self
            .properties
            .get(&(component_name.to_string(), property_key.to_string()))
            .cloned())
    }

    fn set_property(
        &mut self,
        component_name: &str,
        property_key: &str,
        property_value: &str,
    ) -> Result<(), ExtensionError> {
        self.properties.insert(
            (component_name.to_string(), property_key.to_string()),
            property_value.to_string(),
        );
        self.property_changes.push((
            component_name.to_string(),
            property_key.to_string(),
            property_value.to_string(),
        ));
        Ok(())
    }

    fn selected_components(&self) -> Vec<String> {
        self.selected.clone()
    }

    fn select_components(&mut self, names: &[&str]) -> Result<(), ExtensionError> {
        self.selected = names.iter().map(|s| s.to_string()).collect();
        Ok(())
    }

    fn delete_selected(&mut self) -> Result<(), ExtensionError> {
        for sel in &self.selected {
            self.components.retain(|c| &c.name != sel);
            self.deleted_components.push(sel.clone());
        }
        self.selected.clear();
        Ok(())
    }
}
