//! The program's own type layouts, read from its DWARF (ADR 0048 §3.3).
//!
//! `lldb-dap` without Python cannot give a typed view of an element whose type
//! the standard library erased (the `u8*` inside `RawVecInner`). The
//! executable can: its DWARF says how big an `alloc::string::String` is and at
//! which offset its length lives. With that, any value is rebuilt from
//! `readMemory` alone.
//!
//! Only what the presenter needs is kept: base types, structs with their
//! members and — for Rust enums — the variant part, and pointers.

use std::borrow::Cow;
use std::collections::HashMap;
use std::path::Path;

use crawrad_note::note;
use gimli::{AttributeValue, DebuggingInformationEntry, RunTimeEndian, Unit};
use object::{Object, ObjectSection};

pub type TypeId = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseEncoding {
    Signed,
    Unsigned,
    Float,
    Bool,
    Char,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Member {
    pub name: String,
    pub offset: u64,
    pub ty: Option<TypeId>,
}

#[note("dwarf-arm")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arm {
    /// `None` for the arm that holds when no other matches (niche layouts).
    pub discr_value: Option<u64>,
    pub member: Member,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variants {
    /// Where the discriminant lives, when it is stored.
    pub discr: Option<Member>,
    pub arms: Vec<Arm>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    Base(BaseEncoding),
    Struct {
        members: Vec<Member>,
        variants: Option<Variants>,
    },
    Pointer(Option<TypeId>),
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    /// Qualified the way LLDB prints it: `alloc::string::String`.
    pub name: String,
    pub size: u64,
    pub kind: Kind,
}

#[derive(Debug, Default)]
pub struct TypeIndex {
    types: Vec<Type>,
    by_name: HashMap<String, TypeId>,
}

impl TypeIndex {
    #[note("dwarf-typeindex-load")]
    pub fn load(path: &Path) -> Option<Self> {
        let data = std::fs::read(path).ok()?;
        Self::parse(&data)
    }

    pub fn parse(data: &[u8]) -> Option<Self> {
        let file = object::File::parse(data).ok()?;
        let endian = if file.is_little_endian() {
            RunTimeEndian::Little
        } else {
            RunTimeEndian::Big
        };
        let load = |id: gimli::SectionId| -> Result<Cow<'_, [u8]>, gimli::Error> {
            Ok(file
                .section_by_name(id.name())
                .and_then(|s| s.uncompressed_data().ok())
                .unwrap_or(Cow::Borrowed(&[])))
        };
        let owned = gimli::DwarfSections::load(load).ok()?;
        let dwarf = owned.borrow(|s| gimli::EndianSlice::new(s, endian));
        let mut builder = Builder::default();
        let mut units = dwarf.units();
        while let Ok(Some(header)) = units.next() {
            let Ok(unit) = dwarf.unit(header) else {
                continue;
            };
            let Ok(mut tree) = unit.entries_tree(None) else {
                continue;
            };
            let Ok(root) = tree.root() else { continue };
            builder.walk(&dwarf, &unit, root, &mut Vec::new());
        }
        let index = builder.finish();
        (!index.types.is_empty()).then_some(index)
    }

    #[note("dwarf-typeindex-from-types")]
    pub fn from_types(types: Vec<Type>) -> Self {
        let mut by_name = HashMap::new();
        for (id, t) in types.iter().enumerate() {
            by_name.entry(t.name.clone()).or_insert(id);
        }
        Self { types, by_name }
    }

    pub fn get(&self, id: TypeId) -> &Type {
        &self.types[id]
    }

    pub fn by_name(&self, name: &str) -> Option<(TypeId, &Type)> {
        let id = *self.by_name.get(name)?;
        Some((id, &self.types[id]))
    }

    #[note("dwarf-typeindex-iter")]
    pub fn iter(&self) -> impl Iterator<Item = (TypeId, &Type)> {
        self.types.iter().enumerate()
    }

    pub fn len(&self) -> usize {
        self.types.len()
    }

    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }

    #[note("dwarf-typeindex-member-path")]
    pub fn member_path(&self, mut ty: TypeId, path: &[&str]) -> Option<(u64, TypeId)> {
        let mut offset = 0u64;
        for name in path {
            let Kind::Struct { members, .. } = &self.types[ty].kind else {
                return None;
            };
            let m = members.iter().find(|m| m.name == *name)?;
            offset += m.offset;
            ty = m.ty?;
        }
        Some((offset, ty))
    }
}

#[note("dwarf-raw")]
struct Raw {
    name: String,
    size: u64,
    kind: RawKind,
}

