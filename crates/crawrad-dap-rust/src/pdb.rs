//! The type layouts of a `-msvc` build, read from its PDB.
//!
//! A `-msvc` build keeps its debug types in `<program>.pdb` instead of DWARF,
//! and `lldb-dap` cannot cast to them (`*(alloc::string::String*)p` fails,
//! measured on 30/09/2026) — so the elements of a `Vec<String>` or the
//! entries of a `HashMap` need the layouts read here, the same way the DWARF
//! gives them to a `-gnu` build. The result is the same [`TypeIndex`].
//!
//! Names are kept as the PDB spells them, which is how LLDB shows them:
//! `alloc::vec::Vec<alloc::string::String,alloc::alloc::Global>`.
//! A Rust `enum` is a union `enum2$<T>` with one `variantN` per variant and a
//! `tag`; each `VariantN` owns the tag values its constants `DISCR_EXACT`, or
//! `DISCR_BEGIN..=DISCR_END` for the variant a niche encodes, name.

use std::collections::HashMap;
use std::path::Path;

use pdb2::{
    FallibleIterator, PrimitiveKind, SymbolData, TypeData, TypeFinder, TypeIndex as PdbIndex,
};

use crate::dwarf::{Arm, BaseEncoding, Kind, Member, Type, TypeId, TypeIndex, Variants};
use crate::types;
use crawrad_note::note;

#[note("pdb-load")]
pub fn load(path: &Path) -> Option<TypeIndex> {
    let file = std::fs::File::open(path).ok()?;
    let mut pdb = pdb2::PDB::open(file).ok()?;
    let constants = discr_constants(&mut pdb);
    let info = pdb.type_information().ok()?;
    let mut finder = info.finder();
    let mut items = info.iter();
    // The complete definition of every class and union, by name: a member
    // often points at a forward reference.
    let mut complete: HashMap<String, PdbIndex> = HashMap::new();
    while let Ok(Some(item)) = items.next() {
        finder.update(&items);
        let Ok(data) = item.parse() else { continue };
        let (name, forward) = match &data {
            TypeData::Class(c) => (c.name.to_string(), c.properties.forward_reference()),
            TypeData::Union(u) => (u.name.to_string(), u.properties.forward_reference()),
            _ => continue,
        };
        if !forward {
            complete.entry(name.into_owned()).or_insert(item.index());
        }
    }
    let mut builder = Builder {
        finder: &finder,
        complete: &complete,
        constants: &constants,
        types: Vec::new(),
        ids: HashMap::new(),
    };
    let roots: Vec<PdbIndex> = complete.values().copied().collect();
    for index in roots {
        builder.convert(index, 0);
    }
    let types = builder.types;
    (!types.is_empty()).then(|| TypeIndex::from_types(types))
}

#[note("pdb-discr-constants")]
fn discr_constants<'s, S: pdb2::Source<'s> + 's>(
    pdb: &mut pdb2::PDB<'s, S>,
) -> HashMap<String, u64> {
    let mut found = HashMap::new();
    let Ok(symbols) = pdb.global_symbols() else {
        return found;
    };
    let mut iter = symbols.iter();
    while let Ok(Some(symbol)) = iter.next() {
        if let Ok(SymbolData::Constant(c)) = symbol.parse() {
            let name = c.name.to_string();
            if name.ends_with("::DISCR_EXACT")
                || name.ends_with("::DISCR_BEGIN")
                || name.ends_with("::DISCR_END")
            {
                found.insert(name.into_owned(), variant_u64(c.value));
            }
        }
    }
    found
}

fn variant_u64(v: pdb2::Variant) -> u64 {
    match v {
        pdb2::Variant::U8(x) => x.into(),
        pdb2::Variant::U16(x) => x.into(),
        pdb2::Variant::U32(x) => x.into(),
        pdb2::Variant::U64(x) => x,
        pdb2::Variant::I8(x) => x as u64,
        pdb2::Variant::I16(x) => x as u64,
        pdb2::Variant::I32(x) => x as u64,
        pdb2::Variant::I64(x) => x as u64,
    }
}

