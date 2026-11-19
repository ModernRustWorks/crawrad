//! **THE MANIFEST** — `manifest.json`, what the IDE reads before compiling.
//!
//! # Why the format is NOT `ExternalComponent` with `derive(Serialize)`
//!
//! It would be one line, and it would be wrong. `ExternalComponent` is the **model**,
//! and it will change: a new field is born when the catalogue gains a new question. The
//! manifest is the **format**, and its contract is with files that other people wrote
//! and published on crates.io.
//!
//! Tying one to the other would make every new field in the model become a format
//! break — and automatic serialization breaks **silently**: the field vanishes
//! from the file of whoever did not know about it, and the IDE reads the default without warning.
//!
//! They are two types, and the conversion between them is what this file holds. The
//! roundtrip gate is what proves that it loses nothing.
//!
//! # The default value has TWO forms, and the short one is the standard
//!
//! ```toml
//! default = ""          # the `kind` next to it already says it is text
//! default = true
//! default = 1000
//! default = { x = 0.0, y = 0.0, w = 28.0, h = 28.0 }
//! ```
//!
//! The `kind` **already declares the type**, so the value does not need to repeat it. Measured
//! in this codebase's catalogue: 78 of the 97 properties have a scalar default, and the short
//! form removes an entire TOML section from each of them.
//!
//! **The long form continues to exist, and is not legacy:**
//!
//! ```toml
//! default = { t = "Map", v = { … } }
//! ```
//!
//! It is the representation of [`crawrad_ir::Value`] itself — the SAME one that
//! `.crawform` records —, and it is the only **total** one: there are `Value` variants that
//! no `kind` names (`Font`, `Size`, `Point`, `Map`, `Null`), and for them
//! no short form exists. Whoever writes chooses; whoever reads accepts both.
//!
//! Disambiguation is a single rule: **a table with a text key `t` is the
//! long form.** No short form has `t` — `Color` has `r/g/b/a`, `Rect`
//! has `x/y/w/h`, `Edges` has `left/top/right/bottom`.
//!
//! And `Value` was born for this: its doc says, since the first day, that
//! `Box<dyn Any>` from the old core *"prevented plugin boundary crossing"*.

use std::collections::BTreeSet;

use crawrad_ir::Value;
use crawrad_ir::drawing::{Area, Axis, Ink, Limit, Stroke, TextSource, Verb};
use crawrad_ir::icon::IconStep;
use crawrad_note::note;
use crawrad_registry::external::{
    ExternalChildren, ExternalComponent, ExternalEvent, ExternalKind, ExternalProp,
    ExternalRequirement,
};
use crawrad_registry::{Category, TextLine};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[note("manifest-sdk")]
pub const SDK: u32 = crawrad_component::SDK;

#[note("manifest-file-name")]
pub const FILE_NAME: &str = "manifest.json";

#[note("manifest-manifesterror")]
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ManifestError {
    #[error("the manifest could not be read as JSON: {0}")]
    NotJson(String),

    #[error(
        "this file is JSON but it is not a CrawRAD manifest: it has no `sdk`. \
         The name `manifest.json` is shared with other tools, so this is \
         probably somebody else's file"
    )]
    NotACrawradManifest,

    #[error(
        "this package speaks SDK {found} and this IDE speaks {SDK}: install a \
         version of the package built for this IDE, or update the IDE"
    )]
    WrongSdk { found: u32 },

    #[error("the manifest declares no component at all")]
    NoComponent,

    #[error("the package name is empty")]
    NoName,

    #[error("the class '{0}' is declared twice in the same package")]
    ClassTwice(String),

    #[error(
        "'{class}' draws with '{found}', and that is not a verb this IDE knows:          write box, frame, text, dashed or check"
    )]
    UnknownVerb { class: String, found: String },

    #[error(
        "'{class}' paints '{field}' with '{found}', and colours are roles and          never values: write surface, border, text, text dim or accent"
    )]
    UnknownInk {
        class: String,
        field: String,
        found: String,
    },

    #[error(
        "'{class}' draws text and does not say where it comes from: write          prop to follow a property, or text to write the same words always"
    )]
    TextWithoutSource { class: String },

    #[error(
        "'{class}' gives an area and does not say its shape: write inset to          pull the paint in, or fraction of to make it follow a property"
    )]
    AreaWithoutShape { class: String },

    #[error("'{class}' grows an area along '{found}', and the axes are x and y")]
    UnknownAxis { class: String, found: String },

    #[error(
        "'{class}' holds a collection and does not declare the class of its \
         items, so write `item_class = \"...\"` next to \
         `children = \"collection\"`, or the collection takes any component at all"
    )]
    CollectionWithoutItemClass { class: String },

    #[error(
        "'{class}' declares `item_class = \"{item}\"` and does not hold a \
         collection: write `children = \"collection\"`, or remove the \
         `item_class`"
    )]
    ItemClassWithoutCollection { class: String, item: String },

    #[error(
        "the property '{key}' of '{class}' says its kind is {kind}, and its \
         default is not a value of that kind: {found}"
    )]
    DefaultDoesNotMatchKind {
        class: String,
        key: String,
        kind: String,
        found: String,
    },

    #[error(
        "'{class}' has an icon with unknown step '{found}': write move_to, line_to, \
         close_path, circle, box, rect, fill, or stroke"
    )]
    UnknownIconStep { class: String, found: String },

    #[error("'{class}' icon step '{step}' is missing required field '{field}'")]
    IconStepMissingField {
        class: String,
        step: String,
        field: String,
    },

    #[error("'{class}' icon step '{step}' has unexpected field '{field}'")]
    IconStepUnexpectedField {
        class: String,
        step: String,
        field: String,
    },

    #[error("'{class}' declares an invalid icon: {error}")]
    InvalidIcon {
        class: String,
        error: crawrad_ir::icon::IconError,
    },
}

