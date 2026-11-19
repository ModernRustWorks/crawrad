//! Rebuilding one value, shape by shape.

use crawrad_dap::{ValueSource, Variable};

use crate::dwarf::TypeIndex;

use crate::layout;
use crate::types::{self, Primitive};
use crate::{MAX_ELEMENTS, MAX_STRING_BYTES};
use crawrad_note::note;

#[note("render-cx")]
pub struct Cx<'a> {
    pub src: &'a dyn ValueSource,
    pub index: Option<&'a TypeIndex>,
}

#[note("render-present")]
pub fn present(var: &Variable, cx: &Cx<'_>, depth: usize) -> Option<String> {
    let ty = var.type_name.as_deref()?;
    // A local the adapter cannot read (measured: the `args` a `println!`
    // leaves in scope) is its message, not a walk of failed children.
    if var.value.starts_with("<error") {
        return None;
    }
    if types::is_string(ty) {
        return string(var, cx);
    }
    if types::is_str(ty) {
        return str_slice(var, cx);
    }
    if let Some(elem) = types::vec_element(ty) {
        return vec(var, cx, &types::in_rust_spelling(elem, ty));
    }
    if let Some(elem) = types::slice_element(ty) {
        return slice(var, cx, &types::in_rust_spelling(elem, ty));
    }
    if types::is_hash_collection(ty) {
        return hash_collection(var, cx, ty);
    }
    if var.variables_reference == 0 {
        return scalar(ty, &var.value);
    }
    let children = cx.src.children(var.variables_reference).ok()?;
    if types::is_pointer(ty) {
        return pointee(var, ty, &children, cx, depth);
    }
    if types::is_pdb_enum(ty) {
        return pdb_enumeration(var, &children, cx, depth);
    }
    if let [only] = children.as_slice()
        && only.name == "$variants$"
    {
        return enumeration(ty, only, cx, depth);
    }
    if depth == 0 {
        return None;
    }
    structure(ty, &children, cx, depth)
}

#[note("render-field-text")]
fn field_text(var: &Variable, cx: &Cx<'_>, depth: usize) -> String {
    if let Some(text) = present(var, cx, depth) {
        return text;
    }
    let raw = var.value.trim();
    if var.type_name.as_deref().is_some_and(types::is_float) && is_plain_integer(raw) {
        return format!("{raw}.0");
    }
    raw.to_string()
}

fn is_plain_integer(s: &str) -> bool {
    let digits = s.strip_prefix('-').unwrap_or(s);
    !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit())
}

#[note("render-eval-first")]
fn eval_first(cx: &Cx<'_>, paths: &[&str], base: &str) -> Option<String> {
    paths
        .iter()
        .find_map(|p| cx.src.evaluate(&layout::at(p, base), None).ok())
        .map(|v| v.value)
        .filter(|v| !v.is_empty())
}

#[note("render-address")]
fn address(text: &str) -> Option<u64> {
    let first = text.split_whitespace().next()?;
    u64::from_str_radix(first.strip_prefix("0x").unwrap_or(first), 16).ok()
}

#[note("render-items")]
fn items(n: u64) -> String {
    if n == 1 {
        "1 item".to_string()
    } else {
        format!("{n} items")
    }
}

fn number(text: &str) -> Option<u64> {
    text.split_whitespace().next()?.parse().ok()
}

#[note("render-utf8-at")]
fn utf8_at(cx: &Cx<'_>, addr: u64, len: u64) -> Option<String> {
    if len == 0 {
        return Some("\"\"".to_string());
    }
    let take = (len as usize).min(MAX_STRING_BYTES);
    let bytes = cx.src.read_memory(&format!("0x{addr:x}"), take).ok()?;
    if bytes.len() != take {
        return None;
    }
    let text = String::from_utf8_lossy(&bytes);
    Some(if take < len as usize {
        format!("{:?}…", text)
    } else {
        format!("{text:?}")
    })
}

