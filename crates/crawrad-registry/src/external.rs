//! **THE COMPONENT THAT COMES FROM OUTSIDE** — the same descriptor, owned.
//!
//! # Why this module exists
//!
//! [ADR 0033] decides that a component is a crate on crates.io, and that the
//! IDE reads — and later builds and loads — components it did not compile in.
//! A third-party component is born **while the IDE is running**, so it has no
//! `&'static str` to give: the catalogue's [`ComponentDesc`] is made of string
//! literals, and a literal is something only the compiler can produce.
//!
//! So this is the same descriptor with `String` in place of `&'static str`. It
//! is what a manifest deserialises into and what the plugin ABI will carry, and
//! it is **one type for both transports** — the two rungs of ADR 0033 §1b carry
//! the same descriptor and differ only in how it travels.
//!
//! # The round trip is the whole proof
//!
//! [`ExternalComponent::from`] takes a native descriptor apart and
//! [`ExternalComponent::into_catalogue`] puts it back together. A gate runs
//! that round trip over **every component of `Registry::builtin()`** and
//! demands the result be equal to the original.
//!
//! That is what says the outside shape can carry everything a native component
//! has — before a single line of manifest format or ABI exists. If the round
//! trip loses a field, so would the manifest, and so would the library.
//!
//! # What LEAKS here, and why that is bounded
//!
//! Turning a `String` into a `&'static str` means `Box::leak`. It is safe Rust
//! — this crate still carries `#![forbid(unsafe_code)]` — and the memory is
//! never freed.
//!
//! The bound is the reason it is acceptable: a component is registered **once
//! per IDE run**, and ADR 0033 §5 already rules that nothing is ever unloaded,
//! because the catalogue hands out `&'static str` that live in the library's own
//! data segment. Re-registering after a reinstall leaks a second copy of a few
//! hundred bytes, and reinstalling is a gesture a person makes, not a loop.
//!
//! An interner would remove even that, and it is **not** here on purpose: it
//! costs a global lock to save bytes nobody has measured missing.
//!

use crawrad_ir::Value;
use crawrad_ir::icon::IconStep;

use crate::{Category, Children, ComponentDesc, EventDesc, PropDesc, PropKind, TextLine};
use crawrad_note::note;

#[note("external-forever")]
fn forever(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

#[note("external-forever-all")]
fn forever_all(v: Vec<String>) -> &'static [&'static str] {
    let refs: Vec<&'static str> = v.into_iter().map(forever).collect();
    Box::leak(refs.into_boxed_slice())
}

#[note("external-externalchildren")]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ExternalChildren {
    #[default]
    None,
    Area,
    /// The collection, with **the class of the items** — it is the `TCollection` of
    /// ADR 0037.
    Collection {
        item: String,
    },
}

impl From<Children> for ExternalChildren {
    fn from(f: Children) -> Self {
        match f {
            Children::NoChildren => Self::None,
            Children::Area => Self::Area,
            Children::Collection { item } => Self::Collection {
                item: item.to_string(),
            },
        }
    }
}

impl ExternalChildren {
    fn into_catalogue(self) -> Children {
        match self {
            Self::None => Children::NoChildren,
            Self::Area => Children::Area,
            Self::Collection { item } => Children::Collection {
                item: forever(item),
            },
        }
    }
}

#[note("external-externalkind")]
#[derive(Debug, Clone, PartialEq)]
pub enum ExternalKind {
    Text,
    Multiline,
    Bool,
    Int {
        min: i64,
        max: i64,
    },
    Float {
        min: f64,
        max: f64,
        step: f64,
    },
    Color,
    Edges,
    /// The classic RAD IDE set: named flags over a `Value::Map` of
    /// booleans. See [`PropKind::Set`].
    Set(Vec<String>),
    Enum(Vec<String>),
    Collection {
        hierarchical: bool,
        item_class: Option<String>,
    },
    Reference(String),
    /// See [`PropKind::FieldOf`]: the key of the sibling reference property.
    FieldOf(String),
    Rect,
    DateTime,
    FilePath,
    Cursor,
    ModalResult,
    Shortcut,
    Font,
    Picture,
}

