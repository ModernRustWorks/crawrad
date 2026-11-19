//! Unit tests for the documentation note headless model.

use super::*;
use std::fs;

#[test]
fn cursor_positions_fn_with_doc_and_attribute() {
    let code = "impl Foo {\n    /// Computes the value.\n    #[inline]\n    fn foo(\n        x: i32,\n    ) -> i32 {\n        let y = x + 1;\n        y\n    }\n}";

    // Cursor on impl line -> None
    assert!(doc_target_at(code, 0).is_none());

    // Cursor on doc line (1) -> Some
    let target_doc = doc_target_at(code, 1).expect("doc line documents foo");
    assert_eq!(target_doc.kind, ItemKind::Fn);
    assert_eq!(target_doc.name, "foo");
    assert_eq!(target_doc.owner.as_deref(), Some("Foo"));
    assert_eq!(target_doc.item_line, 3);
    assert_eq!(target_doc.insert_line, 1);
    assert_eq!(target_doc.indent, "    ");
    assert_eq!(target_doc.doc_lines, 1..2);
    assert_eq!(target_doc.existing_docs, "Computes the value.");
    assert_eq!(target_doc.existing_note, None);
    assert_eq!(target_doc.body, Some(5..9));

    // Cursor on attribute line (2) -> Some
    let target_attr = doc_target_at(code, 2).expect("attribute line documents foo");
    assert_eq!(target_attr.name, "foo");

    // Cursor on decl line (3) -> Some
    let target_decl = doc_target_at(code, 3).expect("decl line documents foo");
    assert_eq!(target_decl.name, "foo");

    // Cursor on signature continuation lines (4 and 5) -> Some
    let target_cont1 = doc_target_at(code, 4).expect("continuation line 4 documents foo");
    assert_eq!(target_cont1.name, "foo");
    let target_cont2 = doc_target_at(code, 5).expect("continuation line 5 documents foo");
    assert_eq!(target_cont2.name, "foo");

    // Cursor on body lines (6, 7, 8) -> None
    assert!(doc_target_at(code, 6).is_none());
    assert!(doc_target_at(code, 7).is_none());
    assert!(doc_target_at(code, 8).is_none());

    // Cursor on impl closing brace (9) -> None
    assert!(doc_target_at(code, 9).is_none());
}

#[test]
fn owner_in_generic_impl_and_trait_impl() {
    let generic_code = "impl<T> Foo<T> {\n    fn bar() {}\n}";
    let target1 = doc_target_at(generic_code, 1).expect("documents bar");
    assert_eq!(target1.owner.as_deref(), Some("Foo"));

    let trait_impl_code = "impl Display for Foo {\n    fn fmt(&self) {}\n}";
    let target2 = doc_target_at(trait_impl_code, 1).expect("documents fmt");
    assert_eq!(target2.owner.as_deref(), Some("Foo"));
}

#[test]
fn trait_method_owner_and_body() {
    let code = "trait Bar {\n    fn work(&self);\n}";
    let target = doc_target_at(code, 1).expect("documents work");
    assert_eq!(target.kind, ItemKind::Fn);
    assert_eq!(target.name, "work");
    assert_eq!(target.owner.as_deref(), Some("Bar"));
    assert_eq!(target.body, None);
}