#[note("manifest-packageid")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageId {
    /// The crate name on crates.io.
    pub name: String,
    /// Its version, as `Cargo.toml` writes it.
    #[serde(default)]
    pub version: String,
    /// A line about what it brings, for the store to display.
    #[serde(default)]
    pub description: String,
}

#[note("manifest-manifest")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    /// The version of the contract that this package speaks. Checked **before**
    /// anything else.
    pub sdk: u32,
    /// **The notice that this file is GENERATED**, for whoever opens it to edit.
    ///
    /// JSON has no comments, so the notice is a field — and a field is better than
    /// a comment anyway, because a tool can read it.
    ///
    /// The `$` in front is the convention that JSON Schema uses for metadata of the
    /// file itself, and serves to avoid colliding with anything from the domain.
    #[serde(default, rename = "$doc", skip_serializing_if = "String::is_empty")]
    pub doc: String,
    pub package: PackageId,
    /// The components that this package brings. A package can bring several — it is the
    /// `RegisterComponents` of the classic RAD IDE, which receives a list.
    #[serde(default, rename = "components")]
    pub components: Vec<ComponentEntry>,
}

#[note("manifest-drawentry")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DrawEntry {
    /// `box`, `frame`, `text`, `dashed`, `check`.
    pub verb: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fill: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub border: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thickness: Option<f32>,
    /// The text written in the manifest, the same for every instance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// The PROPERTY from which the text comes — it is what makes the drawing follow the
    /// Object Inspector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prop: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub centered: bool,
    /// **WHERE it paints.** Absent means the entire rectangle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area: Option<AreaEntry>,
}

#[note("manifest-areaentry")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AreaEntry {
    /// Inset in design units, on all sides.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inset: Option<f32>,
    /// The numeric PROPERTY from which the fraction comes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fraction_of: Option<String>,
    /// `x` (the default) or `y`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub axis: Option<String>,
    /// The beginning of the range: a NUMBER, or the name of a property.
    ///
    /// ```toml
    /// min = 0.0      # the range is fixed
    /// min = "min"    # the range follows the component's `min` property
    /// ```
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<LimitEntry>,
    /// The end of the range. Same grammar as [`Self::min`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<LimitEntry>,
}

#[note("manifest-limitentry")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LimitEntry {
    /// `min = 0.0`
    Number(f32),
    /// `min = "min"`
    Property(String),
}

impl LimitEntry {
    fn model(&self) -> Limit {
        match self {
            Self::Number(n) => Limit::Fixed(*n),
            Self::Property(k) => Limit::Property(k.clone()),
        }
    }

    // **The return is NOT written**, and could not be alone:
    // `ComponentEntry::from_model` still returns `draw: Vec::new()`, because
    // no component in the catalogue declares a drawing. Writing only this half
    // would be a function nobody calls — and `dead_code` would say so.
}

impl DrawEntry {
    fn model(&self, class: &str) -> Result<Verb, ManifestError> {
        let area = self.model_area(class)?;
        let ink =
            |field: &str, value: &Option<String>, default: Ink| -> Result<Ink, ManifestError> {
                match value {
                    None => Ok(default),
                    Some(name) => Ink::from_name(name).ok_or_else(|| ManifestError::UnknownInk {
                        class: class.to_string(),
                        field: field.to_string(),
                        found: name.clone(),
                    }),
                }
            };
        let stroke = match self.verb.as_str() {
            "box" => Stroke::Box {
                fill: ink("fill", &self.fill, Ink::Surface)?,
                border: ink("border", &self.border, Ink::Border)?,
                radius: self.radius.unwrap_or(0.0),
            },
            "frame" => Stroke::Frame {
                color: ink("color", &self.color, Ink::Border)?,
                thickness: self.thickness.unwrap_or(1.0),
            },
            "text" => Stroke::Text {
                from: match (&self.prop, &self.text) {
                    (Some(p), _) => TextSource::Property(p.clone()),
                    (None, Some(t)) => TextSource::Fixed(t.clone()),
                    (None, None) => {
                        return Err(ManifestError::TextWithoutSource {
                            class: class.to_string(),
                        });
                    }
                },
                color: ink("color", &self.color, Ink::Text)?,
                centered: self.centered,
            },
            "dashed" => Stroke::Dashed {
                color: ink("color", &self.color, Ink::TextDim)?,
            },
            "check" => Stroke::Check {
                color: ink("color", &self.color, Ink::Accent)?,
            },
            other => {
                return Err(ManifestError::UnknownVerb {
                    class: class.to_string(),
                    found: other.to_string(),
                });
            }
        };
        Ok(Verb { area, stroke })
    }

    #[note("manifest-drawentry-model-area")]
    fn model_area(&self, class: &str) -> Result<Area, ManifestError> {
        let Some(a) = &self.area else {
            return Ok(Area::All);
        };
        if let Some(r) = a.inset {
            return Ok(Area::Inset(r));
        }
        let Some(from) = &a.fraction_of else {
            return Err(ManifestError::AreaWithoutShape {
                class: class.to_string(),
            });
        };
        let axis = match &a.axis {
            None => Axis::X,
            Some(n) => Axis::from_name(n).ok_or_else(|| ManifestError::UnknownAxis {
                class: class.to_string(),
                found: n.clone(),
            })?,
        };
        Ok(Area::Fraction {
            from: from.clone(),
            axis,
            // The defaults remain 0 and 100 — the percentage, which is the range that
            // almost every gauge uses and the only one that does not need to be written.
            min: a.min.as_ref().map_or(Limit::Fixed(0.0), LimitEntry::model),
            max: a
                .max
                .as_ref()
                .map_or(Limit::Fixed(100.0), LimitEntry::model),
        })
    }
}