impl From<&PropKind> for ExternalKind {
    fn from(k: &PropKind) -> Self {
        match k {
            PropKind::String => Self::Text,
            PropKind::Multiline => Self::Multiline,
            PropKind::Bool => Self::Bool,
            PropKind::Int { min, max } => Self::Int {
                min: *min,
                max: *max,
            },
            PropKind::Float { min, max, step } => Self::Float {
                min: *min,
                max: *max,
                step: *step,
            },
            PropKind::Color => Self::Color,
            PropKind::Edges => Self::Edges,
            PropKind::Set(f) => Self::Set(f.iter().map(|s| (*s).to_string()).collect()),
            PropKind::Enum(o) => Self::Enum(o.iter().map(|s| (*s).to_string()).collect()),
            PropKind::Collection {
                hierarchical,
                item_class,
            } => Self::Collection {
                hierarchical: *hierarchical,
                item_class: item_class.map(|s| (*s).to_string()),
            },
            PropKind::Reference(c) => Self::Reference((*c).to_string()),
            PropKind::FieldOf(p) => Self::FieldOf((*p).to_string()),
            PropKind::Rect => Self::Rect,
            PropKind::DateTime => Self::DateTime,
            PropKind::FilePath => Self::FilePath,
            PropKind::Cursor => Self::Cursor,
            PropKind::ModalResult => Self::ModalResult,
            PropKind::Shortcut => Self::Shortcut,
            PropKind::Font => Self::Font,
            PropKind::Picture => Self::Picture,
        }
    }
}

impl ExternalKind {
    fn into_catalogue(self) -> PropKind {
        match self {
            Self::Text => PropKind::String,
            Self::Multiline => PropKind::Multiline,
            Self::Bool => PropKind::Bool,
            Self::Int { min, max } => PropKind::Int { min, max },
            Self::Float { min, max, step } => PropKind::Float { min, max, step },
            Self::Color => PropKind::Color,
            Self::Edges => PropKind::Edges,
            Self::Set(f) => PropKind::Set(forever_all(f)),
            Self::Enum(o) => PropKind::Enum(forever_all(o)),
            Self::Collection {
                hierarchical,
                item_class,
            } => PropKind::Collection {
                hierarchical,
                item_class: item_class.map(forever),
            },
            Self::Reference(c) => PropKind::Reference(forever(c)),
            Self::FieldOf(p) => PropKind::FieldOf(forever(p)),
            Self::Rect => PropKind::Rect,
            Self::DateTime => PropKind::DateTime,
            Self::FilePath => PropKind::FilePath,
            Self::Cursor => PropKind::Cursor,
            Self::ModalResult => PropKind::ModalResult,
            Self::Shortcut => PropKind::Shortcut,
            Self::Font => PropKind::Font,
            Self::Picture => PropKind::Picture,
        }
    }
}

#[note("external-externalprop")]
#[derive(Debug, Clone, PartialEq)]
pub struct ExternalProp {
    /// The key, which still obeys the contract of ADR 0002: either it is in the
    /// common core or it is namespaced by a target that `contract::TARGETS`
    /// knows. A component from outside does not get to bypass that.
    pub key: String,
    pub display: String,
    pub default: Value,
    pub category: Category,
    pub kind: ExternalKind,
    pub readonly: bool,
    /// Whether the property can be bound to a data source; a
    /// third-party component declares it like any other flag.
    pub bindable: bool,
    pub options: Vec<String>,
    pub ref_class: Option<String>,
    pub tree_collection: bool,
    pub item_class: Option<String>,
    pub multiline: bool,
    /// `(reference key, capability)`: see `PropDesc::hidden_when`.
    pub hidden_when: Option<(String, String)>,
}

impl From<&PropDesc> for ExternalProp {
    fn from(p: &PropDesc) -> Self {
        Self {
            key: p.key.to_string(),
            display: p.display.to_string(),
            default: p.default.clone(),
            category: p.category,
            kind: (&p.kind).into(),
            readonly: p.readonly,
            bindable: p.bindable,
            options: p.options.iter().map(|s| (*s).to_string()).collect(),
            ref_class: p.ref_class.map(|c| c.to_string()),
            tree_collection: p.tree_collection,
            item_class: p.item_class.map(|s| s.to_string()),
            multiline: p.multiline,
            hidden_when: p.hidden_when.map(|(k, c)| (k.to_string(), c.to_string())),
        }
    }
}

