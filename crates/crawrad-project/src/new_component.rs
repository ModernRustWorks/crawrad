//! Pure model for the `File > New > Component…` wizard (Inherit mode).
//!
//! Handles draft validation, source generation (`Cargo.toml`, `src/lib.rs`, `manifest.json`),
//! target directory resolution, and writing new component packages to disk.

use std::path::{Path, PathBuf};

use crate::installed::{ComponentScope, component_folders};

/// Draft representation of a property to be generated on the component struct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropDraft {
    pub name: String,
    pub ty: String,
    pub default: String,
    pub category: String,
    pub hint: String,
}

impl PropDraft {
    /// Creates a new property draft.
    pub fn new(
        name: impl Into<String>,
        ty: impl Into<String>,
        default: impl Into<String>,
        category: impl Into<String>,
        hint: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            ty: ty.into(),
            default: default.into(),
            category: category.into(),
            hint: hint.into(),
        }
    }
}

/// Draft of a new user component inheriting from a native base component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentDraft {
    pub name: String,
    /// Native class to inherit from, e.g. "SpinEdit".
    pub base: String,
    pub scope: ComponentScope,
    pub props: Vec<PropDraft>,
}

impl ComponentDraft {
    /// Creates a new component draft with default values.
    pub fn new(name: impl Into<String>, base: impl Into<String>, scope: ComponentScope) -> Self {
        Self {
            name: name.into(),
            base: base.into(),
            scope,
            props: Vec::new(),
        }
    }
}

/// Helper to convert PascalCase or CamelCase string to snake_case.
pub fn to_snake_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    let mut prev_is_upper = false;
    let mut prev_is_underscore = true;

    for (i, c) in s.chars().enumerate() {
        if c == '_' || c == '-' || c == ' ' {
            if !prev_is_underscore && !out.is_empty() {
                out.push('_');
            }
            prev_is_underscore = true;
            prev_is_upper = false;
        } else if c.is_ascii_uppercase() {
            if i > 0 && !prev_is_upper && !prev_is_underscore && !out.is_empty() {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
            prev_is_upper = true;
            prev_is_underscore = false;
        } else {
            out.push(c);
            prev_is_upper = false;
            prev_is_underscore = false;
        }
    }
    out
}

/// Checks if a string is a valid PascalCase identifier (starts with uppercase ASCII letter,
/// followed only by alphanumeric ASCII characters).
pub fn is_pascal_case(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_uppercase() => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric())
}

/// Validates a component draft against existing registered classes and base native component properties.
pub fn validate(draft: &ComponentDraft, existing_classes: &[String]) -> Result<(), String> {
    let registry = crawrad_registry::Registry::builtin();
    let base_desc = registry
        .get(&draft.base)
        .ok_or_else(|| format!("Base class '{}' is not a native component", draft.base))?;
    let base_props: Vec<String> = base_desc.props.iter().map(|p| p.key.to_string()).collect();
    validate_with_base_props(draft, existing_classes, &base_props)
}

/// Validates a component draft with explicit base property names.
pub fn validate_with_base_props(
    draft: &ComponentDraft,
    existing_classes: &[String],
    base_props: &[String],
) -> Result<(), String> {
    let name = draft.name.trim();
    if name.is_empty() {
        return Err("Component name cannot be empty".to_string());
    }
    if !is_pascal_case(name) {
        return Err(format!(
            "'{name}' is not a valid PascalCase component name (e.g. 'PriceEdit')"
        ));
    }
    if existing_classes.iter().any(|c| c == name) {
        return Err(format!("A component named '{name}' already exists"));
    }

    let registry = crawrad_registry::Registry::builtin();
    if registry.get(&draft.base).is_none() {
        return Err(format!(
            "Base class '{}' is not a native component",
            draft.base
        ));
    }

    let mut seen_props = std::collections::BTreeSet::new();
    for prop in &draft.props {
        let p_name = prop.name.trim();
        if p_name.is_empty() {
            return Err("Property name cannot be empty".to_string());
        }
        if !crawrad_ir::is_snake_case_name(p_name) {
            return Err(format!(
                "Property '{p_name}' must be snake_case (e.g. 'currency_code')"
            ));
        }
        if !seen_props.insert(p_name.to_string()) {
            return Err(format!("Duplicate property '{p_name}'"));
        }
        if base_props.iter().any(|bp| bp == p_name) || p_name == "name" || p_name == "rect" {
            return Err(format!(
                "Property '{p_name}' clashes with base class '{}'",
                draft.base
            ));
        }
        validate_prop_default(&prop.ty, &prop.default, p_name)?;
    }

    Ok(())
}

