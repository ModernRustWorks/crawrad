//! **Reading the struct** — the attributes turn into a [`ComponentSpec`].
//!
//! Nothing is rendered here. This module only answers *what did the author
//! declare*, and it fails loudly when the answer is missing, because a manifest
//! built on a guess is worse than no manifest at all: it would be wrong in the
//! Object Inspector, silently.

use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    Attribute, Data, DeriveInput, Expr, ExprLit, Field, Fields, Lit, Meta, MetaList, Token, Type,
};

/// The size of a component when it is born, in design units.
///
/// # Examples
///
/// ```rust
/// use crawrad_manifest_gen::Size;
///
/// let s = Size { width: "100".into(), height: "40".into() };
/// assert_eq!(s.width, "100");
/// ```
pub struct Size {
    pub width: String,
    pub height: String,
}

/// One property, as the manifest will write it.
///
/// # Examples
///
/// ```rust
/// use crawrad_manifest_gen::PropSpec;
///
/// let prop = PropSpec {
///     key: "text".into(),
///     display: "Text".into(),
///     category: "Standard".into(),
///     kind: "string".into(),
///     default_json: "\"\"".into(),
///     default_expr: None,
///     field: None,
///     min: None,
///     max: None,
///     step: None,
///     readonly: false,
///     multiline: false,
///     tree_collection: false,
///     options: Vec::new(),
///     ref_class: None,
///     item_class: None,
///     docs: Vec::new(),
/// };
/// assert_eq!(prop.key, "text");
/// ```
pub struct PropSpec {
    pub key: String,
    pub display: String,
    pub category: String,
    pub kind: String,
    /// The default, already rendered as a JSON value.
    pub default_json: String,
    /// The same default, as the Rust expression the author wrote.
    ///
    /// It is what lets the emitted test prove that `Default::default()` and the
    /// manifest agree. Injected properties (`name`, `rect`) have no field, so
    /// they have nothing to compare and carry [`None`].
    pub default_expr: Option<Expr>,
    /// The struct field this property reads, when there is one.
    pub field: Option<syn::Ident>,
    pub min: Option<String>,
    pub max: Option<String>,
    pub step: Option<String>,
    pub readonly: bool,
    pub multiline: bool,
    pub tree_collection: bool,
    pub options: Vec<String>,
    pub ref_class: Option<String>,
    pub item_class: Option<String>,
    /// The doc comment of the field, carried into the manifest as `#` lines.
    pub docs: Vec<String>,
}

/// One event a component publishes.
///
/// # Examples
///
/// ```rust
/// use crawrad_manifest_gen::EventSpec;
///
/// let ev = EventSpec {
///     name: "on_click".into(),
///     display: "OnClick".into(),
///     signature: "fn()".into(),
///     docs: Vec::new(),
/// };
/// assert_eq!(ev.name, "on_click");
/// ```
pub struct EventSpec {
    pub name: String,
    pub display: String,
    pub signature: String,
    pub docs: Vec<String>,
}

/// A dependency the generated project gains because of this component.
///
/// # Examples
///
/// ```rust
/// use crawrad_manifest_gen::RequirementSpec;
///
/// let req = RequirementSpec {
///     crate_name: "serde".into(),
///     version: "1.0".into(),
///     features: vec!["derive".into()],
///     default_features: true,
/// };
/// assert_eq!(req.crate_name, "serde");
/// ```
pub struct RequirementSpec {
    pub crate_name: String,
    pub version: String,
    pub features: Vec<String>,
    /// Do the crate's default features come in? Almost always yes.
    pub default_features: bool,
}

/// One drawing verb, with its payload already rendered as JSON values.
///
/// It carries no doc of its own, and cannot: a step is an item inside one
/// `#[draw(…)]` attribute, and Rust has nowhere to hang a `///` on an item of
/// an attribute list. The reason a drawing looks the way it does belongs to the
/// struct, which does have one.
///
/// # Examples
///
/// ```rust
/// use crawrad_manifest_gen::DrawSpec;
///
/// let draw = DrawSpec {
///     keys: vec![("verb".into(), "\"box\"".into())],
///     area: None,
/// };
/// assert_eq!(draw.keys.len(), 1);
/// ```
pub struct DrawSpec {
    /// `"verb": "box"` and friends, in the order the author wrote them.
    pub keys: Vec<(String, String)>,
    /// The inline `"area": { … }` object, when the verb has one.
    pub area: Option<String>,
}

/// One palette icon step declared by a component.
///
/// # Examples
///
/// ```rust
/// use crawrad_manifest_gen::IconSpec;
///
/// let icon = IconSpec {
///     step: "rect".into(),
///     x: Some("0".into()),
///     y: Some("0".into()),
///     w: Some("16".into()),
///     h: Some("16".into()),
///     r: None,
/// };
/// assert_eq!(icon.step, "rect");
/// ```
pub struct IconSpec {
    pub step: String,
    pub x: Option<String>,
    pub y: Option<String>,
    pub w: Option<String>,
    pub h: Option<String>,
    pub r: Option<String>,
}

