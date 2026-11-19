//! # crawrad-registry — the component catalog
//!
//! Answers "what is a Button": display name, palette category,
//! default size, properties with type and initial value, events, and **which
//! runtimes implement them**.
//!
//! It is the single source of truth for the inspector and the palette — neither has its own
//! list of components or properties. The old core had three competing component
//! models and 10,457 lines just for the palette (§3.8 of the study).

// This crate has no `unsafe`, and the compiler enforces this. CrawRAD
// does not have A SINGLE line of unsafe code: the FFI adapter was removed in ADR 0013 — the rest of the system is safe Rust, and this boundary is as
// important as that of ADR 0002.
#![forbid(unsafe_code)]

pub mod actions;
pub mod class_property;
pub mod components;
pub mod external;
pub mod props;
mod reference;
pub mod registration;

pub use actions::{StandardAction, standard_actions, standard_actions_by_category};
pub use components::*;
#[cfg(any(test, feature = "test-support"))]
pub use external::sample_price_edit;
pub use props::*;

use crawrad_ir::{Node, Rect, Value};
use crawrad_note::note;
use std::collections::BTreeMap;

#[crawrad_note::note("registry-category")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Category {
    Identity,
    Appearance,
    /// Where the data comes from: the source, the table, the design-time rows.
    Data,
    Behavior,
    Layout,
    Advanced,
}

impl Category {
    #[crawrad_note::note("registry-category-label")]
    pub fn label(self) -> &'static str {
        match self {
            Self::Identity => "Identity",
            Self::Appearance => "Appearance",
            Self::Data => "Data",
            Self::Behavior => "Behavior",
            Self::Layout => "Layout",
            Self::Advanced => "Advanced",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropDesc {
    /// Property key — follows ADR 0002 contract (`text`, `win32.PasswordChar`).
    pub key: &'static str,
    pub display: &'static str,
    pub default: Value,
    pub category: Category,
    pub kind: PropKind,
    pub readonly: bool,
    /// Property participates in declarative data binding.
    pub bindable: bool,
    /// Accepted values when kind is enum.
    pub options: &'static [&'static str],
    /// When kind is `Ref`: the class accepted by the reference.
    pub ref_class: Option<&'static str>,
    /// When kind is collection: items have children (tree vs flat collection).
    pub tree_collection: bool,
    /// When kind is collection: typed class of items (if any).
    pub item_class: Option<&'static str>,
    /// When kind is `Str`: accepts line breaks.
    pub multiline: bool,
    /// `(reference key, capability)`: the Inspector hides this property while
    /// that sibling reference points at a component answering to the
    /// capability — decided by TYPE, never per component.
    pub hidden_when: Option<(&'static str, &'static str)>,
}

impl PropDesc {
    pub fn new(
        key: &'static str,
        display: &'static str,
        default: Value,
        category: Category,
    ) -> Self {
        let kind = match &default {
            Value::Str(_) => PropKind::String,
            Value::Bool(_) => PropKind::Bool,
            Value::Int(_) => PropKind::Int {
                min: i64::MIN,
                max: i64::MAX,
            },
            Value::Float(_) => PropKind::Float {
                min: -1e9,
                max: 1e9,
                step: 1.0,
            },
            Value::Color(_) => PropKind::Color,
            Value::Rect(_) => PropKind::Rect,
            Value::Edges(_) => PropKind::Edges,
            Value::Enum(_) => PropKind::Enum(&[]),
            Value::List(_) => PropKind::Collection {
                hierarchical: false,
                item_class: None,
            },
            Value::Ref(_) => PropKind::Reference(""),
            _ => PropKind::String,
        };
        Self {
            key,
            display,
            default,
            category,
            kind,
            readonly: false,
            bindable: false,
            options: &[],
            ref_class: None,
            tree_collection: false,
            item_class: None,
            multiline: false,
            hidden_when: None,
        }
    }

    pub fn new_typed(
        key: &'static str,
        display: &'static str,
        default: Value,
        category: Category,
        kind: PropKind,
    ) -> Self {
        let mut p = Self::new(key, display, default, category);
        p.kind = kind;
        p
    }

    pub fn with_default(mut self, default: Value) -> Self {
        self.default = default;
        self
    }

    pub fn with_display(mut self, display: &'static str) -> Self {
        self.display = display;
        self
    }

    pub fn with_category(mut self, category: Category) -> Self {
        self.category = category;
        self
    }

