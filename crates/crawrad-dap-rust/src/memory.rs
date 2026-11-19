//! Rebuilding a value from its bytes and its DWARF layout.
//!
//! This is the second stage of ADR 0048 §3.3: where the adapter's tree stops
//! (the elements of a `Vec<String>`, whose pointer the `std` erased to `u8*`),
//! the program's own DWARF says how big each element is and where its fields
//! are, and every value comes out of `readMemory`.

use crawrad_dap::ValueSource;

use crate::dwarf::{BaseEncoding, Kind, TypeId, TypeIndex};
use crate::types;
use crate::{MAX_ELEMENTS, MAX_STRING_BYTES};
use crawrad_note::note;

#[note("memory-at")]
pub fn at(
    index: &TypeIndex,
    ty: TypeId,
    addr: u64,
    src: &dyn ValueSource,
    depth: usize,
) -> Option<String> {
    let size = index.get(ty).size as usize;
    if size == 0 {
        return Some(types::short_name(&index.get(ty).name).to_string());
    }
    let bytes = read(src, addr, size)?;
    from_bytes(index, ty, &bytes, addr, src, depth)
}

fn read(src: &dyn ValueSource, addr: u64, len: usize) -> Option<Vec<u8>> {
    let bytes = src.read_memory(&format!("0x{addr:x}"), len).ok()?;
    (bytes.len() == len).then_some(bytes)
}

fn uint(bytes: &[u8]) -> u64 {
    let mut b = [0u8; 8];
    let n = bytes.len().min(8);
    b[..n].copy_from_slice(&bytes[..n]);
    u64::from_le_bytes(b)
}

#[note("memory-from-bytes")]
pub fn from_bytes(
    index: &TypeIndex,
    ty: TypeId,
    bytes: &[u8],
    addr: u64,
    src: &dyn ValueSource,
    depth: usize,
) -> Option<String> {
    let t = index.get(ty);
    match &t.kind {
        Kind::Base(enc) => Some(base(*enc, bytes)),
        Kind::Pointer(target) => {
            let p = uint(&bytes[..8.min(bytes.len())]);
            match target {
                Some(target) if depth > 0 && p != 0 => at(index, *target, p, src, depth - 1),
                _ => Some(format!("0x{p:x}")),
            }
        }
        Kind::Struct { members, variants } => {
            let name = t.name.as_str();
            if types::is_string(name) {
                let len = uint(field(index, ty, bytes, &["vec", "len"])?);
                let ptr = uint(field(
                    index,
                    ty,
                    bytes,
                    &["vec", "buf", "inner", "ptr", "pointer", "pointer"],
                )?);
                return utf8(src, ptr, len);
            }
            if types::is_str(name) {
                let len = uint(field(index, ty, bytes, &["length"])?);
                let ptr = uint(field(index, ty, bytes, &["data_ptr"])?);
                return utf8(src, ptr, len);
            }
            if let Some(elem) = types::vec_element(name) {
                let len = uint(field(index, ty, bytes, &["len"])?);
                let ptr = uint(field(
                    index,
                    ty,
                    bytes,
                    &["buf", "inner", "ptr", "pointer", "pointer"],
                )?);
                return vec_at(index, elem, ptr, len, src, depth).or_else(|| {
                    let n = if len == 1 {
                        "1 item".into()
                    } else {
                        format!("{len} items")
                    };
                    Some(format!("[… {n}]"))
                });
            }
            if let Some(v) = variants {
                let (discr, arms) = (&v.discr, &v.arms);
                let value = match discr {
                    Some(d) => {
                        let size = d.ty.map(|t| index.get(t).size as usize).unwrap_or(8);
                        Some(uint(
                            bytes.get(d.offset as usize..d.offset as usize + size)?,
                        ))
                    }
                    None => None,
                };
                let arm = arms
                    .iter()
                    .find(|a| a.discr_value.is_some() && a.discr_value == value)
                    .or_else(|| arms.iter().find(|a| a.discr_value.is_none()))?;
                let payload = arm.member.ty?;
                return Some(fields(
                    index,
                    payload,
                    &arm.member.name,
                    bytes,
                    addr,
                    src,
                    depth,
                ));
            }
            if types::is_std_internal(name) || depth == 0 {
                return None;
            }
            let _ = members;
            // A tuple `(u32, i32)` prints as `(1, 2)`: no name before the parentheses.
            Some(fields(
                index,
                ty,
                types::debug_name(name),
                bytes,
                addr,
                src,
                depth,
            ))
        }
        Kind::Other => None,
    }
}

