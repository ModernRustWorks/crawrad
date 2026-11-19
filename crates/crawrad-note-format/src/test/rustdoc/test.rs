use super::*;

#[test]
fn missing_sections_ignoring_headings_inside_fences() {
    let suggestions = vec![
        Suggestion {
            section: Section::Errors,
            required: false,
            reason: "returns Result".to_string(),
        },
        Suggestion {
            section: Section::Panics,
            required: false,
            reason: "calls unwrap() on line 10".to_string(),
        },
    ];

    let markdown_fenced = "Summary paragraph.\n\n```rust\n# Errors\n# Panics\n```\n";
    let missing1 = missing_sections(markdown_fenced, &suggestions);
    assert_eq!(missing1.len(), 2);

    let markdown_real =
        "Summary paragraph.\n\n```rust\n# Panics\n```\n\n# Errors\n\nDescribes errors.";
    let missing2 = missing_sections(markdown_real, &suggestions);
    assert_eq!(missing2.len(), 1);
    assert_eq!(missing2[0].section, Section::Panics);
}

#[test]
fn summary_warning_all_cases() {
    // Empty
    assert_eq!(
        summary_warning(""),
        Some("Missing summary: the first paragraph is what rustdoc shows in lists.".to_string())
    );

    // Only headings
    assert_eq!(
        summary_warning("# Title\n\n## Subtitle"),
        Some("Missing summary: the first paragraph is what rustdoc shows in lists.".to_string())
    );

    // Longer than 80 chars
    let long_summary = "This is an extremely long summary sentence that clearly exceeds the eighty character limit for rustdoc summaries.";
    assert_eq!(
        summary_warning(long_summary),
        Some(format!(
            "Summary is {} chars; rustdoc shows it in lists — keep it under 80.",
            long_summary.chars().count()
        ))
    );

    // Missing ending period
    assert_eq!(
        summary_warning("A short summary without a period"),
        Some("Summary should be one sentence ending with a period.".to_string())
    );

    // Valid summary
    assert_eq!(
        summary_warning("A clean and concise summary sentence."),
        None
    );
}

#[test]
fn insert_section_all_cases() {
    // Summary missing vs present
    let empty = "";
    assert_eq!(
        insert_section(empty, Section::Summary),
        "One-line summary.\n\n"
    );
    let with_summary = "Existing summary sentence.\n\n# Details\n\nText.";
    assert_eq!(insert_section(with_summary, Section::Summary), with_summary);

    // Examples
    assert_eq!(
        insert_section("", Section::Examples),
        "# Examples\n\n```\n\n```"
    );
    let md = "One-line summary.";
    assert_eq!(
        insert_section(md, Section::Examples),
        "One-line summary.\n\n# Examples\n\n```\n\n```"
    );
    let already_examples = "One-line summary.\n\n# Examples\n\n```\n\n```";
    assert_eq!(
        insert_section(already_examples, Section::Examples),
        already_examples
    );

    // Errors / Panics / Safety
    assert_eq!(insert_section("", Section::Errors), "# Errors\n\n");
    assert_eq!(
        insert_section(md, Section::Errors),
        "One-line summary.\n\n# Errors\n\n"
    );
    assert_eq!(
        insert_section("One-line summary.\n\n# Errors\n\n", Section::Errors),
        "One-line summary.\n\n# Errors\n\n"
    );

    assert_eq!(insert_section("", Section::Panics), "# Panics\n\n");
    assert_eq!(
        insert_section(md, Section::Panics),
        "One-line summary.\n\n# Panics\n\n"
    );

    assert_eq!(insert_section("", Section::Safety), "# Safety\n\n");
    assert_eq!(
        insert_section(md, Section::Safety),
        "One-line summary.\n\n# Safety\n\n"
    );
}

#[test]
fn doc_problems_clean_body() {
    let md = "Calculates the sum of two numbers.\n\n# Examples\n\n```rust\nassert_eq!(add(1, 2), 3);\n```\n";
    let facts = ItemFacts {
        is_public: true,
        ..Default::default()
    };
    let problems = doc_problems(md, &facts);
    assert!(
        problems.is_empty(),
        "expected no problems, got {problems:?}"
    );
}

#[test]
fn doc_problems_no_summary() {
    let md = "# Examples\n\n```rust\nlet x = 1;\n```\n";
    let facts = ItemFacts::default();
    let problems = doc_problems(md, &facts);
    assert_eq!(problems.len(), 1);
    assert!(problems[0].contains("Missing summary"));
}