fn validate_prop_default(ty: &str, default: &str, prop_name: &str) -> Result<(), String> {
    let trimmed = default.trim();
    match ty {
        "i64" => {
            trimmed.parse::<i64>().map_err(|_| {
                format!("Invalid i64 default value '{trimmed}' for property '{prop_name}'")
            })?;
        }
        "f64" => {
            trimmed.parse::<f64>().map_err(|_| {
                format!("Invalid f64 default value '{trimmed}' for property '{prop_name}'")
            })?;
        }
        "bool" => {
            trimmed
                .parse::<bool>()
                .map_err(|_| format!("Invalid bool default value '{trimmed}' for property '{prop_name}' (expected 'true' or 'false')"))?;
        }
        "String" => {
            // String defaults are always valid text
        }
        other => {
            return Err(format!(
                "Unsupported property type '{other}' for property '{prop_name}'"
            ));
        }
    }
    Ok(())
}

/// Generates the contents of `src/lib.rs` for the component.
pub fn lib_rs(draft: &ComponentDraft) -> String {
    let name = &draft.name;
    let base = &draft.base;

    let mut out = String::new();
    out.push_str("use crawrad_component_derive::Component;\n\n");
    out.push_str(&format!("/// The `{name}` component.\n"));
    out.push_str("#[derive(Component, Debug, Clone, PartialEq)]\n");
    let prefix = name.to_lowercase();
    out.push_str(&format!(
        "#[crawrad(inherits = \"{base}\", palette = \"My components\", prefix = \"{prefix}\", size = (120.0, 24.0))]\n"
    ));

    if draft.props.is_empty() {
        out.push_str(&format!("pub struct {name} {{\n}}\n"));
        return out;
    }

    out.push_str(&format!("pub struct {name} {{\n"));
    for prop in &draft.props {
        let p_name = prop.name.trim();
        let p_ty = &prop.ty;
        let p_cat = if prop.category.trim().is_empty() {
            "behavior"
        } else {
            prop.category.trim()
        };
        let p_default = format_default_for_attr(p_ty, &prop.default);

        if !prop.hint.trim().is_empty() {
            out.push_str(&format!("    /// {}\n", prop.hint.trim()));
        }
        out.push_str(&format!(
            "    #[prop(category = \"{p_cat}\", default = {p_default})]\n"
        ));
        out.push_str(&format!("    pub {p_name}: {p_ty},\n"));
    }
    out.push_str("}\n");
    out
}

fn format_default_for_attr(ty: &str, default: &str) -> String {
    let trimmed = default.trim();
    match ty {
        "String" => {
            if trimmed.starts_with('"') && trimmed.ends_with('"') {
                trimmed.to_string()
            } else {
                format!("\"{trimmed}\"")
            }
        }
        "f64" => {
            if let Ok(f) = trimmed.parse::<f64>() {
                if trimmed.contains('.') || trimmed.contains('e') || trimmed.contains('E') {
                    trimmed.to_string()
                } else {
                    format!("{f:.1}")
                }
            } else {
                "0.0".to_string()
            }
        }
        "bool" => {
            if trimmed == "true" || trimmed == "false" {
                trimmed.to_string()
            } else {
                "false".to_string()
            }
        }
        "i64" => {
            if trimmed.parse::<i64>().is_ok() {
                trimmed.to_string()
            } else {
                "0".to_string()
            }
        }
        _ => trimmed.to_string(),
    }
}

