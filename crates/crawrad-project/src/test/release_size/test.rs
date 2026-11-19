use super::*;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[test]
fn parse_msvc_map_from_fixture() {
    let fixture = include_str!("sample.map");
    let symbols = parse_msvc_map(fixture);

    // Main symbol: size 0x5b0 - 0x30 = 1408
    let main_sym = symbols
        .iter()
        .find(|s| s.name == "_ZN7mapdemo4main17hbf213f186cba126fE")
        .expect("main symbol found");
    assert_eq!(main_sym.size, 1408);
    assert_eq!(main_sym.library, "");

    // Acrt symbols at 0001:000070d0: first dropped (size 0), second kept
    let acrt_init = symbols.iter().find(|s| s.name == "__acrt_initialize");
    assert!(acrt_init.is_none());
    let acrt_uninit = symbols
        .iter()
        .find(|s| s.name == "__acrt_uninitialize")
        .expect("acrt uninitialize symbol found");
    assert_eq!(acrt_uninit.size, 0x7620 - 0x70d0);

    // Section 0003 last symbol runs to section end (0x2d0 - 0x278 = 88)
    let last_sec3 = symbols
        .iter()
        .find(|s| s.name == "?module_local_at_quick_exit_table@@3U_onexit_table_t@@A")
        .expect("last symbol of section 3 found");
    assert_eq!(last_sec3.size, 88);

    // Section 0000 and fffde000 symbols skipped
    assert!(!symbols.iter().any(|s| s.name == "__AbsoluteZero"));
    assert!(!symbols.iter().any(|s| s.name == ".debug$S"));

    // libstd row library extracted
    let std_sym = symbols
        .iter()
        .find(|s| s.name.contains("___rust_end_short_backtrace"))
        .expect("std backtrace symbol found");
    assert_eq!(std_sym.library, "libstd-0cebe7c42cd80226");
}

#[test]
fn crate_of_various_symbols() {
    let main_sym = Symbol {
        name: "_ZN7mapdemo4main17hbf213f186cba126fE".to_string(),
        size: 1408,
        library: String::new(),
    };
    assert_eq!(crate_of(&main_sym), "mapdemo");

    let std_sym = Symbol {
        name: "_RNvNtCs1aoCESei0z2_3std5alloc8rust_oom".to_string(),
        size: 100,
        library: "libstd-0cebe7c42cd80226".to_string(),
    };
    assert_eq!(crate_of(&std_sym), "std");

    let core_sym = Symbol {
        name: "_RNvMs4_NtNtCs8icZUrvfcq9_4core3fmt8buildersNtB5_8DebugSet5entry".to_string(),
        size: 50,
        library: "libcore-7c4a4c66f69609a4".to_string(),
    };
    assert_eq!(crate_of(&core_sym), "std");

    let demangle_sym = Symbol {
        name: "_RNvCs5Y22KTP4mgF_14rustc_demangle8demangle".to_string(),
        size: 80,
        library: "librustc_demangle-eda3326d6d941958".to_string(),
    };
    assert_eq!(crate_of(&demangle_sym), "rustc_demangle");

    let sys_sym = Symbol {
        name: "__imp_GetLastError".to_string(),
        size: 8,
        library: "kernel32".to_string(),
    };
    assert_eq!(crate_of(&sys_sym), "[system]");

    let rust_alloc_sym = Symbol {
        name: "_RNvCsfLfy6EI15iL_7___rustc12___rust_alloc".to_string(),
        size: 32,
        library: String::new(),
    };
    assert_eq!(crate_of(&rust_alloc_sym), "std");

    // Handwritten legacy trait symbol
    let trait_sym_raw = Symbol {
        name: "<order_desk::Order as core::clone::Clone>::clone".to_string(),
        size: 16,
        library: String::new(),
    };
    assert_eq!(crate_of(&trait_sym_raw), "order_desk");

    let trait_sym_mangled = Symbol {
        name:
            "_ZN55$LT$order_desk..Order$u20$as$u20$core..clone..Clone$GT$5clone17h0123456789abcdefE"
                .to_string(),
        size: 16,
        library: String::new(),
    };
    assert_eq!(crate_of(&trait_sym_mangled), "order_desk");
}

#[test]
fn by_crate_sums_match_symbols_total() {
    let fixture = include_str!("sample.map");
    let symbols = parse_msvc_map(fixture);
    let crates = by_crate(&symbols);

    let total_symbols: u64 = symbols.iter().map(|s| s.size).sum();
    let total_crates: u64 = crates.values().sum();
    assert_eq!(total_symbols, total_crates);
    assert!(crates.contains_key("mapdemo"));
    assert!(crates.contains_key("std"));
    assert!(crates.contains_key("rustc_demangle"));
}

#[test]
fn symbols_from_binary_behavior() {
    let garbage = b"not a valid binary header at all";
    assert!(symbols_from_binary(garbage).is_err());

    #[cfg(windows)]
    {
        let exe = std::env::current_exe().expect("current exe");
        let bytes = std::fs::read(exe).expect("read exe bytes");
        let err = symbols_from_binary(&bytes).expect_err("PE file should return error");
        assert_eq!(
            err,
            "no symbol table — the Windows build reads the linker map"
        );
    }

    #[cfg(target_os = "linux")]
    {
        let exe = std::env::current_exe().expect("current exe");
        let bytes = std::fs::read(exe).expect("read exe bytes");
        let symbols = symbols_from_binary(&bytes).expect("ELF symbols");
        assert!(symbols.iter().any(|s| crate_of(s) == "std" && s.size > 0));
    }
}