    pub fn read_only(mut self) -> Self {
        self.readonly = true;
        self
    }

    pub fn bindable(mut self) -> Self {
        self.bindable = true;
        self
    }

    pub fn with_options(mut self, opts: &'static [&'static str]) -> Self {
        self.options = opts;
        self.kind = PropKind::Enum(opts);
        self
    }

    pub fn references(mut self, class: &'static str) -> Self {
        self.ref_class = Some(class);
        self.kind = PropKind::Reference(class);
        self
    }

    pub fn in_tree(mut self) -> Self {
        self.tree_collection = true;
        self.kind = PropKind::Collection {
            hierarchical: true,
            item_class: self.item_class,
        };
        self
    }

    pub fn with_item(mut self, item: &'static str) -> Self {
        self.item_class = Some(item);
        let hierarchical = match self.kind {
            PropKind::Collection { hierarchical, .. } => hierarchical,
            _ => self.tree_collection,
        };
        self.kind = PropKind::Collection {
            hierarchical,
            item_class: Some(item),
        };
        self
    }

    #[note("propdesc-hidden-when-ref")]
    pub fn hidden_when_ref(mut self, reference: &'static str, capability: &'static str) -> Self {
        self.hidden_when = Some((reference, capability));
        self
    }

    pub fn multiline(mut self) -> Self {
        self.multiline = true;
        self.kind = PropKind::Multiline;
        self
    }

    pub fn kind(&self) -> &'static str {
        self.default.kind()
    }

    // There was an `emit_makepad_dsl` and an `emit_egui_code` here. Only
    // tests called them, and they wrote fields neither framework has
    // (`checked:` — makepad's CheckBox is `active`; `border_width:` — no
    // makepad View has it; `.set_enabled()` / `.interval()` — no egui widget
    // has them): a silent-DSL trap waiting for a caller. The catalogue knows
    // no target's syntax; each target's component module is its only emitter.
}

#[note("textline")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextLine {
    /// The text is vertically centered (button, checkbox, edit).
    Centered,
    /// The text sits against the top, with the given margin in design units
    /// (label).
    Top(f32),
}

impl TextLine {
    #[note("textline-text-height")]
    pub const TEXT_HEIGHT: f32 = 14.0;
    #[note("textline-ascent")]
    pub const ASCENT: f32 = 11.0;
    #[note("textline-descent")]
    pub const DESCENT: f32 = Self::TEXT_HEIGHT - Self::ASCENT;

