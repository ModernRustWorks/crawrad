//! # CapabilityRegistry — Unified IDE Capability Registry
//!
//! Unifies the IDE's native actions (`crawrad-actions`) and contributions declared
//! by extensions (`crawrad-extension-api`) into a single dynamic capability catalogue.
//!
//! This registry serves as the central source of truth for:
//! 1. **IDE Menus and Toolbars** (File, Edit, Tools, context menus);
//! 2. **Command Palette** (text search, shortcuts, categories);
//! 3. **MCP Protocol and AI Agents** (`tools/list`, typed JSON schemas and safe dispatch);
//! 4. **Audit Trails** (structured execution logs with author and timestamp).

use crate::{Action, Choice, Context, Effect, Param, ParamKind, ViewAffinity};
use crawrad_doc::Author;
use crawrad_extension_api::manifest::ExtensionManifest;
use crawrad_note::note;
use std::collections::BTreeMap;
use std::sync::Arc;

#[note("capability-capabilitysource")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilitySource {
    /// Native built-in action of CrawRAD.
    Native,
    /// Capability contributed dynamically by an installed extension.
    Extension { extension_id: String },
}

impl std::fmt::Display for CapabilitySource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Native => write!(f, "native"),
            Self::Extension { extension_id } => write!(f, "extension:{}", extension_id),
        }
    }
}

#[note("capability-capabilityparam")]
#[derive(Debug, Clone, PartialEq)]
pub struct CapabilityParam {
    /// Parameter name in the signature (e.g. `"class"`, `"x"`, `"panel"`).
    pub name: String,
    /// Semantic type of the parameter.
    pub kind: ParamKind,
    /// Readable description for documentation and MCP.
    pub description: String,
    /// Indicates whether it accepts multiple values (array).
    pub is_array: bool,
    /// Indicates whether the parameter is required for execution.
    pub required: bool,
    /// Closed set of accepted choices, when applicable.
    pub choices: Vec<Choice>,
}

impl CapabilityParam {
    #[note("capability-capabilityparam-from-param")]
    pub fn from_param(p: &Param) -> Self {
        Self {
            name: p.name.to_string(),
            kind: p.kind,
            description: p.description.to_string(),
            is_array: p.list,
            required: p.required,
            choices: p.choices(),
        }
    }

    #[note("capability-capabilityparam-new")]
    pub fn new(name: impl Into<String>, kind: ParamKind, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind,
            description: description.into(),
            is_array: false,
            required: true,
            choices: Vec::new(),
        }
    }

    pub fn with_array(mut self, is_array: bool) -> Self {
        self.is_array = is_array;
        self
    }

    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn with_choices(mut self, choices: Vec<Choice>) -> Self {
        self.choices = choices;
        self
    }

    #[note("capability-capabilityparam-json-schema")]
    pub fn json_schema(&self) -> serde_json::Value {
        let mut item = match self.kind {
            ParamKind::String => serde_json::json!({ "type": "string" }),
            ParamKind::Integer => serde_json::json!({ "type": "integer" }),
            ParamKind::Number => serde_json::json!({ "type": "number" }),
            ParamKind::Boolean => serde_json::json!({ "type": "boolean" }),
            ParamKind::ComponentId => serde_json::json!({ "type": "string" }),
            ParamKind::Enum(variants) => {
                let list: Vec<serde_json::Value> = if !variants.is_empty() {
                    variants.iter().map(|s| serde_json::json!(s)).collect()
                } else {
                    self.choices
                        .iter()
                        .map(|c| serde_json::json!(c.id))
                        .collect()
                };
                if list.is_empty() {
                    serde_json::json!({ "type": "string" })
                } else {
                    serde_json::json!({ "type": "string", "enum": list })
                }
            }
            ParamKind::Any => serde_json::json!({}),
        };

        if self.is_array {
            serde_json::json!({
                "type": "array",
                "description": self.description,
                "items": item
            })
        } else {
            if let Some(obj) = item.as_object_mut() {
                obj.insert(
                    "description".to_string(),
                    serde_json::json!(self.description),
                );
            }
            item
        }
    }
}