/// Everything a `[[component]]` entry needs.
///
/// # Examples
///
/// ```rust
/// use crawrad_manifest_gen::from_source;
///
/// let comps = from_source("struct Plain;").unwrap();
/// assert!(comps.is_empty());
/// ```
pub struct ComponentSpec {
    pub class: String,
    pub display: String,
    pub palette: String,
    pub prefix: String,
    pub size: Size,
    pub targets: Vec<String>,
    pub visual: bool,
    pub children: Option<String>,
    pub item_class: Option<String>,
    pub derived_geometry: bool,
    pub text_line: Option<String>,
    pub props: Vec<PropSpec>,
    pub events: Vec<EventSpec>,
    pub draws: Vec<DrawSpec>,
    pub icon: Vec<IconSpec>,
    pub docs: Vec<String>,
    /// **What this component OFFERS itself as**, besides its own class.
    ///
    /// It is the side of the one being pointed at. A `SqliteProvider` that
    /// declares `provides = ["DbProvider"]` becomes acceptable to every
    /// reference property that asks for a `DbProvider`, without the component
    /// that declares that property knowing it exists.
    ///
    /// It is an interface, not inheritance: a component can offer several
    /// capabilities, and none of them forces it to inherit anything.
    pub provides: Vec<String>,
    /// **What the GENERATED project gains in its `Cargo.toml`** because of
    /// this component.
    ///
    /// It is the classic RAD *driver link*, and in Rust it is a Cargo feature: without
    /// `portadb/rusqlite`, the driver module is not even compiled. That is
    /// why one component per provider is not redundant with the abstraction's: the
    /// abstraction's says *"I want to connect"*, and the provider's says *"and it
    /// is THIS driver that has to exist in the binary"*.
    pub requires: Vec<RequirementSpec>,
    /// The base native component this component inherits from, if any.
    pub inherits: Option<String>,
    /// Every named field of the struct became a property.
    ///
    /// When it is true the generated application can build this component with
    /// a complete struct literal. When it is false there is state the manifest
    /// does not describe, and the generated code has to start from `Default` so
    /// that state gets its value.
    pub covers_all_fields: bool,
}

type Result<T> = std::result::Result<T, syn::Error>;

impl ComponentSpec {
    /// Read a whole `#[derive(Component)]` input.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_manifest_gen::ComponentSpec;
    /// use syn::parse_quote;
    ///
    /// let input: syn::DeriveInput = parse_quote! {
    ///     #[crawrad(palette = "Standard", prefix = "btn", size = (80.0, 24.0))]
    ///     struct Button;
    /// };
    /// let spec = ComponentSpec::read(&input).unwrap();
    /// assert_eq!(spec.class, "Button");
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a [`syn::Error`] if required attributes are missing or malformed.
    pub fn read(input: &DeriveInput) -> Result<Self> {
        let fields = named_fields(input)?;
        let field_count = fields.len();
        let mut spec = Self::read_header(input)?;

        // **`name` and `rect` come for free, and that is half the point.**
        //
        // Every manifest in the tree declares both by hand -- twelve lines that
        // say the same thing in every package, that nobody can leave out, and
        // that no struct field corresponds to. They are what the IDE needs to
        // place and address an instance, not what the component is about.
        // Inheriting components inherit them from the base instead.
        if spec.inherits.is_none() {
            spec.props.push(injected_name());
            spec.props.push(injected_rect(&spec.size));
        }

        let mut published = 0usize;
        for field in fields {
            if let Some(prop) = PropSpec::read(field)? {
                spec.props.push(prop);
                published += 1;
            }
        }
        spec.covers_all_fields = published == field_count;
        Ok(spec)
    }

    fn read_header(input: &DeriveInput) -> Result<Self> {
        let class = input.ident.to_string();
        let mut display = class.clone();
        let mut palette = None;
        let mut prefix = None;
        let mut size = None;
        let mut targets = vec![String::from("*")];
        let mut visual = true;
        let mut children = None;
        let mut item_class = None;
        let mut derived_geometry = false;
        let mut text_line = None;
        let mut provides = Vec::new();
        let mut inherits = None;

        for meta in args_of(&input.attrs, "crawrad")? {
            let key = meta_name(&meta);
            match key.as_str() {
                "display" => display = string_value(&meta)?,
                "palette" => palette = Some(string_value(&meta)?),
                "prefix" => prefix = Some(string_value(&meta)?),
                "size" => size = Some(size_value(&meta)?),
                "targets" => targets = string_list(&meta)?,
                "visual" => visual = bool_value(&meta)?,
                "children" => children = Some(string_value(&meta)?),
                "item_class" => item_class = Some(string_value(&meta)?),
                "derived_geometry" => derived_geometry = true,
                "text_line" => text_line = Some(text_line_value(&meta)?),
                "provides" => provides = string_list(&meta)?,
                "inherits" => inherits = Some(string_value(&meta)?),
                _ => return Err(unknown(&meta, "crawrad", key.as_str())),
            }
        }

        let span = input.ident.span();
        Ok(Self {
            class,
            display,
            palette: required(palette, span, "palette")?,
            prefix: required(prefix, span, "prefix")?,
            size: required(size, span, "size")?,
            targets,
            visual,
            children,
            item_class,
            derived_geometry,
            text_line,
            props: Vec::new(),
            events: read_events(&input.attrs)?,
            draws: read_draws(&input.attrs)?,
            icon: read_icon(&input.attrs)?,
            docs: docs_of(&input.attrs),
            provides,
            requires: read_requires(&input.attrs)?,
            inherits,
            // Filled in by `read`, once the fields have been counted.
            covers_all_fields: false,
        })
    }
}

