//! Values rebuilt from bytes and a DWARF layout, on a hand-built index that
//! copies the layouts measured in the probe's DWARF on 30/09/2026 (`-gnu`):
//! `String` = 24 bytes with `vec` at +0; `RawVecInner` has `cap` at +0 and
//! `ptr` at +8; `Vec.len` at +16; `Customer` puts `name` at +0, `tags` at +24
//! and `age` at +48 — not in declaration order.

use std::collections::HashMap;

use crawrad_dap::{DapError, ValueSource, Variable};

use crate::dwarf::{Arm, BaseEncoding, Kind, Member, Type, TypeIndex, Variants};
use crate::memory::{self, HashTable};

#[derive(Default)]
struct Memory(HashMap<u64, Vec<u8>>);

impl Memory {
    fn put(&mut self, at: u64, bytes: &[u8]) {
        self.0.insert(at, bytes.to_vec());
    }
}

impl ValueSource for Memory {
    fn children(&self, _r: i64) -> Result<Vec<Variable>, DapError> {
        Ok(vec![])
    }
    fn read_memory(&self, at: &str, count: usize) -> Result<Vec<u8>, DapError> {
        let addr = u64::from_str_radix(at.trim_start_matches("0x"), 16).unwrap();
        let bytes = self.0.get(&addr).ok_or(DapError::Closed)?;
        Ok(bytes[..count.min(bytes.len())].to_vec())
    }
    fn evaluate(&self, _e: &str, _f: Option<i64>) -> Result<Variable, DapError> {
        Err(DapError::Closed)
    }
}

fn m(name: &str, offset: u64, ty: usize) -> Member {
    Member {
        name: name.into(),
        offset,
        ty: Some(ty),
    }
}
fn st(name: &str, size: u64, members: Vec<Member>) -> Type {
    Type {
        name: name.into(),
        size,
        kind: Kind::Struct {
            members,
            variants: None,
        },
    }
}
fn base(name: &str, size: u64, enc: BaseEncoding) -> Type {
    Type {
        name: name.into(),
        size,
        kind: Kind::Base(enc),
    }
}

// Type ids, in the order `index()` lists them.
const U8: usize = 0;
const USIZE: usize = 1;
const U32: usize = 2;
const PTR_U8: usize = 3;
const NONNULL: usize = 4;
const UNIQUE: usize = 5;
const INNER: usize = 6;
const RAWVEC_U8: usize = 7;
const VEC_U8: usize = 8;
const STRING: usize = 9;
const RAWVEC_STRING: usize = 10;
const VEC_STRING: usize = 11;
const CUSTOMER: usize = 12;
const I32: usize = 13;
const CIRCLE: usize = 14;
const RECT: usize = 15;
const EMPTY: usize = 16;
const SHAPE: usize = 17;
const F64: usize = 18;
const OPT_NONE: usize = 19;
const OPT_SOME: usize = 20;
const OPT_STRING: usize = 21;
const BOX_U64: usize = 22;
const U64: usize = 23;
const UNIT: usize = 25;