#[test]
fn crate_rows_sorting_and_breakdown() {
    let mut current = BTreeMap::new();
    current.insert("order_desk".to_string(), 5000);
    current.insert("makepad_widgets".to_string(), 3000);
    current.insert("std".to_string(), 1500);
    current.insert("serde".to_string(), 500);

    let mut previous = BTreeMap::new();
    previous.insert("order_desk".to_string(), 4800); // +200
    previous.insert("makepad_widgets".to_string(), 2000); // +1000 (grew most)
    previous.insert("std".to_string(), 1600); // -100

    let (top_rows, summary) = crate_rows(&current, Some(&previous), "order-desk", 2);
    assert_eq!(top_rows.len(), 2);

    // First row: order_desk (heaviest, own crate matching dash to underscore)
    assert_eq!(top_rows[0].name, "order_desk");
    assert_eq!(top_rows[0].bytes, 5000);
    assert_eq!(top_rows[0].share, 50);
    assert_eq!(top_rows[0].change, Some(200));
    assert!(top_rows[0].own);
    assert!(!top_rows[0].grew_most);

    // Second row: makepad_widgets (grew most)
    assert_eq!(top_rows[1].name, "makepad_widgets");
    assert_eq!(top_rows[1].bytes, 3000);
    assert_eq!(top_rows[1].share, 30);
    assert_eq!(top_rows[1].change, Some(1000));
    assert!(!top_rows[1].own);
    assert!(top_rows[1].grew_most);

    // Summary row for the remaining 2 crates
    let sum = summary.expect("summary row exists");
    assert_eq!(sum.name, "2 more crates");
    assert_eq!(sum.bytes, 2000); // 1500 + 500
    assert_eq!(sum.share, 20);
    assert_eq!(sum.change, None);
    assert!(!sum.own);
    assert!(!sum.grew_most);

    // Test grew_most outside top rows
    let (top_one, _) = crate_rows(&current, Some(&previous), "order_desk", 1);
    assert_eq!(top_one.len(), 1);
    // makepad_widgets grew most, but it's not in top 1 -> grew_most is false
    assert!(!top_one[0].grew_most);

    // Test with no previous build: change is None for all
    let (no_prev_rows, _) = crate_rows(&current, None, "order_desk", 4);
    for row in no_prev_rows {
        assert_eq!(row.change, None);
    }
}

#[test]
fn human_size_and_delta_exact_cases() {
    assert_eq!(human_size(0), "0 B");
    assert_eq!(human_size(1023), "1023 B");
    assert_eq!(human_size(1024), "1 KB");
    assert_eq!(human_size(188_416), "184 KB");
    assert_eq!(human_size(1_048_575), "1024 KB");
    assert_eq!(human_size(1_048_576), "1.00 MB");
    assert_eq!(human_size(8_830_000), "8.42 MB");

    assert_eq!(human_delta(188_416), "+184 KB");
    assert_eq!(human_delta(-12_288), "-12 KB");
    assert_eq!(human_delta(0), "no change");
}