fn string(var: &Variable, cx: &Cx<'_>) -> Option<String> {
    let base = var.evaluate_name.as_deref()?;
    let vec_path = layout::at(layout::STRING_VEC, base);
    let len = number(&eval_first(cx, layout::VEC_LEN, &vec_path)?)?;
    if len == 0 {
        return Some("\"\"".to_string());
    }
    let addr = address(&eval_first(cx, layout::VEC_PTR, &vec_path)?)?;
    utf8_at(cx, addr, len)
}

fn str_slice(var: &Variable, cx: &Cx<'_>) -> Option<String> {
    let children = cx.src.children(var.variables_reference).ok()?;
    let find = |n: &str| children.iter().find(|c| c.name == n);
    let len = number(&find("length")?.value)?;
    let addr = address(&find("data_ptr")?.value)?;
    utf8_at(cx, addr, len)
}

fn vec(var: &Variable, cx: &Cx<'_>, elem: &str) -> Option<String> {
    let base = var.evaluate_name.as_deref()?;
    let len = number(&eval_first(cx, layout::VEC_LEN, base)?)?;
    let addr = eval_first(cx, layout::VEC_PTR, base).and_then(|v| address(&v));
    elements(cx, elem, addr, len)
}

#[note("render-slice")]
fn slice(var: &Variable, cx: &Cx<'_>, elem: &str) -> Option<String> {
    let children = cx.src.children(var.variables_reference).ok()?;
    let find = |n: &str| children.iter().find(|c| c.name == n);
    let len = number(&find("length")?.value)?;
    let addr = find("data_ptr").and_then(|c| address(&c.value));
    elements(cx, elem, addr, len)
}

#[note("render-elements")]
fn elements(cx: &Cx<'_>, elem: &str, addr: Option<u64>, len: u64) -> Option<String> {
    if len == 0 {
        return Some("[]".to_string());
    }
    // Composite elements need a typed view the adapter does not give without
    // Python: the program's DWARF gives it (ADR 0048 §3.3). Without DWARF (a
    // `-msvc` build keeps its types in a PDB) the count is what is honest.
    let Some(kind) = Primitive::of(elem) else {
        let from_dwarf = cx.index.zip(addr).and_then(|(index, addr)| {
            crate::memory::vec_at(index, elem, addr, len, cx.src, crate::MAX_DEPTH - 1)
        });
        return from_dwarf.or_else(|| Some(format!("[… {}]", items(len))));
    };
    let addr = addr?;
    let shown = (len as usize).min(MAX_ELEMENTS);
    let bytes = cx
        .src
        .read_memory(&format!("0x{addr:x}"), shown * kind.size())
        .ok()?;
    if bytes.len() != shown * kind.size() {
        return None;
    }
    let mut items: Vec<String> = bytes.chunks(kind.size()).map(|b| kind.decode(b)).collect();
    if shown < len as usize {
        items.push("…".to_string());
    }
    Some(format!("[{}]", items.join(", ")))
}

#[note("render-pointee")]
fn pointee(
    var: &Variable,
    ty: &str,
    children: &[Variable],
    cx: &Cx<'_>,
    depth: usize,
) -> Option<String> {
    // A pointer to a `-msvc` enum (`event: &Event` in a handler, measured on
    // 30/09/2026): the adapter's children are every variant at once. The PDB's
    // layout, read at the address the pointer holds, gives the one that is set.
    let target = types::pointee_name(ty);
    if types::is_pdb_enum(target)
        && let (Some(index), Some(addr)) = (cx.index, address(&var.value))
        && let Some((id, _)) = index.by_name(target)
    {
        return crate::memory::at(index, id, addr, cx.src, depth);
    }
    if let Some(target) = children.iter().find(|c| c.name.starts_with('*')) {
        return Some(field_text(target, cx, depth));
    }
    if depth == 0 {
        return None;
    }
    structure(types::pointee_name(ty), children, cx, depth)
}

