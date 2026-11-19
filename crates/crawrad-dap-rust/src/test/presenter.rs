//! The Rust presenter against the raw tree `lldb-dap` gave on 30/09/2026
//! (probe program, `-gnu`, no Python): the same names, types, `evaluateName`s,
//! pointers and bytes, served by a fake source.

use std::collections::HashMap;

use crawrad_dap::{DapError, ValuePresenter, ValueSource, Variable};

use crate::RustPresenter;

#[derive(Default)]
struct Fake {
    children: HashMap<i64, Vec<Variable>>,
    memory: HashMap<String, Vec<u8>>,
    evals: HashMap<String, String>,
}

impl ValueSource for Fake {
    fn children(&self, r: i64) -> Result<Vec<Variable>, DapError> {
        self.children.get(&r).cloned().ok_or(DapError::Closed)
    }
    fn read_memory(&self, at: &str, count: usize) -> Result<Vec<u8>, DapError> {
        let bytes = self.memory.get(at).ok_or(DapError::Closed)?;
        Ok(bytes[..count.min(bytes.len())].to_vec())
    }
    fn evaluate(&self, e: &str, _frame: Option<i64>) -> Result<Variable, DapError> {
        let value = self.evals.get(e).cloned().ok_or(DapError::Closed)?;
        Ok(v(e, &value, "", 0))
    }
}

fn v(name: &str, value: &str, ty: &str, reference: i64) -> Variable {
    Variable {
        name: name.to_string(),
        value: value.to_string(),
        type_name: (!ty.is_empty()).then(|| ty.to_string()),
        variables_reference: reference,
        evaluate_name: None,
        memory_reference: None,
    }
}

fn named(mut var: Variable, eval: &str) -> Variable {
    var.evaluate_name = Some(eval.to_string());
    var
}

fn show(var: &Variable, src: &Fake) -> Option<String> {
    RustPresenter::new().present(var, src)
}

/// A `String` as measured: 13 bytes at 0x28d4f46d9c0.
fn string_fixture(src: &mut Fake, base: &str, text: &str, addr: u64) -> Variable {
    src.evals
        .insert(format!("{base}.vec.len"), text.len().to_string());
    src.evals.insert(
        format!("{base}.vec.buf.inner.ptr.pointer.pointer"),
        format!("0x{addr:016x} \"{text}\""),
    );
    src.memory
        .insert(format!("0x{addr:x}"), text.as_bytes().to_vec());
    named(
        v(base, "alloc::string::String", "alloc::string::String", 1),
        base,
    )
}

#[test]
fn a_string_is_read_from_its_buffer() {
    let mut src = Fake::default();
    let greeting = string_fixture(&mut src, "greeting", "Hello, CrawRAD", 0x28d4f46d9c0);
    assert_eq!(show(&greeting, &src).as_deref(), Some("\"Hello, CrawRAD\""));
}

#[test]
fn an_empty_string_needs_no_memory() {
    let mut src = Fake::default();
    src.evals.insert("s.vec.len".into(), "0".into());
    let s = named(
        v("s", "alloc::string::String", "alloc::string::String", 1),
        "s",
    );
    assert_eq!(show(&s, &src).as_deref(), Some("\"\""));
}

#[test]
fn a_str_slice_reads_data_ptr_and_length() {
    let mut src = Fake::default();
    src.children.insert(
        4,
        vec![
            v(
                "data_ptr",
                "0x00007ff6ac669425 \"slice\"",
                "unsigned char *",
                5,
            ),
            v("length", "5", "unsigned long long", 0),
        ],
    );
    src.memory
        .insert("0x7ff6ac669425".into(), b"slice".to_vec());
    let slice = named(v("slice", "&str", "&str", 4), "slice");
    assert_eq!(show(&slice, &src).as_deref(), Some("\"slice\""));
}