impl ExternalProp {
    fn into_catalogue(self) -> PropDesc {
        PropDesc {
            key: forever(self.key),
            display: forever(self.display),
            default: self.default,
            category: self.category,
            kind: self.kind.into_catalogue(),
            readonly: self.readonly,
            bindable: self.bindable,
            options: forever_all(self.options),
            ref_class: self.ref_class.map(forever),
            tree_collection: self.tree_collection,
            item_class: self.item_class.map(forever),
            multiline: self.multiline,
            hidden_when: self.hidden_when.map(|(k, c)| (forever(k), forever(c))),
        }
    }
}

#[note("external-externalevent")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalEvent {
    pub name: String,
    pub display: String,
    /// The Rust signature of the handler, as the generator writes it —
    /// `fn(&mut self)`.
    pub signature: String,
}

impl From<&EventDesc> for ExternalEvent {
    fn from(e: &EventDesc) -> Self {
        Self {
            name: e.name.to_string(),
            display: e.display.to_string(),
            signature: e.signature.to_string(),
        }
    }
}

impl ExternalEvent {
    fn into_catalogue(self) -> EventDesc {
        EventDesc {
            name: forever(self.name),
            display: forever(self.display),
            signature: forever(self.signature),
        }
    }
}

#[note("external-externalcomponent")]
#[derive(Debug, Clone, PartialEq)]
pub struct ExternalComponent {
    pub class: String,
    pub display: String,
    pub base_name: String,
    pub palette: String,
    pub prefix: String,
    pub default_size: (f32, f32),
    pub props: Vec<ExternalProp>,
    pub events: Vec<ExternalEvent>,
    pub targets: Vec<String>,
    /// **What this component offers to be** — see
    /// [`crate::ComponentDesc::provides`].
    pub provides: Vec<String>,
    pub children: ExternalChildren,
    pub derived_geometry: bool,
    pub visual: bool,
    pub emits_code: bool,
    /// **Does the type have state that the manifest does not declare?** See
    /// [`crate::ComponentDesc::private_state`].
    pub private_state: bool,
    pub text_line: Option<TextLine>,
    /// What the generated project needs in `Cargo.toml` because of it —
    /// [`crate::CrateRequirement`], owned.
    pub requires: Vec<ExternalRequirement>,
    /// **The Rust module of this class's type in the generated application.**
    ///
    /// Comes from `[package] name` of `manifest.json`, with hyphens turned to underscores.
    /// Empty for native — the native type is EMITTED, not imported. See
    /// [`crate::ComponentDesc::crate_module`].
    pub crate_module: String,
    /// **How this component paints itself** — see [`crawrad_ir::drawing::Verb`].
    ///
    /// The same type on both sides: the vocabulary is already data, and there is no text to
    /// leak. Empty is the norm — a non-visual paints nothing.
    pub desenho: crawrad_ir::drawing::Drawing,
    /// The palette icon declared from outside.
    pub icon: Vec<IconStep>,
    /// The base native component this component inherits from, if any.
    pub inherits: Option<String>,
}

#[note("external-externalrequirement")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalRequirement {
    pub crate_name: String,
    pub version: String,
    pub features: Vec<String>,
    pub default_features: bool,
}

impl From<&crate::CrateRequirement> for ExternalRequirement {
    fn from(r: &crate::CrateRequirement) -> Self {
        Self {
            crate_name: r.crate_name.to_string(),
            version: r.version.to_string(),
            features: r.features.iter().map(|f| (*f).to_string()).collect(),
            default_features: r.default_features,
        }
    }
}

impl ExternalRequirement {
    fn into_catalogue(self) -> crate::CrateRequirement {
        crate::CrateRequirement {
            crate_name: forever(self.crate_name),
            version: forever(self.version),
            features: forever_all(self.features),
            default_features: self.default_features,
        }
    }
}