#[note("render-enumeration")]
fn enumeration(ty: &str, variants: &Variable, cx: &Cx<'_>, depth: usize) -> Option<String> {
    let all = cx.src.children(variants.variables_reference).ok()?;
    if let Some(parts) = dwarf_arm(ty, &all, cx) {
        return variant_value(&parts, cx, depth);
    }
    let mut untagged = None;
    let mut active = None;
    for v in &all {
        let Some(suffix) = v.name.strip_prefix("$variant$") else {
            continue;
        };
        if suffix.is_empty() {
            untagged = Some(v);
            continue;
        }
        let parts = cx.src.children(v.variables_reference).ok()?;
        let discr = parts.iter().find(|p| p.name == "$discr$")?;
        if number(&discr.value).map(u128::from) == suffix.parse::<u128>().ok() {
            active = Some(parts);
            break;
        }
    }
    let parts = match (active, untagged) {
        (Some(parts), _) => parts,
        (None, Some(v)) => cx.src.children(v.variables_reference).ok()?,
        (None, None) => return None,
    };
    variant_value(&parts, cx, depth)
}

#[note("render-dwarf-arm")]
fn dwarf_arm(ty: &str, all: &[Variable], cx: &Cx<'_>) -> Option<Vec<Variable>> {
    let (_, enum_type) = cx.index?.by_name(ty)?;
    let crate::dwarf::Kind::Struct {
        variants: Some(layout),
        ..
    } = &enum_type.kind
    else {
        return None;
    };
    let mut stored = None;
    let mut by_arm = Vec::new();
    for v in all.iter().filter(|v| v.name.starts_with("$variant$")) {
        let parts = cx.src.children(v.variables_reference).ok()?;
        if let Some(d) = parts.iter().find(|p| p.name == "$discr$") {
            stored = stored.or_else(|| number(&d.value));
        }
        let arm = parts
            .iter()
            .find(|p| p.name == "value")
            .and_then(|p| p.type_name.as_deref())
            .map(|t| {
                types::short_name(t)
                    .split('<')
                    .next()
                    .unwrap_or_default()
                    .to_string()
            })?;
        by_arm.push((arm, parts));
    }
    let arm = layout
        .arms
        .iter()
        .find(|a| a.discr_value.is_some() && a.discr_value == stored)
        .or_else(|| layout.arms.iter().find(|a| a.discr_value.is_none()))?;
    by_arm
        .into_iter()
        .find(|(name, _)| *name == arm.member.name)
        .map(|(_, parts)| parts)
}

#[note("render-pdb-enumeration")]
fn pdb_enumeration(
    var: &Variable,
    children: &[Variable],
    cx: &Cx<'_>,
    depth: usize,
) -> Option<String> {
    let base = var.evaluate_name.as_deref()?;
    let variants: Vec<&Variable> = children
        .iter()
        .filter(|c| c.name.starts_with("variant"))
        .collect();
    let tag = children
        .iter()
        .find(|c| c.name == "tag")
        .and_then(|t| number(&t.value));
    let active = match (tag, variants.as_slice()) {
        (Some(tag), _) => *variants
            .iter()
            .find(|v| pdb_variant_owns(cx, base, &v.name, tag))?,
        // One variant needs no tag.
        (None, [only]) => *only,
        (None, _) => return None,
    };
    let parts = cx.src.children(active.variables_reference).ok()?;
    variant_value(&parts, cx, depth)
}

fn pdb_variant_owns(cx: &Cx<'_>, base: &str, variant: &str, tag: u64) -> bool {
    let constant = |name: &str| {
        cx.src
            .evaluate(&format!("{base}.{variant}.{name}"), None)
            .ok()
            .and_then(|v| number(&v.value))
    };
    if let Some(exact) = constant("DISCR_EXACT") {
        return exact == tag;
    }
    match (constant("DISCR_BEGIN"), constant("DISCR_END")) {
        (Some(begin), Some(end)) if begin <= end => (begin..=end).contains(&tag),
        // A range that wraps around the tag's maximum.
        (Some(begin), Some(end)) => tag >= begin || tag <= end,
        _ => false,
    }
}