#[note("dwarf-rawvariants")]
type RawVariants = (Option<RawMember>, Vec<(Option<u64>, RawMember)>);

enum RawKind {
    Base(BaseEncoding),
    Struct {
        members: Vec<RawMember>,
        variants: Option<RawVariants>,
    },
    Pointer(Option<usize>),
}

struct RawMember {
    name: String,
    offset: u64,
    ty: Option<usize>,
}

#[derive(Default)]
struct Builder {
    raw: Vec<(usize, Raw)>,
}

type R<'a> = gimli::EndianSlice<'a, RunTimeEndian>;

fn text<'a>(
    dwarf: &gimli::Dwarf<R<'a>>,
    unit: &Unit<R<'a>>,
    v: AttributeValue<R<'a>>,
) -> Option<String> {
    let s = dwarf.attr_string(unit, v).ok()?;
    Some(s.to_string_lossy().into_owned())
}

fn name_of<'a>(
    dwarf: &gimli::Dwarf<R<'a>>,
    unit: &Unit<R<'a>>,
    e: &DebuggingInformationEntry<R<'a>>,
) -> Option<String> {
    let v = e.attr_value(gimli::DW_AT_name)?;
    text(dwarf, unit, v)
}

fn udata<'a>(e: &DebuggingInformationEntry<R<'a>>, at: gimli::DwAt) -> Option<u64> {
    e.attr_value(at).and_then(|v| v.udata_value())
}

#[note("dwarf-type-ref")]
fn type_ref<'a>(unit: &Unit<R<'a>>, e: &DebuggingInformationEntry<R<'a>>) -> Option<usize> {
    match e.attr_value(gimli::DW_AT_type)? {
        AttributeValue::UnitRef(off) => off.to_debug_info_offset(&unit.header).map(|o| o.0),
        AttributeValue::DebugInfoRef(off) => Some(off.0),
        _ => None,
    }
}

fn global_offset<'a>(unit: &Unit<R<'a>>, e: &DebuggingInformationEntry<R<'a>>) -> Option<usize> {
    e.offset().to_debug_info_offset(&unit.header).map(|o| o.0)
}

fn member<'a>(
    dwarf: &gimli::Dwarf<R<'a>>,
    unit: &Unit<R<'a>>,
    e: &DebuggingInformationEntry<R<'a>>,
) -> RawMember {
    RawMember {
        name: name_of(dwarf, unit, e).unwrap_or_default(),
        offset: udata(e, gimli::DW_AT_data_member_location).unwrap_or(0),
        ty: type_ref(unit, e),
    }
}