#[test]
fn item_declarations_recognized() {
    let source = "struct MyStruct;\nenum MyEnum { A }\nconst MAX_VAL: usize = 10;\nstatic mut COUNTER: u32 = 0;\ntype MyAlias = u64;\nmod tests;\nunion MyUnion { a: u32 }";

    let t_struct = doc_target_at(source, 0).expect("recognizes struct");
    assert_eq!(t_struct.kind, ItemKind::Struct);
    assert_eq!(t_struct.name, "MyStruct");

    let t_enum = doc_target_at(source, 1).expect("recognizes enum");
    assert_eq!(t_enum.kind, ItemKind::Enum);
    assert_eq!(t_enum.name, "MyEnum");

    let t_const = doc_target_at(source, 2).expect("recognizes const");
    assert_eq!(t_const.kind, ItemKind::Const);
    assert_eq!(t_const.name, "MAX_VAL");

    let t_static = doc_target_at(source, 3).expect("recognizes static mut");
    assert_eq!(t_static.kind, ItemKind::Static);
    assert_eq!(t_static.name, "COUNTER");

    let t_type = doc_target_at(source, 4).expect("recognizes type alias");
    assert_eq!(t_type.kind, ItemKind::TypeAlias);
    assert_eq!(t_type.name, "MyAlias");

    let t_mod = doc_target_at(source, 5).expect("recognizes mod");
    assert_eq!(t_mod.kind, ItemKind::Mod);
    assert_eq!(t_mod.name, "tests");

    let t_union = doc_target_at(source, 6).expect("recognizes union");
    assert_eq!(t_union.kind, ItemKind::Union);
    assert_eq!(t_union.name, "MyUnion");
}

#[test]
fn pub_crate_const_unsafe_fn() {
    let code = "pub(crate) const unsafe fn risky_calc() -> usize { 0 }";
    let target = doc_target_at(code, 0).expect("documents risky_calc");
    assert_eq!(target.kind, ItemKind::Fn);
    assert_eq!(target.name, "risky_calc");
    assert!(target.is_unsafe);

    let suggestions = suggested_sections(code, &target);
    assert_eq!(suggestions.len(), 1);
    assert_eq!(suggestions[0].section, Section::Safety);
    assert!(suggestions[0].required);
    assert_eq!(suggestions[0].reason, "unsafe fn");
}

#[test]
fn multi_line_signature_and_returns_io_result() {
    let code = "pub fn fetch(\n    &self,\n    id: &str,\n) -> io::Result<()> {\n    Ok(())\n}";
    let target = doc_target_at(code, 0).expect("documents fetch");
    assert_eq!(
        target.signature,
        "pub fn fetch( &self, id: &str, ) -> io::Result<()>"
    );
    assert!(target.returns_result);

    let suggestions = suggested_sections(code, &target);
    assert!(
        suggestions
            .iter()
            .any(|s| s.section == Section::Errors && s.reason == "returns Result")
    );
}

#[test]
fn body_with_unwrap_in_comment_only_and_real_panic() {
    let safe_code = "fn safe_check() {\n    // let val = opt.unwrap();\n    // panic!(\"boom\");\n    let x = 42;\n}";
    let safe_target = doc_target_at(safe_code, 0).expect("documents safe_check");
    let safe_suggestions = suggested_sections(safe_code, &safe_target);
    assert!(
        safe_suggestions
            .iter()
            .all(|s| s.section != Section::Panics)
    );

    let panic_code = "fn panic_check() {\n    let a = 1;\n    let b = opt.unwrap();\n}";
    let panic_target = doc_target_at(panic_code, 0).expect("documents panic_check");
    let panic_suggestions = suggested_sections(panic_code, &panic_target);
    let panic_sugg = panic_suggestions
        .iter()
        .find(|s| s.section == Section::Panics)
        .expect("suggests Panics");
    assert_eq!(panic_sugg.reason, "calls unwrap() on line 3");
}

#[test]
fn string_literal_braces_do_not_break_brace_matching() {
    let code = "fn string_braces() {\n    let a = \"}\";\n    let b = \"{\";\n    let c = 123;\n}";
    let target = doc_target_at(code, 0).expect("documents string_braces");
    assert_eq!(target.body, Some(0..5));
}