#[note("render-variant-value")]
fn variant_value(parts: &[Variable], cx: &Cx<'_>, depth: usize) -> Option<String> {
    let value = parts.iter().find(|p| p.name == "value")?;
    let name = types::short_name(value.type_name.as_deref()?).to_string();
    let fields = if value.variables_reference == 0 {
        Vec::new()
    } else {
        cx.src.children(value.variables_reference).ok()?
    };
    Some(render_fields(&name, &fields, cx, depth.saturating_sub(1)))
}

#[note("render-hash-collection")]
fn hash_collection(var: &Variable, cx: &Cx<'_>, ty: &str) -> Option<String> {
    let base = var.evaluate_name.as_deref()?;
    let len = number(&eval_first(cx, layout::HASH_LEN, base)?)?;
    if len == 0 {
        return Some("{}".to_string());
    }
    let from_dwarf = || {
        let index = cx.index?;
        let (entry, set) = types::hash_entry(ty)?;
        let ctrl = address(&eval_first(cx, layout::HASH_CTRL, base)?)?;
        let mask = number(&eval_first(cx, layout::HASH_BUCKET_MASK, base)?)?;
        let table = crate::memory::HashTable {
            ctrl,
            buckets: mask.checked_add(1)?,
            items: len,
        };
        crate::memory::hash_at(index, &entry, set, &table, cx.src, crate::MAX_DEPTH - 1)
    };
    from_dwarf().or_else(|| Some(format!("{{… {}}}", items(len))))
}

fn structure(ty: &str, children: &[Variable], cx: &Cx<'_>, depth: usize) -> Option<String> {
    if children.iter().any(|c| c.name.starts_with('$')) {
        return None;
    }
    // A type of the standard library this presenter does not know is left to
    // the adapter: walking its fields prints implementation details, not the
    // value `{:?}` would show.
    if types::is_std_internal(ty) {
        return None;
    }
    Some(render_fields(
        types::debug_name(ty),
        children,
        cx,
        depth - 1,
    ))
}

#[note("render-scalar")]
fn scalar(ty: &str, value: &str) -> Option<String> {
    match ty {
        "char32_t" => {
            // `U+007A …` on Windows, `U+0x0000007a …` from the Debian LLDB 22.
            let code = value.split_whitespace().next()?.strip_prefix("U+")?;
            let code = code.strip_prefix("0x").unwrap_or(code);
            char::from_u32(u32::from_str_radix(code, 16).ok()?).map(|c| format!("{c:?}"))
        }
        "char" | "signed char" => {
            let inner = value.strip_prefix('\'')?.strip_suffix('\'')?;
            let byte = match inner.strip_prefix('\\') {
                None if inner.len() == 1 => inner.as_bytes()[0],
                None => return None,
                Some(esc) => match esc {
                    "n" => b'\n',
                    "t" => b'\t',
                    "r" => b'\r',
                    "0" => 0,
                    "\\" => b'\\',
                    "'" => b'\'',
                    _ => u8::from_str_radix(esc.strip_prefix('x')?, 16).ok()?,
                },
            };
            Some((byte as i8).to_string())
        }
        _ => None,
    }
}

#[note("render-render-fields")]
fn render_fields(name: &str, fields: &[Variable], cx: &Cx<'_>, depth: usize) -> String {
    if fields.is_empty() {
        return name.to_string();
    }
    let tuple = fields.iter().all(|f| f.name.starts_with("__"));
    let shown = fields.iter().take(MAX_ELEMENTS);
    let parts: Vec<String> = if tuple {
        shown.map(|f| field_text(f, cx, depth)).collect()
    } else {
        shown
            .map(|f| format!("{}: {}", f.name, field_text(f, cx, depth)))
            .collect()
    };
    if tuple {
        format!("{name}({})", parts.join(", "))
    } else {
        format!("{name} {{ {} }}", parts.join(", "))
    }
}