#[note("capability-capabilityexecutor")]
pub trait CapabilityExecutor: Send + Sync {
    fn execute(&self, ctx: &Context<'_>) -> Result<Effect, CapabilityError>;
}

#[note("capability-capabilityenabledcheck")]
pub trait CapabilityEnabledCheck: Send + Sync {
    fn is_enabled(&self, ctx: &Context<'_>) -> bool;
}

struct NativeFnExecutor(fn(&Context<'_>) -> Effect);

impl CapabilityExecutor for NativeFnExecutor {
    fn execute(&self, ctx: &Context<'_>) -> Result<Effect, CapabilityError> {
        Ok((self.0)(ctx))
    }
}

struct NativeEnabledCheck(crate::Gate);

impl CapabilityEnabledCheck for NativeEnabledCheck {
    fn is_enabled(&self, ctx: &Context<'_>) -> bool {
        self.0.allows(ctx)
    }
}

struct ExtensionCommandExecutor {
    action_id: String,
}

impl CapabilityExecutor for ExtensionCommandExecutor {
    fn execute(&self, _ctx: &Context<'_>) -> Result<Effect, CapabilityError> {
        Ok(
            Effect::none()
                .with_status(format!("Extension command '{}' dispatched", self.action_id)),
        )
    }
}

struct ExtensionToolExecutor {
    tool_name: String,
}

impl CapabilityExecutor for ExtensionToolExecutor {
    fn execute(&self, _ctx: &Context<'_>) -> Result<Effect, CapabilityError> {
        Ok(Effect::none().with_status(format!("Extension tool '{}' invoked", self.tool_name)))
    }
}

pub struct ClosureExecutor<F>(pub F)
where
    F: Fn(&Context<'_>) -> Result<Effect, CapabilityError> + Send + Sync;

impl<F> CapabilityExecutor for ClosureExecutor<F>
where
    F: Fn(&Context<'_>) -> Result<Effect, CapabilityError> + Send + Sync,
{
    fn execute(&self, ctx: &Context<'_>) -> Result<Effect, CapabilityError> {
        (self.0)(ctx)
    }
}

pub struct ClosureEnabledCheck<F>(pub F)
where
    F: Fn(&Context<'_>) -> bool + Send + Sync;

impl<F> CapabilityEnabledCheck for ClosureEnabledCheck<F>
where
    F: Fn(&Context<'_>) -> bool + Send + Sync,
{
    fn is_enabled(&self, ctx: &Context<'_>) -> bool {
        (self.0)(ctx)
    }
}

#[note("capability-capability")]
#[derive(Clone)]
pub struct Capability {
    /// Canonical stable identifier (`namespace.command`).
    pub id: String,
    /// Short label for menus and human interface.
    pub label: String,
    /// Detailed description of what the capability performs.
    pub description: String,
    /// Group or category (e.g. `"File"`, `"Edit"`, `"Tools"`).
    pub group: String,
    /// Preferred keyboard shortcut (e.g. `"Ctrl+S"`, `"Ctrl+Shift+P"`).
    pub shortcut: Option<String>,
    /// Formal parameters expected for execution.
    pub params: Vec<CapabilityParam>,
    /// IDE view affinity (any, designer, code, or view switch).
    pub view_affinity: ViewAffinity,
    /// Required permissions (e.g. `"workspace.write"`, `"git.exec"`).
    pub permissions: Vec<String>,
    /// Origin of the registration (native or extension).
    pub source: CapabilitySource,
    /// Explicit custom JSON Schema (e.g. from an extension `contributions.tools`).
    pub custom_schema: Option<serde_json::Value>,
    /// Optional dynamic predicate to check if the capability can act now.
    enabled_check: Option<Arc<dyn CapabilityEnabledCheck>>,
    /// Concrete executor to trigger the action and obtain the [`Effect`].
    executor: Arc<dyn CapabilityExecutor>,
}

impl std::fmt::Debug for Capability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Capability")
            .field("id", &self.id)
            .field("label", &self.label)
            .field("group", &self.group)
            .field("shortcut", &self.shortcut)
            .field("view_affinity", &self.view_affinity)
            .field("permissions", &self.permissions)
            .field("source", &self.source)
            .field("custom_schema", &self.custom_schema)
            .finish()
    }
}

impl Capability {
    #[note("capability-capability-from-action")]
    pub fn from_action(action: &Action) -> Self {
        let params = action
            .params
            .iter()
            .map(CapabilityParam::from_param)
            .collect();
        let enabled = action.enabled;
        let execute = action.execute;

        Self {
            id: action.id.to_string(),
            label: action.label.to_string(),
            description: action.hint.to_string(),
            group: action.group.to_string(),
            shortcut: action.shortcut.map(|s| s.to_string()),
            params,
            view_affinity: action.view,
            permissions: Vec::new(),
            source: CapabilitySource::Native,
            custom_schema: None,
            enabled_check: Some(Arc::new(NativeEnabledCheck(enabled))),
            executor: Arc::new(NativeFnExecutor(execute)),
        }
    }

    #[note("capability-capability-custom")]
    pub fn custom<F>(
        id: impl Into<String>,
        label: impl Into<String>,
        description: impl Into<String>,
        group: impl Into<String>,
        source: CapabilitySource,
        executor: F,
    ) -> Self
    where
        F: Fn(&Context<'_>) -> Result<Effect, CapabilityError> + Send + Sync + 'static,
    {
        Self {
            id: id.into(),
            label: label.into(),
            description: description.into(),
            group: group.into(),
            shortcut: None,
            params: Vec::new(),
            view_affinity: ViewAffinity::Any,
            permissions: Vec::new(),
            source,
            custom_schema: None,
            enabled_check: None,
            executor: Arc::new(ClosureExecutor(executor)),
        }
    }

    #[note("capability-capability-custom-executor")]
    pub fn custom_executor<E>(
        id: impl Into<String>,
        label: impl Into<String>,
        description: impl Into<String>,
        group: impl Into<String>,
        source: CapabilitySource,
        executor: E,
    ) -> Self
    where
        E: CapabilityExecutor + 'static,
    {
        Self {
            id: id.into(),
            label: label.into(),
            description: description.into(),
            group: group.into(),
            shortcut: None,
            params: Vec::new(),
            view_affinity: ViewAffinity::Any,
            permissions: Vec::new(),
            source,
            custom_schema: None,
            enabled_check: None,
            executor: Arc::new(executor),
        }
    }

    pub fn with_custom_schema(mut self, schema: serde_json::Value) -> Self {
        self.custom_schema = Some(schema);
        self
    }

    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn with_params(mut self, params: Vec<CapabilityParam>) -> Self {
        self.params = params;
        self
    }

    pub fn with_view_affinity(mut self, affinity: ViewAffinity) -> Self {
        self.view_affinity = affinity;
        self
    }

    pub fn with_permissions(mut self, permissions: Vec<String>) -> Self {
        self.permissions = permissions;
        self
    }

    pub fn with_enabled_check<C>(mut self, check: C) -> Self
    where
        C: CapabilityEnabledCheck + 'static,
    {
        self.enabled_check = Some(Arc::new(check));
        self
    }

    #[note("capability-capability-is-enabled")]
    pub fn is_enabled(&self, ctx: &Context<'_>) -> bool {
        if let Some(check) = &self.enabled_check {
            check.is_enabled(ctx)
        } else {
            true
        }
    }

    #[note("capability-capability-input-schema")]
    pub fn input_schema(&self) -> serde_json::Value {
        if let Some(schema) = &self.custom_schema {
            return schema.clone();
        }

        let mut properties = serde_json::Map::new();
        let mut required = Vec::new();

        for p in &self.params {
            properties.insert(p.name.clone(), p.json_schema());
            if p.required {
                required.push(serde_json::json!(p.name));
            }
        }

        let mut schema = serde_json::Map::new();
        schema.insert("type".to_string(), serde_json::json!("object"));
        schema.insert(
            "properties".to_string(),
            serde_json::Value::Object(properties),
        );
        schema.insert("required".to_string(), serde_json::Value::Array(required));

        serde_json::Value::Object(schema)
    }

    #[note("capability-capability-execute")]
    pub fn execute(&self, ctx: &Context<'_>) -> Result<Effect, CapabilityError> {
        self.executor.execute(ctx)
    }
}

#[note("capability-auditentry")]
#[derive(Debug, Clone, PartialEq)]
pub struct AuditEntry {
    /// Sequential audit log identifier.
    pub id: u64,
    /// UNIX timestamp in milliseconds.
    pub timestamp_ms: u64,
    /// ID of the invoked capability.
    pub capability_id: String,
    /// Who ordered the dispatch ([`Author::User`], [`Author::Agent`], etc.).
    pub author: Author,
    /// Indicates whether execution finished successfully.
    pub success: bool,
    /// Error message in case of failure.
    pub error: Option<String>,
    /// Summary of the generated effect (whether there was a transaction, selection, status, etc.).
    pub effect_summary: String,
}

#[note("capability-capabilityerror")]
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CapabilityError {
    #[error("Capability not found: '{0}'")]
    NotFound(String),

    #[error("Capability ID collision: '{id}' is already registered by source '{existing}'")]
    IdCollision {
        id: String,
        existing: CapabilitySource,
        attempted: CapabilitySource,
    },

    #[error("Invalid capability ID: '{0}'. ID must follow canonical format 'namespace.command'")]
    InvalidId(String),

    #[error("Capability '{0}' is currently disabled for the active context")]
    Disabled(String),

    #[error(
        "View affinity mismatch for '{id}': requires '{required:?}', active view is '{active:?}'"
    )]
    ViewMismatch {
        id: String,
        required: ViewAffinity,
        active: ViewAffinity,
    },

    #[error("Missing required parameter '{param}' for capability '{capability_id}'")]
    MissingParameter {
        capability_id: String,
        param: String,
    },

    #[error("Execution failed for capability '{id}': {reason}")]
    ExecutionFailed { id: String, reason: String },
}

#[note("capability-capabilityregistry")]
pub struct CapabilityRegistry {
    capabilities: BTreeMap<String, Capability>,
    audit_log: Vec<AuditEntry>,
    next_audit_id: u64,
    max_audit_entries: usize,
}

impl Default for CapabilityRegistry {
    fn default() -> Self {
        Self::with_builtin_actions().unwrap_or_else(|_| Self::new())
    }
}

impl CapabilityRegistry {
    #[note("capability-capabilityregistry-new")]
    pub fn new() -> Self {
        Self {
            capabilities: BTreeMap::new(),
            audit_log: Vec::new(),
            next_audit_id: 1,
            max_audit_entries: 500,
        }
    }

    #[note("capability-capabilityregistry-with-builtin-actions")]
    pub fn with_builtin_actions() -> Result<Self, CapabilityError> {
        let mut reg = Self::new();
        for action in crate::catalogue() {
            reg.register_native(&action)?;
        }
        Ok(reg)
    }

    #[note("capability-capabilityregistry-validate-id")]
    pub fn validate_id(id: &str) -> Result<(), CapabilityError> {
        let trimmed = id.trim();
        if trimmed.is_empty()
            || !trimmed.contains('.')
            || trimmed.starts_with('.')
            || trimmed.ends_with('.')
            || !trimmed
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-')
        {
            return Err(CapabilityError::InvalidId(id.to_string()));
        }
        Ok(())
    }

    #[note("capability-capabilityregistry-register")]
    pub fn register(&mut self, capability: Capability) -> Result<(), CapabilityError> {
        Self::validate_id(&capability.id)?;

        if let Some(existing) = self.capabilities.get(&capability.id) {
            return Err(CapabilityError::IdCollision {
                id: capability.id.clone(),
                existing: existing.source.clone(),
                attempted: capability.source.clone(),
            });
        }

        self.capabilities.insert(capability.id.clone(), capability);
        Ok(())
    }

    #[note("capability-capabilityregistry-replace")]
    pub fn replace(
        &mut self,
        capability: Capability,
    ) -> Result<Option<Capability>, CapabilityError> {
        Self::validate_id(&capability.id)?;
        Ok(self.capabilities.insert(capability.id.clone(), capability))
    }

    #[note("capability-capabilityregistry-register-native")]
    pub fn register_native(&mut self, action: &Action) -> Result<(), CapabilityError> {
        self.register(Capability::from_action(action))
    }

    #[note("capability-capabilityregistry-register-native-actions")]
    pub fn register_native_actions<'a>(
        &mut self,
        actions: impl IntoIterator<Item = &'a Action>,
    ) -> Result<usize, CapabilityError> {
        let mut count = 0;
        for action in actions {
            self.register_native(action)?;
            count += 1;
        }
        Ok(count)
    }

    #[note("capability-capabilityregistry-register-extension-manifest")]
    pub fn register_extension_manifest(
        &mut self,
        manifest: &ExtensionManifest,
    ) -> Result<usize, CapabilityError> {
        let mut count = 0;
        let ext_id = manifest.id.clone();

        for action in &manifest.contributions.actions {
            let canonical_id = if action.id.contains('.') {
                action.id.clone()
            } else {
                format!("{}.{}", ext_id, action.id)
            };

            let act_id = action.id.clone();
            let label = action.label.clone();
            let desc = action.description.clone().unwrap_or_else(|| label.clone());
            let shortcut = action.shortcut.clone();

            let cap = Capability {
                id: canonical_id,
                label,
                description: desc,
                group: manifest.name.clone(),
                shortcut,
                params: Vec::new(),
                view_affinity: ViewAffinity::Any,
                permissions: Vec::new(),
                source: CapabilitySource::Extension {
                    extension_id: ext_id.clone(),
                },
                custom_schema: None,
                enabled_check: None,
                executor: Arc::new(ExtensionCommandExecutor { action_id: act_id }),
            };

            self.register(cap)?;
            count += 1;
        }

        for tool in &manifest.contributions.tools {
            let canonical_id = if tool.name.contains('.') {
                tool.name.clone()
            } else {
                format!("{}.tool.{}", ext_id, tool.name)
            };

            let tool_name = tool.name.clone();
            let desc = tool.description.clone();

            let cap = Capability {
                id: canonical_id,
                label: tool_name.clone(),
                description: desc,
                group: "Tools".to_string(),
                shortcut: None,
                params: Vec::new(),
                view_affinity: ViewAffinity::Any,
                permissions: Vec::new(),
                source: CapabilitySource::Extension {
                    extension_id: ext_id.clone(),
                },
                custom_schema: if tool.schema.is_object()
                    && !tool.schema.as_object().unwrap().is_empty()
                {
                    Some(tool.schema.clone())
                } else {
                    None
                },
                enabled_check: None,
                executor: Arc::new(ExtensionToolExecutor { tool_name }),
            };

            self.register(cap)?;
            count += 1;
        }

        Ok(count)
    }

    #[note("capability-capabilityregistry-bind-executor")]
    pub fn bind_executor<E>(&mut self, id: &str, executor: E) -> Result<(), CapabilityError>
    where
        E: CapabilityExecutor + 'static,
    {
        let cap = self
            .capabilities
            .get_mut(id)
            .ok_or_else(|| CapabilityError::NotFound(id.to_string()))?;
        cap.executor = Arc::new(executor);
        Ok(())
    }

    #[note("capability-capabilityregistry-unregister-extension")]
    pub fn unregister_extension(&mut self, extension_id: &str) -> Vec<String> {
        let to_remove: Vec<String> = self
            .capabilities
            .iter()
            .filter_map(|(id, cap)| match &cap.source {
                CapabilitySource::Extension { extension_id: eid } if eid == extension_id => {
                    Some(id.clone())
                }
                _ => None,
            })
            .collect();

        for id in &to_remove {
            self.capabilities.remove(id);
        }

        to_remove
    }

    #[note("capability-capabilityregistry-get")]
    pub fn get(&self, id: &str) -> Option<&Capability> {
        self.capabilities.get(id)
    }

    #[note("capability-capabilityregistry-contains")]
    pub fn contains(&self, id: &str) -> bool {
        self.capabilities.contains_key(id)
    }

    #[note("capability-capabilityregistry-len")]
    pub fn len(&self) -> usize {
        self.capabilities.len()
    }

    #[note("capability-capabilityregistry-is-empty")]
    pub fn is_empty(&self) -> bool {
        self.capabilities.is_empty()
    }

    #[note("capability-capabilityregistry-list")]
    pub fn list(&self) -> Vec<&Capability> {
        self.capabilities.values().collect()
    }

    #[note("capability-capabilityregistry-list-by-group")]
    pub fn list_by_group(&self, group: &str) -> Vec<&Capability> {
        self.capabilities
            .values()
            .filter(|c| c.group.eq_ignore_ascii_case(group))
            .collect()
    }

    #[note("capability-capabilityregistry-list-by-source")]
    pub fn list_by_source(&self, source: &CapabilitySource) -> Vec<&Capability> {
        self.capabilities
            .values()
            .filter(|c| &c.source == source)
            .collect()
    }

    #[note("capability-capabilityregistry-search")]
    pub fn search(&self, query: &str) -> Vec<&Capability> {
        let q = query.trim().to_lowercase();
        if q.is_empty() {
            return self.list();
        }

        let mut matches: Vec<&Capability> = self
            .capabilities
            .values()
            .filter(|c| {
                c.id.to_lowercase().contains(&q)
                    || c.label.to_lowercase().contains(&q)
                    || c.description.to_lowercase().contains(&q)
                    || c.group.to_lowercase().contains(&q)
            })
            .collect();

        matches.sort_by(|a, b| {
            let a_exact_id = a.id.to_lowercase() == q;
            let b_exact_id = b.id.to_lowercase() == q;
            if a_exact_id != b_exact_id {
                return b_exact_id.cmp(&a_exact_id);
            }
            let a_starts_label = a.label.to_lowercase().starts_with(&q);
            let b_starts_label = b.label.to_lowercase().starts_with(&q);
            if a_starts_label != b_starts_label {
                return b_starts_label.cmp(&a_starts_label);
            }
            a.label.cmp(&b.label)
        });

        matches
    }

    #[note("capability-capabilityregistry-input-schema")]
    pub fn input_schema(&self, id: &str) -> Option<serde_json::Value> {
        self.get(id).map(|c| c.input_schema())
    }

    #[note("capability-capabilityregistry-all-schemas")]
    pub fn all_schemas(&self) -> BTreeMap<String, serde_json::Value> {
        self.capabilities
            .iter()
            .map(|(id, c)| (id.clone(), c.input_schema()))
            .collect()
    }

    #[note("capability-capabilityregistry-resolve-id")]
    pub fn resolve_id(&self, name: &str) -> Option<String> {
        if self.capabilities.contains_key(name) {
            return Some(name.to_string());
        }

        let by_label: Vec<&String> = self
            .capabilities
            .iter()
            .filter(|(_, c)| c.label == name)
            .map(|(id, _)| id)
            .collect();
        if by_label.len() == 1 {
            return Some(by_label[0].clone());
        }

        let suffix = format!(".{name}");
        let by_suffix: Vec<&String> = self
            .capabilities
            .keys()
            .filter(|k| k.ends_with(&suffix))
            .collect();
        if by_suffix.len() == 1 {
            return Some(by_suffix[0].clone());
        }

        None
    }

    #[note("capability-capabilityregistry-execute")]
    pub fn execute(
        &self,
        id: &str,
        ctx: &Context<'_>,
        active_view: Option<ViewAffinity>,
    ) -> Result<Effect, CapabilityError> {
        let cap = self
            .get(id)
            .ok_or_else(|| CapabilityError::NotFound(id.to_string()))?;

        // 1. View affinity validation
        if let Some(active) = active_view {
            match cap.view_affinity {
                ViewAffinity::Any | ViewAffinity::ViewSwitch => {}
                required if required != active => {
                    return Err(CapabilityError::ViewMismatch {
                        id: id.to_string(),
                        required,
                        active,
                    });
                }
                _ => {}
            }
        }

        // 2. Required parameters validation
        for param in &cap.params {
            if param.required && ctx.args.value(&param.name).is_none() {
                return Err(CapabilityError::MissingParameter {
                    capability_id: id.to_string(),
                    param: param.name.clone(),
                });
            }
        }

        if let Some(ref schema) = cap.custom_schema
            && let Some(req_array) = schema.get("required").and_then(|r| r.as_array())
        {
            for req in req_array {
                if let Some(param_name) = req.as_str()
                    && ctx.args.value(param_name).is_none()
                {
                    return Err(CapabilityError::MissingParameter {
                        capability_id: id.to_string(),
                        param: param_name.to_string(),
                    });
                }
            }
        }

        // 3. Enabled predicate validation
        if !cap.is_enabled(ctx) {
            return Err(CapabilityError::Disabled(id.to_string()));
        }

        // 4. Concrete execution
        cap.execute(ctx)
    }

    #[note("capability-capabilityregistry-dispatch")]
    pub fn dispatch(
        &mut self,
        id: &str,
        ctx: &Context<'_>,
        active_view: Option<ViewAffinity>,
    ) -> Result<Effect, CapabilityError> {
        let result = self.execute(id, ctx, active_view);

        let timestamp_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let (success, error, effect_summary) = match &result {
            Ok(effect) => {
                let mut parts = Vec::new();
                if effect.transaction.is_some() {
                    parts.push("transaction");
                }
                if effect.selection.is_some() {
                    parts.push("selection");
                }
                if let Some(status) = &effect.status {
                    parts.push(status.as_str());
                }
                let summary = if parts.is_empty() {
                    "ok:no-op".to_string()
                } else {
                    parts.join(",")
                };
                (true, None, summary)
            }
            Err(err) => (false, Some(err.to_string()), "error".to_string()),
        };

        let audit_entry = AuditEntry {
            id: self.next_audit_id,
            timestamp_ms,
            capability_id: id.to_string(),
            author: ctx.author.clone(),
            success,
            error,
            effect_summary,
        };

        self.next_audit_id += 1;
        self.audit_log.push(audit_entry);

        if self.audit_log.len() > self.max_audit_entries {
            self.audit_log.remove(0);
        }

        result
    }

    #[note("capability-capabilityregistry-audit-log")]
    pub fn audit_log(&self) -> &[AuditEntry] {
        &self.audit_log
    }

    #[note("capability-capabilityregistry-clear-audit-log")]
    pub fn clear_audit_log(&mut self) {
        self.audit_log.clear();
    }
}

#[note("capability-paletteitem")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaletteItem {
    pub id: String,
    pub label: String,
    pub description: String,
    pub group: String,
    pub shortcut: Option<String>,
}

#[note("capability-search-commands")]
pub fn search_commands(registry: &CapabilityRegistry, filter: &str) -> Vec<PaletteItem> {
    let term = filter
        .trim_start()
        .strip_prefix('>')
        .unwrap_or(filter)
        .trim();
    let caps = registry.search(term);
    caps.into_iter()
        .map(|c| PaletteItem {
            id: c.id.clone(),
            label: c.label.clone(),
            description: c.description.clone(),
            group: c.group.clone(),
            shortcut: c.shortcut.clone(),
        })
        .collect()
}

#[cfg(test)]
#[path = "test/capability/test.rs"]
mod tests;