#[test]
fn suggested_note_id_combinations() {
    let target1 = DocTarget {
        kind: ItemKind::Fn,
        name: "fetch_all".to_string(),
        owner: Some("RegistryClient".to_string()),
        signature: String::new(),
        item_line: 0,
        insert_line: 0,
        indent: String::new(),
        doc_lines: 0..0,
        existing_docs: String::new(),
        existing_note: None,
        body: None,
        is_unsafe: false,
        returns_result: false,
    };
    assert_eq!(suggested_note_id(&target1), "registry-client-fetch-all");

    let target2 = DocTarget {
        kind: ItemKind::Fn,
        name: "fetch".to_string(),
        owner: None,
        signature: String::new(),
        item_line: 0,
        insert_line: 0,
        indent: String::new(),
        doc_lines: 0..0,
        existing_docs: String::new(),
        existing_note: None,
        body: None,
        is_unsafe: false,
        returns_result: false,
    };
    assert_eq!(suggested_note_id(&target2), "fetch");
}

#[test]
fn apply_note_attribute_moves_docs_keeps_indent_and_crlf() {
    let source_lf = "    /// First line\n    /// Second line\n    #[inline]\n    fn run() {}\n";
    let target_lf = doc_target_at(source_lf, 0).expect("documents run");
    let updated_lf = apply_note_attribute(source_lf, &target_lf, "run-note");
    assert_eq!(
        updated_lf,
        "    #[crawrad_note::note(\"run-note\")]\n    #[inline]\n    fn run() {}\n"
    );

    // CRLF preservation
    let source_crlf = "    /// Line\r\n    fn run() {}\r\n";
    let target_crlf = doc_target_at(source_crlf, 0).expect("documents run");
    let updated_crlf = apply_note_attribute(source_crlf, &target_crlf, "run-note");
    assert_eq!(
        updated_crlf,
        "    #[crawrad_note::note(\"run-note\")]\r\n    fn run() {}\r\n"
    );

    // Unchanged when existing_note is Some
    let source_existing = "    #[crawrad_note::note(\"existing\")]\n    fn run() {}\n";
    let target_existing = doc_target_at(source_existing, 0).expect("documents run");
    assert_eq!(target_existing.existing_note.as_deref(), Some("existing"));
    let unchanged = apply_note_attribute(source_existing, &target_existing, "new-id");
    assert_eq!(unchanged, source_existing);
}