#[note("manifest-iconentry")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IconEntry {
    /// `move_to`, `line_to`, `close_path`, `circle`, `box`, `rect`, `fill`, `stroke`.
    pub step: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub h: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r: Option<f32>,
}

impl IconEntry {
    fn model(&self, class: &str) -> Result<IconStep, ManifestError> {
        let step = &self.step;
        let require = |val: Option<f32>, field: &str| -> Result<f32, ManifestError> {
            val.ok_or_else(|| ManifestError::IconStepMissingField {
                class: class.to_string(),
                step: step.clone(),
                field: field.to_string(),
            })
        };
        let forbid = |val: Option<f32>, field: &str| -> Result<(), ManifestError> {
            if val.is_some() {
                return Err(ManifestError::IconStepUnexpectedField {
                    class: class.to_string(),
                    step: step.clone(),
                    field: field.to_string(),
                });
            }
            Ok(())
        };

        match step.as_str() {
            "move_to" => {
                let x = require(self.x, "x")?;
                let y = require(self.y, "y")?;
                forbid(self.w, "w")?;
                forbid(self.h, "h")?;
                forbid(self.r, "r")?;
                Ok(IconStep::MoveTo(x, y))
            }
            "line_to" => {
                let x = require(self.x, "x")?;
                let y = require(self.y, "y")?;
                forbid(self.w, "w")?;
                forbid(self.h, "h")?;
                forbid(self.r, "r")?;
                Ok(IconStep::LineTo(x, y))
            }
            "close_path" => {
                forbid(self.x, "x")?;
                forbid(self.y, "y")?;
                forbid(self.w, "w")?;
                forbid(self.h, "h")?;
                forbid(self.r, "r")?;
                Ok(IconStep::ClosePath)
            }
            "circle" => {
                let x = require(self.x, "x")?;
                let y = require(self.y, "y")?;
                let r = require(self.r, "r")?;
                forbid(self.w, "w")?;
                forbid(self.h, "h")?;
                Ok(IconStep::Circle { x, y, r })
            }
            "box" => {
                let x = require(self.x, "x")?;
                let y = require(self.y, "y")?;
                let w = require(self.w, "w")?;
                let h = require(self.h, "h")?;
                let r = require(self.r, "r")?;
                Ok(IconStep::Box { x, y, w, h, r })
            }
            "rect" => {
                let x = require(self.x, "x")?;
                let y = require(self.y, "y")?;
                let w = require(self.w, "w")?;
                let h = require(self.h, "h")?;
                forbid(self.r, "r")?;
                Ok(IconStep::Rect { x, y, w, h })
            }
            "fill" => {
                forbid(self.x, "x")?;
                forbid(self.y, "y")?;
                forbid(self.w, "w")?;
                forbid(self.h, "h")?;
                forbid(self.r, "r")?;
                Ok(IconStep::Fill)
            }
            "stroke" => {
                forbid(self.x, "x")?;
                forbid(self.y, "y")?;
                forbid(self.w, "w")?;
                forbid(self.h, "h")?;
                forbid(self.r, "r")?;
                Ok(IconStep::Stroke)
            }
            other => Err(ManifestError::UnknownIconStep {
                class: class.to_string(),
                found: other.to_string(),
            }),
        }
    }

    pub fn from_model(step: &IconStep) -> Self {
        match *step {
            IconStep::MoveTo(x, y) => Self {
                step: "move_to".to_string(),
                x: Some(x),
                y: Some(y),
                w: None,
                h: None,
                r: None,
            },
            IconStep::LineTo(x, y) => Self {
                step: "line_to".to_string(),
                x: Some(x),
                y: Some(y),
                w: None,
                h: None,
                r: None,
            },
            IconStep::ClosePath => Self {
                step: "close_path".to_string(),
                x: None,
                y: None,
                w: None,
                h: None,
                r: None,
            },
            IconStep::Circle { x, y, r } => Self {
                step: "circle".to_string(),
                x: Some(x),
                y: Some(y),
                w: None,
                h: None,
                r: Some(r),
            },
            IconStep::Box { x, y, w, h, r } => Self {
                step: "box".to_string(),
                x: Some(x),
                y: Some(y),
                w: Some(w),
                h: Some(h),
                r: Some(r),
            },
            IconStep::Rect { x, y, w, h } => Self {
                step: "rect".to_string(),
                x: Some(x),
                y: Some(y),
                w: Some(w),
                h: Some(h),
                r: None,
            },
            IconStep::Fill => Self {
                step: "fill".to_string(),
                x: None,
                y: None,
                w: None,
                h: None,
                r: None,
            },
            IconStep::Stroke => Self {
                step: "stroke".to_string(),
                x: None,
                y: None,
                w: None,
                h: None,
                r: None,
            },
        }
    }
}

