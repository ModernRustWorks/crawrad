//! **What the editor knows about a form's components** (ADR 0050, step 1).
//!
//! Typing `self.btn_ok.` must offer what `btn_ok` really has in code. Each
//! piece of that answer comes from the one who owns it:
//!
//! - **the class** — the live form (IR), or the `btn_ok: Button` declaration
//!   in the `.designer.rs`. Never guessed from the name;
//! - **the fields** — the `pub struct Button { … }` the generator wrote into
//!   the `.designer.rs`: exactly what the compiler sees, for the project's
//!   target and for third-party components. A class just dropped on the form
//!   and not generated yet falls back to the same generator;
//! - **the documentation** — the property's label and category in the
//!   Object Inspector, from `crawrad-registry`.
//!
//! Pure: no window, no process, no server. The editor and, later, `crawrad-ls`
//! are both readers of this crate.
#![forbid(unsafe_code)]

pub mod anchor;
pub mod item;
pub mod list;
pub mod lsp_cell;
pub mod snippet;
pub mod templates;
pub mod typing;

use crawrad_ir::Document;
use crawrad_note::note;
use crawrad_registry::Registry;

#[note("field")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    /// The Rust type as written: `bool`, `String`, `Option<usize>`.
    pub ty: String,
    /// The `///` lines above the field, joined; `None` when there are none.
    pub doc: Option<String>,
}

#[note("member")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Member {
    pub label: String,
    /// The type, shown on the right of the list.
    pub detail: String,
    /// The Object Inspector's word for it, then the field's own doc.
    pub documentation: Option<String>,
}

#[note("fieldsource")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldSource {
    /// The project's `.designer.rs`.
    Designer,
    /// The Makepad generator, for a class not emitted yet.
    Generator,
}

#[note("struct-fields")]
pub fn struct_fields(source: &str, name: &str) -> Option<Vec<Field>> {
    let header = format!("pub struct {name} {{");
    let mut lines = source.lines().map(|l| l.trim_end_matches('\r'));
    lines.find(|l| l.trim_start() == header)?;
    let mut fields = Vec::new();
    let mut doc: Vec<&str> = Vec::new();
    for line in lines {
        let t = line.trim();
        if t == "}" {
            return Some(fields);
        }
        if let Some(d) = t.strip_prefix("///") {
            doc.push(d.trim());
            continue;
        }
        if let Some((field, ty)) = t
            .strip_prefix("pub ")
            .and_then(|rest| rest.strip_suffix(','))
            .and_then(|rest| rest.split_once(':'))
        {
            fields.push(Field {
                name: field.trim().to_string(),
                ty: ty.trim().to_string(),
                doc: (!doc.is_empty()).then(|| doc.join(" ")),
            });
        }
        doc.clear();
    }
    None
}

#[note("field-class")]
pub fn field_class(designer_source: &str, field: &str) -> Option<String> {
    let mut in_form = false;
    let mut block: Vec<&str> = Vec::new();
    for line in designer_source.lines().map(|l| l.trim_end_matches('\r')) {
        let t = line.trim();
        if t.starts_with("pub struct ") && t.ends_with('{') {
            in_form = true;
            block.clear();
            continue;
        }
        if !in_form {
            continue;
        }
        if t == "}" {
            in_form = false;
            if block.contains(&"pub base: FormBase,") {
                let wanted = format!("pub {field}:");
                if let Some(l) = block.iter().find(|l| l.starts_with(&wanted)) {
                    let ty = l[wanted.len()..].trim().trim_end_matches(',').trim();
                    return Some(ty.to_string());
                }
            }
            continue;
        }
        block.push(t);
    }
    None
}

#[note("class-in-form")]
pub fn class_in_form<'a>(form: &'a Document, field: &str) -> Option<&'a str> {
    form.nodes
        .values()
        .find(|n| n.name() == field)
        .map(|n| n.class.as_str())
}

#[note("class-fields")]
pub fn class_fields(
    designer_source: Option<&str>,
    class: &str,
) -> Option<(Vec<Field>, FieldSource)> {
    if let Some(fields) = designer_source.and_then(|s| struct_fields(s, class)) {
        return Some((fields, FieldSource::Designer));
    }
    let generated =
        crawrad_codegen::targets::makepad::components::for_class(class)?.struct_definition();
    struct_fields(&generated, class).map(|f| (f, FieldSource::Generator))
}

#[note("component-items")]
pub fn component_items(
    form: Option<&Document>,
    designer: Option<&str>,
    base: &str,
) -> Vec<item::ItemSugestao> {
    let class = form
        .and_then(|d| class_in_form(d, base).map(str::to_string))
        .or_else(|| field_class(designer?, base));
    let Some(class) = class else {
        return Vec::new();
    };
    let rows = members(&Registry::builtin(), designer, &class, "").into_iter();
    rows.map(|m| item::ItemSugestao::novo(m.label, item::ItemKind::Field).com_detalhe(m.detail))
        .collect()
}

#[note("members")]
pub fn members(
    registry: &Registry,
    designer_source: Option<&str>,
    class: &str,
    prefix: &str,
) -> Vec<Member> {
    let Some((fields, _)) = class_fields(designer_source, class) else {
        return Vec::new();
    };
    let prefix = prefix.to_lowercase();
    let desc = registry.get(class);
    fields
        .into_iter()
        .filter(|f| f.name.to_lowercase().starts_with(&prefix))
        .map(|f| {
            let inspector = desc
                .and_then(|d| d.prop(&f.name))
                .map(|p| format!("{} · {}", p.display, p.category.label()));
            let documentation = match (inspector, f.doc) {
                (Some(i), Some(d)) => Some(format!("{i}\n\n{d}")),
                (i, d) => i.or(d),
            };
            Member {
                label: f.name,
                detail: f.ty,
                documentation,
            }
        })
        .collect()
}

#[cfg(test)]
#[path = "test/lib/test.rs"]
mod tests;