#[test]
fn a_vec_of_primitives_is_read_in_one_go() {
    let mut src = Fake::default();
    src.evals.insert("numbers.len".into(), "3".into());
    src.evals.insert(
        "numbers.buf.inner.ptr.pointer.pointer".into(),
        "0x0000028d4f46da00 \"\\n\"".into(),
    );
    let bytes: Vec<u8> = [10i32, 20, 30]
        .iter()
        .flat_map(|n| n.to_le_bytes())
        .collect();
    src.memory.insert("0x28d4f46da00".into(), bytes);
    let numbers = named(
        v(
            "numbers",
            "alloc::vec::Vec<int, alloc::alloc::Global>",
            "alloc::vec::Vec<int, alloc::alloc::Global>",
            6,
        ),
        "numbers",
    );
    assert_eq!(show(&numbers, &src).as_deref(), Some("[10, 20, 30]"));
}

#[test]
fn lldb_s_char_is_an_i8_and_char32_t_is_a_rust_char() {
    // Measured on 30/09/2026 (-gnu): `Vec<i8>` comes as `Vec<char, …>` and
    // `Vec<char>` as `Vec<char32_t, …>`; reading `char` as a Rust char showed
    // `[<invalid char>]` for `vec![-1i8]`.
    let mut src = Fake::default();
    src.evals.insert("small.len".into(), "2".into());
    src.evals.insert(
        "small.buf.inner.ptr.pointer.pointer".into(),
        "0x0000000000001000".into(),
    );
    src.memory.insert("0x1000".into(), vec![0xFF, 2]);
    let small = named(
        v(
            "small",
            "alloc::vec::Vec<char, alloc::alloc::Global>",
            "alloc::vec::Vec<char, alloc::alloc::Global>",
            7,
        ),
        "small",
    );
    assert_eq!(show(&small, &src).as_deref(), Some("[-1, 2]"));
    assert_eq!(
        crate::types::Primitive::of("char32_t"),
        Some(crate::types::Primitive::Char)
    );
}

#[test]
fn lldb_s_c_spelling_is_turned_into_the_rust_names_dwarf_keeps() {
    use crate::types::{generic_args, hash_entry, rust_spelling};
    assert_eq!(
        rust_spelling(
            "std::collections::hash::map::HashMap<alloc::string::String, unsigned int, std::hash::random::RandomState, alloc::alloc::Global>"
        ),
        "std::collections::hash::map::HashMap<alloc::string::String, u32, std::hash::random::RandomState, alloc::alloc::Global>"
    );
    assert_eq!(
        rust_spelling("(unsigned long long, long long)"),
        "(u64, i64)"
    );
    assert_eq!(rust_spelling("(char32_t, char, short)"), "(char, i8, i16)");
    // Only whole words: a name that merely contains one is left alone.
    assert_eq!(
        rust_spelling("probe::integer::Point"),
        "probe::integer::Point"
    );
    assert_eq!(rust_spelling("probe::Hint<int>"), "probe::Hint<i32>");
    // A tuple argument keeps its comma.
    assert_eq!(
        generic_args("alloc::vec::Vec<(u32, i32), alloc::alloc::Global>"),
        vec!["(u32, i32)", "alloc::alloc::Global"]
    );
    assert_eq!(
        hash_entry(
            "std::collections::hash::set::HashSet<int, std::hash::random::RandomState, alloc::alloc::Global>"
        ),
        Some(("(i32, ())".to_string(), true))
    );
}

#[test]
fn a_vec_of_composites_says_how_many_until_dwarf_gives_the_layout() {
    let mut src = Fake::default();
    src.evals.insert("tags.len".into(), "2".into());
    let tags = named(
        v(
            "tags",
            "",
            "alloc::vec::Vec<alloc::string::String, alloc::alloc::Global>",
            7,
        ),
        "tags",
    );
    assert_eq!(show(&tags, &src).as_deref(), Some("[… 2 items]"));
}