/// Everything an item class needs to be published.
///
/// # Examples
///
/// ```rust
/// use crawrad_manifest_gen::ItemSpec;
///
/// let item = ItemSpec {
///     class: "Tab".into(),
///     props: Vec::new(),
///     docs: Vec::new(),
/// };
/// assert_eq!(item.class, "Tab");
/// ```
pub struct ItemSpec {
    pub class: String,
    pub props: Vec<PropSpec>,
    pub docs: Vec<String>,
}

impl ItemSpec {
    pub fn read(input: &DeriveInput) -> syn::Result<Self> {
        let class = input.ident.to_string();
        let fields = named_fields(input)?;
        let mut props = Vec::new();
        for field in fields {
            if let Some(prop) = PropSpec::read(field)? {
                props.push(prop);
            }
        }
        Ok(Self {
            class,
            props,
            docs: docs_of(&input.attrs),
        })
    }
}

impl PropSpec {
    /// Read one field, or [`None`] when it is marked `#[prop(skip)]`.
    ///
    /// A field with **no** `#[prop]` at all is also skipped: a component is
    /// allowed to have private state that the Object Inspector has no business
    /// showing, and silently publishing it would be the worse default.
    fn read(field: &Field) -> Result<Option<Self>> {
        let Some(ident) = field.ident.clone() else {
            return Ok(None);
        };
        let Some(args) = optional_args(&field.attrs, "prop")? else {
            return Ok(None);
        };

        let key = ident.to_string();
        let mut display = None;
        let mut category = None;
        let mut kind = None;
        let mut default = None;
        let mut min = None;
        let mut max = None;
        let mut step = None;
        let mut readonly = false;
        let mut multiline = false;
        let mut tree_collection = false;
        let mut options = Vec::new();
        let mut ref_class = None;
        let mut item_class = None;

        for meta in args {
            let name = meta_name(&meta);
            match name.as_str() {
                "skip" => return Ok(None),
                "display" => display = Some(string_value(&meta)?),
                "category" => category = Some(string_value(&meta)?),
                "kind" => kind = Some(string_value(&meta)?),
                "default" => default = Some(expr_value(&meta)?),
                "min" => min = Some(expr_value(&meta)?),
                "max" => max = Some(expr_value(&meta)?),
                "step" => step = Some(expr_value(&meta)?),
                "readonly" => readonly = true,
                "multiline" => multiline = true,
                "tree_collection" => tree_collection = true,
                "options" => options = string_list(&meta)?,
                "ref_class" => ref_class = Some(string_value(&meta)?),
                "item_class" => item_class = Some(string_value(&meta)?),
                _ => return Err(unknown(&meta, "prop", name.as_str())),
            }
        }

        let span = ident.span();
        let kind = match kind {
            Some(k) => k,
            None => infer_kind(&field.ty, multiline).ok_or_else(|| {
                syn::Error::new(
                    field.ty.span(),
                    "crawrad-component: this type has no obvious manifest kind -- \
                     write `kind = \"…\"` (text, multiline, bool, int, float, color, \
                     edges, set, enum, collection, reference, rect)",
                )
            })?,
        };
        let item_class = item_class.or_else(|| infer_item_class(&field.ty));

        let (default_json, default_expr) = match default {
            Some(d) => (json_scalar(&d)?, Some(d)),
            None if kind == "collection" => {
                let default_call: Expr = if is_type(&field.ty, "Strings") {
                    syn::parse_quote!(::crawrad_component::Strings::new())
                } else {
                    syn::parse_quote!(::crawrad_component::Collection::new())
                };
                (String::from("[]"), Some(default_call))
            }
            None if kind == "date_time" => {
                let default_call: Expr = syn::parse_quote!(::crawrad_component::DateTime::epoch());
                (String::from("\"1970-01-01T00:00:00\""), Some(default_call))
            }
            None if kind == "file_path" => {
                let default_call: Expr = syn::parse_quote!(::crawrad_component::FilePath::empty());
                (String::from("\"\""), Some(default_call))
            }
            None if kind == "cursor" => {
                let default_call: Expr = syn::parse_quote!(::crawrad_component::Cursor::Default);
                (String::from("\"default\""), Some(default_call))
            }
            None if kind == "modal_result" => {
                let default_call: Expr = syn::parse_quote!(::crawrad_component::ModalResult::None);
                (String::from("\"none\""), Some(default_call))
            }
            None if kind == "shortcut" => {
                let default_call: Expr = syn::parse_quote!(::crawrad_component::Shortcut::NONE);
                (String::from("\"\""), Some(default_call))
            }
            None if kind == "font" => {
                let default_call: Expr = syn::parse_quote!(::crawrad_component::Font::standard());
                (
                    String::from(
                        r#"{"name":"Segoe UI","size":9.0,"bold":false,"italic":false,"underline":false,"strike_out":false}"#,
                    ),
                    Some(default_call),
                )
            }
            // A colour with nothing said is opaque black, as `Color::default()`
            None if kind == "color" => {
                let default_call: Expr = syn::parse_quote!(::crawrad_component::Color::default());
                (
                    String::from(r#"{"r":0,"g":0,"b":0,"a":255}"#),
                    Some(default_call),
                )
            }
            // Margins with nothing said are zero on every side
            None if kind == "edges" => {
                let default_call: Expr =
                    syn::parse_quote!(::crawrad_component::Edges::uniform(0.0));
                (
                    String::from(r#"{"left":0.0,"top":0.0,"right":0.0,"bottom":0.0}"#),
                    Some(default_call),
                )
            }
            None if kind == "picture" => {
                let default_call: Expr = syn::parse_quote!(::crawrad_component::Picture::empty());
                (String::from("\"\""), Some(default_call))
            }
            None => return Err(required::<Expr>(None, span, "default").unwrap_err()),
        };

        Ok(Some(Self {
            key,
            display: display.unwrap_or_else(|| title_case(&ident.to_string())),
            category: required(category, span, "category")?,
            kind,
            default_json,
            default_expr,
            field: Some(ident),
            min: min.as_ref().map(json_scalar).transpose()?,
            max: max.as_ref().map(json_scalar).transpose()?,
            step: step.as_ref().map(json_scalar).transpose()?,
            readonly,
            multiline,
            tree_collection,
            options,
            ref_class,
            item_class,
            docs: docs_of(&field.attrs),
        }))
    }
}

// ---------------------------------------------------------------------------
// THE TWO PROPERTIES NOBODY SHOULD HAVE TO WRITE
// ---------------------------------------------------------------------------

fn injected_name() -> PropSpec {
    PropSpec {
        key: String::from("name"),
        display: String::from("Name"),
        category: String::from("identity"),
        kind: String::from("text"),
        default_json: String::from("\"\""),
        default_expr: None,
        field: None,
        min: None,
        max: None,
        step: None,
        readonly: false,
        multiline: false,
        tree_collection: false,
        options: Vec::new(),
        ref_class: None,
        item_class: None,
        docs: vec![String::from(
            " The instance name. Every component has one, so the derive writes it.",
        )],
    }
}

fn injected_rect(size: &Size) -> PropSpec {
    let Size { width, height } = size;
    PropSpec {
        key: String::from("rect"),
        display: String::from("Rect"),
        category: String::from("layout"),
        kind: String::from("rect"),
        default_json: format!("{{ \"x\": 0.0, \"y\": 0.0, \"w\": {width}, \"h\": {height} }}"),
        default_expr: None,
        field: None,
        min: None,
        max: None,
        step: None,
        readonly: false,
        multiline: false,
        tree_collection: false,
        options: Vec::new(),
        ref_class: None,
        item_class: None,
        docs: vec![String::from(
            " Where the instance sits. The size comes from `size` on the struct.",
        )],
    }
}

// ---------------------------------------------------------------------------
// EVENTS AND DRAWING VERBS -- struct-level, because neither is a field
// ---------------------------------------------------------------------------

fn read_events(attrs: &[Attribute]) -> Result<Vec<EventSpec>> {
    let mut out = Vec::new();
    for attr in attrs.iter().filter(|a| a.path().is_ident("event")) {
        let mut name = None;
        let mut display = None;
        let mut signature = String::from("fn(&mut self)");
        for meta in parse_args(attr)? {
            let key = meta_name(&meta);
            match key.as_str() {
                "name" => name = Some(string_value(&meta)?),
                "display" => display = Some(string_value(&meta)?),
                "signature" => signature = string_value(&meta)?,
                _ => return Err(unknown(&meta, "event", key.as_str())),
            }
        }
        let name = required(name, attr.span(), "name")?;
        out.push(EventSpec {
            display: display.unwrap_or_else(|| event_display(&name)),
            name,
            signature,
            docs: docs_of(std::slice::from_ref(attr)),
        });
    }
    Ok(out)
}

/// Reads dependency requirements from `#[requires]` attributes.
///
/// **The drawing is ONE attribute, and the verb NAMES each step.**
///
/// ```text
/// #[draw(
///     r#box(fill = "surface", border = "border", radius = 4.0),
///     r#box(fill = "accent", area(fraction_of = "value", axis = "x")),
///     frame(color = "accent", thickness = 2.0),
///     text(prop = "text", color = "text", centered),
/// )]
/// ```
///
/// # Why one attribute and not one per step
///
/// **The order is the meaning.** The frame is painted after the bar so the bar
/// does not cover it — that is a fact about the sequence, and a list says it
/// while a pile of attributes does not. It is also the shape this grows into:
/// a step that one day contains steps is a nested list, and nothing above has
/// to move.
///
/// `clippy::duplicated_attributes` says the same thing from the other side: it
/// reads two identical `verb = "box"` items as a copy-paste mistake. For a
/// painter's list it is not — the same verb twice is exactly how a bar gets
/// drawn over a box. Naming the step by its verb makes two `r#box(…)` steps
/// differ by their contents, which is what they really are.
///
/// # `r#box`, and why the raw identifier stays
///
/// `box` is a reserved word in Rust. The alternative was renaming the verb in
/// the attribute only — and then the manifest would say `box`, the attribute
/// would say something else, and the person reading both would have to learn
/// that they are the same thing. One `r#` is cheaper than two names for one
/// verb.
/// `#[requires(crate = "portadb", version = "0.1", features = ["rusqlite"])]`
///
/// One per dependency, and repeating the attribute here does not hit
/// `clippy::duplicated_attributes` the way the drawing did: two identical
/// requirements **are** copy-and-paste, and the lint is right to refuse them.
fn read_requires(attrs: &[Attribute]) -> Result<Vec<RequirementSpec>> {
    let mut out = Vec::new();
    for attr in attrs.iter().filter(|a| a.path().is_ident("requires")) {
        let mut crate_name = None;
        let mut version = None;
        let mut features = Vec::new();
        let mut default_features = true;
        for meta in parse_args(attr)? {
            let key = meta_name(&meta);
            match key.as_str() {
                // `crate` is a reserved word, and `Meta` reads it as a path
                // anyway, so here it fits without `r#`.
                "crate" => crate_name = Some(string_value(&meta)?),
                "version" => version = Some(string_value(&meta)?),
                "features" => features = string_list(&meta)?,
                "default_features" => default_features = bool_value(&meta)?,
                _ => return Err(unknown(&meta, "requires", key.as_str())),
            }
        }
        out.push(RequirementSpec {
            crate_name: required(crate_name, attr.span(), "crate")?,
            version: required(version, attr.span(), "version")?,
            features,
            default_features,
        });
    }
    Ok(out)
}

fn read_draws(attrs: &[Attribute]) -> Result<Vec<DrawSpec>> {
    let mut out = Vec::new();
    for attr in attrs.iter().filter(|a| a.path().is_ident("draw")) {
        for entry in parse_args(attr)? {
            let Meta::List(step) = &entry else {
                return Err(syn::Error::new(
                    entry.span(),
                    "crawrad-component: a drawing step is named by its verb, like \
                     `frame(color = \"accent\")`",
                ));
            };
            out.push(read_step(step)?);
        }
    }
    Ok(out)
}

fn read_step(step: &MetaList) -> Result<DrawSpec> {
    let verb = step
        .path
        .get_ident()
        .map(|i| i.to_string())
        // A raw identifier prints its own `r#`, and the verb is what comes
        // after it -- `r#box` is the verb `box`, spelled so Rust accepts it.
        .map(|name| name.trim_start_matches("r#").to_string())
        .ok_or_else(|| {
            syn::Error::new(
                step.span(),
                "crawrad-component: a drawing step starts with its verb",
            )
        })?;

    let inner = step.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
    let mut keys = vec![(String::from("verb"), crate::render::json_string(&verb))];
    let mut area = None;
    for meta in inner {
        let key = meta_name(&meta);
        match (&meta, key.as_str()) {
            (Meta::List(list), "area") => area = Some(inline_object(list)?),
            (Meta::Path(_), flag) => keys.push((flag.to_string(), String::from("true"))),
            (_, other) => keys.push((other.to_string(), json_scalar(&expr_value(&meta)?)?)),
        }
    }
    Ok(DrawSpec { keys, area })
}

/// Reads icon drawing steps declared in `#[icon]` attributes.
///
/// **The icon is ONE attribute, and the verb names each step with positional numbers.**
///
/// ```text
/// #[icon(
///     move_to(4.0, 3.5),
///     line_to(12.5, 8.0),
///     close_path,
///     circle(8.0, 8.0, 3.0),
///     r#box(1.0, 3.0, 14.0, 10.0, 1.25),
///     rect(1.0, 2.0, 14.0, 4.0),
///     fill(),
///     stroke(),
/// )]
/// ```
///
/// **Write the steps with no numbers with EMPTY PARENTHESES** — `fill()`,
/// `stroke()`, `close_path()`. A bare `stroke` is accepted too, but an icon
/// repeats its paints, and Clippy's `duplicated_attributes` refuses a repeated
/// bare word in an attribute: a component built with `-D warnings` could not
/// declare an icon with two strokes. Only bare words are compared; an empty
/// list is not.
fn read_icon(attrs: &[Attribute]) -> Result<Vec<IconSpec>> {
    let icon_attrs: Vec<&Attribute> = attrs.iter().filter(|a| a.path().is_ident("icon")).collect();
    if icon_attrs.is_empty() {
        return Ok(Vec::new());
    }
    if icon_attrs.len() > 1 {
        return Err(syn::Error::new(
            icon_attrs[1].span(),
            "crawrad-component: `icon` can only be declared once per component",
        ));
    }
    let attr = icon_attrs[0];
    let mut out = Vec::new();
    for entry in parse_args(attr)? {
        out.push(read_icon_step(&entry)?);
    }
    Ok(out)
}

fn read_icon_step(entry: &Meta) -> Result<IconSpec> {
    let (verb, args, span) = match entry {
        Meta::Path(path) => {
            let ident = path.get_ident().ok_or_else(|| {
                syn::Error::new(
                    path.span(),
                    "crawrad-component: an icon step starts with its verb",
                )
            })?;
            let verb = ident.to_string();
            let verb = verb.trim_start_matches("r#").to_string();
            (verb, Vec::new(), path.span())
        }
        Meta::List(list) => {
            let ident = list.path.get_ident().ok_or_else(|| {
                syn::Error::new(
                    list.span(),
                    "crawrad-component: an icon step starts with its verb",
                )
            })?;
            let verb = ident.to_string();
            let verb = verb.trim_start_matches("r#").to_string();
            let inner = list.parse_args_with(Punctuated::<Expr, Token![,]>::parse_terminated)?;
            (verb, inner.into_iter().collect::<Vec<_>>(), list.span())
        }
        Meta::NameValue(nv) => {
            return Err(syn::Error::new(
                nv.span(),
                "crawrad-component: an icon step cannot be a key-value pair",
            ));
        }
    };

    let check_arity = |expected: usize| -> Result<()> {
        if args.len() != expected {
            Err(syn::Error::new(
                span,
                format!(
                    "crawrad-component: icon step `{verb}` expects {expected} argument{}, found {}",
                    if expected == 1 { "" } else { "s" },
                    args.len()
                ),
            ))
        } else {
            Ok(())
        }
    };

    match verb.as_str() {
        "move_to" => {
            check_arity(2)?;
            Ok(IconSpec {
                step: verb,
                x: Some(float_scalar(&args[0])?),
                y: Some(float_scalar(&args[1])?),
                w: None,
                h: None,
                r: None,
            })
        }
        "line_to" => {
            check_arity(2)?;
            Ok(IconSpec {
                step: verb,
                x: Some(float_scalar(&args[0])?),
                y: Some(float_scalar(&args[1])?),
                w: None,
                h: None,
                r: None,
            })
        }
        "close_path" => {
            check_arity(0)?;
            Ok(IconSpec {
                step: verb,
                x: None,
                y: None,
                w: None,
                h: None,
                r: None,
            })
        }
        "circle" => {
            check_arity(3)?;
            Ok(IconSpec {
                step: verb,
                x: Some(float_scalar(&args[0])?),
                y: Some(float_scalar(&args[1])?),
                w: None,
                h: None,
                r: Some(float_scalar(&args[2])?),
            })
        }
        "box" => {
            check_arity(5)?;
            Ok(IconSpec {
                step: verb,
                x: Some(float_scalar(&args[0])?),
                y: Some(float_scalar(&args[1])?),
                w: Some(float_scalar(&args[2])?),
                h: Some(float_scalar(&args[3])?),
                r: Some(float_scalar(&args[4])?),
            })
        }
        "rect" => {
            check_arity(4)?;
            Ok(IconSpec {
                step: verb,
                x: Some(float_scalar(&args[0])?),
                y: Some(float_scalar(&args[1])?),
                w: Some(float_scalar(&args[2])?),
                h: Some(float_scalar(&args[3])?),
                r: None,
            })
        }
        "fill" => {
            check_arity(0)?;
            Ok(IconSpec {
                step: verb,
                x: None,
                y: None,
                w: None,
                h: None,
                r: None,
            })
        }
        "stroke" => {
            check_arity(0)?;
            Ok(IconSpec {
                step: verb,
                x: None,
                y: None,
                w: None,
                h: None,
                r: None,
            })
        }
        _ => Err(syn::Error::new(
            span,
            format!("crawrad-component: unknown icon step `{verb}`"),
        )),
    }
}

/// Where a one-line text sits: `text_line = "centered"`, or `text_line(top = 4.0)`.
///
/// Two shapes because the model has two: a placement with no payload, and one
/// that carries a margin. They share one key, the way the manifest already
/// shares one key between `min = 0.0` and `min = "min"`.
fn text_line_value(meta: &Meta) -> Result<String> {
    match meta {
        Meta::List(list) => inline_object(list),
        _ => Ok(crate::render::json_string(&string_value(meta)?)),
    }
}

/// Renders an inline JSON object from a metadata list.
///
/// `area(fraction_of = "value", axis = "x", min = "min", max = "max")` as an
/// inline JSON object.
///
/// It stays on one line on purpose: it is a leaf, and breaking it across lines
/// would make the file taller without making the tree any clearer.
fn inline_object(list: &MetaList) -> Result<String> {
    let inner = list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
    let mut parts = Vec::new();
    for meta in inner {
        let key = crate::render::json_string(&meta_name(&meta));
        parts.push(format!("{key}: {}", json_scalar(&expr_value(&meta)?)?));
    }
    Ok(format!("{{ {} }}", parts.join(", ")))
}

// ---------------------------------------------------------------------------
// THE SMALL READERS
// ---------------------------------------------------------------------------

fn named_fields(input: &DeriveInput) -> Result<Vec<&Field>> {
    match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(named) => Ok(named.named.iter().collect()),
            Fields::Unit => Ok(Vec::new()),
            Fields::Unnamed(_) => Err(syn::Error::new(
                input.ident.span(),
                "crawrad-component: a component needs named fields -- a tuple struct \
                 has no property names to publish",
            )),
        },
        _ => Err(syn::Error::new(
            input.ident.span(),
            "crawrad-component: only a struct can be a component",
        )),
    }
}