#[note("manifest-componententry")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentEntry {
    /// The class name. Becomes TYPE in the generated project.
    pub class: String,
    /// The name in the palette and in the Object Inspector.
    pub display: String,
    /// The palette group — the `RegisterComponents('My Components', …)` of the
    /// classic RAD IDE.
    pub palette: String,
    /// The start of the automatic name: `dbe` gives `conn1`, `conn2`.
    pub prefix: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherits: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_name: Option<String>,
    /// Width and height at creation, in design units.
    pub default_size: [f32; 2],
    /// Appears on the application screen, or is it non-visual like `Timer`?
    #[serde(default = "default_true", skip_serializing_if = "is_true")]
    pub visual: bool,
    /// Becomes runnable code?
    #[serde(default = "default_true", skip_serializing_if = "is_true")]
    pub emits_code: bool,
    /// **Does the type have state that THIS file does not declare?**
    ///
    /// Yes is the default, and it is the safe answer: a handwritten manifest
    /// cannot know which fields the crate's `struct` has, and the generator needs to
    /// start from the crate's `Default` so as not to leave any field without a value.
    ///
    /// `#[derive(Component)]` writes `false` when it has published ALL fields
    /// — and then the generator emits a complete literal. This is not a style detail: with
    /// the complete literal, `..Default::default()` becomes `needless_update` and the
    /// user's project **stops compiling** under `-D warnings`. Measured on
    /// 18/09/2026, the day the macro started publishing everything.
    #[serde(default = "default_true", skip_serializing_if = "is_true")]
    pub private_state: bool,
    /// Accepts children, and where they appear.
    #[serde(default, skip_serializing_if = "Children::is_default")]
    pub children: Children,
    /// **The class of the items**, when `children = "collection"`.
    ///
    /// Required in this case, and refused in any other. It is the `TCollection`
    /// of ADR 0037: a collection that does not say what its items are accepts
    /// anything inside — today nothing prevents a `Button` from becoming a child of
    /// an `ActionList`.
    ///
    /// It sits in a field NEXT TO IT, not inside `children`, because it is the grammar
    /// this manifest already uses for `kind`: bare name, neighboring payload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_class: Option<String>,
    /// Geometry is a consequence and not a choice — the `TabSheet` of
    /// `PageControl`.
    #[serde(default, skip_serializing_if = "is_false")]
    pub derived_geometry: bool,
    /// Where the text goes, when the component has single-line text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_line: Option<TextPlacement>,
    /// The runtimes that draw it. `["*"]` for universal.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<String>,
    /// **What this component OFFERS to be**, besides its own class.
    ///
    /// It is the side of whoever is POINTED TO. A reference property states the name
    /// it accepts (`ref_class`), and a component is accepted when that name is
    /// its class **or** is in this list.
    ///
    /// # Why listing classes in the reference was not enough
    ///
    /// The connection abstraction accepts any native provider. Writing the five
    /// names inside it would make the SIXTH provider require editing the abstraction — which
    /// is the opposite of what abstraction means. With the capability, the new provider
    /// is born and is already accepted, and nobody touches `Connection`.
    ///
    /// **It is interface, not inheritance:** a component offers as many capabilities
    /// as it wants, and none of them forces it to inherit anything.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provides: Vec<String>,
    #[serde(default, rename = "props", skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<PropEntry>,
    #[serde(default, rename = "events", skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<EventEntry>,
    /// **How this component paints itself in the designer.**
    ///
    /// Empty is normal — a non-visual paints nothing, and a component that
    /// brings drawing code does not need this. See [`DrawEntry`].
    #[serde(default, rename = "draw", skip_serializing_if = "Vec::is_empty")]
    pub draw: Vec<DrawEntry>,
    /// **The palette icon this component declares.**
    ///
    /// Empty is normal for native components, whose icon is compiled into the IDE.
    #[serde(default, rename = "icon", skip_serializing_if = "Vec::is_empty")]
    pub icon: Vec<IconEntry>,
    /// What the GENERATED project needs in `Cargo.toml` because of this
    /// component.
    #[serde(default, rename = "requires", skip_serializing_if = "Vec::is_empty")]
    pub requires: Vec<RequirementEntry>,
}

#[note("manifest-requiremententry")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequirementEntry {
    /// The crate name, as crates.io knows it.
    #[serde(rename = "crate")]
    pub crate_name: String,
    /// The version requirement, as `Cargo.toml` writes it.
    pub version: String,
    /// The features that THIS component turns on.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<String>,
    /// Do the crate's default features go in? Almost always yes.
    #[serde(default = "default_true", skip_serializing_if = "is_true")]
    pub default_features: bool,
}

impl RequirementEntry {
    fn model(&self) -> ExternalRequirement {
        ExternalRequirement {
            crate_name: self.crate_name.clone(),
            version: self.version.clone(),
            features: self.features.clone(),
            default_features: self.default_features,
        }
    }

    fn from_model(r: &ExternalRequirement) -> Self {
        Self {
            crate_name: r.crate_name.clone(),
            version: r.version.clone(),
            features: r.features.clone(),
            default_features: r.default_features,
        }
    }
}

fn default_true() -> bool {
    true
}

#[note("manifest-is-false")]
fn is_false(b: &bool) -> bool {
    !*b
}

fn is_true(b: &bool) -> bool {
    *b
}

#[note("manifest-propentry")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropEntry {
    /// The key, which follows the contract of ADR 0002: either it is in the common core,
    /// or it has a known target prefix.
    pub key: String,
    pub display: String,
    pub category: Group,
    pub kind: Kind,
    /// The initial value — see the header note about the two forms.
    pub default: JsonValue,
    #[serde(default, skip_serializing_if = "is_false")]
    pub readonly: bool,
    /// The property can be bound to a data source in the Inspector.
    #[serde(default, skip_serializing_if = "is_false")]
    pub bindable: bool,
    /// The payload of `kind = "enum"`: the possible values.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<String>,
    /// The payload of `kind = "reference"`: the class that the reference accepts.
    ///
    /// It is what makes the Object Inspector list only actions in `Button.Action`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ref_class: Option<String>,
    /// The payload of `kind = "field_of"`: the key of the sibling reference property
    /// whose dataset's fields this property names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_of: Option<String>,
    /// The payload of `kind = "collection"`: do the items have children?
    #[serde(default, skip_serializing_if = "is_false")]
    pub tree_collection: bool,
    /// The payload of `kind = "collection"`: the typed class of the items.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_class: Option<String>,
    /// The payload of `kind = "int"` and `kind = "float"`: the lowest accepted value.
    ///
    /// Absent means *unbounded*, and the limit that the catalogue uses when
    /// nobody says anything is that of the type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<JsonValue>,
    /// The highest accepted value. See [`PropEntry::min`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<JsonValue>,
    /// The payload of `kind = "float"`: how much an Inspector arrow steps.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub multiline: bool,
    /// `[reference key, capability]`: the Inspector hides this property while
    /// that sibling reference points at a component answering to it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden_when: Option<(String, String)>,
}