/// `shape = Circle { radius: 2.0 }` and `rect = Rect(3, 4)`: the same three
/// variants, told apart only by `$discr$`.
fn shape_fixture(
    src: &mut Fake,
    reference: i64,
    discr: &str,
    payload: [(&str, &str); 3],
) -> Variable {
    let (variants, v0, v1, v2) = (reference + 1, reference + 2, reference + 3, reference + 4);
    let (p0, p1, p2) = (reference + 5, reference + 6, reference + 7);
    src.children.insert(
        reference,
        vec![v(
            "$variants$",
            "",
            "probe::Shape::probe::Shape$Inner",
            variants,
        )],
    );
    src.children.insert(
        variants,
        vec![
            v(
                "$variant$0",
                "",
                "probe::Shape::probe::Shape$Inner::Circle$Variant",
                v0,
            ),
            v(
                "$variant$1",
                "",
                "probe::Shape::probe::Shape$Inner::Rect$Variant",
                v1,
            ),
            v(
                "$variant$2",
                "",
                "probe::Shape::probe::Shape$Inner::Empty$Variant",
                v2,
            ),
        ],
    );
    for (variant, payload_ref, ty) in [
        (v0, p0, "probe::Shape::Circle"),
        (v1, p1, "probe::Shape::Rect"),
        (v2, p2, "probe::Shape::Empty"),
    ] {
        src.children.insert(
            variant,
            vec![
                v("$discr$", discr, "unsigned int", 0),
                v("value", ty, ty, payload_ref),
            ],
        );
    }
    src.children
        .insert(p0, vec![v("radius", payload[0].1, "double", 0)]);
    src.children.insert(
        p1,
        vec![
            v("__0", payload[1].0, "unsigned int", 0),
            v("__1", payload[1].1, "unsigned int", 0),
        ],
    );
    src.children.insert(p2, vec![]);
    v("shape", "probe::Shape", "probe::Shape", reference)
}

#[test]
fn an_enum_shows_the_variant_its_discriminant_names() {
    let mut src = Fake::default();
    let shape = shape_fixture(&mut src, 100, "0", [("", "2"), ("0", "0"), ("", "")]);
    assert_eq!(
        show(&shape, &src).as_deref(),
        Some("Circle { radius: 2.0 }")
    );
    let rect = shape_fixture(
        &mut src,
        200,
        "1",
        [("", "1.9762625833649862E-323"), ("3", "4"), ("", "")],
    );
    assert_eq!(show(&rect, &src).as_deref(), Some("Rect(3, 4)"));
    let empty = shape_fixture(&mut src, 300, "2", [("", "0"), ("0", "0"), ("", "")]);
    assert_eq!(show(&empty, &src).as_deref(), Some("Empty"));
}

#[test]
fn option_some_and_a_niche_encoded_none() {
    let mut src = Fake::default();
    // Some(7): discriminant 1 names $variant$1.
    src.children.insert(
        10,
        vec![v(
            "$variants$",
            "",
            "core::option::Option<i32>::core::option::Option<i32>$Inner",
            11,
        )],
    );
    src.children.insert(
        11,
        vec![v("$variant$0", "", "", 12), v("$variant$1", "", "", 13)],
    );
    src.children.insert(
        12,
        vec![
            v("$discr$", "1", "unsigned int", 0),
            v("value", "", "core::option::Option<i32>::None<int>", 0),
        ],
    );
    src.children.insert(
        13,
        vec![
            v("$discr$", "1", "unsigned int", 0),
            v("value", "", "core::option::Option<i32>::Some<int>", 14),
        ],
    );
    src.children.insert(14, vec![v("__0", "7", "int", 0)]);
    let some = v(
        "some",
        "core::option::Option<i32>",
        "core::option::Option<i32>",
        10,
    );
    assert_eq!(show(&some, &src).as_deref(), Some("Some(7)"));

    // None of Option<String>: the niche. The numbered variant carries the
    // discriminant value in its name; the unnumbered one holds the data.
    src.children.insert(
        20,
        vec![v(
            "$variants$",
            "",
            "core::option::Option<alloc::string::String>::…",
            21,
        )],
    );
    src.children.insert(
        21,
        vec![
            v("$variant$18446744073709551615", "", "", 22),
            v("$variant$", "", "", 23),
        ],
    );
    src.children.insert(
        22,
        vec![
            v("$discr$", "18446744073709551615", "unsigned long long", 0),
            v(
                "value",
                "",
                "core::option::Option<alloc::string::String>::None<alloc::string::String>",
                0,
            ),
        ],
    );
    src.children.insert(
        23,
        vec![v(
            "value",
            "",
            "core::option::Option<alloc::string::String>::Some<alloc::string::String>",
            24,
        )],
    );
    let none = v(
        "none",
        "core::option::Option<alloc::string::String>",
        "core::option::Option<alloc::string::String>",
        20,
    );
    assert_eq!(show(&none, &src).as_deref(), Some("None"));
}