impl From<&ComponentDesc> for ExternalComponent {
    fn from(d: &ComponentDesc) -> Self {
        Self {
            class: d.class.to_string(),
            display: d.display.to_string(),
            base_name: d.base_name.to_string(),
            palette: d.palette.to_string(),
            prefix: d.prefix.to_string(),
            default_size: d.default_size,
            props: d.props.iter().map(ExternalProp::from).collect(),
            events: d.events.iter().map(ExternalEvent::from).collect(),
            targets: d.targets.iter().map(|s| (*s).to_string()).collect(),
            provides: d.provides.iter().map(|s| (*s).to_string()).collect(),
            children: ExternalChildren::from(d.children),
            derived_geometry: d.derived_geometry,
            visual: d.visual,
            emits_code: d.emits_code,
            private_state: d.private_state,
            text_line: d.text_line,
            requires: d.requires.iter().map(ExternalRequirement::from).collect(),
            crate_module: d.crate_module.to_string(),
            desenho: d.desenho.clone(),
            icon: d.icon.clone(),
            inherits: d.inherits.map(|s| s.to_string()),
        }
    }
}

impl ExternalComponent {
    #[note("external-externalcomponent-into-catalogue")]
    pub fn into_catalogue(self) -> ComponentDesc {
        ComponentDesc {
            class: forever(self.class),
            display: forever(self.display),
            base_name: forever(self.base_name),
            palette: forever(self.palette),
            prefix: forever(self.prefix),
            default_size: self.default_size,
            props: self
                .props
                .into_iter()
                .map(ExternalProp::into_catalogue)
                .collect(),
            events: self
                .events
                .into_iter()
                .map(ExternalEvent::into_catalogue)
                .collect(),
            targets: forever_all(self.targets),
            provides: forever_all(self.provides),
            children: self.children.into_catalogue(),
            derived_geometry: self.derived_geometry,
            visual: self.visual,
            emits_code: self.emits_code,
            private_state: self.private_state,
            text_line: self.text_line,
            requires: self
                .requires
                .into_iter()
                .map(ExternalRequirement::into_catalogue)
                .collect(),
            crate_module: forever(self.crate_module),
            desenho: self.desenho,
            icon: self.icon,
            inherits: self.inherits.map(forever),
        }
    }
}

/// A sample external component (`PriceEdit` inheriting `SpinEdit`) for tests, shared across crates.
#[cfg(any(test, feature = "test-support"))]
pub fn sample_price_edit() -> ExternalComponent {
    ExternalComponent {
        class: "PriceEdit".to_string(),
        crate_module: "finance_crawrad".to_string(),
        desenho: Vec::new(),
        icon: Vec::new(),
        display: "Price Edit".to_string(),
        base_name: "price_edit".to_string(),
        palette: "Finance".to_string(),
        prefix: "ped".to_string(),
        // Not SpinEdit's size on purpose: the base's wins in v1.
        default_size: (40.0, 10.0),
        inherits: Some("SpinEdit".to_string()),
        props: vec![
            ExternalProp {
                key: "currency".to_string(),
                display: "Currency".to_string(),
                default: Value::Str("USD".to_string()),
                category: Category::Appearance,
                kind: ExternalKind::Text,
                readonly: false,
                bindable: false,
                options: Vec::new(),
                ref_class: None,
                tree_collection: false,
                item_class: None,
                multiline: false,
                hidden_when: None,
            },
            ExternalProp {
                key: "decimals".to_string(),
                display: "Decimals".to_string(),
                default: Value::Int(2),
                category: Category::Behavior,
                kind: ExternalKind::Int { min: 0, max: 8 },
                readonly: false,
                bindable: false,
                options: Vec::new(),
                ref_class: None,
                tree_collection: false,
                item_class: None,
                multiline: false,
                hidden_when: None,
            },
        ],
        events: Vec::new(),
        targets: vec!["egui".to_string(), "makepad".to_string()],
        children: ExternalChildren::None,
        derived_geometry: false,
        visual: true,
        emits_code: true,
        private_state: false,
        provides: Vec::new(),
        text_line: None,
        requires: Vec::new(),
    }
}

#[cfg(test)]
#[path = "test/external/test.rs"]
mod test;