fn args_of(attrs: &[Attribute], name: &str) -> Result<Vec<Meta>> {
    Ok(optional_args(attrs, name)?.unwrap_or_default())
}

fn optional_args(attrs: &[Attribute], name: &str) -> Result<Option<Vec<Meta>>> {
    let mut found: Option<Vec<Meta>> = None;
    for attr in attrs.iter().filter(|a| a.path().is_ident(name)) {
        found.get_or_insert_with(Vec::new).extend(parse_args(attr)?);
    }
    Ok(found)
}

fn parse_args(attr: &Attribute) -> Result<Vec<Meta>> {
    let parsed = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
    Ok(parsed.into_iter().collect())
}

fn meta_name(meta: &Meta) -> String {
    meta.path()
        .get_ident()
        .map(ToString::to_string)
        .unwrap_or_default()
}

fn expr_value(meta: &Meta) -> Result<Expr> {
    match meta {
        Meta::NameValue(nv) => Ok(nv.value.clone()),
        other => Err(syn::Error::new(
            other.span(),
            "crawrad-component: this one needs a value -- write `key = value`",
        )),
    }
}

fn string_value(meta: &Meta) -> Result<String> {
    match expr_value(meta)? {
        Expr::Lit(ExprLit {
            lit: Lit::Str(s), ..
        }) => Ok(s.value()),
        other => Err(syn::Error::new(
            other.span(),
            "crawrad-component: this one needs a string in quotes",
        )),
    }
}