#[test]
fn history_parsing_and_recording() {
    let sample = r#"
[[release]]
package = "order_desk"
at = 1762958000
bytes = 8830000

[release.crates]
makepad_widgets = 2420000
std = 1260000
"#;

    let parsed = parse_history(sample);
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0].package, "order_desk");
    assert_eq!(parsed[0].at, 1762958000);
    assert_eq!(parsed[0].bytes, 8830000);
    assert_eq!(parsed[0].crates.get("std"), Some(&1260000));

    let rendered = history_text(&parsed);
    let round_trip = parse_history(&rendered);
    assert_eq!(parsed, round_trip);

    assert!(parse_history("not toml = valid [[[").is_empty());

    // Record into temporary directory
    let temp_root =
        std::env::temp_dir().join(format!("crawrad-release-size-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&temp_root);

    for i in 0..25 {
        let rel = Release {
            package: "pkg_a".to_string(),
            at: 1000 + i,
            bytes: 5000 + i,
            crates: BTreeMap::new(),
        };
        record(&temp_root, rel).expect("record pkg_a");
    }

    let rel_b = Release {
        package: "pkg_b".to_string(),
        at: 2000,
        bytes: 9000,
        crates: BTreeMap::new(),
    };
    record(&temp_root, rel_b).expect("record pkg_b");

    let history = read_history(&temp_root);
    let pkg_a_entries: Vec<_> = history.iter().filter(|r| r.package == "pkg_a").collect();
    let pkg_b_entries: Vec<_> = history.iter().filter(|r| r.package == "pkg_b").collect();

    // Kept exactly the last 20 for pkg_a
    assert_eq!(pkg_a_entries.len(), 20);
    assert_eq!(pkg_a_entries[0].at, 1005); // skipped first 5
    assert_eq!(pkg_a_entries[19].at, 1024);

    // pkg_b left untouched
    assert_eq!(pkg_b_entries.len(), 1);
    assert_eq!(pkg_b_entries[0].at, 2000);

    // last_of returns oldest first and capped
    let last_three = last_of(&history, "pkg_a", 3);
    assert_eq!(last_three.len(), 3);
    assert_eq!(last_three[0].at, 1022);
    assert_eq!(last_three[1].at, 1023);
    assert_eq!(last_three[2].at, 1024);

    let _ = std::fs::remove_dir_all(&temp_root);
}

#[test]
fn shrink_tips_and_apply_flow() {
    let manifest_empty = "# Project root\n[package]\nname = \"demo\"\n";
    let tips = shrink_tips(manifest_empty);
    assert_eq!(tips.len(), 5);
    for tip in &tips {
        assert_eq!(tip.current, tip.key.cargo_default());
        assert!(!tip.applied);
    }

    let initial = initially_checked(manifest_empty);
    assert_eq!(initial, vec![ShrinkKey::Strip, ShrinkKey::Lto]);

    let preview = shrink_preview(manifest_empty, &initial);
    assert_eq!(preview, vec!["+ strip = \"symbols\"", "+ lto = \"thin\""]);

    // Apply shrink tips: adds [profile.release] and preserves header comments
    let applied = apply_shrink(manifest_empty, &initial).expect("apply shrink");
    assert!(applied.starts_with("# Project root\n[package]\nname = \"demo\"\n"));
    assert!(applied.contains("[profile.release]"));
    assert!(applied.contains("strip = \"symbols\""));
    assert!(applied.contains("lto = \"thin\""));

    // Tips now recognize applied settings
    let tips_applied = shrink_tips(&applied);
    let strip_tip = tips_applied
        .iter()
        .find(|t| t.key == ShrinkKey::Strip)
        .expect("strip tip");
    assert!(strip_tip.applied);
    assert_eq!(strip_tip.current, "\"symbols\"");

    // Unchecking Lto removes lto key
    let applied_no_lto = apply_shrink(&applied, &[ShrinkKey::Strip]).expect("apply uncheck lto");
    assert!(!applied_no_lto.contains("lto"));
    assert!(applied_no_lto.contains("strip = \"symbols\""));

    // User's customized lto stays when unchecked
    let custom_manifest = "[profile.release]\nlto = true\n";
    let custom_applied = apply_shrink(custom_manifest, &[]).expect("apply empty checked");
    assert!(custom_applied.contains("lto = true"));

    // Malformed manifest returns error
    assert!(apply_shrink("invalid = = toml", &initial).is_err());
}

#[test]
fn map_file_and_args_exact_strings() {
    let dir = Path::new("target");
    let map = map_file(dir, "order_desk");
    assert_eq!(
        map,
        PathBuf::from("target")
            .join("release")
            .join("crawrad-size-order_desk.map")
    );

    let link_args = map_link_args(&map);
    assert_eq!(link_args.len(), 2);
    assert_eq!(link_args[0], "-C");
    assert_eq!(link_args[1], format!("link-arg=/MAP:{}", map.display()));
}

#[test]
fn function_rows_aggregation() {
    let symbols = vec![
        Symbol {
            name: "_ZN7mapdemo4main17hbf213f186cba126fE".to_string(),
            size: 500,
            library: String::new(),
        },
        Symbol {
            name: "_ZN7mapdemo4main17hbf213f186cba126fE".to_string(),
            size: 300,
            library: String::new(),
        },
        Symbol {
            name: "raw_helper".to_string(),
            size: 100,
            library: String::new(),
        },
    ];

    let rows = function_rows(&symbols, 2);
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].0, "mapdemo::main");
    assert_eq!(rows[0].1, 800);
    assert_eq!(rows[1].0, "raw_helper");
    assert_eq!(rows[1].1, 100);
}

#[test]
fn sized_by_gaps_never_crosses_into_the_next_section() {
    let placed = vec![
        Placed {
            section: 2,
            offset: 0,
            name: "c".into(),
            library: String::new(),
        },
        Placed {
            section: 1,
            offset: 10,
            name: "b".into(),
            library: String::new(),
        },
        Placed {
            section: 1,
            offset: 0,
            name: "a".into(),
            library: String::new(),
        },
    ];
    let ends = BTreeMap::from([(1, 30), (2, 5)]);
    let sizes: Vec<(String, u64)> = sized_by_gaps(placed, &ends)
        .into_iter()
        .map(|s| (s.name, s.size))
        .collect();
    assert_eq!(
        sizes,
        vec![("a".into(), 10), ("b".into(), 20), ("c".into(), 5)]
    );
}

#[test]
fn map_headers_are_matched_whole_and_short_rows_skipped() {
    // Symbol names that carry ONE word of the section header must not reset the parser,
    // a row with no Rva+Base column still counts, a two-token row does not.
    let map = "\
 Start         Length     Name                   Class
 0001:00000000 00000100H .text$mn                CODE

  Address         Publics by Value              Rva+Base               Lib:Object

 0001:00000000       StartUp                    0000000140001000 f   a.obj
 0001:00000010       ClassInit                  0000000140001010 f   a.obj
 0001:00000020       LengthOf                   0000000140001020
 0001:00000040       two_tokens
 0001:00000080       last                       0000000140001080 f   a.obj
";
    let sizes: Vec<(String, u64)> = parse_msvc_map(map)
        .into_iter()
        .map(|s| (s.name, s.size))
        .collect();
    assert_eq!(
        sizes,
        vec![
            ("StartUp".into(), 0x10),
            ("ClassInit".into(), 0x10),
            ("LengthOf".into(), 0x60),
            ("last".into(), 0x80),
        ]
    );
}