    #[note("textline-line")]
    pub fn line(self, y: f32, h: f32) -> f32 {
        match self {
            Self::Centered => y + h / 2.0 + (Self::ASCENT - Self::DESCENT) / 2.0,
            Self::Top(p) => y + p + Self::ASCENT,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventDesc {
    pub name: &'static str,
    pub display: &'static str,
    pub signature: &'static str,
}

impl EventDesc {
    pub const fn new(name: &'static str, display: &'static str, signature: &'static str) -> Self {
        Self {
            name,
            display,
            signature,
        }
    }

    #[note("eventdesc-method-params")]
    pub fn method_params(&self) -> String {
        let sig = self.signature.trim();
        let inner = match sig.strip_prefix("fn(").and_then(|s| s.strip_suffix(')')) {
            Some(s) => s.trim(),
            None if sig.contains(':') => sig,
            None => "&mut self",
        };

        let parts = split_top_level(inner);
        let mut out = Vec::new();
        if !parts
            .first()
            .is_some_and(|p| *p == "&mut self" || *p == "&self")
        {
            out.push("&mut self".to_string());
        }
        for (i, p) in parts.iter().enumerate() {
            if p.is_empty() {
                continue;
            }
            if i == 0 && (*p == "&mut self" || *p == "&self") {
                out.push(p.to_string());
            } else if let Some((name, typ)) = p.split_once(':') {
                let name = name.trim();
                let typ = typ.trim();
                let param_name = if name.starts_with('_') {
                    name.to_string()
                } else {
                    format!("_{name}")
                };
                out.push(format!("{param_name}: {typ}"));
            } else {
                out.push(p.to_string());
            }
        }
        out.join(", ")
    }
}

#[note("split-top-level")]
fn split_top_level(inner: &str) -> Vec<&str> {
    let bytes = inner.as_bytes();
    let mut parts = Vec::new();
    let mut depth = 0usize;
    let mut start = 0usize;
    for (i, c) in inner.char_indices() {
        match c {
            '<' | '(' | '[' => depth += 1,
            '>' if i > 0 && bytes[i - 1] == b'-' => {}
            '>' | ')' | ']' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                parts.push(inner[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    parts.push(inner[start..].trim());
    parts
}

impl From<(&'static str, &'static str, &'static str)> for EventDesc {
    fn from((name, display, signature): (&'static str, &'static str, &'static str)) -> Self {
        Self {
            name,
            display,
            signature,
        }
    }
}

#[note("events")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Events;

impl Events {
    #[note("events-mouse-click")]
    pub const MOUSE_CLICK: EventDesc = EventDesc::new("on_click", "OnClick", "fn(&mut self)");
    pub const MOUSE_DOUBLE_CLICK: EventDesc =
        EventDesc::new("on_dbl_click", "OnDblClick", "fn(&mut self)");
    pub const MOUSE_ENTER: EventDesc =
        EventDesc::new("on_mouse_enter", "OnMouseEnter", "fn(&mut self)");
    pub const MOUSE_LEAVE: EventDesc =
        EventDesc::new("on_mouse_leave", "OnMouseLeave", "fn(&mut self)");
    pub const MOUSE_WHEEL: EventDesc =
        EventDesc::new("on_mouse_wheel", "OnMouseWheel", "fn(&mut self)");
    pub const ACTIVATE: EventDesc = EventDesc::new("on_activate", "OnActivate", "fn(&mut self)");
    pub const DEACTIVATE: EventDesc =
        EventDesc::new("on_deactivate", "OnDeactivate", "fn(&mut self)");
    pub const MOUSE_ACTIVATE: EventDesc = EventDesc::new(
        "on_mouse_activate",
        "OnMouseActivate",
        "fn(&mut self, e: MouseActivateEvent)",
    );
    pub const MOUSE_DOWN: EventDesc = EventDesc::new(
        "on_mouse_down",
        "OnMouseDown",
        "fn(&mut self, e: MouseEvent)",
    );
    pub const MOUSE_UP: EventDesc =
        EventDesc::new("on_mouse_up", "OnMouseUp", "fn(&mut self, e: MouseEvent)");
    pub const MOUSE_MOVE: EventDesc = EventDesc::new(
        "on_mouse_move",
        "OnMouseMove",
        "fn(&mut self, e: MouseMoveEvent)",
    );
    pub const CHANGE: EventDesc = EventDesc::new("on_change", "OnChange", "fn(&mut self)");
    #[note("events-moved")]
    pub const MOVED: EventDesc = EventDesc::new("on_moved", "OnMoved", "fn(&mut self)");
    #[note("events-changing")]
    pub const CHANGING: EventDesc = EventDesc::new(
        "on_changing",
        "OnChanging",
        "fn(&mut self, allow_change: &mut bool)",
    );
    pub const SELECT: EventDesc = EventDesc::new("on_select", "OnSelect", "fn(&mut self)");
    #[note("events-select-item")]
    pub const SELECT_ITEM: EventDesc = EventDesc::new(
        "on_select_item",
        "OnSelectItem",
        "fn(&mut self, index: usize, selected: bool)",
    );
    #[note("events-select-cell")]
    pub const SELECT_CELL: EventDesc = EventDesc::new(
        "on_select_cell",
        "OnSelectCell",
        "fn(&mut self, col: usize, row: usize)",
    );
    #[note("events-column-click")]
    pub const COLUMN_CLICK: EventDesc = EventDesc::new(
        "on_column_click",
        "OnColumnClick",
        "fn(&mut self, column: usize)",
    );
    pub const ENTER: EventDesc = EventDesc::new("on_enter", "OnEnter", "fn(&mut self)");
    pub const EXIT: EventDesc = EventDesc::new("on_exit", "OnExit", "fn(&mut self)");
    pub const KEY_DOWN: EventDesc =
        EventDesc::new("on_key_down", "OnKeyDown", "fn(&mut self, e: KeyEvent)");
    pub const KEY_UP: EventDesc =
        EventDesc::new("on_key_up", "OnKeyUp", "fn(&mut self, e: KeyEvent)");
    pub const KEY_PRESS: EventDesc =
        EventDesc::new("on_key_press", "OnKeyPress", "fn(&mut self, ch: char)");
    #[note("events-return")]
    pub const RETURN: EventDesc = EventDesc::new("on_return", "OnReturn", "fn(&mut self)");
    pub const RESIZE: EventDesc = EventDesc::new("on_resize", "OnResize", "fn(&mut self)");
    #[note("events-paint")]
    pub const PAINT: EventDesc =
        EventDesc::new("on_paint", "OnPaint", "fn(&mut self, canvas: &mut Canvas)");
    pub const CREATE: EventDesc = EventDesc::new("on_create", "OnCreate", "fn(&mut self)");
    pub const DESTROY: EventDesc = EventDesc::new("on_destroy", "OnDestroy", "fn(&mut self)");
    pub const SHOW: EventDesc = EventDesc::new("on_show", "OnShow", "fn(&mut self)");
    pub const HIDE: EventDesc = EventDesc::new("on_hide", "OnHide", "fn(&mut self)");
    pub const CLOSE: EventDesc = EventDesc::new("on_close", "OnClose", "fn(&mut self)");
    pub const CLOSE_QUERY: EventDesc = EventDesc::new(
        "on_close_query",
        "OnCloseQuery",
        "fn(&mut self, can_close: &mut bool)",
    );
    pub const TIMER: EventDesc = EventDesc::new("on_timer", "OnTimer", "fn(&mut self)");
    pub const SCROLL: EventDesc = EventDesc::new("on_scroll", "OnScroll", "fn(&mut self)");
    pub const EXPANDING: EventDesc = EventDesc::new("on_expanding", "OnExpanding", "fn(&mut self)");
    pub const COLLAPSING: EventDesc =
        EventDesc::new("on_collapsing", "OnCollapsing", "fn(&mut self)");
    pub const EXPANDED: EventDesc = EventDesc::new("on_expanded", "OnExpanded", "fn(&mut self)");
    pub const COLLAPSED: EventDesc = EventDesc::new("on_collapsed", "OnCollapsed", "fn(&mut self)");
    pub const DRAG_OVER: EventDesc = EventDesc::new("on_drag_over", "OnDragOver", "fn(&mut self)");
    pub const DRAG_DROP: EventDesc = EventDesc::new("on_drag_drop", "OnDragDrop", "fn(&mut self)");
    pub const EXECUTE: EventDesc = EventDesc::new("on_execute", "OnExecute", "fn(&mut self)");
    pub const UPDATE: EventDesc = EventDesc::new("on_update", "OnUpdate", "fn(&mut self)");
    pub const POPUP: EventDesc = EventDesc::new("on_popup", "OnPopup", "fn(&mut self)");
    pub const GET_TEXT: EventDesc = EventDesc::new(
        "on_get_text",
        "OnGetText",
        "fn(&mut self, row: &crawrad_runtime::grid::RowValues, text: &mut String)",
    );
    pub const SET_TEXT: EventDesc = EventDesc::new(
        "on_set_text",
        "OnSetText",
        "fn(&mut self, row: &crawrad_runtime::grid::RowValues, text: &mut String)",
    );
    pub const VALIDATE: EventDesc = EventDesc::new(
        "on_validate",
        "OnValidate",
        "fn(&mut self, row: &crawrad_runtime::grid::RowValues, value: &str, error: &mut String)",
    );
    pub const BUTTON_CLICK: EventDesc = EventDesc::new(
        "on_button_click",
        "OnButtonClick",
        "fn(&mut self, row: &crawrad_runtime::grid::RowValues, text: &mut String)",
    );
    pub const DRAW_CELL: EventDesc = EventDesc::new(
        "on_draw_cell",
        "OnDrawCell",
        "fn(&mut self, row: &crawrad_runtime::grid::RowValues, style: &mut crawrad_runtime::grid::CellStyle)",
    );
}

#[note("children")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Children {
    /// Does not accept children: dragging something on top of it drops it on its parent.
    NoChildren,
    /// Is a container with its own surface: accepts children and they appear
    /// **inside its area**, clipped by it and with coordinates
    /// relative to it (`Panel`, `GroupBox`, `PageControl`).
    Area,
    /// Accepts children in the model and structure tree, but **not on
    /// the screen**: has no client area and children do not appear on the form
    /// (`ActionList` — the `Action`s are its children in `.crawform`, but it is
    /// a 28×28 icon).
    ///
    /// # It is the `TCollection`, and the item class enters HERE
    ///
    /// This is the piece that ADR 0037
    /// measured as missing, and it was not missing: it was **undeclared**.
    /// The `ActionList` already said `Collection` and `Action` was already a class with
    /// its own properties — a `TCollectionItem` in fact. What was missing was
    /// someone saying **of which class** the items are.
    ///
    /// Without this, nothing prevented a `Button` from becoming a child of an `ActionList`:
    /// `accepts_children` answered yes and `Document::add_child` only checked for duplicate
    /// id and existing parent.
    ///
    /// The class lives **inside the variant**, and not in a neighbouring field, because
    /// that way there is no way to declare the collection and forget the item — the compiler
    /// enforces it.
    Collection {
        /// The class that children must belong to — `"Action"` for
        /// `ActionList`, `"Column"` for a grid someone writes tomorrow.
        item: &'static str,
    },
}

#[note("craterequirement")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrateRequirement {
    /// The crate name, as crates.io knows it.
    pub crate_name: &'static str,
    /// The version requirement, as `Cargo.toml` writes it — `0.1`, `^1.2`.
    pub version: &'static str,
    /// The features that THIS component enables. The union of features from all
    /// form components is what goes into the generated `Cargo.toml`.
    pub features: &'static [&'static str],
    /// Do the crate's default features enter?
    pub default_features: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComponentDesc {
    pub class: &'static str,
    pub display: &'static str,
    /// The name with which a component of this class **is born**: `button`,
    /// `page_control`, `tab_sheet`. [`Registry::suggest_id`] appends the first
    /// free number after it and produces `button1`.
    ///
    /// # Why it is declared, and not derived from `display`
    ///
    /// Because they are two things, and one is not the other. The `display` is a LABEL —
    /// `"PageControl"` is what the user reads on the palette and in the tree. This is an
    /// IDENTIFIER — it becomes a field in `.designer.rs`, and therefore obeys
    /// Rust convention.
    ///
    /// Until 16/09/2026 the second was guessed from the first via runtime case
    /// conversion. The owner's decision ended it: *"here
    /// it is Rust, not a classic RAD IDE. I don't want to help the classic RAD dev come write as
    /// if they were in the classic RAD IDE and the IDE deal with it."* The conversion that
    /// fixed names underneath was completely removed; what remains of it is advice upon
    /// rejection, not silent fixing.
    ///
    /// Declared, the name is **visible in the catalog** — which is where someone will
    /// look for it — and a new class cannot forget it: the field is
    /// mandatory in `component!`, and
    /// `every_base_name_in_the_catalogue_is_snake_case` enforces the form.
    pub base_name: &'static str,
    pub palette: &'static str,
    /// Automatic name prefix: `btn_ok`, `edt_1`…
    pub prefix: &'static str,
    pub default_size: (f32, f32),
    pub props: Vec<PropDesc>,
    pub events: Vec<EventDesc>,
    /// Runtimes that implement this component (ADR 0002).
    pub targets: &'static [&'static str],
    /// **What this component offers to be**, besides its own class.
    ///
    /// It is the side of who is POINTED TO by a reference property. See
    /// [`ComponentDesc::answers_to`], which answers the question.
    ///
    /// Empty for almost every component: who offers nothing beyond itself
    /// continues to be accepted by whoever asks for its class by name.
    pub provides: &'static [&'static str],
    /// Accepts children, and **where** they appear. See [`Children`].
    pub children: Children,
    /// **Geometry is not the user's choice** — it is a consequence.
    ///
    /// `TabSheet` is the case: in the classic library it is `alClient` and does not move or
    /// resize; its rectangle IS the client area of `PageControl`.
    ///
    /// Without a flag, this becomes a cat-and-mouse game: free `Align` in the
    /// inspector put `layout::apply` and `normalize_pages` fighting over the
    /// same rectangle — measured, six rounds oscillating without converging, with a
    /// spurious transaction pushed to undo at each user gesture.
    pub derived_geometry: bool,
    /// **Appears on the user's screen** — in the generated application and preview.
    ///
    /// `Timer` and `ActionList` are `false`: in the designer they become a 28×28
    /// icon to click on, and in the application they have no window at all.
    ///
    /// It is what separates non-visuals from the rest: without this flag, the
    /// conformance test would demand a widget for `Timer` in egui and during compilation
    /// of the generated project, and a component that genuinely has no widget but was
    /// forgotten would slip through unnoticed.
    pub visual: bool,
    /// **Becomes running code** — has a representation in the generated application.
    ///
    /// `Timer` is `true`: has `interval` and `on_timer` emitted in the generator.
    /// `ActionList` is `false`: exists only for the designer and `.crawform`, and
    /// nothing from it becomes code today — who links the button to the action is the designer.
    ///
    /// It is the second tier of non-visuals: `visual` says whether it has a SCREEN,
    /// `emits_code` says whether it has CODE. Without this flag, a purely data-oriented
    /// non-visual would be demanded by the generator test, and a
    /// forgotten one would pass unnoticed.
    pub emits_code: bool,
    /// **Does this component type have state that the catalog DOES NOT know?**
    ///
    /// Only makes sense for what comes from outside: the type of a native component is
    /// EMITTED by us, field by field, so there is never a hidden field — and
    /// `component!` answers `false`.
    ///
    /// A third-party crate may have private fields that `manifest.json` does not
    /// declare, and its initial value belongs to it. When that is true the generator
    /// starts from the crate's `Default`; when not, it emits a full
    /// literal — because a full literal with `..Default::default()` is
    /// `needless_update`, and fails the user's project under `-D warnings`.
    pub private_state: bool,
    /// Where the text sits, when the component has text.
    pub text_line: Option<TextLine>,
    /// What the generated project needs in `Cargo.toml` because of it.
    ///
    /// **Empty for every native component**, and that is the truth, not an
    /// oversight: the generator already writes the application dependencies that it
    /// emits itself. See [`CrateRequirement`].
    pub requires: Vec<CrateRequirement>,
    /// **The Rust module where the TYPE of this class lives, in the generated application.**
    ///
    /// `"portadb_crawrad"` for `Connection`, and the generator writes
    /// `portadb_crawrad::Connection`. Comes from `[package] name` of
    /// `manifest.json`, with hyphens turned into underscores — which is what Cargo
    /// does with a crate's name when turning it into a module path.
    ///
    /// **Empty for every native component**, and for a reason: their type is
    /// EMITTED inside the user's project (`pub struct Button { … }` in the
    /// designer), not imported from anywhere. The generated project does not depend
    /// on our crate, and that is proof that this is generation and not a
    /// disguised interpreter.
    ///
    /// It was what was missing for a third-party class to become CODE: until
    /// 18/09/2026 it reached the generator and came out as a fake `Widget` with
    /// a `// TODO` beside it, because nobody could say what its type was
    /// called.
    pub crate_module: &'static str,
    /// **HOW THIS CLASS PAINTS ITSELF**, when it brings no code for that.
    ///
    /// Empty for every native component — their drawing is Rust, in
    /// `crawrad-runtime-makepad/src/componentes/`, and stays that way. This vocabulary
    /// exists for whoever CANNOT bring code: third-party components, which
    /// until 18/09/2026 appeared on the canvas as a dashed rectangle with the class
    /// name inside.
    ///
    /// See [`crawrad_ir::drawing::Verb`] — and note that it does not accept literal colors, only
    /// theme roles ([`crawrad_ir::drawing::Ink`]).
    pub desenho: crawrad_ir::drawing::Drawing,
    /// The palette icon a third-party component declares; EMPTY for every native
    /// component, whose icon is `crawrad_project::icons::ClassIcon` (the native set
    /// lives in the IDE's project crate, and moving it is not this change).
    /// Same 16×16 grid and theme ink as the natives.
    pub icon: Vec<crawrad_ir::icon::IconStep>,
    /// The base native component this component inherits from, if any.
    pub inherits: Option<&'static str>,
}

#[note("universal-target")]
pub const UNIVERSAL_TARGET: &str = "*";

#[note("universal-targets")]
pub const UNIVERSAL_TARGETS: &[&str] = &[UNIVERSAL_TARGET];

impl ComponentDesc {
    pub fn prop(&self, key: &str) -> Option<&PropDesc> {
        self.props.iter().find(|p| p.key == key)
    }

    #[note("componentdesc-answers-to")]
    pub fn answers_to(&self, capability: &str) -> bool {
        self.class == capability
            || self.provides.contains(&capability)
            || (capability == "Control" && self.visual)
            || (capability == props::WIN_CONTROL
                && components::base::WINDOWED.contains(&self.class))
    }

    #[note("componentdesc-accepts-child")]
    pub fn accepts_child(&self, class: &str) -> bool {
        match self.children {
            Children::NoChildren => false,
            Children::Area => true,
            Children::Collection { item } => item == class,
        }
    }

    #[note("componentdesc-item-class")]
    pub fn item_class(&self) -> Option<&'static str> {
        match self.children {
            Children::Collection { item } => Some(item),
            _ => None,
        }
    }

    /// The name of the child collection, in the Object Inspector and in the
    /// collection dialog's title — `Columns` for a `Grid`, `Panels` for a
    /// `StatusBar`, `Items` for a `MainMenu`, `StyleRules` for a `GridColumn`.
    ///
    /// Derived from the item class: the words the owner's class already says
    /// are dropped (`Grid`+`GridColumn` → `Column`, `MainMenu`+`MenuItem` →
    /// `Item`), the rest goes in the plural; when nothing is left the last word
    /// stays (`ActionList`+`Action` → `Actions`). A package that declares
    /// `Children::Collection { item: "ChartSeries" }` on a `Chart` gets `Series`,
    /// with nothing new to declare.
    pub fn collection_label(&self) -> Option<String> {
        fn words(class: &str) -> Vec<&str> {
            let mut out = Vec::new();
            let mut start = 0;
            for (i, c) in class.char_indices().skip(1) {
                if c.is_ascii_uppercase() {
                    out.push(&class[start..i]);
                    start = i;
                }
            }
            out.push(&class[start..]);
            out
        }
        let item = self.item_class()?;
        let owner = words(self.class);
        let item_words = words(item);
        let kept: String = item_words
            .iter()
            .filter(|w| !owner.contains(w))
            .copied()
            .collect();
        let word = if kept.is_empty() {
            item_words.last().copied().unwrap_or(item).to_string()
        } else {
            kept
        };
        Some(if word.ends_with('s') {
            word
        } else {
            format!("{word}s")
        })
    }

    #[note("componentdesc-accepts-children")]
    pub fn accepts_children(&self) -> bool {
        match self.children {
            Children::NoChildren => false,
            Children::Area | Children::Collection { .. } => true,
        }
    }

    #[note("componentdesc-children-area")]
    pub fn children_area(&self) -> bool {
        match self.children {
            Children::Area => true,
            Children::NoChildren | Children::Collection { .. } => false,
        }
    }

    #[note("componentdesc-props-for")]
    pub fn props_for<'a>(&'a self, target: &'a str) -> impl Iterator<Item = &'a PropDesc> {
        self.props.iter().filter(move |p| {
            crawrad_ir::PropKey::parse(p.key)
                .map(|k| k.applies_to(target))
                .unwrap_or(false)
        })
    }

    pub fn supports(&self, target: &str) -> bool {
        self.targets.is_empty()
            || self.targets.contains(&UNIVERSAL_TARGET)
            || self.targets.contains(&target)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Registry {
    by_class: BTreeMap<&'static str, ComponentDesc>,
}

impl Registry {
    #[note("registry-builtin")]
    pub fn builtin() -> Self {
        let mut r = Self::default();
        for d in components::all_builtin() {
            r.register(d);
        }
        r
    }

    pub fn register(&mut self, d: ComponentDesc) {
        self.by_class.insert(d.class, d);
    }

    pub fn get(&self, class: &str) -> Option<&ComponentDesc> {
        self.by_class.get(class)
    }

    #[note("registry-all")]
    pub fn all(&self) -> impl Iterator<Item = &ComponentDesc> {
        self.by_class.values()
    }

    #[note("registry-palette")]
    pub fn palette(&self, target: &str) -> Vec<(&'static str, Vec<&ComponentDesc>)> {
        let mut groups: BTreeMap<&'static str, Vec<&ComponentDesc>> = BTreeMap::new();
        for d in self.by_class.values() {
            if d.palette.is_empty() || !d.supports(target) {
                continue;
            }
            groups.entry(d.palette).or_default().push(d);
        }
        groups.into_iter().collect()
    }

    #[note("registry-instantiate")]
    pub fn instantiate(&self, class: &str, id: impl Into<String>, at: Rect) -> Option<Node> {
        let d = self.get(class)?;
        let id = id.into();
        let mut n = Node::new(id.clone(), d.class);
        for p in &d.props {
            if p.key == "rect" || p.key == "name" {
                continue;
            }
            n.props.insert(p.key.to_string(), p.default.clone());
        }
        // A masked box opens empty, as the classic one does: its name would be
        // shaped into the mask's slots (`mask_edit1` opened as `(1_) …`).
        if d.text_line.is_some() && d.prop("text").is_some() && d.prop("edit_mask").is_none() {
            n.set_text(&id);
        }
        n.set_rect(at);
        n.set_name(id);
        Some(n)
    }

    #[note("registry-suggest-id")]
    pub fn suggest_id(&self, class: &str, existing: impl Fn(&str) -> bool) -> String {
        let base = self.get(class).map(|d| d.base_name).unwrap_or("component");
        self.suggest_id_base(base, existing)
    }

    #[note("registry-suggest-id-base")]
    pub fn suggest_id_base(&self, base: &str, existing: impl Fn(&str) -> bool) -> String {
        (1..)
            .map(|i| format!("{base}{i}"))
            .find(|id| !existing(id))
            .unwrap_or_else(|| format!("{base}X"))
    }

    /// Returns whether the given property or event key was inherited from the base component.
    pub fn is_inherited(&self, desc: &ComponentDesc, key: &str) -> bool {
        let Some(base_name) = desc.inherits else {
            return false;
        };
        let Some(base) = self.get(base_name) else {
            return false;
        };
        base.prop(key).is_some() || base.events.iter().any(|e| e.name == key)
    }

    /// Returns the effective class name for styling/dispatch: the base class if inheriting, or the class itself.
    pub fn effective_class<'a>(&'a self, class: &'a str) -> &'a str {
        if let Some(desc) = self.get(class)
            && let Some(base) = desc.inherits
        {
            base
        } else {
            class
        }
    }
}

// ------------------------------------------------------------- descriptors

/// Declares a component.
#[macro_export]
macro_rules! component {
    (
        $class:literal,
        display: $display:literal,
        base_name: $base_name:literal,
        palette: $palette:literal,
        prefix: $prefix:literal,
        size: ($w:expr, $h:expr),
        $(targets: [$($target:literal),* $(,)?],)?
        $(visual: $visual:expr,)?
        $(emits_code: $emits:expr,)?
        $(children: $children:expr,)?
        $(derived_geometry: $derived:expr,)?
        $(text: $text:expr,)?
        $(base: $base:expr,)?
        props: [$($prop:expr),* $(,)?]
        $(, events: [$($ev:expr),* $(,)?])?
        $(,)?
    ) => {{
        let mut props: Vec<$crate::PropDesc> = $crate::component!(@base $($base)?);
        props.extend([$($prop),*]);
        let events: Vec<$crate::EventDesc> = $crate::component!(@events $([$($ev),*])?);
        $crate::ComponentDesc {
            class: $class,
            display: $display,
            base_name: $base_name,
            palette: $palette,
            prefix: $prefix,
            default_size: ($w, $h),
            props,
            events,
            targets: $crate::component!(@targets $([$($target),*])?),
            children: $crate::component!(@or_no_children $($children)?),
            derived_geometry: $crate::component!(@or_false $($derived)?),
            visual: $crate::component!(@or_true $($visual)?),
            emits_code: $crate::component!(@or_true $($emits)?),
            // Native offers no capabilities beyond its own class.
            provides: &[],
            // Native has no hidden fields: its type is emitted by us.
            private_state: false,
            text_line: $crate::component!(@or_none $($text)?),
            // NATIVE NEEDS NOTHING — see `ComponentDesc::requires`.
            requires: Vec::new(),
            // Native's type is EMITTED, not imported.
            crate_module: "",
            // Native's drawing is CODE, not data.
            desenho: Vec::new(),
            // Native's icon lives in ClassIcon, not here.
            icon: Vec::new(),
            inherits: None,
        }
    }};

    (@base) => { $crate::window_props() };
    (@base $b:expr) => { $b };
    (@targets) => { $crate::UNIVERSAL_TARGETS };
    (@targets [$($target:expr),*]) => { &[$($target),*] };
    (@or_no_children) => { $crate::Children::NoChildren };
    (@or_no_children $v:expr) => { $v };
    (@or_false) => { false };
    (@or_false $v:expr) => { $v };
    (@or_true) => { true };
    (@or_true $v:expr) => { $v };
    (@or_none) => { None };
    (@or_none $v:expr) => { Some($v) };
    (@events) => { Vec::new() };
    (@events [$($ev:expr),*]) => { vec![$($crate::EventDesc::from($ev)),*] };
}

#[cfg(test)]
mod tests;

#[cfg(test)]
#[path = "test/typed_collection/test.rs"]
mod typed_collection_test;

#[cfg(test)]
#[path = "test/enum_defaults/test.rs"]
mod enum_defaults_test;

#[cfg(test)]
#[path = "test/event_desc/test.rs"]
mod event_desc_test;

#[cfg(test)]
#[path = "test/base_name/test.rs"]
mod base_name_test;

#[cfg(test)]
#[path = "test/breadcrumb/test.rs"]
mod breadcrumb_test;