fn bool_value(meta: &Meta) -> Result<bool> {
    match expr_value(meta)? {
        Expr::Lit(ExprLit {
            lit: Lit::Bool(b), ..
        }) => Ok(b.value()),
        other => Err(syn::Error::new(
            other.span(),
            "crawrad-component: this one needs `true` or `false`",
        )),
    }
}

fn string_list(meta: &Meta) -> Result<Vec<String>> {
    match expr_value(meta)? {
        Expr::Array(array) => array
            .elems
            .iter()
            .map(|e| match e {
                Expr::Lit(ExprLit {
                    lit: Lit::Str(s), ..
                }) => Ok(s.value()),
                other => Err(syn::Error::new(
                    other.span(),
                    "crawrad-component: this list holds strings in quotes",
                )),
            })
            .collect(),
        other => Err(syn::Error::new(
            other.span(),
            "crawrad-component: this one needs a list, like `[\"a\", \"b\"]`",
        )),
    }
}

fn size_value(meta: &Meta) -> Result<Size> {
    let Expr::Tuple(tuple) = expr_value(meta)? else {
        return Err(syn::Error::new(
            meta.span(),
            "crawrad-component: `size` is a pair, like `size = (140.0, 48.0)`",
        ));
    };
    let mut it = tuple.elems.iter();
    let (Some(w), Some(h), None) = (it.next(), it.next(), it.next()) else {
        return Err(syn::Error::new(
            tuple.span(),
            "crawrad-component: `size` is a pair -- width and height, nothing else",
        ));
    };
    Ok(Size {
        width: float_scalar(w)?,
        height: float_scalar(h)?,
    })
}