#[note("manifest-evententry")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventEntry {
    pub name: String,
    pub display: String,
    /// The handler signature, as the generator writes it.
    #[serde(default = "default_event_signature")]
    pub signature: String,
}

fn default_event_signature() -> String {
    "fn(&mut self)".to_string()
}

#[note("manifest-group")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Group {
    Identity,
    Appearance,
    Data,
    Behavior,
    Layout,
    Advanced,
}

#[note("manifest-children")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Children {
    /// Does not accept: dropping something on top of it falls to its parent.
    #[default]
    None,
    /// Has its own surface, and children appear inside it.
    Area,
    /// Accepts children in the document and tree, but not on screen — the
    /// `ActionList`.
    Collection,
}

#[note("manifest-textplacement")]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextPlacement {
    /// Vertically centered — button, checkbox, edit.
    Centered,
    /// Pushed to the top, with the margin in design units — label.
    Top(f32),
}

#[note("manifest-kind")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Text,
    Multiline,
    Bool,
    Int,
    Float,
    Color,
    Edges,
    /// Named flags — the *set*. The flags come in the neighboring `options`,
    /// and the default value is a table of booleans.
    Set,
    Enum,
    Collection,
    Reference,
    /// The name of a field of the dataset that the sibling property in `field_of`
    /// references; the Inspector lists that dataset's columns.
    FieldOf,
    Rect,
    DateTime,
    FilePath,
    Cursor,
    ModalResult,
    Shortcut,
    Font,
    Picture,
}

// ---------------------------------------------------------------------------
// THE CONVERSION — format ⇄ model
// ---------------------------------------------------------------------------

impl Manifest {
    #[note("manifest-manifest-parse")]
    pub fn parse(text: &str) -> Result<Self, ManifestError> {
        let raw: JsonValue =
            serde_json::from_str(text).map_err(|e| ManifestError::NotJson(e.to_string()))?;
        // THE VERSION FIRST, and read from raw JSON: asking serde to construct
        // the entire `Manifest` before looking at `sdk` would cause a file of another
        // contract to be refused by the wrong field, with the wrong phrasing.
        // **NOT BEING our manifest is another refusal, and comes before the version.**
        //
        // The file is named `manifest.json`, a name that many tools use. Without
        // this check, the `manifest.json` of another tool would fall into the VERSION
        // complaint — *"this package speaks SDK 0"* — and would send the user
        // looking for an update that does not exist.
        let Some(sdk_num) = raw.get("sdk").and_then(|v| v.as_i64()) else {
            return Err(ManifestError::NotACrawradManifest);
        };
        let Ok(sdk) = u32::try_from(sdk_num) else {
            return Err(ManifestError::WrongSdk { found: 0 });
        };
        if sdk != SDK && sdk != crawrad_component::manifest::SDK_INHERITS {
            return Err(ManifestError::WrongSdk { found: sdk });
        }

        let m: Manifest =
            serde_json::from_str(text).map_err(|e| ManifestError::NotJson(e.to_string()))?;
        if m.package.name.trim().is_empty() {
            return Err(ManifestError::NoName);
        }
        if m.components.is_empty() {
            return Err(ManifestError::NoComponent);
        }
        // TWO CLASSES WITH THE SAME NAME in the same package: the second would win
        // silently at registration time, and the author would have no way to know
        // which of the two is in the palette.
        let mut seen = BTreeSet::new();
        for c in &m.components {
            if !seen.insert(c.class.as_str()) {
                return Err(ManifestError::ClassTwice(c.class.clone()));
            }
        }
        Ok(m)
    }

    #[note("manifest-manifest-to-json")]
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("a manifest always serializes")
    }

    #[note("manifest-manifest-to-components")]
    pub fn to_components(&self) -> Result<Vec<ExternalComponent>, ManifestError> {
        // The type MODULE comes from the package name, and not from a field the
        // author would write again — Cargo already does this translation when exposing a
        // crate as a path, and writing it twice is the duplication this
        // codebase has already paid for.
        let module = self.package.name.replace('-', "_");
        self.components.iter().map(|c| c.model(&module)).collect()
    }
}

impl ComponentEntry {
    fn model(&self, crate_module: &str) -> Result<ExternalComponent, ManifestError> {
        Ok(ExternalComponent {
            crate_module: crate_module.to_string(),
            class: self.class.clone(),
            display: self.display.clone(),
            base_name: self.base_name.clone().unwrap_or_else(|| {
                if !self.prefix.is_empty() {
                    self.prefix.clone()
                } else {
                    self.class.to_ascii_lowercase()
                }
            }),
            palette: self.palette.clone(),
            prefix: self.prefix.clone(),
            default_size: (self.default_size[0], self.default_size[1]),
            props: self
                .props
                .iter()
                .map(|p| p.model(&self.class))
                .collect::<Result<Vec<_>, _>>()?,
            events: self.events.iter().map(EventEntry::model).collect(),
            desenho: self
                .draw
                .iter()
                .map(|d| d.model(&self.class))
                .collect::<Result<Vec<_>, _>>()?,
            icon: {
                let steps = self
                    .icon
                    .iter()
                    .map(|i| i.model(&self.class))
                    .collect::<Result<Vec<_>, _>>()?;
                if !steps.is_empty() {
                    crawrad_ir::icon::validate(&steps).map_err(|error| {
                        ManifestError::InvalidIcon {
                            class: self.class.clone(),
                            error,
                        }
                    })?;
                }
                steps
            },
            targets: self.targets.clone(),
            provides: self.provides.clone(),
            children: self.children.model(self.item_class.clone(), &self.class)?,
            derived_geometry: self.derived_geometry,
            visual: self.visual,
            emits_code: self.emits_code,
            private_state: self.private_state,
            text_line: self.text_line.map(TextPlacement::model),
            requires: self.requires.iter().map(RequirementEntry::model).collect(),
            inherits: self.inherits.clone(),
        })
    }