fn index() -> TypeIndex {
    TypeIndex::from_types(vec![
        base("u8", 1, BaseEncoding::Unsigned),
        base("usize", 8, BaseEncoding::Unsigned),
        base("u32", 4, BaseEncoding::Unsigned),
        Type {
            name: "*const u8".into(),
            size: 8,
            kind: Kind::Pointer(Some(U8)),
        },
        st(
            "core::ptr::non_null::NonNull<u8>",
            8,
            vec![m("pointer", 0, PTR_U8)],
        ),
        st(
            "core::ptr::unique::Unique<u8>",
            8,
            vec![m("pointer", 0, NONNULL)],
        ),
        st(
            "alloc::raw_vec::RawVecInner<alloc::alloc::Global>",
            16,
            vec![m("ptr", 8, UNIQUE), m("cap", 0, USIZE)],
        ),
        st(
            "alloc::raw_vec::RawVec<u8, alloc::alloc::Global>",
            16,
            vec![m("inner", 0, INNER)],
        ),
        st(
            "alloc::vec::Vec<u8, alloc::alloc::Global>",
            24,
            vec![m("buf", 0, RAWVEC_U8), m("len", 16, USIZE)],
        ),
        st("alloc::string::String", 24, vec![m("vec", 0, VEC_U8)]),
        st(
            "alloc::raw_vec::RawVec<alloc::string::String, alloc::alloc::Global>",
            16,
            vec![m("inner", 0, INNER)],
        ),
        st(
            "alloc::vec::Vec<alloc::string::String, alloc::alloc::Global>",
            24,
            vec![m("buf", 0, RAWVEC_STRING), m("len", 16, USIZE)],
        ),
        st(
            "probe::Customer",
            56,
            vec![
                m("name", 0, STRING),
                m("age", 48, U32),
                m("tags", 24, VEC_STRING),
            ],
        ),
        base("i32", 4, BaseEncoding::Signed),
        st("probe::Shape::Circle", 16, vec![m("radius", 8, F64)]),
        st(
            "probe::Shape::Rect",
            16,
            vec![m("__0", 4, U32), m("__1", 8, U32)],
        ),
        st("probe::Shape::Empty", 16, vec![]),
        Type {
            name: "probe::Shape".into(),
            size: 16,
            kind: Kind::Struct {
                members: vec![],
                variants: Some(Variants {
                    discr: Some(m("", 0, U32)),
                    arms: vec![
                        Arm {
                            discr_value: Some(0),
                            member: m("Circle", 0, CIRCLE),
                        },
                        Arm {
                            discr_value: Some(1),
                            member: m("Rect", 0, RECT),
                        },
                        Arm {
                            discr_value: Some(2),
                            member: m("Empty", 0, EMPTY),
                        },
                    ],
                }),
            },
        },
        base("f64", 8, BaseEncoding::Float),
        st(
            "core::option::Option<alloc::string::String>::None",
            24,
            vec![],
        ),
        st(
            "core::option::Option<alloc::string::String>::Some",
            24,
            vec![m("__0", 0, STRING)],
        ),
        Type {
            name: "core::option::Option<alloc::string::String>".into(),
            size: 24,
            kind: Kind::Struct {
                members: vec![],
                variants: Some(Variants {
                    // The niche: the discriminant is the String's capacity word.
                    discr: Some(m("", 0, U64)),
                    arms: vec![
                        Arm {
                            discr_value: Some(u64::MAX),
                            member: m("None", 0, OPT_NONE),
                        },
                        Arm {
                            discr_value: None,
                            member: m("Some", 0, OPT_SOME),
                        },
                    ],
                }),
            },
        },
        Type {
            name: "*mut u64".into(),
            size: 8,
            kind: Kind::Pointer(Some(U64)),
        },
        base("u64", 8, BaseEncoding::Unsigned),
        st(
            "(alloc::string::String, u32)",
            32,
            vec![m("__0", 0, STRING), m("__1", 24, U32)],
        ),
        st("()", 0, vec![]),
        st("(i32, ())", 4, vec![m("__0", 0, I32), m("__1", 4, UNIT)]),
        st("(u32, i32)", 8, vec![m("__0", 0, U32), m("__1", 4, I32)]),
    ])
}

/// The 24 bytes of a `String` (cap, ptr, len) whose text lives at `at`.
fn string_bytes(mem: &mut Memory, text: &str, at: u64) -> Vec<u8> {
    mem.put(at, text.as_bytes());
    let mut b = Vec::new();
    b.extend_from_slice(&(text.len() as u64).to_le_bytes()); // cap +0
    b.extend_from_slice(&at.to_le_bytes()); // ptr +8
    b.extend_from_slice(&(text.len() as u64).to_le_bytes()); // len +16
    b
}