// ---------------------------------------------------------------------------
// RUST LITERAL -> TOML SCALAR
// ---------------------------------------------------------------------------

/// Render a literal the author wrote as the JSON value the manifest holds.
///
/// Only literals get through, and on purpose: anything the macro would have to
/// *evaluate* it cannot evaluate, and a manifest that quietly held the text
/// `SOME_CONST` instead of its value would be a lie the reader cannot see.
///
/// # Examples
///
/// ```rust,ignore
/// // private module spec: not exposed in crate root
/// use syn::parse_quote;
///
/// let expr: syn::Expr = parse_quote!(42);
/// let s = json_scalar(&expr).unwrap();
/// assert_eq!(s, "42");
/// ```
///
/// # Errors
///
/// Returns a [`syn::Error`] if the expression is not a supported literal (string, bool, or number).
pub fn json_scalar(expr: &Expr) -> Result<String> {
    match expr {
        Expr::Lit(ExprLit { lit, .. }) => match lit {
            Lit::Str(s) => Ok(crate::render::json_string(&s.value())),
            Lit::Bool(b) => Ok(b.value().to_string()),
            Lit::Int(i) => Ok(i.base10_digits().to_string()),
            Lit::Float(f) => Ok(with_point(f.base10_digits())),
            other => Err(syn::Error::new(
                other.span(),
                "crawrad-component: a manifest value is a string, a bool or a number",
            )),
        },
        Expr::Unary(unary) if matches!(unary.op, syn::UnOp::Neg(_)) => {
            Ok(format!("-{}", json_scalar(&unary.expr)?))
        }
        Expr::Group(group) => json_scalar(&group.expr),
        other => Err(syn::Error::new(
            other.span(),
            "crawrad-component: this has to be a literal -- the macro cannot run code \
             to find out what it is worth",
        )),
    }
}