fn object_with(
    format: object::BinaryFormat,
    arch: object::Architecture,
    symbols: &[(&str, u64, u64, object::SymbolKind, bool)],
) -> Vec<u8> {
    use object::write::{Mangling, Object, StandardSection, Symbol as Written, SymbolSection};
    let mut obj = Object::new(format, arch, object::Endianness::Little);
    obj.set_mangling(Mangling::None);
    let text = obj.section_id(StandardSection::Text);
    obj.append_section_data(text, &[0x90; 48], 1);
    for &(name, value, size, kind, defined) in symbols {
        obj.add_symbol(Written {
            name: name.as_bytes().to_vec(),
            value,
            size,
            kind,
            scope: object::SymbolScope::Linkage,
            weak: false,
            section: if defined {
                SymbolSection::Section(text)
            } else {
                SymbolSection::Undefined
            },
            flags: object::SymbolFlags::None,
        });
    }
    obj.write().expect("write object")
}

#[test]
fn elf_symbols_keep_their_declared_sizes() {
    use object::SymbolKind::{Data, Text, Tls};
    let bytes = object_with(
        object::BinaryFormat::Elf,
        object::Architecture::X86_64,
        &[
            ("alpha", 0, 4, Text, true),
            ("beta", 16, 8, Data, true),
            ("zero", 24, 0, Text, true),
            ("thread_local", 32, 4, Tls, true),
            ("imported", 0, 4, Text, false),
        ],
    );
    let mut sizes: Vec<(String, u64)> = symbols_from_binary(&bytes)
        .expect("ELF symbols")
        .into_iter()
        .map(|s| (s.name, s.size))
        .collect();
    sizes.sort();
    assert_eq!(sizes, vec![("alpha".into(), 4), ("beta".into(), 8)]);
}

#[test]
fn mach_o_symbols_are_sized_by_the_gaps() {
    use object::SymbolKind::Text;
    let bytes = object_with(
        object::BinaryFormat::MachO,
        object::Architecture::Aarch64,
        &[("first", 0, 0, Text, true), ("second", 16, 0, Text, true)],
    );
    let sizes: Vec<(String, u64)> = symbols_from_binary(&bytes)
        .expect("Mach-O symbols")
        .into_iter()
        .map(|s| (s.name, s.size))
        .collect();
    assert_eq!(sizes, vec![("first".into(), 16), ("second".into(), 32)]);
}

#[test]
fn crate_of_reads_paths_types_and_libraries() {
    let sym = |name: &str, library: &str| Symbol {
        name: name.to_string(),
        size: 1,
        library: library.to_string(),
    };
    assert_eq!(crate_of(&sym("plain_mod::func", "")), "plain_mod");
    assert_eq!(crate_of(&sym("<Foo as Bar>", "")), "Foo");
    assert_eq!(
        crate_of(&sym(
            "<&mut alloc::vec::Vec<u8> as std::io::Write>::write",
            ""
        )),
        "std"
    );
    assert_eq!(
        crate_of(&sym("<dyn order_desk::Shape as core::fmt::Debug>::fmt", "")),
        "order_desk"
    );
    assert_eq!(
        crate_of(&sym("<[order_desk::Item] as core::fmt::Debug>::fmt", "")),
        "order_desk"
    );
    assert_eq!(
        crate_of(&sym("some_c_func", "libfoo_bar-0123456789abcdef")),
        "foo_bar"
    );
    assert_eq!(
        crate_of(&sym("some_c_func", "libfoo-0123456789abcde")),
        "[system]"
    );
    assert_eq!(
        crate_of(&sym("some_c_func", "libfoo-0123456789abcdeg")),
        "[system]"
    );
    assert_eq!(
        crate_of(&sym("some_c_func", "foo-0123456789abcdef")),
        "[system]"
    );
}

#[test]
fn share_rounds_half_up_and_unchanged_crates_win_nothing() {
    assert_eq!(share_of(1, 8), 13);
    assert_eq!(share_of(1, 0), 0);
    let current = BTreeMap::from([("a".to_string(), 10), ("b".to_string(), 5)]);
    let (rows, summary) = crate_rows(&current, Some(&current), "a", 5);
    assert!(rows.iter().all(|r| !r.grew_most && r.change == Some(0)));
    assert!(summary.is_none());
}