    #[note("manifest-componententry-from-model")]
    pub fn from_model(c: &ExternalComponent) -> Self {
        Self {
            class: c.class.clone(),
            display: c.display.clone(),
            base_name: Some(c.base_name.clone()),
            palette: c.palette.clone(),
            prefix: c.prefix.clone(),
            inherits: c.inherits.clone(),
            default_size: [c.default_size.0, c.default_size.1],
            visual: c.visual,
            emits_code: c.emits_code,
            private_state: c.private_state,
            provides: c.provides.clone(),
            children: Children::from_model(&c.children),
            derived_geometry: c.derived_geometry,
            // `crate_module` does NOT return to the file: it is DERIVED from
            // `[package] name`, and recording it would be the second copy.
            item_class: match &c.children {
                ExternalChildren::Collection { item } => Some(item.clone()),
                _ => None,
            },
            text_line: c.text_line.map(TextPlacement::from_model),
            targets: c.targets.clone(),
            props: c.props.iter().map(PropEntry::from_model).collect(),
            events: c.events.iter().map(EventEntry::from_model).collect(),
            // Returning the drawing is not written yet, and the roundtrip gate
            // is who will demand it when a component in the catalogue
            // declares one. Today none declares — the native ones draw via
            // code —, so this is truth and not a shortcut.
            draw: Vec::new(),
            icon: c.icon.iter().map(IconEntry::from_model).collect(),
            requires: c
                .requires
                .iter()
                .map(RequirementEntry::from_model)
                .collect(),
        }
    }
}

#[note("manifest-tag")]
const TAG: &str = "t";

#[note("manifest-short-form")]
fn short_form(v: &Value) -> Option<JsonValue> {
    let t = match v {
        Value::Str(s) | Value::Enum(s) | Value::Ref(s) => JsonValue::String(s.clone()),
        Value::Int(i) => JsonValue::from(*i),
        Value::Float(f) => JsonValue::from(*f),
        Value::Bool(b) => JsonValue::Bool(*b),
        Value::Color(c) => serde_json::to_value(c).ok()?,
        Value::Rect(r) => serde_json::to_value(r).ok()?,
        Value::Edges(e) => serde_json::to_value(e).ok()?,
        Value::DateTime(dt) => serde_json::to_value(dt).ok()?,
        Value::FilePath(fp) => JsonValue::String(fp.path.clone()),
        Value::Cursor(c) => JsonValue::String(c.as_str().to_string()),
        Value::ModalResult(mr) => JsonValue::String(mr.as_str().to_string()),
        Value::Shortcut(sc) => JsonValue::String(sc.to_string()),
        Value::Font(f) => serde_json::to_value(f).ok()?,
        Value::Picture(p) => serde_json::to_value(p).ok()?,
        Value::List(l) => JsonValue::Array(l.iter().map(long_form).collect()),
        _ => return None,
    };
    Some(t)
}

#[note("manifest-long-form")]
fn long_form(v: &Value) -> JsonValue {
    serde_json::to_value(v).expect("every `Value` serializes — it was born serializable")
}

#[note("manifest-write-value")]
fn write_value(v: &Value) -> JsonValue {
    short_form(v).unwrap_or_else(|| long_form(v))
}

#[note("manifest-number")]
fn number(t: &JsonValue) -> Option<f64> {
    t.as_f64()
}

#[note("manifest-min-limit")]
fn min_limit(k: &ExternalKind) -> Option<JsonValue> {
    match k {
        ExternalKind::Int { min, .. } if *min != i64::MIN => Some(JsonValue::from(*min)),
        ExternalKind::Float { min, .. } if *min != -1e9 => Some(JsonValue::from(*min)),
        _ => None,
    }
}

#[note("manifest-max-limit")]
fn max_limit(k: &ExternalKind) -> Option<JsonValue> {
    match k {
        ExternalKind::Int { max, .. } if *max != i64::MAX => Some(JsonValue::from(*max)),
        ExternalKind::Float { max, .. } if *max != 1e9 => Some(JsonValue::from(*max)),
        _ => None,
    }
}

#[note("manifest-step-value")]
fn step_value(k: &ExternalKind) -> Option<f64> {
    match k {
        ExternalKind::Float { step, .. } if (*step - 1.0).abs() > f64::EPSILON => Some(*step),
        _ => None,
    }
}

