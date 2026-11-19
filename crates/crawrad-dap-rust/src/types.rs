//! Reading Rust type names as the debugger spells them.
//!
//! The names come from DWARF through LLDB, so a primitive can show up under
//! its C name (`int`, `unsigned long long`, `double`) as well as its Rust one
//! (`i32`, `u64`, `f64`) — both are accepted.

use crawrad_note::note;
#[note("types-primitive")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Primitive {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bool,
    Char,
}

impl Primitive {
    pub fn of(name: &str) -> Option<Self> {
        Some(match name.trim() {
            // A bare `char` is Rust's here: the caller turns a DWARF name
            // into Rust spelling first (`rust_spelling`), where LLDB's `char`
            // — an `i8` — becomes `i8`.
            "i8" | "signed char" => Self::I8,
            "i16" | "short" => Self::I16,
            "i32" | "int" => Self::I32,
            "i64" | "long long" | "isize" => Self::I64,
            "u8" | "unsigned char" => Self::U8,
            "u16" | "unsigned short" => Self::U16,
            "u32" | "unsigned int" => Self::U32,
            "u64" | "unsigned long long" | "usize" => Self::U64,
            "f32" | "float" => Self::F32,
            "f64" | "double" => Self::F64,
            "bool" => Self::Bool,
            "char" | "char32_t" => Self::Char,
            _ => return None,
        })
    }

    pub fn size(self) -> usize {
        match self {
            Self::I8 | Self::U8 | Self::Bool => 1,
            Self::I16 | Self::U16 => 2,
            Self::I32 | Self::U32 | Self::F32 | Self::Char => 4,
            Self::I64 | Self::U64 | Self::F64 => 8,
        }
    }

    #[note("types-primitive-decode")]
    pub fn decode(self, b: &[u8]) -> String {
        let a = |n: usize| -> [u8; 8] {
            let mut x = [0u8; 8];
            x[..n].copy_from_slice(&b[..n]);
            x
        };
        match self {
            Self::I8 => format!("{:?}", b[0] as i8),
            Self::U8 => format!("{:?}", b[0]),
            Self::Bool => format!("{:?}", b[0] != 0),
            Self::I16 => format!("{:?}", i16::from_le_bytes([b[0], b[1]])),
            Self::U16 => format!("{:?}", u16::from_le_bytes([b[0], b[1]])),
            Self::I32 => format!("{:?}", i32::from_le_bytes([b[0], b[1], b[2], b[3]])),
            Self::U32 => format!("{:?}", u32::from_le_bytes([b[0], b[1], b[2], b[3]])),
            Self::F32 => format!("{:?}", f32::from_le_bytes([b[0], b[1], b[2], b[3]])),
            Self::Char => match char::from_u32(u32::from_le_bytes([b[0], b[1], b[2], b[3]])) {
                Some(c) => format!("{c:?}"),
                None => "<invalid char>".to_string(),
            },
            Self::I64 => format!("{:?}", i64::from_le_bytes(a(8))),
            Self::U64 => format!("{:?}", u64::from_le_bytes(a(8))),
            Self::F64 => format!("{:?}", f64::from_le_bytes(a(8))),
        }
    }
}