#[note("memory-field")]
fn field<'b>(index: &TypeIndex, ty: TypeId, bytes: &'b [u8], path: &[&str]) -> Option<&'b [u8]> {
    let (offset, fty) = index.member_path(ty, path)?;
    let size = index.get(fty).size as usize;
    bytes.get(offset as usize..offset as usize + size)
}

#[note("memory-fields")]
fn fields(
    index: &TypeIndex,
    ty: TypeId,
    name: &str,
    bytes: &[u8],
    addr: u64,
    src: &dyn ValueSource,
    depth: usize,
) -> String {
    let Kind::Struct { members, .. } = &index.get(ty).kind else {
        return name.to_string();
    };
    let shown: Vec<_> = members
        .iter()
        .filter(|m| !m.name.starts_with("$"))
        .collect();
    if shown.is_empty() {
        return name.to_string();
    }
    let text = |m: &crate::dwarf::Member| -> String {
        let Some(mty) = m.ty else { return "?".into() };
        let size = index.get(mty).size as usize;
        let off = m.offset as usize;
        match bytes.get(off..off + size) {
            Some(b) if depth > 0 => from_bytes(index, mty, b, addr + m.offset, src, depth - 1)
                .unwrap_or_else(|| types::short_name(&index.get(mty).name).to_string()),
            _ => "…".to_string(),
        }
    };
    if shown.iter().all(|m| m.name.starts_with("__")) {
        let parts: Vec<String> = shown.iter().map(|m| text(m)).collect();
        format!("{name}({})", parts.join(", "))
    } else {
        let parts: Vec<String> = shown
            .iter()
            .map(|m| format!("{}: {}", m.name, text(m)))
            .collect();
        format!("{name} {{ {} }}", parts.join(", "))
    }
}

#[note("memory-vec-at")]
pub fn vec_at(
    index: &TypeIndex,
    elem: &str,
    ptr: u64,
    len: u64,
    src: &dyn ValueSource,
    depth: usize,
) -> Option<String> {
    if len == 0 {
        return Some("[]".into());
    }
    let (ety, et) = index
        .by_name(elem)
        .or_else(|| index.by_name(&types::rust_spelling(elem)))?;
    let size = et.size as usize;
    if size == 0 {
        return None;
    }
    let shown = (len as usize).min(MAX_ELEMENTS);
    let bytes = read(src, ptr, shown * size)?;
    let read: Vec<Option<String>> = bytes
        .chunks(size)
        .enumerate()
        .map(|(i, b)| {
            from_bytes(
                index,
                ety,
                b,
                ptr + (i * size) as u64,
                src,
                depth.saturating_sub(1),
            )
        })
        .collect();
    // No element could be read (`Box<dyn Trait>`, measured on 30/09/2026 in a
    // handler's `actions`): `[…]` would hide how many there are; the caller's
    // count says it.
    if read.iter().all(Option::is_none) {
        return None;
    }
    let mut items: Vec<String> = read
        .into_iter()
        .map(|item| item.unwrap_or_else(|| "…".into()))
        .collect();
    if shown < len as usize {
        items.push("…".into());
    }
    Some(format!("[{}]", items.join(", ")))
}

#[note("memory-hashtable")]
pub struct HashTable {
    /// The control bytes: one per bucket, top bit clear when the bucket is full.
    pub ctrl: u64,
    pub buckets: u64,
    pub items: u64,
}