#[test]
fn a_vec_of_strings_comes_out_element_by_element() {
    let index = index();
    let mut mem = Memory::default();
    let mut elements = string_bytes(&mut mem, "rad", 0x5000);
    elements.extend(string_bytes(&mut mem, "pascal", 0x6000));
    mem.put(0x4000, &elements);
    let shown = memory::vec_at(&index, "alloc::string::String", 0x4000, 2, &mem, 3);
    assert_eq!(shown.as_deref(), Some(r#"["rad", "pascal"]"#));
}

#[test]
fn struct_fields_are_read_at_their_dwarf_offsets_not_in_declaration_order() {
    let index = index();
    let mut mem = Memory::default();
    let mut customer = string_bytes(&mut mem, "Isaque", 0x7000); // name +0
    let mut tags = Vec::new();
    tags.extend(string_bytes(&mut mem, "rad", 0x7100));
    mem.put(0x7200, &tags);
    customer.extend_from_slice(&1u64.to_le_bytes()); // tags.cap +24
    customer.extend_from_slice(&0x7200u64.to_le_bytes()); // tags.ptr +32
    customer.extend_from_slice(&1u64.to_le_bytes()); // tags.len +40
    customer.extend_from_slice(&42u32.to_le_bytes()); // age +48
    customer.extend_from_slice(&[0; 4]);
    mem.put(0x8000, &customer);
    let shown = memory::at(&index, CUSTOMER, 0x8000, &mem, 3);
    assert_eq!(
        shown.as_deref(),
        Some(r#"Customer { name: "Isaque", age: 42, tags: ["rad"] }"#)
    );
}

#[test]
fn an_enum_in_memory_takes_the_arm_its_discriminant_names() {
    let index = index();
    let mut mem = Memory::default();
    let mut circle = vec![0u8; 16];
    circle[8..16].copy_from_slice(&2.0f64.to_le_bytes());
    let mut rect = vec![0u8; 16];
    rect[0..4].copy_from_slice(&1u32.to_le_bytes());
    rect[4..8].copy_from_slice(&3u32.to_le_bytes());
    rect[8..12].copy_from_slice(&4u32.to_le_bytes());
    let mut empty = vec![0u8; 16];
    empty[0..4].copy_from_slice(&2u32.to_le_bytes());
    let mut shapes = circle;
    shapes.extend(rect);
    shapes.extend(empty);
    mem.put(0x9000, &shapes);
    let shown = memory::vec_at(&index, "probe::Shape", 0x9000, 3, &mem, 3);
    assert_eq!(
        shown.as_deref(),
        Some("[Circle { radius: 2.0 }, Rect(3, 4), Empty]")
    );
    // One value on its own, not only as an element (the fake memory answers
    // reads by block start, so the second element is stored as its own block).
    mem.put(0x9010, &shapes[16..32]);
    assert_eq!(
        memory::at(&index, SHAPE, 0x9010, &mem, 3).as_deref(),
        Some("Rect(3, 4)")
    );
}

#[test]
fn a_niche_encoded_option_is_none_by_its_value_and_some_otherwise() {
    let index = index();
    let mut mem = Memory::default();
    let mut none = vec![0u8; 24];
    none[0..8].copy_from_slice(&u64::MAX.to_le_bytes());
    let mut items = none;
    items.extend(string_bytes(&mut mem, "x", 0xA100));
    mem.put(0xA000, &items);
    let shown = memory::vec_at(
        &index,
        "core::option::Option<alloc::string::String>",
        0xA000,
        2,
        &mem,
        3,
    );
    assert_eq!(shown.as_deref(), Some(r#"[None, Some("x")]"#));
    assert_eq!(
        memory::at(&index, OPT_STRING, 0xA000, &mem, 3).as_deref(),
        Some("None")
    );
}

#[test]
fn a_box_is_followed_to_what_it_points_at() {
    let index = index();
    let mut mem = Memory::default();
    mem.put(0xB000, &99u64.to_le_bytes());
    mem.put(0xB100, &0xB000u64.to_le_bytes());
    assert_eq!(
        memory::at(&index, BOX_U64, 0xB100, &mem, 2).as_deref(),
        Some("99")
    );
}

#[test]
fn signed_integers_keep_their_sign() {
    let index = index();
    let mut mem = Memory::default();
    mem.put(0xC000, &(-7i32).to_le_bytes());
    assert_eq!(
        memory::at(&index, I32, 0xC000, &mem, 1).as_deref(),
        Some("-7")
    );
}

#[test]
fn an_element_type_the_dwarf_does_not_have_is_not_guessed() {
    let index = index();
    let mem = Memory::default();
    assert_eq!(
        memory::vec_at(&index, "probe::Unknown", 0xD000, 2, &mem, 3),
        None
    );
}

#[test]
fn a_file_without_dwarf_gives_no_index_so_the_count_stays() {
    // A -msvc build keeps its types in a PDB: no DWARF, no index, and the
    // presenter falls back to the count instead of guessing a layout.
    assert!(TypeIndex::parse(b"not an executable at all").is_none());
    assert!(
        !crate::RustPresenter::for_program(std::path::Path::new("no/such/program")).has_layouts()
    );
}

/// The measured hashbrown layout: one control byte per bucket at `ctrl`, top
/// bit clear when full, and bucket `i` at `ctrl - (i + 1) * size`.
fn put_entry(mem: &mut Memory, ctrl: u64, slot: u64, size: u64, bytes: &[u8]) {
    mem.put(ctrl - (slot + 1) * size, bytes);
}

#[test]
fn a_hash_map_is_read_bucket_by_bucket_in_hashbrown_s_order() {
    let index = index();
    let mut mem = Memory::default();
    let ctrl = 0xE000;
    // Buckets 1 and 3 are full; 0 and 2 are empty (0xFF).
    mem.put(ctrl, &[0xFF, 0x12, 0xFF, 0x34]);
    let mut rui = string_bytes(&mut mem, "Rui", 0xE100);
    rui.extend_from_slice(&41u32.to_le_bytes());
    rui.extend_from_slice(&[0; 4]);
    let mut ana = string_bytes(&mut mem, "Ana", 0xE200);
    ana.extend_from_slice(&30u32.to_le_bytes());
    ana.extend_from_slice(&[0; 4]);
    put_entry(&mut mem, ctrl, 1, 32, &rui);
    put_entry(&mut mem, ctrl, 3, 32, &ana);
    let table = HashTable {
        ctrl,
        buckets: 4,
        items: 2,
    };
    // Bucket order, as `{:?}` iterates — not address order (bucket 3 is lower).
    assert_eq!(
        memory::hash_at(
            &index,
            "(alloc::string::String, u32)",
            false,
            &table,
            &mem,
            3
        )
        .as_deref(),
        Some(r#"{"Rui": 41, "Ana": 30}"#)
    );
}

#[test]
fn a_hash_set_shows_its_keys_only() {
    let index = index();
    let mut mem = Memory::default();
    let ctrl = 0xF000;
    mem.put(ctrl, &[0x05, 0x06]);
    put_entry(&mut mem, ctrl, 0, 4, &7i32.to_le_bytes());
    put_entry(&mut mem, ctrl, 1, 4, &(-3i32).to_le_bytes());
    let table = HashTable {
        ctrl,
        buckets: 2,
        items: 2,
    };
    assert_eq!(
        memory::hash_at(&index, "(i32, ())", true, &table, &mem, 3).as_deref(),
        Some("{7, -3}")
    );
}

#[test]
fn a_table_that_does_not_add_up_is_not_shown() {
    let index = index();
    let mut mem = Memory::default();
    let ctrl = 0xF000;
    mem.put(ctrl, &[0x05, 0xFF]);
    put_entry(&mut mem, ctrl, 0, 4, &7i32.to_le_bytes());
    // `items` says 2, the control bytes say 1: read from the wrong place.
    let wrong_count = HashTable {
        ctrl,
        buckets: 2,
        items: 2,
    };
    assert_eq!(
        memory::hash_at(&index, "(i32, ())", true, &wrong_count, &mem, 3),
        None
    );
    // hashbrown's bucket count is a power of two.
    let odd = HashTable {
        ctrl,
        buckets: 3,
        items: 1,
    };
    assert_eq!(
        memory::hash_at(&index, "(i32, ())", true, &odd, &mem, 3),
        None
    );
}

#[test]
fn a_vec_of_tuples_prints_them_without_a_name() {
    let index = index();
    let mut mem = Memory::default();
    let mut bytes = 1u32.to_le_bytes().to_vec();
    bytes.extend_from_slice(&(-2i32).to_le_bytes());
    mem.put(0xF100, &bytes);
    assert_eq!(
        memory::vec_at(&index, "(u32, i32)", 0xF100, 1, &mem, 3).as_deref(),
        Some("[(1, -2)]")
    );
}

#[test]
fn a_pdb_names_primitives_the_way_a_msvc_build_spells_them() {
    use crate::pdb::primitive;
    use pdb2::PrimitiveKind as P;
    // `Vec<char,…>` in a PDB holds Rust's 4-byte char; its `i8` is `Char`.
    assert_eq!(primitive(P::RChar32), Some(("char", 4, BaseEncoding::Char)));
    assert_eq!(primitive(P::Char), Some(("i8", 1, BaseEncoding::Signed)));
    assert_eq!(
        primitive(P::UQuad),
        Some(("u64", 8, BaseEncoding::Unsigned))
    );
    assert_eq!(primitive(P::Bool8), Some(("bool", 1, BaseEncoding::Bool)));
    assert_eq!(primitive(P::Void), None);
    // Something that is not a PDB gives no layouts.
    assert!(crate::pdb::load(std::path::Path::new("no/such/program.pdb")).is_none());
}

#[test]
fn a_hash_map_entry_is_named_as_each_encoding_names_its_tuple() {
    use crate::types::hash_entry;
    // Measured on 30/09/2026: `tuple$<alloc::string::String,u32>` in a PDB,
    // where the unit is `tuple$<>` and `> >` keeps its space.
    assert_eq!(
        hash_entry(
            "std::collections::hash::map::HashMap<alloc::string::String,u32,std::hash::random::RandomState,alloc::alloc::Global>"
        ),
        Some(("tuple$<alloc::string::String,u32>".to_string(), false))
    );
    assert_eq!(
        hash_entry(
            "std::collections::hash::set::HashSet<i32,std::hash::random::RandomState,alloc::alloc::Global>"
        ),
        Some(("tuple$<i32,tuple$<> >".to_string(), true))
    );
    assert_eq!(
        hash_entry(
            "std::collections::hash::map::HashMap<alloc::string::String, unsigned int, std::hash::random::RandomState, alloc::alloc::Global>"
        ),
        Some(("(alloc::string::String, u32)".to_string(), false))
    );
}

#[test]
fn a_pointer_to_a_pdb_enum_is_read_through_its_layout() {
    // `event: &Event` in a `-msvc` handler: the adapter showed every variant
    // (`enum2$ { variant0: …, variant1: … }`); the layout says which one is set.
    let base = index();
    let mut types: Vec<Type> = (0..base.len()).map(|i| base.get(i).clone()).collect();
    let mut shape = base.get(SHAPE).clone();
    shape.name = "enum2$<probe::Shape>".into();
    types.push(shape);
    let index = TypeIndex::from_types(types);
    let mut mem = Memory::default();
    let mut rect = vec![0u8; 16];
    rect[0..4].copy_from_slice(&1u32.to_le_bytes());
    rect[4..8].copy_from_slice(&3u32.to_le_bytes());
    rect[8..12].copy_from_slice(&4u32.to_le_bytes());
    mem.put(0x9100, &rect);
    let var = Variable {
        name: "event".into(),
        value: "0x0000000000009100".into(),
        type_name: Some("enum2$<probe::Shape> *".into()),
        variables_reference: 7,
        evaluate_name: None,
        memory_reference: None,
    };
    let cx = crate::render::Cx {
        src: &mem,
        index: Some(&index),
    };
    assert_eq!(
        crate::render::present(&var, &cx, 3).as_deref(),
        Some("Rect(3, 4)")
    );
}

#[test]
fn a_vec_whose_elements_cannot_be_read_leaves_the_count_to_the_caller() {
    // Two `String`s whose buffers are not in memory: `[…, …]` would hide what
    // the caller knows — how many there are (`actions` in a handler, 30/09/2026).
    let index = index();
    let mut mem = Memory::default();
    let mut elements = string_bytes(&mut Memory::default(), "rad", 0x5000);
    elements.extend(string_bytes(&mut Memory::default(), "pascal", 0x6000));
    mem.put(0x4000, &elements);
    assert_eq!(
        memory::vec_at(&index, "alloc::string::String", 0x4000, 2, &mem, 3),
        None
    );
}