#[test]
fn record_keeps_the_forty_heaviest_crates() {
    let root = std::env::temp_dir().join(format!("crawrad-release-crates-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let crates: BTreeMap<String, u64> = (0..45u64).map(|i| (format!("c{i:02}"), i + 1)).collect();
    let release = Release {
        package: "p".into(),
        at: 1,
        bytes: 9,
        crates,
    };
    record(&root, release).expect("record");
    let kept = &read_history(&root)[0].crates;
    assert_eq!(kept.len(), 40);
    assert!(!kept.contains_key("c04"));
    assert!(kept.contains_key("c05"));
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn function_rows_cut_at_top_and_delta_sign() {
    let syms: Vec<Symbol> = ["a", "b", "c"]
        .iter()
        .map(|n| Symbol {
            name: (*n).to_string(),
            size: 1,
            library: String::new(),
        })
        .collect();
    assert_eq!(function_rows(&syms, 1), vec![("a".to_string(), 1)]);
    assert_eq!(function_rows(&syms, 9).len(), 3);
    assert_eq!(human_delta(1), "+1 B");
    assert_eq!(human_delta(-1), "-1 B");
}

#[test]
fn shrink_texts_and_unchecking_an_applied_key() {
    assert_eq!(
        ShrinkKey::Lto.label(),
        "Optimize across crates at link time"
    );
    assert_eq!(ShrinkKey::Lto.cost(), "Release builds take longer.");
    let defaults: Vec<&str> = ShrinkKey::ALL.iter().map(|k| k.cargo_default()).collect();
    assert_eq!(defaults, vec!["false", "false", "16", "3", "\"unwind\""]);
    let applied = "[profile.release]\nstrip = \"symbols\"\nlto = \"thin\"\n";
    assert_eq!(
        shrink_preview(applied, &[ShrinkKey::Strip]),
        vec!["- lto = \"thin\""]
    );
    assert!(shrink_preview(applied, &[ShrinkKey::Strip, ShrinkKey::Lto]).is_empty());
}

#[test]
fn shrink_status_variations() {
    // 1. Added only
    let added = vec!["+ strip = \"symbols\"", "+ lto = \"thin\""];
    assert_eq!(shrink_status(&added), "Cargo.toml updated: strip, lto");

    // 2. Removed only
    let removed = vec!["- opt-level = \"s\""];
    assert_eq!(
        shrink_status(&removed),
        "Cargo.toml updated: removed: opt-level"
    );

    // 3. Both added and removed
    let both = vec!["+ strip = \"symbols\"", "- codegen-units = 1"];
    assert_eq!(
        shrink_status(&both),
        "Cargo.toml updated: strip; removed: codegen-units"
    );

    // 4. Multiple added and removed
    let multi = vec![
        "+ strip = \"symbols\"",
        "+ lto = \"thin\"",
        "- codegen-units = 1",
        "- panic = \"abort\"",
    ];
    assert_eq!(
        shrink_status(&multi),
        "Cargo.toml updated: strip, lto; removed: codegen-units, panic"
    );

    // 5. Empty preview
    let empty: Vec<&str> = vec![];
    assert_eq!(shrink_status(&empty), "Cargo.toml updated");
}

#[test]
fn shrink_dialog_state_construction() {
    let manifest = "[profile.release]\nstrip = \"symbols\"\n";
    let path = PathBuf::from("Cargo.toml");
    let dlg = ShrinkDialog::new(path.clone(), manifest.to_string());
    assert_eq!(dlg.manifest_path, path);
    assert_eq!(dlg.manifest, manifest);
    assert_eq!(dlg.tips.len(), 5);
    assert!(dlg.checked.contains(&ShrinkKey::Strip));
    assert!(dlg.checked.contains(&ShrinkKey::Lto));
}

#[test]
fn measure_from_map_and_binary() {
    let temp_dir =
        std::env::temp_dir().join(format!("crawrad-measure-test-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&temp_dir);
    let exe_path = temp_dir.join("sample.exe");
    let dummy_bytes = b"fake binary content for size testing";
    std::fs::write(&exe_path, dummy_bytes).expect("write dummy exe");

    let fixture = include_str!("sample.map");
    let measured = measure("demo_pkg", &exe_path, Some(fixture), 12345).expect("measure with map");
    assert_eq!(measured.package, "demo_pkg");
    assert_eq!(measured.at, 12345);
    assert_eq!(
        measured.bytes,
        u64::try_from(dummy_bytes.len()).unwrap_or_default()
    );
    let expected_crates = by_crate(&parse_msvc_map(fixture));
    assert_eq!(measured.crates, expected_crates);

    // Missing exe -> Err naming the path
    let missing_path = temp_dir.join("nonexistent.exe");
    let err =
        measure("demo_pkg", &missing_path, Some(fixture), 0).expect_err("missing exe should fail");
    assert!(err.contains("nonexistent.exe"));

    // Non-object bytes without map -> Ok with empty crates
    let no_map = measure("demo_pkg", &exe_path, None, 54321).expect("non-object without map");
    assert_eq!(
        no_map.bytes,
        u64::try_from(dummy_bytes.len()).unwrap_or_default()
    );
    assert!(no_map.crates.is_empty());

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn size_line_variations() {
    let current = Release {
        package: "app".into(),
        at: 1000,
        bytes: 4_404_019,
        crates: BTreeMap::new(),
    };

    // 1. First Release (empty history)
    assert_eq!(size_line(&[], &current), "Release size: 4.20 MB");

    // 2. History of ANOTHER package ignored -> still first Release
    let other_history = vec![Release {
        package: "other".into(),
        at: 900,
        bytes: 1_000_000,
        crates: BTreeMap::new(),
    }];
    assert_eq!(size_line(&other_history, &current), "Release size: 4.20 MB");

    // 3. Growth
    let prev_smaller = vec![Release {
        package: "app".into(),
        at: 900,
        bytes: 4_404_019 - 122_880,
        crates: BTreeMap::new(),
    }];
    assert_eq!(
        size_line(&prev_smaller, &current),
        "Release size: 4.20 MB (+120 KB since the last Release)"
    );

    // 4. Shrink
    let prev_larger = vec![Release {
        package: "app".into(),
        at: 900,
        bytes: 4_404_019 + 51_200,
        crates: BTreeMap::new(),
    }];
    assert_eq!(
        size_line(&prev_larger, &current),
        "Release size: 4.20 MB (-50 KB since the last Release)"
    );

    // 5. No change
    let prev_same = vec![Release {
        package: "app".into(),
        at: 900,
        bytes: 4_404_019,
        crates: BTreeMap::new(),
    }];
    assert_eq!(
        size_line(&prev_same, &current),
        "Release size: 4.20 MB (no change since the last Release)"
    );
}

#[test]
fn size_page_no_history_returns_none() {
    assert_eq!(size_page(&[], "app", "app.exe", &[], "app"), None);
}

#[test]
fn size_page_first_release() {
    let history = vec![Release {
        package: "app".into(),
        at: 1_731_421_920, // 12/11 14:32:00 UTC
        bytes: 8_828_928,  // 8.42 MB
        crates: BTreeMap::new(),
    }];
    let page = size_page(&history, "app", "order_desk.exe", &[], "app").expect("size page");
    assert_eq!(page.exe_label, "order_desk.exe · Release");
    assert_eq!(page.when, "12/11 14:32");
    assert_eq!(page.size, "8.42 MB");
    assert_eq!(page.change, None);
    assert_eq!(page.since, None);
    assert_eq!(page.bars, vec![1.0]);
}

#[test]
fn size_page_growth_shrink_same_trends_and_texts() {
    let at_prev = 1_731_335_520; // 11/11 14:32:00 UTC
    let at_curr = 1_731_421_920; // 12/11 14:32:00 UTC
    let bytes_prev = 8_640_512; // 8.24 MB
    let bytes_curr = 8_828_928; // 8.42 MB (+184 KB)

    // 1. Growth
    let history_growth = vec![
        Release {
            package: "app".into(),
            at: at_prev,
            bytes: bytes_prev,
            crates: BTreeMap::new(),
        },
        Release {
            package: "app".into(),
            at: at_curr,
            bytes: bytes_curr,
            crates: BTreeMap::new(),
        },
    ];
    let page_growth =
        size_page(&history_growth, "app", "order_desk.exe", &[], "app").expect("growth page");
    assert_eq!(
        page_growth.change,
        Some(("+184 KB".to_string(), Trend::Grew))
    );
    assert_eq!(
        page_growth.since,
        Some("since the last Release (8.24 MB, 11/11)".to_string())
    );

    // 2. Shrink
    let history_shrink = vec![
        Release {
            package: "app".into(),
            at: at_prev,
            bytes: bytes_curr,
            crates: BTreeMap::new(),
        },
        Release {
            package: "app".into(),
            at: at_curr,
            bytes: bytes_prev,
            crates: BTreeMap::new(),
        },
    ];
    let page_shrink =
        size_page(&history_shrink, "app", "order_desk.exe", &[], "app").expect("shrink page");
    assert_eq!(
        page_shrink.change,
        Some(("-184 KB".to_string(), Trend::Shrank))
    );
    assert_eq!(
        page_shrink.since,
        Some("since the last Release (8.42 MB, 11/11)".to_string())
    );

    // 3. Same
    let history_same = vec![
        Release {
            package: "app".into(),
            at: at_prev,
            bytes: bytes_curr,
            crates: BTreeMap::new(),
        },
        Release {
            package: "app".into(),
            at: at_curr,
            bytes: bytes_curr,
            crates: BTreeMap::new(),
        },
    ];
    let page_same =
        size_page(&history_same, "app", "order_desk.exe", &[], "app").expect("same page");
    assert_eq!(
        page_same.change,
        Some(("no change".to_string(), Trend::Same))
    );
    assert_eq!(
        page_same.since,
        Some("since the last Release (8.42 MB, 11/11)".to_string())
    );
}

#[test]
fn size_page_bars_normalised_to_max_and_capped_at_6() {
    let history: Vec<Release> = (1..=8)
        .map(|i| Release {
            package: "app".into(),
            at: i * 100,
            bytes: i * 100,
            crates: BTreeMap::new(),
        })
        .collect();

    let page = size_page(&history, "app", "app.exe", &[], "app").expect("page with 8 releases");
    assert_eq!(page.bars.len(), 6);
    // The last 6 are 300, 400, 500, 600, 700, 800 normalized to max (800)
    let expected = vec![
        300.0 / 800.0,
        400.0 / 800.0,
        500.0 / 800.0,
        600.0 / 800.0,
        700.0 / 800.0,
        1.0,
    ];
    assert_eq!(page.bars, expected);
}

#[test]
fn size_page_another_package_history_ignored() {
    let history = vec![
        Release {
            package: "other_pkg".into(),
            at: 100,
            bytes: 50_000_000,
            crates: BTreeMap::new(),
        },
        Release {
            package: "app".into(),
            at: 200,
            bytes: 1_000_000,
            crates: BTreeMap::new(),
        },
    ];
    let page = size_page(&history, "app", "app.exe", &[], "app").expect("single app release");
    assert_eq!(page.bars, vec![1.0]);
    assert_eq!(page.change, None);
    assert_eq!(page.since, None);
}

#[test]
fn size_page_exact_date_formatting_for_fixed_at() {
    assert_eq!(format_when(1_731_421_920), "12/11 14:32");
    assert_eq!(format_day_month(1_731_421_920), "12/11");
    assert_eq!(format_when(1_700_000_000), "14/11 22:13");
    assert_eq!(format_day_month(1_700_000_000), "14/11");
    assert_eq!(format_when(0), "01/01 00:00");
    assert_eq!(format_day_month(0), "01/01");
}

#[test]
fn measure_with_symbols_returns_symbols() {
    let fixture = include_str!("sample.map");
    let temp_dir = std::env::temp_dir().join(format!("crawrad-sym-test-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&temp_dir);
    let exe_path = temp_dir.join("sample.exe");
    std::fs::write(&exe_path, b"test payload").expect("write test payload");

    let (rel, syms) =
        measure_with_symbols("demo", &exe_path, Some(fixture), 500).expect("measure_with_symbols");
    assert_eq!(rel.package, "demo");
    assert_eq!(rel.at, 500);
    assert!(!syms.is_empty());

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_size_page_parts_exact_strings() {
    let mut current_crates = BTreeMap::new();
    // Total = 100_000 bytes. 12_000 bytes is 12% share.
    current_crates.insert("my_own_crate".to_string(), 12_000);
    current_crates.insert("dep_growing".to_string(), 50_000);
    current_crates.insert("dep_stable".to_string(), 38_000);

    let mut prev_crates = BTreeMap::new();
    prev_crates.insert("my_own_crate".to_string(), 11_000);
    prev_crates.insert("dep_growing".to_string(), 40_000); // grew by 10_000 (grew most)
    prev_crates.insert("dep_stable".to_string(), 38_000);

    let history = vec![
        Release {
            package: "my_app".into(),
            at: 1_700_000_000,
            bytes: 89_000,
            crates: prev_crates,
        },
        Release {
            package: "my_app".into(),
            at: 1_700_086_400,
            bytes: 100_000,
            crates: current_crates,
        },
    ];

    let page = size_page(&history, "my_app", "my_app.exe", &[], "my_own_crate")
        .expect("size page created");
    let parts = page.parts();

    assert_eq!(parts.0, "my_app.exe · Release, 15/11 22:13");
    assert_eq!(parts.1, "98 KB");
    assert_eq!(parts.2, Some(("+11 KB".to_string(), 0))); // 0 = Trend::Grew
    assert_eq!(
        parts.3,
        Some("since the last Release (87 KB, 14/11)".to_string())
    );

    let crates = parts.5;
    // Crates sorted by size descending: dep_growing (50_000), dep_stable (38_000), my_own_crate (12_000)
    let grew_row = crates
        .iter()
        .find(|c| c.0 == "dep_growing")
        .expect("dep_growing");
    assert_eq!(grew_row.3, "▲ +10 KB"); // '▲ ' before grew-most change

    let own_row = crates
        .iter()
        .find(|c| c.0 == "my_own_crate")
        .expect("my_own_crate");
    assert_eq!(own_row.2, "12%"); // '12%' share text
    assert!(own_row.4); // own flag true

    // Now test a release with NO previous release -> '—'
    let single_history = vec![history[1].clone()];
    let single_page = size_page(&single_history, "my_app", "my_app.exe", &[], "my_own_crate")
        .expect("single size page");
    let single_parts = single_page.parts();
    assert_eq!(single_parts.2, None);
    assert_eq!(single_parts.3, None);
    for row in &single_parts.5 {
        assert_eq!(row.3, "—"); // '—' with no previous Release
    }

    assert_eq!(
        parts.7,
        "Read from the linker's map of this build — no extra tool installed. Your own crate in purple."
    );
}

#[test]
fn test_shrink_status_exact_strings() {
    assert_eq!(
        shrink_status(&["+ strip = \"symbols\"", "+ lto = \"thin\""]),
        "Cargo.toml updated: strip, lto"
    );
    assert_eq!(
        shrink_status(&["- opt-level = \"s\""]),
        "Cargo.toml updated: removed: opt-level"
    );
    assert_eq!(
        shrink_status(&["+ strip = \"symbols\"", "- codegen-units = 1"]),
        "Cargo.toml updated: strip; removed: codegen-units"
    );
    assert_eq!(shrink_status::<&str>(&[]), "Cargo.toml updated");
}

#[test]
fn format_build_clock_exact_formats() {
    assert_eq!(format_build_clock(Duration::from_secs(45)), "0:45");
    assert_eq!(format_build_clock(Duration::from_secs(108)), "1:48");
    assert_eq!(format_build_clock(Duration::from_secs(3661)), "1:01:01");
}

#[test]
fn release_card_first_release() {
    let current = Release {
        package: "app".into(),
        at: 1_731_421_920,
        bytes: 8_828_928, // 8.42 MB
        crates: BTreeMap::new(),
    };
    let card = release_card(&[], &current, Duration::from_secs(108));
    assert_eq!(card.headline, "Release built in 1:48");
    assert_eq!(card.size, "8.42 MB");
    assert_eq!(card.change, None);
    assert_eq!(card.mostly, None);
    assert_eq!(card.status_text(), "Release 8.42 MB");
}

#[test]
fn release_card_grew_with_champion_crate() {
    let mut prev_crates = BTreeMap::new();
    prev_crates.insert("crawrad_portadb".to_string(), 1_000_000);
    prev_crates.insert("other_crate".to_string(), 200_000);

    let history = vec![Release {
        package: "app".into(),
        at: 1_731_335_520,
        bytes: 8_640_512, // 8.24 MB
        crates: prev_crates,
    }];

    let mut current_crates = BTreeMap::new();
    current_crates.insert("crawrad_portadb".to_string(), 1_188_416); // +184 KB
    current_crates.insert("other_crate".to_string(), 200_000);

    let current = Release {
        package: "app".into(),
        at: 1_731_421_920,
        bytes: 8_828_928, // 8.42 MB (+184 KB)
        crates: current_crates,
    };

    let card = release_card(&history, &current, Duration::from_secs(108));
    assert_eq!(card.headline, "Release built in 1:48");
    assert_eq!(card.size, "8.42 MB");
    assert_eq!(card.change, Some(("+184 KB".to_string(), true)));
    assert_eq!(card.mostly, Some("mostly crawrad_portadb".to_string()));
    assert_eq!(card.status_text(), "Release 8.42 MB (+184 KB)");
}

#[test]
fn release_card_shrank() {
    let history = vec![Release {
        package: "app".into(),
        at: 1_731_335_520,
        bytes: 8_828_928, // 8.42 MB
        crates: BTreeMap::new(),
    }];

    let current = Release {
        package: "app".into(),
        at: 1_731_421_920,
        bytes: 8_816_640, // 8.41 MB (-12 KB)
        crates: BTreeMap::new(),
    };

    let card = release_card(&history, &current, Duration::from_secs(45));
    assert_eq!(card.headline, "Release built in 0:45");
    assert_eq!(card.size, "8.41 MB");
    assert_eq!(card.change, Some(("-12 KB".to_string(), false)));
    assert_eq!(card.mostly, None);
    assert_eq!(card.status_text(), "Release 8.41 MB (-12 KB)");
}

#[test]
fn release_card_same_size() {
    let history = vec![Release {
        package: "app".into(),
        at: 1_731_335_520,
        bytes: 8_828_928,
        crates: BTreeMap::new(),
    }];

    let current = Release {
        package: "app".into(),
        at: 1_731_421_920,
        bytes: 8_828_928,
        crates: BTreeMap::new(),
    };

    let card = release_card(&history, &current, Duration::from_secs(75));
    assert_eq!(card.headline, "Release built in 1:15");
    assert_eq!(card.size, "8.42 MB");
    assert_eq!(card.change, None);
    assert_eq!(card.mostly, None);
    assert_eq!(card.status_text(), "Release 8.42 MB");
}

#[test]
fn release_card_grew_but_no_crate_grew() {
    let history = vec![Release {
        package: "app".into(),
        at: 1_731_335_520,
        bytes: 8_640_512,
        crates: BTreeMap::new(),
    }];

    let current = Release {
        package: "app".into(),
        at: 1_731_421_920,
        bytes: 8_828_928,
        crates: BTreeMap::new(), // no crate symbols recorded
    };

    let card = release_card(&history, &current, Duration::from_secs(108));
    assert_eq!(card.headline, "Release built in 1:48");
    assert_eq!(card.size, "8.42 MB");
    assert_eq!(card.change, Some(("+184 KB".to_string(), true)));
    assert_eq!(card.mostly, None);
    assert_eq!(card.status_text(), "Release 8.42 MB (+184 KB)");
}

#[test]
fn release_card_and_size_line_with_interleaved_packages() {
    let prev_app = Release {
        package: "app".into(),
        at: 1_731_335_520,
        bytes: 8_640_512,
        crates: BTreeMap::new(),
    };
    let other_pkg = Release {
        package: "other_tool".into(),
        at: 1_731_350_000,
        bytes: 1_000_000,
        crates: BTreeMap::new(),
    };
    let current_app = Release {
        package: "app".into(),
        at: 1_731_421_920,
        bytes: 8_828_928,
        crates: BTreeMap::new(),
    };

    // Case 1: current is not yet in history
    let history_before = vec![prev_app.clone(), other_pkg.clone()];
    let card1 = release_card(&history_before, &current_app, Duration::from_secs(108));
    assert_eq!(card1.change, Some(("+184 KB".to_string(), true)));
    assert_eq!(
        size_line(&history_before, &current_app),
        "Release size: 8.42 MB (+184 KB since the last Release)"
    );
}
