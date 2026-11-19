//! **Writing the manifest** — a [`ComponentSpec`] becomes the JSON object that
//! `manifest.json` holds.
//!
//! # Why JSON, and why the owner asked for it
//!
//! The format was TOML until 18/09/2026. Looking at the flat run of
//! `[[component.draw]]` tables, the owner asked:
//!
//! > *"Couldn't this be a JSON, with trees, where the parent node is the
//! > component and it has child nodes, grandchild nodes and so on — showing the
//! > component's branching as a hierarchy?"*
//!
//! TOML **can** hold that tree — measured, three levels deep, and the siblings
//! resolve correctly. What it cannot do is **show** it: every table header sits
//! flush left, and the nesting lives inside a dotted path
//! (`[[component.draw.child.child]]`) that has to be read character by
//! character. JSON shows the shape by indentation, which is the whole point.
//!
//! # The comments did not die, they became DATA
//!
//! JSON has no comments, and the manifest had just started carrying the doc
//! comment of every property. Instead of losing them, they became a `doc`
//! field — which is strictly better: a `#` line is dead weight that only a
//! person reading the file ever sees, while a `doc` field is something the
//! Object Inspector can show as a hint, the way a classic RAD IDE shows one.
//!
//! # Only what differs from the default is written
//!
//! `visual: true` and an empty `children` are what a component is unless it
//! says otherwise. Writing them anyway would teach every author to carry four
//! lines that say nothing, which is how the hand-written manifests got their
//! twelve lines of `name` and `rect` boilerplate in the first place.

use crate::spec::{
    ComponentSpec, DrawSpec, EventSpec, IconSpec, ItemSpec, PropSpec, RequirementSpec,
};

/// One level of indentation in the generated file.
const STEP: usize = 2;

impl ComponentSpec {
    /// The component's JSON object, indented to sit inside the file's array.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_manifest_gen::from_source;
    ///
    /// let src = "struct Plain;";
    /// let comps = from_source(src).unwrap();
    /// assert!(comps.is_empty());
    /// ```
    pub fn to_json(&self, indent: usize) -> String {
        let mut fields: Vec<(String, String)> = Vec::new();
        let pad = " ".repeat(indent + STEP);

        fields.push(("class".into(), json_string(&self.class)));
        fields.push(("display".into(), json_string(&self.display)));
        fields.push(("palette".into(), json_string(&self.palette)));
        fields.push(("prefix".into(), json_string(&self.prefix)));
        if let Some(inherits) = &self.inherits {
            fields.push(("inherits".into(), json_string(inherits)));
        }
        fields.push((
            "default_size".into(),
            format!("[{}, {}]", self.size.width, self.size.height),
        ));
        if !self.visual {
            fields.push(("visual".into(), "false".into()));
        }
        if let Some(children) = &self.children {
            fields.push(("children".into(), json_string(children)));
        }
        if let Some(item_class) = &self.item_class {
            fields.push(("item_class".into(), json_string(item_class)));
        }
        if self.derived_geometry {
            fields.push(("derived_geometry".into(), "true".into()));
        }
        // **Only the `false` is written**, because `true` is what a manifest
        // means when it says nothing -- a hand-written one cannot know the
        // struct's fields, and assuming it covered them all would make the
        // generated application miss a field's initial value.
        if self.covers_all_fields {
            fields.push(("private_state".into(), "false".into()));
        }
        if let Some(text_line) = &self.text_line {
            fields.push(("text_line".into(), text_line.clone()));
        }
        if !self.targets.is_empty() {
            fields.push(("targets".into(), string_array(&self.targets)));
        }
        if !self.provides.is_empty() {
            fields.push(("provides".into(), string_array(&self.provides)));
        }
        if let Some(doc) = joined_doc(&self.docs) {
            fields.push(("doc".into(), json_string(&doc)));
        }
        if !self.props.is_empty() {
            let items: Vec<String> = self
                .props
                .iter()
                .map(|p| p.to_json(indent + STEP * 2))
                .collect();
            fields.push(("props".into(), array_of_objects(&items, indent + STEP)));
        }
        if !self.events.is_empty() {
            let items: Vec<String> = self
                .events
                .iter()
                .map(|e| e.to_json(indent + STEP * 2))
                .collect();
            fields.push(("events".into(), array_of_objects(&items, indent + STEP)));
        }
        if !self.draws.is_empty() {
            let items: Vec<String> = self
                .draws
                .iter()
                .map(|d| d.to_json(indent + STEP * 2))
                .collect();
            fields.push(("draw".into(), array_of_objects(&items, indent + STEP)));
        }
        if !self.icon.is_empty() {
            let items: Vec<String> = self.icon.iter().map(IconSpec::to_json).collect();
            fields.push(("icon".into(), array_of_objects(&items, indent + STEP)));
        }
        if !self.requires.is_empty() {
            let items: Vec<String> = self
                .requires
                .iter()
                .map(|r| r.to_json(indent + STEP * 2))
                .collect();
            fields.push(("requires".into(), array_of_objects(&items, indent + STEP)));
        }

        object(&fields, indent, &pad)
    }
}

impl ItemSpec {
    /// The item's JSON object.
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
    /// assert!(item.to_json(0).contains("\"class\": \"Tab\""));
    /// ```
    pub fn to_json(&self, indent: usize) -> String {
        let pad = " ".repeat(indent + STEP);
        let mut fields: Vec<(String, String)> = vec![("class".into(), json_string(&self.class))];
        if let Some(doc) = joined_doc(&self.docs) {
            fields.push(("doc".into(), json_string(&doc)));
        }
        if !self.props.is_empty() {
            let items: Vec<String> = self
                .props
                .iter()
                .map(|p| p.to_json(indent + STEP * 2))
                .collect();
            fields.push(("props".into(), array_of_objects(&items, indent + STEP)));
        }
        object(&fields, indent, &pad)
    }
}