#[test]
fn crate_dir_of_with_temp_dir() {
    let unique_name = format!(
        "crawrad_test_doc_note_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let temp_dir = std::env::temp_dir().join(unique_name);
    let src_dir = temp_dir.join("src");
    fs::create_dir_all(&src_dir).expect("create src dir");

    let cargo_toml = temp_dir.join("Cargo.toml");
    fs::write(
        &cargo_toml,
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    )
    .expect("write Cargo.toml");

    let lib_rs = src_dir.join("lib.rs");
    fs::write(&lib_rs, "pub fn demo() {}\n").expect("write lib.rs");

    let found = crate_dir_of(&lib_rs);
    assert_eq!(found, Some(temp_dir.clone()));

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn depends_on_crawrad_note_all_forms_and_absent() {
    let plain_key = "[package]\nname = \"demo\"\n\n[dependencies]\ncrawrad-note = \"0.1\"\n";
    assert!(depends_on_crawrad_note(plain_key));

    let workspace_form =
        "[package]\nname = \"demo\"\n\n[dependencies]\ncrawrad-note.workspace = true\n";
    assert!(depends_on_crawrad_note(workspace_form));

    let inline_table = "[package]\nname = \"demo\"\n\n[dependencies]\ncrawrad-note = { version = \"0.1\", path = \"../crawrad-note\" }\n";
    assert!(depends_on_crawrad_note(inline_table));

    let table_form =
        "[package]\nname = \"demo\"\n\n[dependencies.crawrad-note]\nversion = \"0.1\"\n";
    assert!(depends_on_crawrad_note(table_form));

    let absent = "[package]\nname = \"demo\"\n\n[dependencies]\nserde = \"1.0\"\n";
    assert!(!depends_on_crawrad_note(absent));

    let dev_only = "[package]\nname = \"demo\"\n\n[dev-dependencies]\ncrawrad-note = \"0.1\"\n";
    assert!(!depends_on_crawrad_note(dev_only));
}

#[test]
fn array_type_semicolon_does_not_end_the_signature() {
    let src = "pub fn read(n: usize) -> Result<[u8; 4], Error> {\n    todo!()\n}\n";
    let t = doc_target_at(src, 0).expect("target");
    assert_eq!(
        t.signature,
        "pub fn read(n: usize) -> Result<[u8; 4], Error>"
    );
    assert!(t.returns_result);
    assert_eq!(t.body, Some(0..3));
}

#[test]
fn attribute_between_doc_lines_survives_the_move() {
    let src = "/// First.\n#[inline]\n/// Second.\nfn f() {}\n";
    let t = doc_target_at(src, 3).expect("target");
    let out = apply_note_attribute(src, &t, "f");
    assert_eq!(out, "#[crawrad_note::note(\"f\")]\n#[inline]\nfn f() {}\n");
}

#[test]
fn note_alert_reports_clean_missing_and_stale_docs() {
    let src = "#[crawrad_note::note(\"my-fn\")]\npub fn my_fn() -> i32 {\n    42\n}\n";

    let clean_content =
        "Computes forty-two.\n\n# Examples\n\n```rust\nassert_eq!(my_fn(), 42);\n```\n";
    let clean_note = SidecarNote {
        id: "my-fn".to_string(),
        kind: NoteKind::Doc,
        title: "my_fn".to_string(),
        target_file: "src/lib.rs".to_string(),
        line_hint: 1,
        signature: "pub fn my_fn() -> i32".to_string(),
        content: clean_content.to_string(),
        ..Default::default()
    };
    // Clean doc -> false
    assert!(!note_alert(src, 0, &clean_note));

    // Doc missing # Examples -> true
    let missing_examples_note = SidecarNote {
        content: "Computes forty-two without examples.\n".to_string(),
        ..clean_note.clone()
    };
    assert!(note_alert(src, 0, &missing_examples_note));

    // Doc whose saved signature differs -> true
    let stale_sig_note = SidecarNote {
        signature: "pub fn my_fn(x: i32) -> i32".to_string(),
        ..clean_note.clone()
    };
    assert!(note_alert(src, 0, &stale_sig_note));

    // Internal note -> false
    let internal_note = SidecarNote {
        kind: NoteKind::Internal,
        content: "Just an internal note without examples.\n".to_string(),
        signature: "pub fn different()".to_string(),
        ..clean_note.clone()
    };
    assert!(!note_alert(src, 0, &internal_note));

    // Comment -> false
    let comment_note = SidecarNote {
        kind: NoteKind::Comment,
        content: "Just a comment.\n".to_string(),
        signature: "pub fn different()".to_string(),
        ..clean_note
    };
    assert!(!note_alert(src, 0, &comment_note));
}

#[test]
fn doc_skeleton_summaries_all_cases() {
    let make_target = |kind: ItemKind, name: &str, owner: Option<&str>| DocTarget {
        kind,
        name: name.to_string(),
        owner: owner.map(str::to_string),
        signature: format!("pub fn {name}()"),
        item_line: 0,
        insert_line: 0,
        indent: String::new(),
        doc_lines: 0..0,
        existing_docs: String::new(),
        existing_note: None,
        body: None,
        is_unsafe: false,
        returns_result: false,
    };

    // fn/method summaries
    assert_eq!(
        doc_skeleton("", &make_target(ItemKind::Fn, "parse_date", None), None),
        "Parses date.\n\n# Examples\n\n```no_run\nparse_date();\n```"
    );
    assert_eq!(
        summary_for_target(&make_target(ItemKind::Fn, "is_empty", None)),
        "Returns whether empty."
    );
    assert_eq!(
        summary_for_target(&make_target(ItemKind::Fn, "has_note", None)),
        "Returns whether it has note."
    );
    assert_eq!(
        summary_for_target(&make_target(ItemKind::Fn, "new", Some("Foo"))),
        "Creates a new `Foo`."
    );
    assert_eq!(
        summary_for_target(&make_target(ItemKind::Fn, "new", None)),
        "Creates a new value."
    );
    assert_eq!(
        summary_for_target(&make_target(ItemKind::Fn, "to_string", None)),
        "Converts to string."
    );
    assert_eq!(
        summary_for_target(&make_target(ItemKind::Fn, "get_name", None)),
        "Returns the name."
    );
    assert_eq!(
        summary_for_target(&make_target(ItemKind::Fn, "name", None)),
        "Returns the name."
    );
    assert_eq!(
        summary_for_target(&make_target(ItemKind::Fn, "try_parse", None)),
        "Parses (fails instead of panicking)."
    );
    assert_eq!(
        summary_for_target(&make_target(ItemKind::Fn, "fetch", None)),
        "Fetches."
    );
    assert_eq!(
        summary_for_target(&make_target(ItemKind::Fn, "copy", None)),
        "Copies."
    );
    assert_eq!(
        summary_for_target(&make_target(ItemKind::Fn, "push", None)),
        "Pushes."
    );
    assert_eq!(
        summary_for_target(&make_target(ItemKind::Fn, "go", None)),
        "Goes."
    );

    // struct, enum, const
    let struct_target = DocTarget {
        kind: ItemKind::Struct,
        name: "NoteKind".to_string(),
        owner: None,
        signature: "pub struct NoteKind;".to_string(),
        item_line: 0,
        insert_line: 0,
        indent: String::new(),
        doc_lines: 0..0,
        existing_docs: String::new(),
        existing_note: None,
        body: None,
        is_unsafe: false,
        returns_result: false,
    };
    assert_eq!(doc_skeleton("", &struct_target, None), "A note kind.");

    let enum_target = DocTarget {
        kind: ItemKind::Enum,
        name: "Item".to_string(),
        owner: None,
        signature: "pub enum Item {}".to_string(),
        item_line: 0,
        insert_line: 0,
        indent: String::new(),
        doc_lines: 0..0,
        existing_docs: String::new(),
        existing_note: None,
        body: None,
        is_unsafe: false,
        returns_result: false,
    };
    assert_eq!(doc_skeleton("", &enum_target, None), "An item.");

    let const_target = DocTarget {
        kind: ItemKind::Const,
        name: "MAX_LINES".to_string(),
        owner: None,
        signature: "pub const MAX_LINES: usize = 100;".to_string(),
        item_line: 0,
        insert_line: 0,
        indent: String::new(),
        doc_lines: 0..0,
        existing_docs: String::new(),
        existing_note: None,
        body: None,
        is_unsafe: false,
        returns_result: false,
    };
    assert_eq!(doc_skeleton("", &const_target, None), "The max lines.");
}

#[test]
fn doc_skeleton_fn_cases_and_missing_sections() {
    let check_fn_skeleton = |src: &str, line: usize, crate_name: Option<&str>| {
        let target = doc_target_at(src, line).expect("target at line");
        let skeleton = doc_skeleton(src, &target, crate_name);
        assert!(
            !skeleton.ends_with('\n'),
            "skeleton must have no trailing newline"
        );
        let suggested = suggested_sections(src, &target);
        let missing = missing_sections(&skeleton, &suggested);
        assert!(
            missing.is_empty(),
            "expected no missing sections for `{}` in {src}, got {missing:?}",
            target.name
        );
        (target, skeleton)
    };

    // 1. Free fn with two params and Result return (Errors yes)
    let src1 =
        "pub fn parse_date(s: &str, format: &str) -> Result<Date, ParseError> {\n    Ok(Date)\n}\n";
    let (_, skel1) = check_fn_skeleton(src1, 0, Some("my-crate"));
    assert!(skel1.starts_with("Parses date."));
    assert!(skel1.contains("use my_crate::parse_date;"));
    assert!(skel1.contains("let s: &str = todo!();"));
    assert!(skel1.contains("let format: &str = todo!();"));
    assert!(skel1.contains("let result = parse_date(s, format);"));
    assert!(skel1.contains("# Errors\n\nReturns an error when \u{2026}"));

    // 2. &self method
    let src2 = "impl Client {\n    pub fn name(&self) -> &str {\n        \"client\"\n    }\n}\n";
    let (_, skel2) = check_fn_skeleton(src2, 1, Some("my-crate"));
    assert!(skel2.starts_with("Returns the name."));
    assert!(skel2.contains("use my_crate::Client;"));
    assert!(skel2.contains("let value: Client = todo!();"));
    assert!(skel2.contains("let result = value.name();"));

    // 3. &mut self method
    let src3 = "impl Buffer {\n    pub fn clear(&mut self) {\n    }\n}\n";
    let (_, skel3) = check_fn_skeleton(src3, 1, Some("my-crate"));
    assert!(skel3.starts_with("Clears."));
    assert!(skel3.contains("use my_crate::Buffer;"));
    assert!(skel3.contains("let mut value: Buffer = todo!();"));
    assert!(skel3.contains("value.clear();"));

    // 4. Associated fn without self
    let src4 = "impl Widget {\n    pub fn create(id: u32) -> Self {\n        Widget\n    }\n}\n";
    let (_, skel4) = check_fn_skeleton(src4, 1, Some("my-crate"));
    assert!(skel4.contains("use my_crate::Widget;"));
    assert!(skel4.contains("let id: u32 = todo!();"));
    assert!(skel4.contains("let result = Widget::create(id);"));

    // 5. Unsafe fn (Safety yes)
    let src5 = "pub unsafe fn deref_ptr(ptr: *const u8) -> u8 {\n    *ptr\n}\n";
    let (_, skel5) = check_fn_skeleton(src5, 0, Some("my-crate"));
    assert!(skel5.contains("# Safety\n\nThe caller must ensure \u{2026}"));
    assert!(skel5.contains("let result = deref_ptr(ptr);"));

    // 6. Fn with panic in body (Panics yes)
    let src6 =
        "pub fn lookup(idx: usize) -> u32 {\n    let v: Option<u32> = None;\n    v.unwrap()\n}\n";
    let (_, skel6) = check_fn_skeleton(src6, 0, Some("my-crate"));
    assert!(skel6.contains("# Panics\n\nPanics when \u{2026}"));

    // 7. Lifetimes and generic params
    let src7 = "impl<T> Container<T> {\n    pub fn put<'a, U: Display>(&mut self, item: T, label: &'a str, extra: U) {\n    }\n}\n";
    let (_, skel7) = check_fn_skeleton(src7, 1, Some("my-crate"));
    assert!(skel7.contains("let item: _ = todo!();"));
    assert!(skel7.contains("let label: &str = todo!();"));
    assert!(skel7.contains("let extra: _ = todo!();"));
    assert!(skel7.contains("value.put(item, label, extra);"));

    // 8. Unit fn
    let src8 = "pub fn run() -> () {\n}\n";
    let (_, skel8) = check_fn_skeleton(src8, 0, Some("my-crate"));
    assert!(skel8.contains("run();"));
    assert!(!skel8.contains("let result ="));

    // 9. Use line only with a crate name
    let (_, skel_no_crate) = check_fn_skeleton(src1, 0, None);
    assert!(!skel_no_crate.contains("use "));
    assert!(skel1.contains("use my_crate::parse_date;"));

    // 10. Pattern parameter (not plain identifier)
    let src10 = "pub fn process((a, b): (u8, u8), mut count: usize) {\n}\n";
    let (_, skel10) = check_fn_skeleton(src10, 0, None);
    assert!(skel10.contains("let arg1: (u8, u8) = todo!();"));
    assert!(skel10.contains("let count: usize = todo!();"));
    assert!(skel10.contains("process(arg1, count);"));
}

#[test]
fn note_mark_hides_resolved_comment_and_shows_others() {
    let tmp = std::env::temp_dir().join(format!(
        "crawrad_test_note_mark_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&tmp);

    let resolved_comment = SidecarNote {
        id: "resolved-cmt".to_string(),
        kind: NoteKind::Comment,
        resolved: true,
        title: "Comment Title".to_string(),
        content: "Some comment".to_string(),
        ..Default::default()
    };
    crate::sidecar_notes::save_note(&tmp, &resolved_comment).expect("save resolved comment");

    let open_comment = SidecarNote {
        id: "open-cmt".to_string(),
        kind: NoteKind::Comment,
        resolved: false,
        title: "Open Comment".to_string(),
        content: "Open remark".to_string(),
        ..Default::default()
    };
    crate::sidecar_notes::save_note(&tmp, &open_comment).expect("save open comment");

    let doc_note = SidecarNote {
        id: "doc-sample".to_string(),
        kind: NoteKind::Doc,
        resolved: true,
        title: "Doc Sample".to_string(),
        content: "Documentation text".to_string(),
        ..Default::default()
    };
    crate::sidecar_notes::save_note(&tmp, &doc_note).expect("save doc note");

    let internal_note = SidecarNote {
        id: "internal-sample".to_string(),
        kind: NoteKind::Internal,
        resolved: true,
        title: "Internal Sample".to_string(),
        content: "Internal text".to_string(),
        ..Default::default()
    };
    crate::sidecar_notes::save_note(&tmp, &internal_note).expect("save internal note");

    let source = "fn dummy() {}\n";

    // 1. Resolved comment is hidden
    let (prev_res, alert_res, hidden_res) = note_mark(&tmp, "resolved-cmt", source, 0);
    assert_eq!(prev_res, "Comment Title\nSome comment");
    assert!(!alert_res);
    assert!(hidden_res);

    // 2. Open comment is shown (not hidden)
    let (prev_open, alert_open, hidden_open) = note_mark(&tmp, "open-cmt", source, 0);
    assert_eq!(prev_open, "Open Comment\nOpen remark");
    assert!(!alert_open);
    assert!(!hidden_open);

    // 3. Doc note is never hidden
    let (prev_doc, _, hidden_doc) = note_mark(&tmp, "doc-sample", source, 0);
    assert_eq!(prev_doc, "Doc Sample\nDocumentation text");
    assert!(!hidden_doc);

    // 4. Internal note is never hidden
    let (prev_int, _, hidden_int) = note_mark(&tmp, "internal-sample", source, 0);
    assert_eq!(prev_int, "Internal Sample\nInternal text");
    assert!(!hidden_int);

    // 5. Unreadable / missing note returns empty preview, not in alert, not hidden
    let (prev_missing, alert_missing, hidden_missing) = note_mark(&tmp, "non-existent", source, 0);
    assert_eq!(prev_missing, "");
    assert!(!alert_missing);
    assert!(!hidden_missing);

    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn the_crawrad_note_dependency_is_added_once_as_a_path() {
    let manifest = "[package]\nname = \"demo\"\n\n[dependencies]\nserde = \"1\"\n";
    let dir = Path::new("/work/demo");
    let note = Path::new("/work/crawrad/crates/crawrad-note");
    let updated = with_crawrad_note_dependency(manifest, dir, note).expect("added");
    assert!(
        updated.contains("crawrad-note = { path = \"../crawrad/crates/crawrad-note\" }"),
        "{updated}"
    );
    assert!(updated.contains("serde = \"1\""));
    assert!(depends_on_crawrad_note(&updated));
    assert_eq!(
        with_crawrad_note_dependency(&updated, dir, note),
        None,
        "already there"
    );

    let bare = "[package]\nname = \"demo\"\n";
    let with_table = with_crawrad_note_dependency(bare, dir, note).expect("added");
    assert!(depends_on_crawrad_note(&with_table), "{with_table}");
    assert_eq!(with_crawrad_note_dependency("not = [toml", dir, note), None);
}