#[note("pdb-max-nesting")]
const MAX_NESTING: usize = 64;

struct Builder<'a, 't> {
    finder: &'a TypeFinder<'t>,
    complete: &'a HashMap<String, PdbIndex>,
    constants: &'a HashMap<String, u64>,
    types: Vec<Type>,
    ids: HashMap<PdbIndex, TypeId>,
}

impl<'t> Builder<'_, 't> {
    fn parse(&self, index: PdbIndex) -> Option<TypeData<'t>> {
        self.finder.find(index).ok()?.parse().ok()
    }

    #[note("pdb-builder-convert")]
    fn convert(&mut self, index: PdbIndex, nesting: usize) -> Option<TypeId> {
        if let Some(id) = self.ids.get(&index) {
            return Some(*id);
        }
        if nesting > MAX_NESTING {
            return None;
        }
        let data = self.parse(index)?;
        let id = match data {
            TypeData::Primitive(p) => {
                let (name, size, enc) = primitive(p.kind)?;
                let base = Type {
                    name: name.into(),
                    size,
                    kind: Kind::Base(enc),
                };
                // A primitive index can carry a pointer (`u8*` is one index):
                // the index is then the pointer, not the value it points at.
                if p.indirection.is_none() {
                    self.push(index, base)
                } else {
                    let target = self.push_new(base);
                    self.push(
                        index,
                        Type {
                            name: format!("{name}*"),
                            size: 8,
                            kind: Kind::Pointer(Some(target)),
                        },
                    )
                }
            }
            TypeData::Modifier(m) => {
                let id = self.convert(m.underlying_type, nesting + 1)?;
                self.ids.insert(index, id);
                id
            }
            TypeData::Pointer(p) => {
                let id = self.push(
                    index,
                    Type {
                        name: String::new(),
                        size: u64::from(p.attributes.size()),
                        kind: Kind::Pointer(None),
                    },
                );
                let target = self.convert(p.underlying_type, nesting + 1);
                let name = target
                    .map(|t| format!("{}*", self.types[t].name))
                    .unwrap_or_else(|| "void*".into());
                self.types[id].name = name;
                self.types[id].kind = Kind::Pointer(target);
                id
            }
            TypeData::Class(c) => {
                let name = c.name.to_string().into_owned();
                if c.properties.forward_reference() {
                    let full = *self.complete.get(&name)?;
                    let id = self.convert(full, nesting + 1)?;
                    self.ids.insert(index, id);
                    return Some(id);
                }
                let id = self.push(
                    index,
                    Type {
                        name,
                        size: c.size,
                        kind: Kind::Other,
                    },
                );
                let members = match c.fields {
                    Some(fields) => self.members(fields, nesting),
                    None => Vec::new(),
                };
                self.types[id].kind = Kind::Struct {
                    members,
                    variants: None,
                };
                id
            }
            TypeData::Union(u) => {
                let name = u.name.to_string().into_owned();
                if u.properties.forward_reference() {
                    let full = *self.complete.get(&name)?;
                    let id = self.convert(full, nesting + 1)?;
                    self.ids.insert(index, id);
                    return Some(id);
                }
                let id = self.push(
                    index,
                    Type {
                        name: name.clone(),
                        size: u.size,
                        kind: Kind::Other,
                    },
                );
                if types::is_pdb_enum(&name) {
                    let members = self.members(u.fields, nesting);
                    if let Some(variants) = self.variants(&members) {
                        self.types[id].kind = Kind::Struct {
                            members: Vec::new(),
                            variants: Some(variants),
                        };
                    }
                }
                id
            }
            _ => return None,
        };
        Some(id)
    }

    fn push(&mut self, index: PdbIndex, ty: Type) -> TypeId {
        let id = self.push_new(ty);
        self.ids.insert(index, id);
        id
    }

    fn push_new(&mut self, ty: Type) -> TypeId {
        self.types.push(ty);
        self.types.len() - 1
    }

    #[note("pdb-builder-members")]
    fn members(&mut self, fields: PdbIndex, nesting: usize) -> Vec<Member> {
        let mut out = Vec::new();
        let mut next = Some(fields);
        let mut hops = 0;
        while let Some(index) = next.take() {
            hops += 1;
            if hops > MAX_NESTING {
                break;
            }
            let Some(TypeData::FieldList(list)) = self.parse(index) else {
                break;
            };
            let raw: Vec<(String, u64, PdbIndex)> = list
                .fields
                .iter()
                .filter_map(|f| match f {
                    TypeData::Member(m) => {
                        Some((m.name.to_string().into_owned(), m.offset, m.field_type))
                    }
                    _ => None,
                })
                .collect();
            next = list.continuation;
            for (name, offset, ty) in raw {
                let ty = self.convert(ty, nesting + 1);
                out.push(Member { name, offset, ty });
            }
        }
        out
    }

    #[note("pdb-builder-variants")]
    fn variants(&self, members: &[Member]) -> Option<Variants> {
        let discr = members.iter().find(|m| m.name == "tag").cloned();
        let mut arms = Vec::new();
        let mut ranges = 0;
        for variant in members.iter().filter(|m| m.name.starts_with("variant")) {
            let vty = variant.ty?;
            let Kind::Struct { members: parts, .. } = &self.types[vty].kind else {
                continue;
            };
            let value = parts.iter().find(|p| p.name == "value")?;
            let payload = value.ty?;
            let class = &self.types[vty].name;
            let constant = |c: &str| self.constants.get(&format!("{class}::{c}")).copied();
            let discr_value = match constant("DISCR_EXACT") {
                Some(exact) => Some(exact),
                // The niche-encoded variant owns a range: it is the arm taken
                // when no exact value matches, as in the DWARF encoding.
                None if constant("DISCR_BEGIN").is_some() => {
                    ranges += 1;
                    None
                }
                None => None,
            };
            arms.push(Arm {
                discr_value,
                member: Member {
                    name: types::short_name(&self.types[payload].name).to_string(),
                    offset: variant.offset + value.offset,
                    ty: Some(payload),
                },
            });
        }
        // Two ranges cannot both be "the others": the layout is not guessed.
        (!arms.is_empty() && ranges <= 1).then_some(Variants { discr, arms })
    }
}