#[test]
fn a_struct_prints_its_fields_like_debug() {
    let mut src = Fake::default();
    let mut name = string_fixture(&mut src, "customer.name", "Isaque", 0x1c2afed4300);
    name.name = "name".into();
    src.evals.insert("customer.tags.len".into(), "2".into());
    let tags = named(
        v(
            "tags",
            "",
            "alloc::vec::Vec<alloc::string::String, alloc::alloc::Global>",
            32,
        ),
        "customer.tags",
    );
    src.children
        .insert(30, vec![name, v("age", "42", "unsigned int", 0), tags]);
    let customer = named(
        v("customer", "probe::Customer", "probe::Customer", 30),
        "customer",
    );
    assert_eq!(
        show(&customer, &src).as_deref(),
        Some("Customer { name: \"Isaque\", age: 42, tags: [… 2 items] }")
    );
}

#[test]
fn a_box_shows_what_it_points_at() {
    let mut src = Fake::default();
    src.children
        .insert(40, vec![v("*boxed", "99", "unsigned long long", 0)]);
    let boxed = v("boxed", "0x0000028d4f4749e0", "unsigned long long *", 40);
    assert_eq!(show(&boxed, &src).as_deref(), Some("99"));
}

#[test]
fn what_it_does_not_know_it_leaves_to_the_adapter() {
    let src = Fake::default();
    // A primitive: the adapter's own text is already right.
    assert_eq!(show(&v("age", "42", "unsigned int", 0), &src), None);
    // A String whose internals do not resolve (a future std layout): raw, not
    // a number read from the wrong place.
    let s = named(
        v("s", "alloc::string::String", "alloc::string::String", 1),
        "s",
    );
    assert_eq!(show(&s, &src), None);
}

#[test]
fn an_older_std_layout_is_still_found() {
    let mut src = Fake::default();
    src.evals.insert("v.len".into(), "2".into());
    src.evals
        .insert("v.buf.ptr.pointer.pointer".into(), "0x1000".into());
    src.memory.insert("0x1000".into(), vec![1, 0, 2, 0]);
    let old = named(
        v("v", "", "alloc::vec::Vec<u16, alloc::alloc::Global>", 1),
        "v",
    );
    assert_eq!(show(&old, &src).as_deref(), Some("[1, 2]"));
}

#[test]
fn a_hash_map_shows_its_count_not_hashbrowns_internals() {
    let mut src = Fake::default();
    src.evals
        .insert("map.base.table.table.items".into(), "1".into());
    let map = named(
        v(
            "map",
            "",
            "std::collections::hash::map::HashMap<alloc::string::String, int, std::hash::random::RandomState>",
            50,
        ),
        "map",
    );
    assert_eq!(show(&map, &src).as_deref(), Some("{… 1 item}"));
    src.evals
        .insert("empty.base.table.table.items".into(), "0".into());
    let empty = named(
        v(
            "empty",
            "",
            "std::collections::hash::map::HashMap<u8, u8, std::hash::random::RandomState>",
            51,
        ),
        "empty",
    );
    assert_eq!(show(&empty, &src).as_deref(), Some("{}"));
}

#[test]
fn a_std_type_it_does_not_know_is_left_raw_not_walked() {
    // Measured on 30/09/2026 before this guard: the HashMap came out as
    // `HashMap { base: HashMap { hash_builder: RandomState { k0: …, k1: … } …`.
    let mut src = Fake::default();
    src.children.insert(
        60,
        vec![
            v("k0", "3313798373847711978", "unsigned long long", 0),
            v("k1", "10709155143684687715", "unsigned long long", 0),
        ],
    );
    let state = v(
        "hash_builder",
        "std::hash::random::RandomState",
        "std::hash::random::RandomState",
        60,
    );
    assert_eq!(show(&state, &src), None);
    // A user type with the same fields is still shown.
    let mine = v("mine", "probe::Keys", "probe::Keys", 60);
    assert_eq!(
        show(&mine, &src).as_deref(),
        Some("Keys { k0: 3313798373847711978, k1: 10709155143684687715 }")
    );
}