#[note("types-generic-args")]
pub fn generic_args(name: &str) -> Vec<&str> {
    let Some(open) = name.find('<') else {
        return Vec::new();
    };
    let Some(close) = name.rfind('>') else {
        return Vec::new();
    };
    let inner = &name[open + 1..close];
    let mut args = Vec::new();
    let (mut depth, mut start) = (0usize, 0usize);
    for (i, c) in inner.char_indices() {
        match c {
            '<' | '(' | '[' => depth += 1,
            '>' | ')' | ']' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                args.push(inner[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    args.push(inner[start..].trim());
    args
}

#[note("types-short-name")]
pub fn short_name(name: &str) -> &str {
    let mut depth = 0usize;
    let mut last = 0usize;
    let bytes = name.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'<' => depth += 1,
            b'>' => depth = depth.saturating_sub(1),
            b':' if depth == 0 && bytes.get(i + 1) == Some(&b':') => {
                last = i + 2;
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }
    let seg = &name[last..];
    seg.split('<').next().unwrap_or(seg)
}

pub fn is_string(name: &str) -> bool {
    name == "alloc::string::String" || name == "String"
}

pub fn is_str(name: &str) -> bool {
    name == "&str" || name == "ref$<str$>" || name == "&mut str"
}

#[note("types-vec-element")]
pub fn vec_element(name: &str) -> Option<&str> {
    if name.starts_with("alloc::vec::Vec<") || name.starts_with("Vec<") {
        generic_args(name).first().copied()
    } else {
        None
    }
}

#[note("types-slice-element")]
pub fn slice_element(name: &str) -> Option<&str> {
    let n = name.trim();
    if n.starts_with("ref$<slice2$<") || n.starts_with("ref_mut$<slice2$<") {
        let inner = *generic_args(n).first()?;
        return generic_args(inner).first().copied();
    }
    let n = n.strip_prefix('&')?.trim_start();
    let n = n.strip_prefix("mut ").unwrap_or(n).trim_start();
    n.strip_prefix('[')?.strip_suffix(']').map(str::trim)
}

#[note("types-is-pointer")]
pub fn is_pointer(name: &str) -> bool {
    name.ends_with('*') || name.starts_with('&')
}

pub fn is_float(name: &str) -> bool {
    matches!(name, "f32" | "f64" | "float" | "double")
}

#[note("types-is-hash-collection")]
pub fn is_hash_collection(name: &str) -> bool {
    name.starts_with("std::collections::hash::map::HashMap<")
        || name.starts_with("std::collections::hash::set::HashSet<")
}

#[note("types-hash-entry")]
pub fn hash_entry(name: &str) -> Option<(String, bool)> {
    let args = generic_args(name);
    let pdb = spelled_by_pdb(name);
    // The tuple as each encoding names it: `(K, V)` in DWARF, `tuple$<K,V>` in
    // a PDB, where the unit is `tuple$<>` and `> >` keeps its space (measured
    // on 30/09/2026).
    let tuple = |k: &str, v: &str| {
        if pdb {
            let inner = format!("{k},{v}");
            let gap = if inner.ends_with('>') { " " } else { "" };
            format!("tuple$<{inner}{gap}>")
        } else {
            format!("({}, {})", rust_spelling(k), rust_spelling(v))
        }
    };
    if name.starts_with("std::collections::hash::map::HashMap<") {
        let [k, v, ..] = args.as_slice() else {
            return None;
        };
        Some((tuple(k, v), false))
    } else if name.starts_with("std::collections::hash::set::HashSet<") {
        let unit = if pdb { "tuple$<>" } else { "()" };
        Some((tuple(args.first()?, unit), true))
    } else {
        None
    }
}

#[note("types-c-primitives")]
const C_PRIMITIVES: &[(&str, &str)] = &[
    ("unsigned long long", "u64"),
    ("unsigned __int128", "u128"),
    ("unsigned short", "u16"),
    ("unsigned char", "u8"),
    ("unsigned int", "u32"),
    ("signed char", "i8"),
    ("long long", "i64"),
    ("__int128", "i128"),
    ("char32_t", "char"),
    ("double", "f64"),
    ("float", "f32"),
    ("short", "i16"),
    ("char", "i8"),
    ("int", "i32"),
];

#[note("types-spelled-by-pdb")]
pub fn spelled_by_pdb(name: &str) -> bool {
    name.contains('$') || (name.contains(',') && !name.contains(", "))
}

#[note("types-in-rust-spelling")]
pub fn in_rust_spelling(arg: &str, container: &str) -> String {
    if spelled_by_pdb(container) {
        arg.to_string()
    } else {
        rust_spelling(arg)
    }
}

#[note("types-is-tuple")]
pub fn is_tuple(name: &str) -> bool {
    name.starts_with('(') || name.starts_with("tuple$<")
}

#[note("types-is-pdb-enum")]
pub fn is_pdb_enum(name: &str) -> bool {
    name.starts_with("enum2$<")
}

#[note("types-debug-name")]
pub fn debug_name(name: &str) -> &str {
    if is_tuple(name) { "" } else { short_name(name) }
}

#[note("types-rust-spelling")]
pub fn rust_spelling(name: &str) -> String {
    let ident = |c: char| c.is_ascii_alphanumeric() || c == '_';
    let mut out = String::with_capacity(name.len());
    let mut rest = name;
    while let Some(c) = rest.chars().next() {
        let at_start = !out.chars().next_back().is_some_and(ident);
        let hit = C_PRIMITIVES.iter().find(|(c_name, _)| {
            at_start && rest.starts_with(c_name) && !rest[c_name.len()..].starts_with(ident)
        });
        match hit {
            Some((c_name, rust)) => {
                out.push_str(rust);
                rest = &rest[c_name.len()..];
            }
            None => {
                out.push(c);
                rest = &rest[c.len_utf8()..];
            }
        }
    }
    out
}

#[note("types-is-std-internal")]
pub fn is_std_internal(name: &str) -> bool {
    ["std::", "core::", "alloc::", "hashbrown::"]
        .iter()
        .any(|p| name.starts_with(p))
}

#[note("types-pointee-name")]
pub fn pointee_name(name: &str) -> &str {
    let n = name.trim();
    let n = n.strip_suffix('*').unwrap_or(n).trim_end();
    let n = n.strip_prefix('&').unwrap_or(n).trim_start();
    n.strip_prefix("mut ").unwrap_or(n).trim()
}