/// Generates the contents of `Cargo.toml` for the component crate.
pub fn cargo_toml(draft: &ComponentDraft) -> String {
    let crate_name = to_snake_case(&draft.name);
    let desc = format!("The {} component", draft.name);
    let sdk = |name: &str| {
        crawrad_codegen::dependencies::caminho_local_crate(name)
            .display()
            .to_string()
            .replace('\\', "/")
    };
    let component = sdk("crawrad-component");
    let derive = sdk("crawrad-component-derive");
    format!(
        r#"[package]
name = "{crate_name}"
version = "0.1.0"
edition = "2024"
description = "{desc}"

# Its own workspace: it lives inside the project folder but builds on its own.
[workspace]

[dependencies]
crawrad-component = {{ path = "{component}" }}
crawrad-component-derive = {{ path = "{derive}" }}

[lints.rust]
unsafe_code = "forbid"
"#
    )
}

/// Resolves the target package directory using the folders of `component_folders`.
pub fn target_dir(draft: &ComponentDraft, project_root: Option<&Path>) -> Option<PathBuf> {
    let crate_name = to_snake_case(&draft.name);
    let exe = std::env::current_exe().ok();
    let exe_dir = exe.as_deref().and_then(Path::parent);
    let cwd = std::env::current_dir().ok();
    let config = crate::workspace::config_folder();

    let folders = component_folders(exe_dir, cwd.as_deref(), config.as_deref(), project_root);

    // If an existing component folder with the target scope is already present, use it.
    for (folder, scope) in folders {
        if scope == draft.scope {
            return Some(folder.join(&crate_name));
        }
    }

    // Otherwise compute the canonical target path even if the components/ folder doesn't exist yet.
    let base_components_dir = match draft.scope {
        ComponentScope::Project => project_root?.join("components"),
        ComponentScope::Mine => config?.join("components"),
        ComponentScope::Installed => cwd?.join("components"),
    };
    Some(base_components_dir.join(crate_name))
}

/// The one-line note under "Available in": where the crate goes and what that means.
pub fn scope_hint(draft: &ComponentDraft) -> String {
    let crate_name = to_snake_case(&draft.name);
    match draft.scope {
        ComponentScope::Mine => {
            format!("{crate_name}/ in your CrawRAD folder — available in every project")
        }
        _ => format!("components/{crate_name}/ — goes to git with the project"),
    }
}

/// Writes `Cargo.toml`, `src/lib.rs`, and `manifest.json` into `dir`.
/// Refuses if `dir` already exists.
pub fn write(draft: &ComponentDraft, dir: &Path) -> Result<Vec<PathBuf>, String> {
    if dir.exists() {
        return Err(format!("Directory '{}' already exists", dir.display()));
    }

    let src_dir = dir.join("src");
    std::fs::create_dir_all(&src_dir)
        .map_err(|e| format!("Failed to create directory '{}': {e}", src_dir.display()))?;

    let cargo_content = cargo_toml(draft);
    let cargo_path = dir.join("Cargo.toml");
    std::fs::write(&cargo_path, cargo_content)
        .map_err(|e| format!("Failed to write '{}': {e}", cargo_path.display()))?;

    let lib_content = lib_rs(draft);
    let lib_path = src_dir.join("lib.rs");
    std::fs::write(&lib_path, &lib_content)
        .map_err(|e| format!("Failed to write '{}': {e}", lib_path.display()))?;

    let specs = crawrad_manifest_gen::from_source(&lib_content)
        .map_err(|e| format!("Failed to parse component specification from source: {e}"))?;

    let spec = specs
        .iter()
        .find(|s| s.class == draft.name)
        .ok_or_else(|| format!("Class '{}' was not found in generated source", draft.name))?;

    let comp_json = spec.to_json(4);
    let crate_name = to_snake_case(&draft.name);
    let desc = format!("The {} component", draft.name);
    let manifest_content = format!(
        r#"{{
  "$doc": "This file is generated from the Rust source.",
  "sdk": 2,
  "package": {{
    "name": "{crate_name}",
    "version": "0.1.0",
    "description": "{desc}"
  }},
  "components": [
{comp_json}
  ]
}}
"#
    );

    let manifest_path = dir.join("manifest.json");
    std::fs::write(&manifest_path, manifest_content)
        .map_err(|e| format!("Failed to write '{}': {e}", manifest_path.display()))?;

    Ok(vec![cargo_path, lib_path, manifest_path])
}