impl Builder {
    fn walk<'a>(
        &mut self,
        dwarf: &gimli::Dwarf<R<'a>>,
        unit: &Unit<R<'a>>,
        node: gimli::EntriesTreeNode<'_, '_, R<'a>>,
        scope: &mut Vec<String>,
    ) {
        let e = node.entry().clone();
        let tag = e.tag();
        let own_name = name_of(dwarf, unit, &e);
        let qualified = |n: &str, scope: &[String]| {
            if scope.is_empty() {
                n.to_string()
            } else {
                format!("{}::{n}", scope.join("::"))
            }
        };
        let mut pushed = false;
        match tag {
            gimli::DW_TAG_namespace => {
                if let Some(n) = &own_name {
                    scope.push(n.clone());
                    pushed = true;
                }
            }
            gimli::DW_TAG_base_type => {
                if let (Some(n), Some(at)) = (&own_name, global_offset(unit, &e)) {
                    let enc = match e.attr_value(gimli::DW_AT_encoding) {
                        Some(AttributeValue::Encoding(gimli::DW_ATE_signed)) => {
                            BaseEncoding::Signed
                        }
                        Some(AttributeValue::Encoding(gimli::DW_ATE_unsigned)) => {
                            BaseEncoding::Unsigned
                        }
                        Some(AttributeValue::Encoding(gimli::DW_ATE_float)) => BaseEncoding::Float,
                        Some(AttributeValue::Encoding(gimli::DW_ATE_boolean)) => BaseEncoding::Bool,
                        Some(AttributeValue::Encoding(gimli::DW_ATE_UTF)) => BaseEncoding::Char,
                        _ => BaseEncoding::Other,
                    };
                    self.raw.push((
                        at,
                        Raw {
                            name: n.clone(),
                            size: udata(&e, gimli::DW_AT_byte_size).unwrap_or(0),
                            kind: RawKind::Base(enc),
                        },
                    ));
                }
            }
            gimli::DW_TAG_pointer_type | gimli::DW_TAG_reference_type => {
                if let Some(at) = global_offset(unit, &e) {
                    self.raw.push((
                        at,
                        Raw {
                            name: own_name.clone().unwrap_or_default(),
                            size: udata(&e, gimli::DW_AT_byte_size).unwrap_or(8),
                            kind: RawKind::Pointer(type_ref(unit, &e)),
                        },
                    ));
                }
            }
            gimli::DW_TAG_structure_type | gimli::DW_TAG_union_type => {
                if let (Some(n), Some(at)) = (&own_name, global_offset(unit, &e)) {
                    let name = qualified(n, scope);
                    let size = udata(&e, gimli::DW_AT_byte_size).unwrap_or(0);
                    let mut members = Vec::new();
                    let mut variants = None;
                    let mut children = node.children();
                    // Nested types (a Rust enum's variants) are walked with this
                    // struct as their scope, after its own members are read.
                    let mut nested = Vec::new();
                    while let Ok(Some(child)) = children.next() {
                        let c = child.entry().clone();
                        match c.tag() {
                            gimli::DW_TAG_member => members.push(member(dwarf, unit, &c)),
                            gimli::DW_TAG_variant_part => {
                                variants = Some(self.variant_part(dwarf, unit, child))
                            }
                            gimli::DW_TAG_structure_type
                            | gimli::DW_TAG_union_type
                            | gimli::DW_TAG_enumeration_type => {
                                if let Some(off) = global_offset(unit, &c) {
                                    nested.push(off);
                                }
                                scope.push(n.clone());
                                self.walk(dwarf, unit, child, scope);
                                scope.pop();
                            }
                            _ => {}
                        }
                    }
                    let _ = nested;
                    self.raw.push((
                        at,
                        Raw {
                            name,
                            size,
                            kind: RawKind::Struct { members, variants },
                        },
                    ));
                    return;
                }
            }
            _ => {}
        }
        let mut children = node.children();
        while let Ok(Some(child)) = children.next() {
            self.walk(dwarf, unit, child, scope);
        }
        if pushed {
            scope.pop();
        }
    }

    fn variant_part<'a>(
        &mut self,
        dwarf: &gimli::Dwarf<R<'a>>,
        unit: &Unit<R<'a>>,
        node: gimli::EntriesTreeNode<'_, '_, R<'a>>,
    ) -> RawVariants {
        let discr_at = match node.entry().attr_value(gimli::DW_AT_discr) {
            Some(AttributeValue::UnitRef(off)) => Some(off),
            _ => None,
        };
        let mut discr = None;
        let mut arms = Vec::new();
        let mut children = node.children();
        while let Ok(Some(child)) = children.next() {
            let c = child.entry().clone();
            match c.tag() {
                gimli::DW_TAG_member if Some(c.offset()) == discr_at => {
                    discr = Some(member(dwarf, unit, &c))
                }
                gimli::DW_TAG_variant => {
                    let value = udata(&c, gimli::DW_AT_discr_value);
                    let mut inner = child.children();
                    while let Ok(Some(m)) = inner.next() {
                        if m.entry().tag() == gimli::DW_TAG_member {
                            arms.push((value, member(dwarf, unit, m.entry())));
                        }
                    }
                }
                _ => {}
            }
        }
        (discr, arms)
    }

    fn finish(self) -> TypeIndex {
        let mut ids: HashMap<usize, TypeId> = HashMap::new();
        for (i, (at, _)) in self.raw.iter().enumerate() {
            ids.insert(*at, i);
        }
        let resolve = |r: Option<usize>| r.and_then(|o| ids.get(&o).copied());
        let fix = |m: RawMember| Member {
            name: m.name,
            offset: m.offset,
            ty: resolve(m.ty),
        };
        let mut index = TypeIndex::default();
        for (_, raw) in self.raw {
            let kind = match raw.kind {
                RawKind::Base(e) => Kind::Base(e),
                RawKind::Pointer(t) => Kind::Pointer(resolve(t)),
                RawKind::Struct { members, variants } => Kind::Struct {
                    members: members.into_iter().map(fix).collect(),
                    variants: variants.map(|(d, arms)| Variants {
                        discr: d.map(fix),
                        arms: arms
                            .into_iter()
                            .map(|(v, m)| Arm {
                                discr_value: v,
                                member: fix(m),
                            })
                            .collect(),
                    }),
                },
            };
            let id = index.types.len();
            // The first definition of a name wins; declarations without a
            // size are skipped so a forward declaration never shadows it.
            if raw.size > 0 || matches!(kind, Kind::Base(_)) {
                index.by_name.entry(raw.name.clone()).or_insert(id);
            }
            index.types.push(Type {
                name: raw.name,
                size: raw.size,
                kind,
            });
        }
        index
    }
}