#[note("memory-ctrl-chunk")]
const CTRL_CHUNK: u64 = 256;

#[note("memory-hash-at")]
pub fn hash_at(
    index: &TypeIndex,
    entry: &str,
    set: bool,
    table: &HashTable,
    src: &dyn ValueSource,
    depth: usize,
) -> Option<String> {
    if table.items == 0 {
        return Some("{}".into());
    }
    // A table read before it was built, or from the wrong place, is not shown.
    if !table.buckets.is_power_of_two() || table.items > table.buckets {
        return None;
    }
    let (_, et) = index.by_name(entry)?;
    let size = et.size;
    let Kind::Struct { members, .. } = &et.kind else {
        return None;
    };
    let key = members.iter().find(|m| m.name == "__0")?;
    let value = members.iter().find(|m| m.name == "__1")?;
    if size == 0 {
        return None;
    }
    let member = |m: &crate::dwarf::Member, bytes: &[u8], at: u64| -> Option<String> {
        let mty = m.ty?;
        let off = m.offset as usize;
        let b = bytes.get(off..off + index.get(mty).size as usize)?;
        Some(
            from_bytes(index, mty, b, at + m.offset, src, depth)
                .unwrap_or_else(|| types::short_name(&index.get(mty).name).to_string()),
        )
    };
    let mut shown = Vec::new();
    let mut full = 0u64;
    let mut i = 0u64;
    while i < table.buckets && shown.len() < MAX_ELEMENTS {
        let chunk = (table.buckets - i).min(CTRL_CHUNK);
        let ctrl = read(src, table.ctrl.checked_add(i)?, chunk as usize)?;
        for (j, c) in ctrl.iter().enumerate() {
            if c & 0x80 != 0 {
                continue;
            }
            full += 1;
            let at = table.ctrl.checked_sub((i + j as u64 + 1) * size)?;
            let bytes = read(src, at, size as usize)?;
            let k = member(key, &bytes, at)?;
            shown.push(if set {
                k
            } else {
                format!("{k}: {}", member(value, &bytes, at)?)
            });
            if shown.len() == MAX_ELEMENTS {
                break;
            }
        }
        i += chunk;
    }
    // Every bucket scanned and a count that differs from `items`: not a table.
    if i >= table.buckets && shown.len() < MAX_ELEMENTS && full != table.items {
        return None;
    }
    if (shown.len() as u64) < table.items {
        shown.push("…".into());
    }
    Some(format!("{{{}}}", shown.join(", ")))
}

fn utf8(src: &dyn ValueSource, ptr: u64, len: u64) -> Option<String> {
    if len == 0 {
        return Some("\"\"".into());
    }
    let take = (len as usize).min(MAX_STRING_BYTES);
    let bytes = read(src, ptr, take)?;
    let text = String::from_utf8_lossy(&bytes);
    Some(if take < len as usize {
        format!("{text:?}…")
    } else {
        format!("{text:?}")
    })
}

fn base(enc: BaseEncoding, b: &[u8]) -> String {
    let n = b.len();
    match (enc, n) {
        (BaseEncoding::Bool, _) => format!("{:?}", b.first().copied().unwrap_or(0) != 0),
        (BaseEncoding::Char, 4) => char::from_u32(uint(b) as u32)
            .map(|c| format!("{c:?}"))
            .unwrap_or_else(|| "<invalid char>".into()),
        (BaseEncoding::Float, 4) => format!("{:?}", f32::from_le_bytes([b[0], b[1], b[2], b[3]])),
        (BaseEncoding::Float, 8) => format!("{:?}", f64::from_le_bytes(b[..8].try_into().unwrap())),
        (BaseEncoding::Signed, _) => {
            let v = uint(b);
            let shift = 64 - 8 * n.min(8) as u32;
            format!("{}", ((v << shift) as i64) >> shift)
        }
        _ => format!("{}", uint(b)),
    }
}