fn float_scalar(expr: &Expr) -> Result<String> {
    let rendered = json_scalar(expr)?;
    Ok(with_point(&rendered))
}

/// A size is a pair of floats, and `140` would read back as an integer.
fn with_point(digits: &str) -> String {
    if digits.contains(['.', 'e', 'E']) {
        digits.to_string()
    } else {
        format!("{digits}.0")
    }
}

/// The kind a Rust type obviously maps to, when the author did not say.
fn infer_kind(ty: &Type, multiline: bool) -> Option<String> {
    let Type::Path(path) = ty else {
        return None;
    };
    let last = path.path.segments.last()?.ident.to_string();
    let kind = match last.as_str() {
        "String" => {
            if multiline {
                "multiline"
            } else {
                "text"
            }
        }
        "bool" => "bool",
        "i8" | "i16" | "i32" | "i64" | "isize" | "u8" | "u16" | "u32" | "u64" | "usize" => "int",
        "f32" | "f64" => "float",
        "Strings" | "Collection" => "collection",
        "DateTime" => "date_time",
        "FilePath" => "file_path",
        "Cursor" => "cursor",
        "ModalResult" => "modal_result",
        "Shortcut" => "shortcut",
        "Font" => "font",
        "Color" => "color",
        "Edges" => "edges",
        "Picture" => "picture",
        _ => return None,
    };
    Some(kind.to_string())
}