#[test]
fn a_pointer_to_a_struct_shows_the_struct_like_self_in_a_handler() {
    // `&mut self` in `btn_ok_click`, measured on 30/09/2026: the pointer's
    // children ARE the struct's fields — there is no `*self` node.
    let mut src = Fake::default();
    src.children.insert(
        70,
        vec![
            v("enabled", "true", "bool", 0),
            v("tab_order", "1", "unsigned int", 0),
        ],
    );
    let btn = v("btn_ok", "", "meu_app::form_main_designer::Button", 70);
    src.children.insert(71, vec![btn]);
    let this = v(
        "self",
        "0x000002a3c40cf1d8",
        "meu_app::form_main_designer::FormMain *",
        71,
    );
    assert_eq!(
        show(&this, &src).as_deref(),
        Some("FormMain { btn_ok: Button { enabled: true, tab_order: 1 } }")
    );
}

/// One variant of a PDB enum: its name, its payload, its `DISCR_*` constants.
type Arm<'a> = (&'a str, &'a str, &'a [(&'a str, &'a str)]);

/// `enum2$<T>` as measured on 30/09/2026 (`-msvc`): `variantN` children with a
/// `value`, a `tag`, and each variant's `DISCR_*` constants by evaluation.
fn pdb_enum(src: &mut Fake, base: &str, reference: i64, tag: &str, arms: &[Arm]) -> Variable {
    let mut children = Vec::new();
    for (i, (name, payload, discr)) in arms.iter().enumerate() {
        let variant = format!("variant{i}");
        let vref = reference * 100 + i as i64 * 2 + 1;
        let pref = vref + 1;
        children.push(v(&variant, "", "", vref));
        src.children.insert(
            vref,
            vec![v("value", "", &format!("enum2$<probe::T>::{name}"), pref)],
        );
        let fields: Vec<Variable> = if payload.is_empty() {
            vec![]
        } else {
            vec![v("__0", payload, "int", 0)]
        };
        src.children.insert(pref, fields);
        for (constant, value) in discr.iter() {
            src.evals
                .insert(format!("{base}.{variant}.{constant}"), (*value).into());
        }
    }
    children.push(v("tag", tag, "unsigned long long", 0));
    src.children.insert(reference, children);
    named(v(base, "", "enum2$<probe::T>", reference), base)
}

#[test]
fn a_pdb_enum_takes_the_variant_whose_exact_discriminant_is_the_tag() {
    let mut src = Fake::default();
    let arms: &[Arm] = &[
        ("None", "", &[("DISCR_EXACT", "0")]),
        ("Some", "7", &[("DISCR_EXACT", "1")]),
    ];
    let some = pdb_enum(&mut src, "some", 70, "1", arms);
    assert_eq!(show(&some, &src).as_deref(), Some("Some(7)"));
    let none = pdb_enum(&mut src, "none", 71, "0", arms);
    assert_eq!(show(&none, &src).as_deref(), Some("None"));
}

#[test]
fn a_pdb_enum_niche_is_the_variant_whose_range_holds_the_tag() {
    // `Option<String>`, measured: `None` is exactly 2^63, `Some` is the range
    // of capacities `0..=2^63-1`.
    let arms: &[Arm] = &[
        ("None", "", &[("DISCR_EXACT", "9223372036854775808")]),
        (
            "Some",
            "5",
            &[("DISCR_BEGIN", "0"), ("DISCR_END", "9223372036854775807")],
        ),
    ];
    let mut src = Fake::default();
    let none = pdb_enum(&mut src, "none", 72, "9223372036854775808", arms);
    assert_eq!(show(&none, &src).as_deref(), Some("None"));
    let some = pdb_enum(&mut src, "some", 73, "24", arms);
    assert_eq!(show(&some, &src).as_deref(), Some("Some(5)"));
    // A range that wraps past the tag's maximum still owns both ends.
    let wrapping: &[Arm] = &[
        ("Low", "", &[("DISCR_EXACT", "5")]),
        ("Wrap", "1", &[("DISCR_BEGIN", "250"), ("DISCR_END", "2")]),
    ];
    let wrapped = pdb_enum(&mut src, "w", 74, "1", wrapping);
    assert_eq!(show(&wrapped, &src).as_deref(), Some("Wrap(1)"));
    // No variant owns the tag: nothing is guessed.
    let lost = pdb_enum(&mut src, "l", 75, "100", wrapping);
    assert_eq!(show(&lost, &src), None);
}