/// Thin wizard state for the GUI dialog.
#[derive(Debug, Clone)]
pub struct NewComponentWizard {
    pub draft: ComponentDraft,
    pub step: usize,
    pub error: Option<String>,
    pub base_index: usize,
    pub native_classes: Vec<String>,
    pub project_root: Option<PathBuf>,
}

impl NewComponentWizard {
    /// Initialises a new wizard dialog model.
    pub fn new(project_root: Option<&Path>) -> Self {
        let registry = crawrad_registry::Registry::builtin();
        let mut native_classes: Vec<String> = registry.all().map(|d| d.class.to_string()).collect();
        native_classes.sort();

        let base_index = native_classes
            .iter()
            .position(|c| c == "SpinEdit")
            .unwrap_or(0);

        let base = native_classes
            .get(base_index)
            .cloned()
            .unwrap_or_else(|| "SpinEdit".to_string());

        let scope = if project_root.is_some() {
            ComponentScope::Project
        } else {
            ComponentScope::Mine
        };

        Self {
            draft: ComponentDraft::new("PriceEdit", base, scope),
            step: 1,
            error: None,
            base_index,
            native_classes,
            project_root: project_root.map(Path::to_path_buf),
        }
    }

    /// Moves forward to the next wizard step.
    pub fn next_step(&mut self, existing_classes: &[String]) -> bool {
        if self.step == 1 {
            if let Some(base) = self.native_classes.get(self.base_index) {
                self.draft.base = base.clone();
            }
            if let Err(err) = validate_step_one(&self.draft, existing_classes) {
                self.error = Some(err);
                return false;
            }
            self.error = None;
            self.step = 2;
            true
        } else {
            true
        }
    }

    /// Moves backward to the previous wizard step.
    pub fn prev_step(&mut self) {
        if self.step > 1 {
            self.step -= 1;
            self.error = None;
        }
    }

    /// Appends a new property row.
    pub fn add_prop(&mut self) {
        let n = self.draft.props.len() + 1;
        self.draft.props.push(PropDraft::new(
            format!("prop_{n}"),
            "i64",
            "0",
            "behavior",
            "",
        ));
    }

    /// Removes a property row by index.
    pub fn remove_prop(&mut self, index: usize) {
        if index < self.draft.props.len() {
            self.draft.props.remove(index);
        }
    }

    /// Completes the wizard and writes the component files to disk.
    pub fn finish(&mut self, existing_classes: &[String]) -> Result<PathBuf, String> {
        validate(&self.draft, existing_classes)?;
        let target = target_dir(&self.draft, self.project_root.as_deref())
            .ok_or_else(|| "Could not determine target component directory".to_string())?;
        let written = write(&self.draft, &target)?;
        let lib_rs = written
            .into_iter()
            .find(|p| p.ends_with("lib.rs"))
            .unwrap_or(target.join("src").join("lib.rs"));
        Ok(lib_rs)
    }
}

fn validate_step_one(draft: &ComponentDraft, existing_classes: &[String]) -> Result<(), String> {
    let name = draft.name.trim();
    if name.is_empty() {
        return Err("Component name cannot be empty".to_string());
    }
    if !is_pascal_case(name) {
        return Err(format!(
            "'{name}' is not a valid PascalCase component name (e.g. 'PriceEdit')"
        ));
    }
    if existing_classes.iter().any(|c| c == name) {
        return Err(format!("A component named '{name}' already exists"));
    }
    let registry = crawrad_registry::Registry::builtin();
    if registry.get(&draft.base).is_none() {
        return Err(format!(
            "Base class '{}' is not a native component",
            draft.base
        ));
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/new_component/test.rs"]
mod test;