/// Infers the item class from a Collection<T> type, if any.
fn infer_item_class(ty: &Type) -> Option<String> {
    let Type::Path(path) = ty else {
        return None;
    };
    let last_seg = path.path.segments.last()?;
    if last_seg.ident != "Collection" {
        return None;
    }
    let syn::PathArguments::AngleBracketed(args) = &last_seg.arguments else {
        return None;
    };
    let first_arg = args.args.first()?;
    let syn::GenericArgument::Type(Type::Path(item_path)) = first_arg else {
        return None;
    };
    Some(item_path.path.segments.last()?.ident.to_string())
}

fn is_type(ty: &Type, name: &str) -> bool {
    if let Type::Path(path) = ty
        && let Some(last) = path.path.segments.last()
    {
        return last.ident == name;
    }
    false
}

/// `value` becomes `Value`, `max_connections` becomes `Max connections`.
///
/// It is only the fallback: `display` on the attribute always wins, because the
/// Object Inspector shows classic RAD names (`Caption`, `OnClick`) that no
/// mechanical rule would produce from a Rust field name.
fn title_case(key: &str) -> String {
    let mut chars = key.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// `on_click` becomes `OnClick` -- the name the Object Inspector shows.
fn event_display(name: &str) -> String {
    name.split('_').map(title_case).collect()
}

fn docs_of(attrs: &[Attribute]) -> Vec<String> {
    attrs
        .iter()
        .filter(|a| a.path().is_ident("doc"))
        .filter_map(|a| match &a.meta {
            Meta::NameValue(nv) => match &nv.value {
                Expr::Lit(ExprLit {
                    lit: Lit::Str(s), ..
                }) => Some(s.value()),
                _ => None,
            },
            _ => None,
        })
        .collect()
}

fn required<T>(value: Option<T>, span: Span, what: &str) -> Result<T> {
    value.ok_or_else(|| {
        syn::Error::new(
            span,
            format!("crawrad-component: `{what}` is missing, and the manifest needs it"),
        )
    })
}

fn unknown(meta: &Meta, attr: &str, key: &str) -> syn::Error {
    syn::Error::new(
        meta.span(),
        format!("crawrad-component: `{attr}` does not know `{key}`"),
    )
}

#[cfg(test)]
#[path = "test/spec/test.rs"]
mod spec_test;