#[note("manifest-read-value")]
fn read_value(kind: Kind, t: &JsonValue) -> Option<Value> {
    if let Some(tab) = t.as_object()
        && tab.get(TAG).and_then(JsonValue::as_str).is_some()
    {
        return serde_json::from_value(t.clone()).ok();
    }
    let v = match (kind, t) {
        (Kind::Text | Kind::Multiline | Kind::FieldOf, JsonValue::String(s)) => {
            Value::Str(s.clone())
        }
        (Kind::Enum, JsonValue::String(s)) => Value::Enum(s.clone()),
        (Kind::Reference, JsonValue::String(s)) => Value::Ref(s.clone()),
        (Kind::Bool, JsonValue::Bool(b)) => Value::Bool(*b),
        (Kind::Int, JsonValue::Number(n)) => Value::Int(n.as_i64()?),
        // AN INTEGER WORKS FOR A DECIMAL FIELD, and JSON requires it: `0.0`
        // written as `0` arrives as integer, and refusing that would make the format
        // reject a file that a person wrote correctly.
        (Kind::Float, JsonValue::Number(n)) => Value::Float(n.as_f64()?),
        (Kind::Color, _) => Value::Color(serde_json::from_value(t.clone()).ok()?),
        (Kind::Rect, _) => Value::Rect(serde_json::from_value(t.clone()).ok()?),
        (Kind::Edges, _) => Value::Edges(serde_json::from_value(t.clone()).ok()?),
        (Kind::DateTime, JsonValue::String(s)) => {
            let dt = crawrad_ir::DateTime::parse(s).ok()?;
            Value::DateTime(dt)
        }
        (Kind::DateTime, _) => Value::DateTime(serde_json::from_value(t.clone()).ok()?),
        (Kind::FilePath, JsonValue::String(s)) => {
            Value::FilePath(crawrad_ir::FilePath::new(s.as_str()))
        }
        (Kind::FilePath, _) => Value::FilePath(serde_json::from_value(t.clone()).ok()?),
        (Kind::Cursor, JsonValue::String(s)) => {
            Value::Cursor(crawrad_ir::Cursor::from_str_name(s.as_str())?)
        }
        (Kind::Cursor, _) => Value::Cursor(serde_json::from_value(t.clone()).ok()?),
        (Kind::ModalResult, JsonValue::String(s)) => {
            Value::ModalResult(crawrad_ir::ModalResult::from_str_name(s.as_str())?)
        }
        (Kind::ModalResult, _) => Value::ModalResult(serde_json::from_value(t.clone()).ok()?),
        (Kind::Shortcut, JsonValue::String(s)) => {
            Value::Shortcut(crawrad_ir::Shortcut::parse(s.as_str())?)
        }
        (Kind::Shortcut, _) => Value::Shortcut(serde_json::from_value(t.clone()).ok()?),
        (Kind::Font, JsonValue::String(s)) => {
            Value::Font(crawrad_ir::Font::from_str_formatted(s.as_str())?)
        }
        (Kind::Font, _) => Value::Font(serde_json::from_value(t.clone()).ok()?),
        (Kind::Picture, JsonValue::String(s)) => {
            use std::str::FromStr;
            Value::Picture(crawrad_ir::Picture::from_str(s.as_str()).ok()?)
        }
        (Kind::Picture, _) => Value::Picture(serde_json::from_value(t.clone()).ok()?),
        (Kind::Collection, JsonValue::Array(a)) => Value::List(
            a.iter()
                .map(|i| serde_json::from_value(i.clone()).ok())
                .collect::<Option<Vec<Value>>>()?,
        ),
        _ => return None,
    };
    Some(v)
}

impl PropEntry {
    #[note("manifest-propentry-model-kind")]
    fn model_kind(&self) -> ExternalKind {
        match self.kind {
            Kind::Text => ExternalKind::Text,
            Kind::Multiline => ExternalKind::Multiline,
            Kind::Bool => ExternalKind::Bool,
            Kind::Int => ExternalKind::Int {
                min: self
                    .min
                    .as_ref()
                    .and_then(JsonValue::as_i64)
                    .unwrap_or(i64::MIN),
                max: self
                    .max
                    .as_ref()
                    .and_then(JsonValue::as_i64)
                    .unwrap_or(i64::MAX),
            },
            Kind::Float => ExternalKind::Float {
                min: self.min.as_ref().and_then(number).unwrap_or(-1e9),
                max: self.max.as_ref().and_then(number).unwrap_or(1e9),
                step: self.step.unwrap_or(1.0),
            },
            Kind::Color => ExternalKind::Color,
            Kind::Edges => ExternalKind::Edges,
            Kind::Set => ExternalKind::Set(self.options.clone()),
            Kind::Enum => ExternalKind::Enum(self.options.clone()),
            Kind::Collection => ExternalKind::Collection {
                hierarchical: self.tree_collection,
                item_class: self.item_class.clone(),
            },
            Kind::Reference => ExternalKind::Reference(self.ref_class.clone().unwrap_or_default()),
            Kind::FieldOf => ExternalKind::FieldOf(self.field_of.clone().unwrap_or_default()),
            Kind::Rect => ExternalKind::Rect,
            Kind::DateTime => ExternalKind::DateTime,
            Kind::FilePath => ExternalKind::FilePath,
            Kind::Cursor => ExternalKind::Cursor,
            Kind::ModalResult => ExternalKind::ModalResult,
            Kind::Shortcut => ExternalKind::Shortcut,
            Kind::Font => ExternalKind::Font,
            Kind::Picture => ExternalKind::Picture,
        }
    }

    fn model(&self, class: &str) -> Result<ExternalProp, ManifestError> {
        let kind = self.model_kind();
        let default = read_value(self.kind, &self.default).ok_or_else(|| {
            ManifestError::DefaultDoesNotMatchKind {
                class: class.to_string(),
                key: self.key.clone(),
                kind: self.kind.name().to_string(),
                found: self.default.to_string(),
            }
        })?;
        Ok(ExternalProp {
            key: self.key.clone(),
            display: self.display.clone(),
            default,
            category: self.category.model(),
            readonly: self.readonly,
            bindable: self.bindable,
            // **For `Set`, the flags live in `kind` and nowhere else.**
            //
            // The model's `options` belongs to `Enum`: it is what the Inspector lists in a
            // combobox. `Set` uses the same field IN THE FILE, because the manifest's
            // grammar is uniform — but returning it also to the model would create a
            // second copy of the same list, and the roundtrip would cease to be
            // equal to the input. It was the gate of `every_catalogue_component_fits_in_manifest`
            // that demonstrated this, in both directions.
            options: match kind {
                ExternalKind::Set(_) => Vec::new(),
                _ => self.options.clone(),
            },
            kind,
            ref_class: self.ref_class.clone(),
            tree_collection: self.tree_collection,
            item_class: self.item_class.clone(),
            multiline: self.multiline,
            hidden_when: self.hidden_when.clone(),
        })
    }