#[test]
fn doc_problems_summary_without_period() {
    let md = "Calculates the sum\n\n# Examples\n\n```rust\nlet x = 1;\n```\n";
    let facts = ItemFacts::default();
    let problems = doc_problems(md, &facts);
    assert_eq!(problems.len(), 1);
    assert!(problems[0].contains("period"));
}

#[test]
fn doc_problems_summary_over_80_chars() {
    let long_line = "This is a summary paragraph that is intentionally authored to exceed eighty characters in total length.";
    let md = format!("{long_line}\n\n# Examples\n\n```rust\nlet x = 1;\n```\n");
    let facts = ItemFacts::default();
    let problems = doc_problems(&md, &facts);
    assert_eq!(problems.len(), 1);
    assert!(problems[0].contains("keep it under 80"));
}

#[test]
fn doc_problems_no_examples() {
    let md = "Calculates the sum of two numbers.\n";
    let facts = ItemFacts {
        is_public: true,
        ..Default::default()
    };
    let problems = doc_problems(md, &facts);
    assert_eq!(problems.len(), 1);
    assert_eq!(problems[0], "missing `# Examples` section");
}

#[test]
fn doc_problems_examples_with_only_text_block() {
    let md = "Calculates the sum.\n\n# Examples\n\n```text\nresult is 42\n```\n";
    let facts = ItemFacts {
        is_public: true,
        ..Default::default()
    };
    let problems = doc_problems(md, &facts);
    assert_eq!(problems.len(), 1);
    assert_eq!(problems[0], "# Examples has no rust code block");
}

#[test]
fn doc_problems_examples_rustdoc_fence_tags_count_as_rust() {
    let tags = ["no_run", "ignore", "should_panic", "rust,no_run"];
    for tag in tags {
        let md = format!("Calculates the sum.\n\n# Examples\n\n```{tag}\nassert!(true);\n```\n");
        let facts = ItemFacts {
            is_public: true,
            ..Default::default()
        };
        let problems = doc_problems(&md, &facts);
        assert!(
            problems.is_empty(),
            "expected no problems for fence tag `{tag}`, got {problems:?}"
        );
    }
}

#[test]
fn doc_problems_result_fn_without_errors() {
    let md = "Parses the input string.\n\n# Examples\n\n```rust\nlet _ = parse(\"1\");\n```\n";
    let facts = ItemFacts {
        is_public: true,
        returns_result: true,
        ..Default::default()
    };
    let problems = doc_problems(md, &facts);
    assert_eq!(problems.len(), 1);
    assert_eq!(problems[0], "missing `# Errors`: returns Result");
}

#[test]
fn doc_problems_unsafe_fn_without_safety() {
    let md = "Dereferences a raw pointer.\n\n# Examples\n\n```rust\nlet _ = read_ptr();\n```\n";
    let facts = ItemFacts {
        is_public: true,
        is_unsafe: true,
        is_trait: false,
        ..Default::default()
    };
    let problems = doc_problems(md, &facts);
    assert_eq!(problems.len(), 1);
    assert_eq!(problems[0], "missing `# Safety`: unsafe fn");
}

#[test]
fn doc_problems_body_with_unwrap_without_panics() {
    let md = "Computes internal cache value.\n\n# Examples\n\n```rust\nlet _ = compute();\n```\n";
    let facts = ItemFacts {
        is_public: true,
        first_panic: Some("calls unwrap()".to_string()),
        ..Default::default()
    };
    let problems = doc_problems(md, &facts);
    assert_eq!(problems.len(), 1);
    assert_eq!(problems[0], "missing `# Panics`: calls unwrap()");
}

#[test]
fn doc_problems_headings_inside_code_fence_do_not_count() {
    let md = "Calculates value.\n\n```rust\n# Examples\n# Errors\n# Panics\n# Safety\n```\n";
    let facts = ItemFacts {
        is_public: true,
        returns_result: true,
        is_unsafe: true,
        is_trait: false,
        first_panic: Some("calls panic!".to_string()),
    };
    let problems = doc_problems(md, &facts);
    assert_eq!(
        problems,
        vec![
            "missing `# Examples` section".to_string(),
            "missing `# Errors`: returns Result".to_string(),
            "missing `# Panics`: calls panic!".to_string(),
            "missing `# Safety`: unsafe fn".to_string(),
        ]
    );
}

#[test]
fn private_fn_with_summary_and_no_examples_has_no_problems() {
    let md = "Calculates the sum of two numbers.\n";
    let facts = ItemFacts {
        is_public: false,
        ..Default::default()
    };
    let problems = doc_problems(md, &facts);
    assert!(
        problems.is_empty(),
        "expected no problems, got {problems:?}"
    );
}