#[note("pdb-primitive")]
pub(crate) fn primitive(kind: PrimitiveKind) -> Option<(&'static str, u64, BaseEncoding)> {
    use BaseEncoding::*;
    Some(match kind {
        PrimitiveKind::Char | PrimitiveKind::RChar | PrimitiveKind::I8 => ("i8", 1, Signed),
        PrimitiveKind::UChar | PrimitiveKind::U8 | PrimitiveKind::Char8 => ("u8", 1, Unsigned),
        PrimitiveKind::Short | PrimitiveKind::I16 => ("i16", 2, Signed),
        PrimitiveKind::UShort
        | PrimitiveKind::U16
        | PrimitiveKind::WChar
        | PrimitiveKind::RChar16 => ("u16", 2, Unsigned),
        PrimitiveKind::Long | PrimitiveKind::I32 => ("i32", 4, Signed),
        PrimitiveKind::ULong | PrimitiveKind::U32 => ("u32", 4, Unsigned),
        PrimitiveKind::Quad | PrimitiveKind::I64 => ("i64", 8, Signed),
        PrimitiveKind::UQuad | PrimitiveKind::U64 => ("u64", 8, Unsigned),
        PrimitiveKind::Octa | PrimitiveKind::I128 => ("i128", 16, Signed),
        PrimitiveKind::UOcta | PrimitiveKind::U128 => ("u128", 16, Unsigned),
        PrimitiveKind::RChar32 => ("char", 4, Char),
        PrimitiveKind::F32 => ("f32", 4, Float),
        PrimitiveKind::F64 => ("f64", 8, Float),
        PrimitiveKind::Bool8 => ("bool", 1, Bool),
        _ => return None,
    })
}