    fn from_model(p: &ExternalProp) -> Self {
        Self {
            key: p.key.clone(),
            display: p.display.clone(),
            category: Group::from_model(p.category),
            kind: Kind::from_model(&p.kind),
            default: write_value(&p.default),
            readonly: p.readonly,
            bindable: p.bindable,
            // **The flags of a `Set` also live in `options`.**
            //
            // `options` belonged to `Enum` and only to it; `Set` (ADR 0037) uses the
            // same neighboring field, because this file's grammar is uniform —
            // `kind` is a bare name and the payload sits next to it. Without this line
            // `Set` returned from the file with an EMPTY list, and it was the roundtrip
            // gate that caught it.
            options: match &p.kind {
                ExternalKind::Set(f) => f.clone(),
                _ => p.options.clone(),
            },
            ref_class: p.ref_class.clone(),
            field_of: match &p.kind {
                ExternalKind::FieldOf(source) => Some(source.clone()),
                _ => None,
            },
            tree_collection: p.tree_collection,
            item_class: match &p.kind {
                ExternalKind::Collection { item_class, .. } => item_class.clone(),
                _ => p.item_class.clone(),
            },
            min: min_limit(&p.kind),
            max: max_limit(&p.kind),
            step: step_value(&p.kind),
            multiline: p.multiline,
            hidden_when: p.hidden_when.clone(),
        }
    }
}

impl EventEntry {
    fn model(&self) -> ExternalEvent {
        ExternalEvent {
            name: self.name.clone(),
            display: self.display.clone(),
            signature: self.signature.clone(),
        }
    }

    fn from_model(e: &ExternalEvent) -> Self {
        Self {
            name: e.name.clone(),
            display: e.display.clone(),
            signature: e.signature.clone(),
        }
    }
}

impl Group {
    fn model(self) -> Category {
        match self {
            Self::Identity => Category::Identity,
            Self::Appearance => Category::Appearance,
            Self::Data => Category::Data,
            Self::Behavior => Category::Behavior,
            Self::Layout => Category::Layout,
            Self::Advanced => Category::Advanced,
        }
    }

    fn from_model(c: Category) -> Self {
        match c {
            Category::Identity => Self::Identity,
            Category::Appearance => Self::Appearance,
            Category::Data => Self::Data,
            Category::Behavior => Self::Behavior,
            Category::Layout => Self::Layout,
            Category::Advanced => Self::Advanced,
        }
    }
}

impl Children {
    #[note("manifest-children-is-default")]
    fn is_default(&self) -> bool {
        matches!(self, Self::None)
    }

    #[note("manifest-children-model")]
    fn model(
        self,
        item_class: Option<String>,
        class: &str,
    ) -> Result<ExternalChildren, ManifestError> {
        match (self, item_class) {
            (Self::None, None) => Ok(ExternalChildren::None),
            (Self::Area, None) => Ok(ExternalChildren::Area),
            (Self::Collection, Some(item)) => Ok(ExternalChildren::Collection { item }),
            (Self::Collection, None) => Err(ManifestError::CollectionWithoutItemClass {
                class: class.to_string(),
            }),
            (_, Some(item)) => Err(ManifestError::ItemClassWithoutCollection {
                class: class.to_string(),
                item,
            }),
        }
    }

    fn from_model(f: &ExternalChildren) -> Self {
        match f {
            ExternalChildren::None => Self::None,
            ExternalChildren::Area => Self::Area,
            ExternalChildren::Collection { .. } => Self::Collection,
        }
    }
}

impl TextPlacement {
    fn model(self) -> TextLine {
        match self {
            Self::Centered => TextLine::Centered,
            Self::Top(m) => TextLine::Top(m),
        }
    }

    fn from_model(t: TextLine) -> Self {
        match t {
            TextLine::Centered => Self::Centered,
            TextLine::Top(m) => Self::Top(m),
        }
    }
}

impl Kind {
    #[note("manifest-kind-name")]
    fn name(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Multiline => "multiline",
            Self::Bool => "bool",
            Self::Int => "int",
            Self::Float => "float",
            Self::Color => "color",
            Self::Edges => "edges",
            Self::Set => "set",
            Self::Enum => "enum",
            Self::Collection => "collection",
            Self::Reference => "reference",
            Self::FieldOf => "field_of",
            Self::Rect => "rect",
            Self::DateTime => "date_time",
            Self::FilePath => "file_path",
            Self::Cursor => "cursor",
            Self::ModalResult => "modal_result",
            Self::Shortcut => "shortcut",
            Self::Font => "font",
            Self::Picture => "picture",
        }
    }

    fn from_model(k: &ExternalKind) -> Self {
        match k {
            ExternalKind::Text => Self::Text,
            ExternalKind::Multiline => Self::Multiline,
            ExternalKind::Bool => Self::Bool,
            ExternalKind::Int { .. } => Self::Int,
            ExternalKind::Float { .. } => Self::Float,
            ExternalKind::Color => Self::Color,
            ExternalKind::Edges => Self::Edges,
            ExternalKind::Set(_) => Self::Set,
            ExternalKind::Enum(_) => Self::Enum,
            ExternalKind::Collection { .. } => Self::Collection,
            ExternalKind::Reference(_) => Self::Reference,
            ExternalKind::FieldOf(_) => Self::FieldOf,
            ExternalKind::Rect => Self::Rect,
            ExternalKind::DateTime => Self::DateTime,
            ExternalKind::FilePath => Self::FilePath,
            ExternalKind::Cursor => Self::Cursor,
            ExternalKind::ModalResult => Self::ModalResult,
            ExternalKind::Shortcut => Self::Shortcut,
            ExternalKind::Font => Self::Font,
            ExternalKind::Picture => Self::Picture,
        }
    }
}