#[test]
fn a_char_and_an_i8_print_as_debug_not_as_lldb_s_text() {
    let src = Fake::default();
    // Measured on 30/09/2026, both toolchains.
    let ch = v("ch", "U+007A U+007A U'z'", "char32_t", 0);
    assert_eq!(show(&ch, &src).as_deref(), Some("'z'"));
    // Measured on 01/10/2026 on Linux, LLDB 22: the code point comes in hex.
    let linux = v("ch", "U+0x0000007a U+0x0000007a U'z'", "char32_t", 0);
    assert_eq!(show(&linux, &src).as_deref(), Some("'z'"));
    let from_dwarf = v("b8", "'\\xff'", "char", 0);
    assert_eq!(show(&from_dwarf, &src).as_deref(), Some("-1"));
    let from_pdb = v("b8", "'\\xff'", "signed char", 0);
    assert_eq!(show(&from_pdb, &src).as_deref(), Some("-1"));
    let letter = v("b8", "'a'", "char", 0);
    assert_eq!(show(&letter, &src).as_deref(), Some("97"));
    let newline = v("b8", "'\\n'", "char", 0);
    assert_eq!(show(&newline, &src).as_deref(), Some("10"));
    // Anything else is the adapter's.
    let number = v("n", "7", "int", 0);
    assert_eq!(show(&number, &src), None);
}

#[test]
fn a_tuple_prints_without_a_name_in_both_spellings() {
    let mut src = Fake::default();
    src.children.insert(
        80,
        vec![v("__0", "1", "unsigned int", 0), v("__1", "-2", "int", 0)],
    );
    let dwarf = v("pair", "(u32, i32)", "(u32, i32)", 80);
    assert_eq!(show(&dwarf, &src).as_deref(), Some("(1, -2)"));
    let pdb = v("pair", "tuple$<u32,i32>", "tuple$<u32,i32>", 80);
    assert_eq!(show(&pdb, &src).as_deref(), Some("(1, -2)"));
}

#[test]
fn a_local_the_adapter_cannot_read_is_not_walked() {
    // The `args` a `println!` leaves in scope, measured on `-msvc`: walking it
    // printed a wall of `<error: parent failed to evaluate …>`.
    let mut src = Fake::default();
    src.children.insert(
        81,
        vec![v(
            "__0",
            "<error: parent failed to evaluate: variable not available>",
            "int",
            0,
        )],
    );
    let args = v(
        "args",
        "<error: variable not available>",
        "tuple$<ref$<int> >",
        81,
    );
    assert_eq!(show(&args, &src), None);
}

#[test]
fn a_bare_char_is_rust_s_in_a_pdb_name_and_an_i8_in_a_dwarf_one() {
    use crate::types::{in_rust_spelling, spelled_by_pdb};
    // Measured on 30/09/2026: `Vec<char>` is `Vec<char,alloc::alloc::Global>`
    // from a PDB, `Vec<char32_t, alloc::alloc::Global>` from DWARF, where
    // `Vec<i8>` is `Vec<char, alloc::alloc::Global>`.
    assert!(spelled_by_pdb("alloc::vec::Vec<char,alloc::alloc::Global>"));
    assert!(spelled_by_pdb("enum2$<core::option::Option<i32> >"));
    assert!(!spelled_by_pdb(
        "alloc::vec::Vec<char, alloc::alloc::Global>"
    ));
    assert_eq!(
        in_rust_spelling("char", "alloc::vec::Vec<char,alloc::alloc::Global>"),
        "char"
    );
    assert_eq!(
        in_rust_spelling("char", "alloc::vec::Vec<char, alloc::alloc::Global>"),
        "i8"
    );
    let mut src = Fake::default();
    src.evals.insert("n.len".into(), "1".into());
    src.evals
        .insert("n.buf.inner.ptr.pointer.pointer".into(), "0x2000".into());
    src.memory
        .insert("0x2000".into(), 97u32.to_le_bytes().to_vec());
    let pdb = named(
        v("n", "", "alloc::vec::Vec<char,alloc::alloc::Global>", 82),
        "n",
    );
    assert_eq!(show(&pdb, &src).as_deref(), Some("['a']"));
}