#[test]
fn pub_fn_reports_missing_examples() {
    let md = "Calculates the sum of two numbers.\n";
    let facts = ItemFacts {
        is_public: true,
        ..Default::default()
    };
    let problems = doc_problems(md, &facts);
    assert_eq!(problems, vec!["missing `# Examples` section".to_string()]);
}

#[test]
fn private_fn_with_120_char_summary_reports_summary_problem() {
    let long_summary = "This is a summary paragraph that is authored to exceed eighty characters in total length and easily passes one hundred twenty characters.";
    assert!(long_summary.chars().count() >= 120);
    let md = format!("{long_summary}\n");
    let facts = ItemFacts {
        is_public: false,
        ..Default::default()
    };
    let problems = doc_problems(&md, &facts);
    assert_eq!(problems.len(), 1);
    assert!(problems[0].contains("keep it under 80"));
}

#[cfg(feature = "syn")]
mod syn_tests {
    use super::*;
    use quote::quote;

    #[test]
    fn result_return_plain_and_io() {
        let plain = item_facts(&quote!(
            pub fn f() -> Result<u32, String> {
                Ok(1)
            }
        ));
        assert!(plain.returns_result);

        let io = item_facts(&quote!(
            pub fn f() -> std::io::Result<()> {
                Ok(())
            }
        ));
        assert!(io.returns_result);

        let not_result = item_facts(&quote!(
            pub fn f() -> Option<u32> {
                None
            }
        ));
        assert!(!not_result.returns_result);
    }

    #[test]
    fn unsafe_fn_detected() {
        let unsafe_f = item_facts(&quote!(
            pub unsafe fn deref_raw(ptr: *const u8) -> u8 {
                *ptr
            }
        ));
        assert!(unsafe_f.is_unsafe);
        assert!(!unsafe_f.is_trait);

        let safe_f = item_facts(&quote!(
            pub fn safe_calc() -> u8 {
                1
            }
        ));
        assert!(!safe_f.is_unsafe);
    }

    #[test]
    fn unsafe_trait_detected() {
        let unsafe_t = item_facts(&quote!(
            pub unsafe trait NativeMemory {}
        ));
        assert!(unsafe_t.is_unsafe);
        assert!(unsafe_t.is_trait);

        let safe_t = item_facts(&quote!(
            pub trait Worker {}
        ));
        assert!(!safe_t.is_unsafe);
        assert!(safe_t.is_trait);
    }

    #[test]
    fn unwrap_in_body_detected() {
        let f = item_facts(&quote!(
            pub fn run() {
                let x: Option<i32> = None;
                let _ = x.unwrap();
            }
        ));
        assert_eq!(f.first_panic.as_deref(), Some("calls unwrap()"));
    }

    #[test]
    fn debug_assert_not_counted() {
        let f = item_facts(&quote!(
            pub fn run() {
                debug_assert!(true);
                debug_assert_eq!(1, 1);
            }
        ));
        assert_eq!(f.first_panic, None);
    }

    #[test]
    fn item_facts_visibility_private_pub_and_pub_crate() {
        let priv_fn = item_facts(&quote!(
            fn f() {}
        ));
        assert!(!priv_fn.is_public);

        let pub_fn = item_facts(&quote!(
            pub fn f() {}
        ));
        assert!(pub_fn.is_public);

        let pub_crate_fn = item_facts(&quote!(
            pub(crate) fn f() {}
        ));
        assert!(!pub_crate_fn.is_public);

        let pub_super_fn = item_facts(&quote!(
            pub(super) fn f() {}
        ));
        assert!(!pub_super_fn.is_public);

        let pub_in_fn = item_facts(&quote!(
            pub(in crate::sub) fn f() {}
        ));
        assert!(!pub_in_fn.is_public);

        let pub_struct = item_facts(&quote!(
            pub struct S;
        ));
        assert!(pub_struct.is_public);

        let priv_struct = item_facts(&quote!(
            struct S;
        ));
        assert!(!priv_struct.is_public);

        let trait_item = item_facts(&quote!(
            fn trait_fn(&self);
        ));
        assert!(trait_item.is_public);
    }

    #[test]
    fn pub_crate_fn_counts_as_private_in_doc_problems() {
        let facts = item_facts(&quote!(
            pub(crate) fn f() {}
        ));
        assert!(!facts.is_public);
        let md = "Calculates the sum of two numbers.\n";
        let problems = doc_problems(md, &facts);
        assert!(
            problems.is_empty(),
            "expected no problems, got {problems:?}"
        );
    }
}