impl PropSpec {
    fn to_json(&self, indent: usize) -> String {
        let pad = " ".repeat(indent + STEP);
        let mut fields: Vec<(String, String)> = vec![
            ("key".into(), json_string(&self.key)),
            ("display".into(), json_string(&self.display)),
            ("category".into(), json_string(&self.category)),
            ("kind".into(), json_string(&self.kind)),
            ("default".into(), self.default_json.clone()),
        ];
        if self.readonly {
            fields.push(("readonly".into(), "true".into()));
        }
        if !self.options.is_empty() {
            fields.push(("options".into(), string_array(&self.options)));
        }
        if let Some(ref_class) = &self.ref_class {
            fields.push(("ref_class".into(), json_string(ref_class)));
        }
        if self.tree_collection {
            fields.push(("tree_collection".into(), "true".into()));
        }
        if let Some(item_class) = &self.item_class {
            fields.push(("item_class".into(), json_string(item_class)));
        }
        if let Some(min) = &self.min {
            fields.push(("min".into(), min.clone()));
        }
        if let Some(max) = &self.max {
            fields.push(("max".into(), max.clone()));
        }
        if let Some(step) = &self.step {
            fields.push(("step".into(), step.clone()));
        }
        if self.multiline {
            fields.push(("multiline".into(), "true".into()));
        }
        if let Some(doc) = joined_doc(&self.docs) {
            fields.push(("doc".into(), json_string(&doc)));
        }
        object(&fields, indent, &pad)
    }
}

impl EventSpec {
    fn to_json(&self, indent: usize) -> String {
        let pad = " ".repeat(indent + STEP);
        let mut fields = vec![
            ("name".to_string(), json_string(&self.name)),
            ("display".to_string(), json_string(&self.display)),
            ("signature".to_string(), json_string(&self.signature)),
        ];
        if let Some(doc) = joined_doc(&self.docs) {
            fields.push(("doc".to_string(), json_string(&doc)));
        }
        object(&fields, indent, &pad)
    }
}

impl RequirementSpec {
    fn to_json(&self, indent: usize) -> String {
        let pad = " ".repeat(indent + STEP);
        let mut fields = vec![
            ("crate".to_string(), json_string(&self.crate_name)),
            ("version".to_string(), json_string(&self.version)),
        ];
        if !self.features.is_empty() {
            fields.push(("features".to_string(), string_array(&self.features)));
        }
        if !self.default_features {
            fields.push(("default_features".to_string(), "false".to_string()));
        }
        object(&fields, indent, &pad)
    }
}

impl DrawSpec {
    fn to_json(&self, indent: usize) -> String {
        let pad = " ".repeat(indent + STEP);
        let mut fields: Vec<(String, String)> = self.keys.clone();
        if let Some(area) = &self.area {
            fields.push(("area".into(), area.clone()));
        }
        object(&fields, indent, &pad)
    }
}

impl IconSpec {
    fn to_json(&self) -> String {
        let mut parts = vec![format!("\"step\": {}", json_string(&self.step))];
        if let Some(x) = &self.x {
            parts.push(format!("\"x\": {x}"));
        }
        if let Some(y) = &self.y {
            parts.push(format!("\"y\": {y}"));
        }
        if let Some(w) = &self.w {
            parts.push(format!("\"w\": {w}"));
        }
        if let Some(h) = &self.h {
            parts.push(format!("\"h\": {h}"));
        }
        if let Some(r) = &self.r {
            parts.push(format!("\"r\": {r}"));
        }
        format!("{{ {} }}", parts.join(", "))
    }
}

// ---------------------------------------------------------------------------
// THE SHAPE OF THE TEXT
// ---------------------------------------------------------------------------

fn object(fields: &[(String, String)], indent: usize, pad: &str) -> String {
    if fields.is_empty() {
        return String::from("{}");
    }
    let body: Vec<String> = fields
        .iter()
        .map(|(k, v)| format!("{pad}{}: {v}", json_string(k)))
        .collect();
    format!("{{\n{}\n{}}}", body.join(",\n"), " ".repeat(indent))
}

fn array_of_objects(items: &[String], indent: usize) -> String {
    let pad = " ".repeat(indent + STEP);
    let body: Vec<String> = items.iter().map(|i| format!("{pad}{i}")).collect();
    format!("[\n{}\n{}]", body.join(",\n"), " ".repeat(indent))
}

fn string_array(items: &[String]) -> String {
    let inner: Vec<String> = items.iter().map(|i| json_string(i)).collect();
    format!("[{}]", inner.join(", "))
}

/// The doc comment of a struct, a field or an attribute, as one string.
///
/// `///` lines arrive one per line with a leading space; they become one
/// paragraph with real newlines, which JSON writes as `\n`.
fn joined_doc(docs: &[String]) -> Option<String> {
    let text = docs
        .iter()
        .map(|l| l.strip_prefix(' ').unwrap_or(l).trim_end())
        .collect::<Vec<_>>()
        .join("\n");
    let trimmed = text.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

/// Escapes a string according to JSON specification.
///
/// **A JSON string, escaped by the specification and not by eye.**
///
/// Rust's `{:?}` is *almost* this and that is the trap: it writes a control
/// character as `\u{1b}`, which JSON does not accept — JSON wants ``.
/// A manifest that a parser refuses is worse than one that looks wrong.
///
/// # Examples
///
/// ```rust,ignore
/// // private module render: not exposed in crate root
/// let escaped = json_string("hello\n");
/// assert_eq!(escaped, "\"hello\\n\"");
/// ```
pub fn json_string(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len() + 2);
    out.push('"');
    for c in raw.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