#[test]
fn a_slice_reference_reads_its_elements_in_either_encoding() {
    // `-msvc` spells it `ref$<slice2$<T> >`, `-gnu` spells it `&[T]`; both
    // give `data_ptr` and `length` (a handler's `actions`, 30/09/2026).
    for ty in [
        "ref$<slice2$<i32> >",
        "&[i32]",
        "ref_mut$<slice2$<i32> >",
        "&mut [i32]",
    ] {
        let mut src = Fake::default();
        src.children.insert(
            4,
            vec![
                v("data_ptr", "0x000002b8492920e0", "i32 *", 5),
                v("length", "3", "unsigned long long", 0),
            ],
        );
        let bytes: Vec<u8> = [7i32, 8, 9].iter().flat_map(|n| n.to_le_bytes()).collect();
        src.memory.insert("0x2b8492920e0".into(), bytes);
        let actions = named(
            v(
                "actions",
                "ref$ { data_ptr: 0x000002b8492920e0, length: 3 }",
                ty,
                4,
            ),
            "actions",
        );
        assert_eq!(show(&actions, &src).as_deref(), Some("[7, 8, 9]"), "{ty}");
    }
}

#[test]
fn a_slice_of_composites_without_a_layout_says_how_many() {
    let mut src = Fake::default();
    src.children.insert(
        4,
        vec![
            v("data_ptr", "0x000002b8492920e0", "makepad::Action *", 5),
            v("length", "1", "unsigned long long", 0),
        ],
    );
    let actions = named(
        v(
            "actions",
            "ref$ { … }",
            "ref$<slice2$<alloc::boxed::Box<dyn$<makepad::ActionTrait> > > >",
            4,
        ),
        "actions",
    );
    assert_eq!(show(&actions, &src).as_deref(), Some("[… 1 item]"));
}

/// **A niche `None` is `None` when the variants are numbered by position.**
///
/// The tree LLDB 22 gave on Linux on 01/10/2026: `$variant$0` is `None`, and
/// its `$discr$` is the value stored — the niche `9223372036854775808`, which
/// never equals the `0` in the name. Without the program's DWARF that read
/// `Some(alloc::string::String)`; with it, the arm that owns the stored value
/// is picked.
#[test]
fn a_niche_none_is_read_from_the_dwarf_arms() {
    use crate::dwarf::{Arm, Kind, Member, Type, TypeIndex, Variants};
    const OPT: &str = "core::option::Option<alloc::string::String>";
    let niche = 1u64 << 63;
    let arm = |name: &str, discr_value| Arm {
        discr_value,
        member: Member {
            name: name.to_string(),
            offset: 0,
            ty: None,
        },
    };
    let index = TypeIndex::from_types(vec![Type {
        name: OPT.to_string(),
        size: 24,
        kind: Kind::Struct {
            members: Vec::new(),
            variants: Some(Variants {
                discr: None,
                arms: vec![arm("None", Some(niche)), arm("Some", None)],
            }),
        },
    }]);
    let mut src = Fake::default();
    src.children.insert(10, vec![v("$variants$", "", OPT, 11)]);
    src.children.insert(
        11,
        vec![v("$variant$0", "", "", 12), v("$variant$", "", "", 13)],
    );
    src.children.insert(
        12,
        vec![
            v("$discr$", &niche.to_string(), "unsigned long", 0),
            v(
                "value",
                "",
                &format!("{OPT}::None<alloc::string::String>"),
                0,
            ),
        ],
    );
    src.children.insert(
        13,
        vec![v(
            "value",
            "",
            &format!("{OPT}::Some<alloc::string::String>"),
            14,
        )],
    );
    src.children.insert(
        14,
        vec![v(
            "__0",
            "alloc::string::String",
            "alloc::string::String",
            0,
        )],
    );
    let none = v("none", OPT, OPT, 10);

    let with_layouts = RustPresenter {
        index: Some(std::sync::Arc::new(index)),
    };
    assert_eq!(with_layouts.present(&none, &src).as_deref(), Some("None"));
}
