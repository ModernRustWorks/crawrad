use super::*;

/// A clean group in `%TEMP%`: virtual root and requested members.
fn group(name: &str, member_list: &[&str]) -> PathBuf {
    let r = std::env::temp_dir().join(name);
    let _ = std::fs::remove_dir_all(&r);
    std::fs::create_dir_all(&r).expect("create test root");
    let list = member_list
        .iter()
        .map(|m| format!("\"{m}\""))
        .collect::<Vec<_>>()
        .join(", ");
    std::fs::write(
        r.join(MANIFEST),
        format!("[workspace]\nresolver = \"2\"\nmembers = [{list}]\n"),
    )
    .expect("write group");
    for m in member_list {
        std::fs::create_dir_all(r.join(m).join("src")).expect("create member");
        std::fs::write(
            r.join(m).join(MANIFEST),
            format!("[package]\nname = \"{m}\"\n"),
        )
        .expect("write member");
    }
    r
}

fn names(paths: &[PathBuf]) -> Vec<String> {
    paths
        .iter()
        .filter_map(|c| c.file_name()?.to_str().map(str::to_string))
        .collect()
}

// -------------------------------------------- who has what to run

/// The response of `cargo metadata`, **measured** and trimmed.
///
/// Not an invented JSON: real output from `cargo 1.95` on a group with
/// three members — one `--bin`, one `--lib`, and one with declared `[[bin]]` —,
/// with fields not read by this module removed. An example written from
/// scratch would measure our idea of the format, which ADR 0018 prohibits.
///
/// `workspace_root` stays intentionally, and is a field no one here reads:
/// it proves serde ignores what is not in the structs — which is what the
/// decision not to describe the entire format relies on. (Its slash was
/// inverted so the literal fits without escaping; cargo emits `C:\work` on Windows.)
const CARGO_RESPONSE: &str = r#"{
  "packages": [
    {"name":"meu_app","targets":[
      {"kind":["bin"],"crate_types":["bin"],"name":"meu_app"}]},
    {"name":"minha_lib","targets":[
      {"kind":["lib"],"crate_types":["lib"],"name":"minha_lib"}]},
    {"name":"mista","targets":[
      {"kind":["lib"],"crate_types":["rlib","cdylib"],"name":"mista"},
      {"kind":["bin"],"crate_types":["bin"],"name":"ferramenta"},
      {"kind":["custom-build"],"crate_types":["bin"],"name":"build-script-build"}]}
  ],
  "workspace_root": "C:/work/grupo",
  "version": 1
}"#;

/// Does the package run? `None` when not in cargo's response.
fn runs(json: &str, package: &str) -> Option<bool> {
    executables(json)
        .expect("cargo response is readable")
        .get(package)
        .copied()
}

#[test]
fn only_items_with_bin_target_enter_the_list() {
    // Defect that Amendment 3 fixes, measured in format: `minha_lib` does
    // not run, and it was to it that the IDE was offering `Run`.
    assert_eq!(
        runs(CARGO_RESPONSE, "meu_app"),
        Some(true),
        "executable was not recognized"
    );
    assert_eq!(
        runs(CARGO_RESPONSE, "minha_lib"),
        Some(false),
        "library entered as executable — it is what `cargo run` refuses with `a bin target must be available`"
    );
    // The MIXED package is the reason why the read is per-target and not per-package:
    // it is `rlib` + `cdylib` and STILL has a `[[bin]]`. Whoever asked "is package
    // crate_type lib?" would classify it wrongly.
    assert_eq!(
        runs(CARGO_RESPONSE, "mista"),
        Some(true),
        "package with lib AND bin was left out: a `[[bin]]` next to the lib runs"
    );
    // And what is NOT in the group returns `None`, not `false`: it is what allows
    // the shell to separate "this is the library" from "I don't know who this
    // package is" — and not disable `Run` in the second case.
    assert_eq!(runs(CARGO_RESPONSE, "never_heard_of"), None);
}

#[test]
fn build_script_does_not_count_as_runnable() {
    // `custom-build` is also a binary — compiled `build.rs` —, and it appears
    // in the target list of any package that has one. If the read were "exists
    // target that produces an executable", every package with `build.rs` would
    // pass as runnable, and the library of this entire repository would again
    // receive `Run`.
    let script_only = r#"{"packages":[{"name":"lib_with_script","targets":[
        {"kind":["lib"],"crate_types":["lib"],"name":"lib_with_script"},
        {"kind":["custom-build"],"crate_types":["bin"],"name":"build-script-build"}]}]}"#;
    assert_eq!(
        runs(script_only, "lib_with_script"),
        Some(false),
        "`build.rs` was counted as runnable"
    );
}

#[test]
fn non_json_response_becomes_error_not_empty_list() {
    // The difference the shell needs: empty list means "cargo answered, and
    // nobody runs here"; error means "could not query" — and in that case the
    // IDE does not disable any button (`Project::default`). Returning `Ok(empty)`
    // on a failure would disable `Run` for a perfectly runnable project.
    assert!(executables("this is not json").is_err());
    assert!(
        executables("{}").is_err(),
        "response without `packages` is failure"
    );
}

#[test]
fn real_cargo_separates_library_from_executable() {
    // **The real `cargo`**, not our trimmed version of it: the rule measured
    // here is cargo's target discovery — `src/main.rs` becomes `bin` without a
    // line of manifest —, and it is precisely the rule our own reader of `Cargo.toml`
    // would get wrong. A stunt double here would measure our idea of it, which is
    // what ADR 0018 exists to avoid doing.
    //
    // It is the same tool as `project_created_inside_external_repo_opens`, which
    // already runs cargo in this suite.
    let r = group("crawrad_ws_executables", &["app", "lib_pura"]);
    std::fs::write(r.join("app/src/main.rs"), "fn main() {}\n").expect("main");
    std::fs::write(r.join("lib_pura/src/lib.rs"), "pub fn f() {}\n").expect("lib");

    let q = match group_executables(&r) {
        Ok(q) => q,
        // Without `cargo` in PATH there is nothing to measure, and failing here
        // would blame the machine instead of the code. The rest of this file's
        // suite covers the format.
        Err(e) => panic!("cargo did not respond in this group: {e}"),
    };
    assert_eq!(
        q.get("app"),
        Some(&true),
        "`src/main.rs` did not become bin target: {q:?}"
    );
    assert_eq!(
        q.get("lib_pura"),
        Some(&false),
        "library entered as executable: {q:?}"
    );
}

#[test]
fn targets_timestamp_changes_when_main_appears() {
    // What invalidates the cache: without this, whoever added a `src/main.rs`
    // to the library would have `Run` disabled until closing the IDE — and
    // would have no way to know why.
    let r = group("crawrad_ws_timestamp", &["app"]);
    let member = r.join("app");
    let before = targets_timestamp(&r, Some(&member));
    assert!(before.is_some(), "timestamp comes from existing files");

    // The Windows file clock has granularity, and two writes in the same
    // tick would give the same timestamp — which would make this test pass
    // or fail depending on machine speed, which is the worst kind of test.
    // The sleep is short and is the price of measuring mtime.
    std::thread::sleep(std::time::Duration::from_millis(20));
    std::fs::write(member.join("src/main.rs"), "fn main() {}\n").expect("new main");
    assert_ne!(
        targets_timestamp(&r, Some(&member)),
        before,
        "a new `src/main.rs` did not change the timestamp — the `Run` cache \
         would stay stuck in the previous answer"
    );

    // And with no folder there is no timestamp: it is the state of the welcome
    // screen, and inventing one would make the answer of one group apply to the next.
    assert!(targets_timestamp(&r.join("does-not-exist"), None).is_none());
}

#[test]
fn document_lives_in_member_src_and_path_is_absolute() {
    // Amendment 1 layout: `.crawform` inside `<member>/src/`, beside the `.rs`
    // it generates. And the path comes out ABSOLUTE — the rule that replaced
    // the relative `FILE` that silently swapped the user's document.
    let r = group("crawrad_ws_member", &["app"]);
    std::fs::write(r.join("app/src/form_main.crawform"), "{}").expect("write");
    // These three are not documents and cannot enter: one is generated code,
    // the second is editor noise, and the third is in the group ROOT, where
    // Amendment 1 places no design. Without them the extension filter and folder
    // level are never exercised, and removing either would pass green.
    std::fs::write(r.join("app/src/form_main.rs"), "x").expect("write");
    std::fs::write(r.join("app/src/form_main.crawform.bak"), "x").expect("write");
    std::fs::write(r.join("on_root.crawform"), "{}").expect("write");

    let found = documents_in(&r);

    assert_eq!(names(&found), ["form_main.crawform"]);
    assert!(found[0].is_absolute(), "path must be absolute: {found:?}");
}

#[test]
fn active_member_comes_before_others() {
    // `default-members` IS the active project (ADR 0017, Amendment 1), and it
    // decides which form the IDE offers first. Without the ordering, the list
    // would come out in `members` order and active would appear in the middle of it.
    let r = group("crawrad_ws_active", &["app", "core"]);
    std::fs::write(
        r.join(MANIFEST),
        "[workspace]\nmembers = [\"app\", \"core\"]\ndefault-members = [\"core\"]\n",
    )
    .expect("write");
    std::fs::write(r.join("app/src/a.crawform"), "{}").expect("write");
    std::fs::write(r.join("core/src/c.crawform"), "{}").expect("write");

    assert_eq!(names(&members(&r)), ["core", "app"]);
    assert_eq!(
        names(&documents_in(&r)),
        ["c.crawform", "a.crawform"],
        "active member form comes first"
    );
}

#[test]
fn member_documents_come_out_in_order() {
    // **The second mutation that survived revision.** `sort()` was inside the
    // `documents_in` loop, and on NTFS `read_dir` already returns names in order —
    // the directory is a B-tree indexed by name. Removing the sort left the hundred
    // tests green, because no test passing through disk can distinguish the two.
    //
    // With a pure function, the input is chosen here: out of order deliberately,
    // and with a case disk order would never produce.
    let out_of_order = vec![
        PathBuf::from("/g/app/src/zebra.crawform"),
        PathBuf::from("/g/app/src/abelha.crawform"),
        PathBuf::from("/g/app/src/macaco.crawform"),
    ];
    assert_eq!(
        in_order(out_of_order)
            .iter()
            .filter_map(|c| c.file_name()?.to_str())
            .collect::<Vec<_>>(),
        ["abelha.crawform", "macaco.crawform", "zebra.crawform"],
        "list appears on screen: out of order is hard to navigate"
    );
}

#[test]
fn member_designs_are_only_its_own_and_only_crawform() {
    // The narrow scan, in one single place. It was duplicated in
    // `explorer::forms_on_disk` — line by line, down to the comment on case —
    // and revision caught the second copy (rule 6). This test came along with the function.
    //
    // **`Z` and `a`, not `a` and `b`:** NTFS indexes directories by UPPERCASE
    // name, so `read_dir` already returns `a` before `Z` — and removing the sort
    // would pass green with any pair differing only in letter. `PathBuf::sort`
    // compares bytes, where `Z` precedes `a`, and that difference is what makes
    // the order measured instead of asserted.
    let r = std::env::temp_dir().join("crawrad_ws_member_designs");
    let _ = std::fs::remove_dir_all(&r);
    std::fs::create_dir_all(r.join("src")).expect("create");
    std::fs::write(r.join("src/a.crawform"), "{}").expect("write");
    std::fs::write(r.join("src/Z.CRAWFORM"), "{}").expect("write");
    std::fs::write(r.join("src/main.rs"), "x").expect("write");
    std::fs::write(r.join("src/notas.txt"), "x").expect("write");
    std::fs::create_dir_all(r.join("src/sub")).expect("create");
    std::fs::write(r.join("src/sub/fundo.crawform"), "{}").expect("write");
    // A FOLDER with a design name. Without `is_file` it enters the list, and
    // the IDE offers to "open" a directory — `Document::load` fails and the UI
    // blames the document. It is the same trap as `a_folder_with_a_generated_file_name_does_not_count_as_generated`
    // in Explorer, and without it the mutation `is_file() || is_dir()` survives.
    std::fs::create_dir_all(r.join("src/impostor.crawform")).expect("create");

    let found: Vec<String> = member_documents(&r)
        .iter()
        .filter_map(|c| Some(c.file_name()?.to_str()?.to_owned()))
        .collect();

    // Casing cannot make a document vanish on Windows, order is what appears
    // on screen, and nothing that is not a design enters — not `.rs`, not `.txt`,
    // nor what is in a subfolder.
    assert_eq!(found, ["Z.CRAWFORM", "a.crawform"], "{found:?}");
}

#[test]
fn member_and_root_are_not_confused_in_design_scan() {
    // **Why the function is per MEMBER and not solved by `documents_in(member)`.**
    // In a root that is package AND workspace, `members()` returns declared
    // members, and `documents_in` scans all of them — Explorer's tree would show,
    // inside a node, the designs of its neighbor.
    let r = std::env::temp_dir().join("crawrad_ws_hybrid_root");
    let _ = std::fs::remove_dir_all(&r);
    std::fs::create_dir_all(r.join("src")).expect("create");
    std::fs::create_dir_all(r.join("core/src")).expect("create");
    std::fs::write(
        r.join(MANIFEST),
        "[package]\nname = \"app\"\n[workspace]\nmembers = [\"core\"]\n",
    )
    .expect("write");
    std::fs::write(r.join("core/Cargo.toml"), "[package]\nname = \"core\"\n").expect("write");
    std::fs::write(r.join("src/meu.crawform"), "{}").expect("write");
    std::fs::write(r.join("core/src/do_vizinho.crawform"), "{}").expect("write");

    let root_only: Vec<String> = member_documents(&r)
        .iter()
        .filter_map(|c| Some(c.file_name()?.to_str()?.to_owned()))
        .collect();

    assert_eq!(
        root_only,
        ["meu.crawform"],
        "neighbor design cannot enter this folder's node: {root_only:?}"
    );
    // And root scan continues doing what it promises: both.
    assert_eq!(
        documents_in(&r).len(),
        1,
        "hybrid root only declares `core`"
    );
}

#[test]
fn standalone_package_is_a_member_of_itself() {
    // Opening a folder that is a regular Rust package — made outside the IDE,
    // or cloned from a repository — cannot return "no project here" in front
    // of a valid project.
    let r = std::env::temp_dir().join("crawrad_ws_standalone");
    let _ = std::fs::remove_dir_all(&r);
    std::fs::create_dir_all(r.join("src")).expect("create");
    std::fs::write(r.join(MANIFEST), "[package]\nname = \"standalone\"\n").expect("write");
    std::fs::write(r.join("src/form_main.crawform"), "{}").expect("write");

    assert_eq!(members(&r), std::slice::from_ref(&r));
    assert_eq!(names(&documents_in(&r)), ["form_main.crawform"]);
}

#[test]
fn folder_without_manifest_is_not_error_and_ghost_member_does_not_enter() {
    // First run, photo folder opened by mistake, group still without any member:
    // none of these is a failure, and reading a `Cargo.toml` that does not exist
    // must not crash anything.
    let r = std::env::temp_dir().join("crawrad_ws_no_manifest");
    let _ = std::fs::remove_dir_all(&r);
    std::fs::create_dir_all(&r).expect("create");
    assert!(members(&r).is_empty());
    assert!(documents_in(&r).is_empty());

    // And a declared member not on disk cannot become a folder: `members` accepts
    // globs, which this parser does not resolve, and a ghost line in Explorer is
    // worse than one less line.
    std::fs::write(r.join(MANIFEST), "[workspace]\nmembers = [\"gone\"]\n").expect("write");
    assert!(members(&r).is_empty());
}

#[test]
fn members_and_default_members_are_not_confused() {
    // `fake-members` is what makes this test valid, and it comes BEFORE real
    // `members` deliberately: without checking `=` after prefix, `strip_prefix("members")`
    // matches ANY key starting with `members`, and the first to match is emitted —
    // its values would enter the list deciding which folders are projects. With the
    // line after, search would already stop on the right key and removing the check
    // would pass green: that was the mutation that survived the first round.
    let cargo = "\
[workspace]
resolver = \"2\"
fake-members = [\"not_a_member\"]
members = [\"app\", \"core\"]
default-members = [\"app\"]

[other-section]
members = [\"not_here\"]
";
    assert_eq!(workspace_list(cargo, "members"), ["app", "core"]);
    assert_eq!(workspace_list(cargo, "default-members"), ["app"]);

    // List split across multiple lines is how cargo writes when it grows,
    // and people write like that from day one.
    let multiline = "[workspace]\nmembers = [\n  \"app\",\n  \"core\",\n]\n";
    assert_eq!(workspace_list(multiline, "members"), ["app", "core"]);

    // Outside `[workspace]` does not count, and without `[workspace]` there is no list.
    assert!(workspace_list("[package]\nmembers = [\"x\"]\n", "members").is_empty());
}

#[test]
fn group_root_is_virtual_and_born_with_active() {
    // Amendment 1 decision: root is not a package. With `[package]` in it, the
    // day of the second application forces a choice between permanent asymmetry
    // and restructuring the user's folder.
    let text = group_cargo_toml("meu_app");
    assert!(!has_package(&text), "group root is not a package:\n{text}");
    // File comes out COMPLETE, with member and active. Writing half and amending
    // the rest later was the defect that broke revision: amendment fell inside
    // multiline `members` and cargo rejected the manifest.
    assert_eq!(workspace_list(&text, "members"), ["meu_app"]);
    assert_eq!(
        workspace_list(&text, "default-members"),
        ["meu_app"],
        "first member is born active"
    );
    // Without written resolver, cargo warns on EVERY compile of a virtual root —
    // and a warning that appears always is a warning one learns to ignore.
    assert!(text.contains("resolver"), "{text}");
}

#[test]
fn cargo_refusal_speaks_of_typed_name() {
    // The list of reserved names belongs to cargo, and continues to: measured,
    // `cargo new` refuses `con` ("reserved Windows filename"), `test`, `self`, `build`.
    // Its message alone does not say what to do; ours names what the person controls,
    // which is now the `Name` field rather than folder name (ADR 0017, Amendment 3).
    let r = refusal_new(
        "con",
        "invalid package name `con`: it is a reserved Windows filename",
    );
    assert!(
        r.phrase.contains("reserved Windows filename"),
        "{}",
        r.phrase
    );
    assert!(
        r.phrase.contains("`con`"),
        "phrase must name what the person typed: {}",
        r.phrase
    );
    assert!(
        !r.phrase.contains("rename the folder"),
        "phrase still mentions folder rename, and name no longer comes from folder: {}",
        r.phrase
    );
    // And the promise backed by creation ordering: nothing of ours is written
    // before cargo speaks.
    assert!(r.phrase.contains("Nothing was written"), "{}", r.phrase);
    // There is no project there to offer.
    assert!(
        !r.open_instead,
        "a refused package name is not an existing project — offering to open \
         would tell the person to open an empty folder"
    );
}

#[test]
fn occupied_folder_is_refused_before_cargo_and_offers_to_open() {
    // **The user's report turned into a test.** They created in `Downloads\teste`,
    // the IDE did not open the new project, they repeated the gesture in the SAME
    // folder, and received a technically correct sentence on the wrong subject:
    // *package name comes from folder `teste`; rename the folder*. They did not want
    // to rename anything — they wanted that project.
    //
    // With the typed name, collision is detected BEFORE cargo, and the phrase
    // explains itself.
    let parent = std::env::temp_dir().join(format!("crawrad-occupied-{}", std::process::id()));
    let with_project = parent.join("teste");
    std::fs::create_dir_all(&with_project).expect("test folder");
    std::fs::write(with_project.join(MANIFEST), "[package]\nname = \"teste\"\n").expect("manifest");

    let r = refusal_occupied_folder(&parent, "teste").expect("folder is occupied");
    assert!(
        r.open_instead,
        "refusal does not offer to open — and opening is exactly what the person \
         wanted in both attempts: {}",
        r.phrase
    );
    assert!(
        r.phrase.contains("`teste` already exists"),
        "phrase does not say what actually happened: {}",
        r.phrase
    );
    assert!(
        !r.phrase.contains("package name"),
        "phrase still talks about package name in a case that is not about name: {}",
        r.phrase
    );
    // Protection was NOT relaxed: offering to open is offering to open, not
    // overwrite — and now the promise is structural, because this refusal
    // happens before creation writes anything.
    assert!(
        r.phrase.contains("Nothing was written"),
        "promise not to write anything left the phrase: {}",
        r.phrase
    );

    // Folder with name, but WITHOUT project: also refused (nothing is overwritten),
    // and WITHOUT offering to open — opening an empty folder leads nowhere.
    let empty = parent.join("empty");
    std::fs::create_dir_all(&empty).expect("empty folder");
    let r = refusal_occupied_folder(&parent, "empty").expect("folder exists");
    assert!(
        !r.open_instead,
        "offered to open a folder with no project: {}",
        r.phrase
    );
    assert!(r.phrase.contains("Nothing was written"), "{}", r.phrase);

    // And clear path proceeds: without this line, a fixed `Some` would refuse every
    // creation and the suite above would remain green.
    assert_eq!(
        refusal_occupied_folder(&parent, "does_not_exist_yet"),
        None,
        "creation stopped being able to create any project"
    );
    let _ = std::fs::remove_dir_all(&parent);
}

#[test]
fn typed_name_is_rejected_on_the_spot_with_suggestion() {
    // The name ceased being deduced and started being typed, and with that
    // became capable of being wrong. Refusing on the spot, with reason AND
    // suggestion, is the difference between returning the problem and returning
    // the answer.
    let phrase = refusal_typed_name("Meu App").expect("`Meu App` is not a crate name");
    assert!(
        phrase.contains("meu_app"),
        "refusal suggests nothing, and `Meu App` is a perfectly normal name \
         for a human: {phrase}"
    );
    // Leading digit: Rust identifier cannot start with a number, and cargo refuses.
    let phrase = refusal_typed_name("2048").expect("name does not start with digit");
    assert!(phrase.contains("digit"), "{phrase}");
    assert!(
        phrase.contains("_2048"),
        "suggestion did not appear: {phrase}"
    );
    // Empty disables the button, and says what to do.
    assert!(refusal_typed_name("").is_some());

    // **The countermeasure**, and it is the half that matters: good names pass.
    // Without this line, a fixed `Some` would pass everything above — and the IDE
    // would create no project at all, with a green suite.
    for good in ["teste", "meu_app", "meu-app", "app2", "MeuApp", "_x"] {
        assert_eq!(
            refusal_typed_name(good),
            None,
            "`{good}` is a legitimate package name and was refused"
        );
    }
    // And cargo's word list was NOT copied here: `con` and `test` pass in form,
    // and who refuses them remains cargo — our copy would rot with toolchain (ADR 0018).
    assert_eq!(refusal_typed_name("con"), None);
    assert_eq!(refusal_typed_name("test"), None);
}

#[test]
fn leading_hyphen_is_rejected_here_not_by_cargo_clap() {
    // **The comment that was here claimed that `-app` fell into the form phrase,
    // and revision measured that it did not.** `-app` is ASCII letters and hyphen,
    // so it passed completely. What the user received in return was `error: unexpected
    // argument '-a' found` — cargo's clap reading the typed name as a flag.
    for bad in ["-app", "-", "--", "-meu-app"] {
        let phrase = refusal_typed_name(bad).unwrap_or_else(|| panic!("`{bad}` passed validation"));
        assert!(
            phrase.contains("cannot start with `-`"),
            "`{bad}` was refused for another reason, and phrase does not say what problem is: {phrase}"
        );
        // The useful half: a suggestion the person can accept. `-` and `--`
        // leave nothing, and then the suggestion is `app` — never an empty one.
        let suggested = suggested_name(bad);
        assert!(!suggested.is_empty(), "`{bad}` suggested nothing");
        assert!(
            refusal_typed_name(&suggested).is_none(),
            "suggestion for `{bad}` was `{suggested}`, which validation itself refuses"
        );
        assert!(
            phrase.contains(&suggested),
            "suggestion not included: {phrase}"
        );
    }
    // **The countermeasure.** Hyphen in the MIDDLE and END continue to be valid: whoever
    // refuses `app-`, if anyone does, is cargo, and this rule is only about the
    // first character — the only one that turns into a flag.
    for good in ["meu-app", "app-", "a-b-c"] {
        assert_eq!(
            refusal_typed_name(good),
            None,
            "`{good}` does not start with `-` and was refused"
        );
    }
}

#[test]
fn adoption_by_external_workspace_is_read_from_cargo_output() {
    // Measured on 08/09/2026, twice, one of them in this repository's `Cargo.toml`:
    // `cargo new` inside a folder that already lives in a workspace adds the package
    // to `members` there. The line below is what it wrote.
    //
    // Without a group (ADR 0017, Amendment 3) one cannot prevent this without
    // reimplementing cargo's decision, which is the opposite of ADR 0018. What we
    // can do is not keep silent — and whoever doesn't keep silent must know it happened.
    let output = "    Creating binary (application) `teste` package\n      \
                 Adding `teste` as member of workspace at `E:\\dev\\CrawRAD`\n\
                 note: see more `Cargo.toml` keys\n";
    assert_eq!(
        adopting_workspace(output).as_deref(),
        Some("E:\\dev\\CrawRAD"),
        "adoption became invisible, and with it the edit of someone else's file"
    );
    // Countermeasure: normal creation in a standalone folder warns nothing —
    // a warning appearing always is a warning one learns to ignore.
    let standalone = "    Creating binary (application) `teste` package\n\
                 note: see more `Cargo.toml` keys\n";
    assert_eq!(adopting_workspace(standalone), None);
    // And package name, which also comes between backticks, is not confused with path:
    // taking the FIRST pair of backticks would return `teste`.
    assert_ne!(adopting_workspace(output).as_deref(), Some("teste"));
}

/// **Upper output never had color, and runner cargo does**.
///
/// This is the test that was missing, and missing it cost four merges on top of
/// a red `main`. The one above passed on every machine and did not measure anything
/// CI does: it invents the cargo line by hand, and by hand nobody writes ANSI escapes.
///
/// The text below **is not invented**: it is byte-for-byte what
/// `CARGO_TERM_COLOR=always cargo new meu_app` wrote on this machine on 08/09/2026,
/// cargo 1.95.0, inside a workspace with `members = []` — the same command, in the
/// same cargo, that CI runs. CI exports that variable so GitHub logs come out colored,
/// `cargo test` inherits it, and `cargo new` in tests inherits it again.
///
/// Without cleaning, `starts_with("Adding ")` is false, adoption disappears, and
/// someone else's `Cargo.toml` is edited **without anyone reporting** — which is
/// the thing this product exists not to do.
#[test]
fn adoption_is_read_even_when_cargo_writes_colored_output() {
    let colored = "\u{1b}[1m\u{1b}[92m    Creating\u{1b}[0m binary (application) \
                    `meu_app` package\n\u{1b}[1m\u{1b}[92m      Adding\u{1b}[0m \
                    `meu_app` as member of workspace at \
                    `C:\\Users\\User\\AppData\\Local\\Temp\\medicao63`\n";
    assert_eq!(
        adopting_workspace(colored).as_deref(),
        Some("C:\\Users\\User\\AppData\\Local\\Temp\\medicao63"),
        "with CARGO_TERM_COLOR=always adoption becomes invisible, and someone else's Cargo.toml \
         changes without anyone reporting"
    );
    // And countermeasure holds up with color: standalone folder warns nothing.
    let standalone = "\u{1b}[1m\u{1b}[92m    Creating\u{1b}[0m binary (application) \
                 `meu_app` package\n";
    assert_eq!(adopting_workspace(standalone), None);
}

/// Cargo's colored `error:` also failed to match, and the damage
/// hit the USER, not CI: without this line creation in an occupied folder returned
/// `cargo failed`, [`refusal_new`] did not find `destination` nor `already exists`,
/// and the user lost *"open it instead"* — the button opening the project already there.
///
/// The line is measured on 08/09/2026 with `CARGO_TERM_COLOR=always cargo new`
/// in a folder that already had a project.
#[test]
fn cargo_error_is_read_even_colored_and_refusal_knows_which() {
    let colored = "\u{1b}[1m\u{1b}[92m    Creating\u{1b}[0m binary (application) \
                    `meu_app` package\n\u{1b}[1m\u{1b}[91merror\u{1b}[0m: \
                    destination `C:\\tmp\\meu_app` already exists\n\n\
                    Use `cargo init` to initialize the directory\n";
    let error = first_error(colored);
    assert!(
        error.starts_with("error: destination"),
        "cargo error line became generic phrase: {error}"
    );
    // And what that phrase DECIDES: the button to open instead of create.
    let refusal = refusal_new("meu_app", &error);
    assert!(
        refusal.open_instead,
        "occupied folder failed to offer `open it instead`: {}",
        refusal.phrase
    );
    // Without any error line, generic phrase remains the outcome.
    assert_eq!(first_error("    Creating binary\n"), "cargo failed");
}

/// [`without_color`] strips what the terminal adds and **strips nothing else**.
///
/// The half that matters is the second: cleanup eating real characters would
/// swap one defect for another, and the Windows paths it traverses are full of `\` and `[`.
#[test]
fn color_stripping_does_not_eat_text() {
    assert_eq!(without_color("no color at all"), "no color at all");
    assert_eq!(
        without_color("C:\\dev\\[meu] app\\src"),
        "C:\\dev\\[meu] app\\src",
        "bracket that did not come from ESC is text, and paths have them"
    );
    assert_eq!(
        without_color("\u{1b}[1m\u{1b}[92mAdding\u{1b}[0m x"),
        "Adding x"
    );
    assert_eq!(
        without_color("\u{1b}[38;5;9merror\u{1b}[m"),
        "error",
        "256 colors"
    );
    assert_eq!(
        without_color("a\u{1b}"),
        "a",
        "stray ESC at end does not hang"
    );
}

#[test]
fn creation_plan_has_steps_that_actually_happen() {
    // The list the dialog draws. What is measured here is its shape; matching
    // what `create_project` DOES is measured there, running real cargo.
    //
    // GUI with form: the full path. **Three steps, and they used to be five** —
    // group root and `git init` left with the group (ADR 0017, Amendment 3),
    // and whoever writes `.git` and `.gitignore` today is `cargo new`.
    assert_eq!(
        steps_for(ProjectType::Gui, true),
        [Step::Package, Step::Dependency, Step::Code]
    );
    // Console: no `cargo add` and no code — meaning, none of the 6.8 s.
    // It is the case where dialog appears and disappears.
    assert_eq!(steps_for(ProjectType::Console, false), [Step::Package]);
    // And `with_form` belongs to CALLER, not the type: `Option<Document>`
    // decides whether `.crawform` is written. Without this parameter, `Gui`
    // called without document would put an unexecuted step in the plan — and
    // the list on screen would stay stuck on the last step forever.
    assert_eq!(
        steps_for(ProjectType::Gui, false),
        [Step::Package, Step::Dependency],
        "plan followed type instead of caller's document"
    );
}

#[test]
fn most_expensive_step_shows_on_screen_which_dependency_it_fetches() {
    // The measured 6.8 s is this step, and only it. Label names dependency
    // and command: without this, seven seconds of waiting are indistinguishable
    // from a frozen IDE, which is exactly the user's complaint.
    let r = Step::Dependency.label(ProjectType::Gui);
    assert!(r.contains("eframe@0.36"), "{r}");
    assert!(
        r.contains("crates.io"),
        "phrase does not say delay is NETWORK: {r}"
    );
    // And number is not copied into label: comes from `dependency()`,
    // which is where version lives. A literal here would rot on next version
    // bump without warning.
    assert!(
        r.contains(ProjectType::Gui.dependency().expect("GUI has dependency")),
        "label ceased coming from `dependency()`: {r}"
    );
    // Every step has a label, and none is empty: blank line in middle of
    // list is worse than not showing the list.
    for e in [Step::Package, Step::Dependency, Step::Code] {
        assert!(
            !e.label(ProjectType::Console).trim().is_empty(),
            "{e:?} has no label"
        );
    }
}

#[test]
fn document_member_is_its_grandparent() {
    // `<member>/src/<name>.crawform` — member is grandparent. It decides
    // WHERE generation writes, and deriving it from path avoids a "current member"
    // field that could disagree with the open document.
    assert_eq!(
        document_member(Path::new("C:/g/app/src/form_main.crawform")),
        Some(PathBuf::from("C:/g/app"))
    );
    // Outside `src/` no member can be asserted, and guessing parent would
    // make generation write to wrong folder.
    assert_eq!(
        document_member(Path::new("C:/g/app/form_main.crawform")),
        None
    );
}

#[test]
fn none_one_and_several_are_three_different_answers() {
    // Decision of which document to open, without touching disk. Middle case
    // is what this enum exists not to miss: with more than one document IDE
    // DOES NOT CHOOSE — showing first in alphabetical order would be the same
    // family of error as relative path.
    assert_eq!(decide(Vec::new()), OnOpen::NoForm);

    let one = PathBuf::from("/g/app/src/a.crawform");
    assert_eq!(decide(vec![one.clone()]), OnOpen::One(one.clone()));

    let two = PathBuf::from("/g/app/src/b.crawform");
    assert_eq!(
        decide(vec![one.clone(), two.clone()]),
        OnOpen::Choose(vec![one, two]),
        "with two, user chooses"
    );
}

#[test]
fn reopened_folder_moves_to_top_without_duplication() {
    // Three classic recents list errors in one test: duplicate, push reopened
    // to bottom, and grow without limit.
    let a = PathBuf::from("/p/a");
    let b = PathBuf::from("/p/b");
    let c = PathBuf::from("/p/c");

    let list = register_recent(&[], &a);
    assert_eq!(list, std::slice::from_ref(&a));

    let list = register_recent(&list, &b);
    assert_eq!(list, [b.clone(), a.clone()], "new one enters at top");

    let list = register_recent(&list, &c);
    let list = register_recent(&list, &a);
    assert_eq!(
        list,
        [a.clone(), c.clone(), b.clone()],
        "reopening moves to top without leaving second copy"
    );
}

#[test]
fn recent_list_does_not_grow_forever() {
    // Without truncation, list becomes menu that doesn't fit screen — and a
    // menu that doesn't fit ceases to be a shortcut. Test exceeds limit on
    // purpose: with `RECENT_LIMIT` items it would still pass without any truncation.
    let mut list: Vec<PathBuf> = Vec::new();
    for i in 0..RECENT_LIMIT + 5 {
        list = register_recent(&list, Path::new(&format!("/p/{i}")));
    }
    assert_eq!(list.len(), RECENT_LIMIT);
    assert_eq!(
        list[0],
        PathBuf::from(format!("/p/{}", RECENT_LIMIT + 4)),
        "last opened remains at top after truncate"
    );
    assert!(!list.contains(&PathBuf::from("/p/0")), "oldest is pruned");
}

#[test]
fn register_recent_respects_configured_limit() {
    let mut list: Vec<PathBuf> = Vec::new();
    let limit = 5;
    for i in 0..8 {
        list = register_recent_with_limit(&list, Path::new(&format!("/p/{i}")), limit);
    }
    assert_eq!(list.len(), 5, "must respect configured limit of 5, not 10");
    assert_eq!(list[0], PathBuf::from("/p/7"), "last opened remains at top");
    assert!(!list.contains(&PathBuf::from("/p/0")), "oldest is pruned");
}

#[test]
fn shrinking_limit_shrinks_saved_list() {
    // Start with 10 projects
    let mut list: Vec<PathBuf> = Vec::new();
    for i in 0..10 {
        list = register_recent_with_limit(&list, Path::new(&format!("/p/{i}")), 10);
    }
    assert_eq!(list.len(), 10);

    // Lower limit to 5: the next registration shrinks the list to 5 items,
    // dropping projects beyond the fifth from the stored list.
    let list = register_recent_with_limit(&list, Path::new("/p/new"), 5);
    assert_eq!(list.len(), 5, "lowering limit must shrink list to 5");
    assert_eq!(list[0], PathBuf::from("/p/new"));
    assert_eq!(list[1], PathBuf::from("/p/9"));
    assert_eq!(list[4], PathBuf::from("/p/6"));
    assert!(!list.contains(&PathBuf::from("/p/5")));
    assert!(!list.contains(&PathBuf::from("/p/0")));
}

#[test]
fn raising_limit_does_not_invent_projects() {
    let mut list: Vec<PathBuf> = Vec::new();
    for i in 0..5 {
        list = register_recent_with_limit(&list, Path::new(&format!("/p/{i}")), 5);
    }
    assert_eq!(list.len(), 5);

    // Raising limit to 20 does not invent new projects
    let list = register_recent_with_limit(&list, Path::new("/p/4"), 20);
    assert_eq!(list.len(), 5, "raising limit does not invent projects");
    assert_eq!(list[0], PathBuf::from("/p/4"));
}

#[test]
fn vanished_folder_leaves_the_list() {
    // Deleted folder, unplugged thumb drive, moved project. Predicate enters
    // as parameter so test doesn't need to create and delete real folders.
    let alive = PathBuf::from("/p/alive");
    let dead = PathBuf::from("/p/dead");
    let pruned = prune(&[dead.clone(), alive.clone()], |c| c != dead);
    assert_eq!(pruned, [alive], "only what still exists stays");
}

#[test]
fn recent_file_survives_round_trip() {
    // It is hand-editable, and someone will edit it by hand: blank line and
    // duplicate line enter, and cannot become list entries.
    let list = vec![PathBuf::from("C:\\p\\one"), PathBuf::from("C:\\p\\two")];
    assert_eq!(from_text(&to_text(&list)), list);

    assert_eq!(
        from_text("  C:\\p\\one  \n\n C:\\p\\one \nC:\\p\\two\n"),
        list,
        "spaces, empty lines and duplicates do not enter"
    );
}

#[test]
fn human_text_becomes_crate_name_suggestion() {
    // `Meu App` is a normal name for a human, and `cargo new` refuses spaces,
    // accents, and leading digits. The function USED TO be what decided name
    // silently from folder; today it SUGGESTS, and suggestion is the useful
    // half of rejection phrase (ADR 0017, Amendment 3).
    assert_eq!(suggested_name("Meu App"), "meu_app");
    assert_eq!(suggested_name("meu-app"), "meu_app");
    assert_eq!(suggested_name("Ação"), "ao");
    assert_eq!(
        suggested_name("2048"),
        "_2048",
        "Rust identifier cannot start with digit"
    );
    assert_eq!(
        suggested_name("日本"),
        "app",
        "nothing left: a fixed name is better than empty suggestion"
    );
    // **Suggestion is always acceptable**, and is guard connecting both functions:
    // suggesting a name the IDE itself would reject would be worse than not suggesting.
    for raw in ["Meu App", "meu-app", "Ação", "2048", "日本", "a b c", "---"] {
        let s = suggested_name(raw);
        assert_eq!(
            refusal_typed_name(&s),
            None,
            "suggestion for `{raw}` was `{s}`, which IDE rejects"
        );
    }
}

#[test]
fn application_name_comes_from_package_and_nowhere_else() {
    // Answer of ADR 0017 to "what is the application name". `name` appears
    // in `[[bin]]`, in `[lib]`, and in `[dependencies]` long form — reading
    // first `name` in file would pick wrong one in any project with named binary.
    let cargo = "\
[package]
name = \"meu_app\"
version = \"0.1.0\"

[[bin]]
name = \"outro\"

[dependencies]
eframe = \"0.36\"
";
    assert_eq!(package_name_from_manifest(cargo), Some("meu_app".into()));

    let upside_down = "\
[[bin]]
name = \"primeiro_do_arquivo\"

[package]
name = \"o_certo\"
";
    assert_eq!(
        package_name_from_manifest(upside_down),
        Some("o_certo".into()),
        "order in file does not decide: section decides"
    );

    assert_eq!(
        package_name_from_manifest("[package]\nversion = \"0.1.0\"\n"),
        None
    );
}

#[test]
fn the_three_types_differ_only_in_what_cargo_receives() {
    // The ruler of ADR 0018, Amendment 1: a type enters when it only changes
    // HOW `cargo` is invoked. This test is what enforces that — a new type
    // needing something else (different generator, different layout) would have
    // no way to describe itself here, and would hit this assert.
    assert_eq!(ProjectType::ALL.len(), 3);

    assert_eq!(ProjectType::Gui.flag(), "--bin");
    assert_eq!(ProjectType::Console.flag(), "--bin");
    assert_eq!(ProjectType::Library.flag(), "--lib");

    // Only GUI carries a dependency, which is eframe.
    assert_eq!(ProjectType::Gui.dependency(), Some("eframe@0.36"));
    assert_eq!(ProjectType::Console.dependency(), None);
    assert_eq!(ProjectType::Library.dependency(), None);

    // And form goes hand-in-hand with eframe: a type with form and without
    // dependency generates uncompilable code, and one with dependency without
    // form installs eframe for nobody.
    for t in ProjectType::ALL {
        assert_eq!(
            t.with_form(),
            t.dependency().is_some(),
            "{}: form and eframe must go together",
            t.label()
        );
        assert!(!t.label().is_empty());
        assert!(!t.description().is_empty());
    }

    // Two types with the same label would be two identical rows in the dialog.
    let labels: std::collections::BTreeSet<&str> =
        ProjectType::ALL.iter().map(|t| t.label()).collect();
    assert_eq!(labels.len(), ProjectType::ALL.len());
}

#[test]
fn cargo_new_creates_project_in_subfolder_with_typed_name() {
    // `new` not `init`: project is born in a SUBFOLDER with the typed name
    // (ADR 0017, Amendment 3) — what `cargo new` does in terminal, and what
    // the user requested. Without `--name` cargo deduces name from path; without
    // flag "Library" comes out as binary, which is the entire wrong type.
    assert_eq!(
        new_args("meu_app", ProjectType::Gui),
        ["new", "meu_app", "--name", "meu_app", "--bin"]
    );
    assert_eq!(
        new_args("minha_lib", ProjectType::Library),
        ["new", "minha_lib", "--name", "minha_lib", "--lib"]
    );
    // **`--vcs none` LEFT**, and its reason left with the group: it existed
    // because repository and `.gitignore` were ours, written at group root.
    // Without a group, `cargo new` decides VCS — and measured on 08/09/2026 it
    // writes `.git` and a `.gitignore` with `/target`, exactly what was handwritten.
    for t in ProjectType::ALL {
        let args = new_args("x", *t);
        assert!(
            !args.iter().any(|a| a == "--vcs"),
            "creation resumed deciding VCS instead of cargo, and nobody writes \
             `.gitignore` anymore — {args:?} ({})",
            t.label()
        );
        // And `--name` stays on all three: ensures typed name is what goes to
        // `[package]`, instead of cargo deducing it.
        assert!(
            args.windows(2).any(|p| p == ["--name", "x"]),
            "{}: typed name ceased being asserted — {args:?}",
            t.label()
        );
    }
}
// ------------------------------------------- recents and preferences

/// Instant of `days` ago, for [`time_ago`] boundaries.
fn days_ago(days: u64) -> std::time::SystemTime {
    std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(days * 86_400)
}

#[test]
fn recent_age_is_expressed_in_words_at_correct_boundaries() {
    // "Now" enters as parameter precisely so test can step on day boundary —
    // with internal clock, proving "yesterday" would require waiting for midnight.
    let now = days_ago(1_000);
    for (days, expected) in [
        (0, "today"),
        (1, "yesterday"),
        (2, "2 days ago"),
        (13, "13 days ago"),
        (14, "2 weeks ago"),
        (60, "8 weeks ago"),
        (61, "2 months ago"),
        (364, "12 months ago"),
        (365, "a year ago"),
        (900, "2 years ago"),
    ] {
        assert_eq!(
            time_ago(now, days_ago(1_000 - days)),
            expected,
            "{days} days ago"
        );
    }
}

#[test]
fn future_date_does_not_become_negative() {
    // File copied from another machine, wrong timezone, NTP correcting clock
    // backwards: folder has future date, and it is common enough to have a
    // written answer. Without this, `duration_since` returns error and line on
    // screen would have no age at all.
    assert_eq!(time_ago(days_ago(1), days_ago(10)), "today");
}

#[test]
fn open_preference_survives_text_round_trip() {
    for on in [false, true] {
        for &scheme in crawrad_actions::KeymapScheme::ALL {
            for &color_scheme in EditorColorScheme::ALL {
                let p = Preferences {
                    welcome_on_startup: on,
                    last_form: on
                        .then(|| PathBuf::from("C:/projects/my_app/src/form_main.crawform")),
                    keymap_scheme: scheme,
                    editor_font_size: 14,
                    editor_tab_size: 2,
                    editor_line_numbers: !on,
                    editor_inlay_hints: on,
                    editor_current_line: !on,
                    editor_breadcrumb_chevron: on,
                    editor_color_scheme: color_scheme,
                    editor_color_base: if on {
                        EditorColorScheme::ClassicDark
                    } else {
                        EditorColorScheme::ModernDark
                    },
                    editor_custom_colors: EditorCustomColors::default(),
                    editor_diagnostic_colors: EditorDiagnosticColors::default(),
                    color_recent: if on {
                        vec!["#ff0000".to_string(), "#00ff00".to_string()]
                    } else {
                        Vec::new()
                    },
                    default_projects_dir: if on {
                        Some(PathBuf::from("C:\\Projects"))
                    } else {
                        None
                    },
                    history_enabled: on,
                    history_limit: if on { 20 } else { 10 },
                    history_merge_seconds: if on { 30 } else { 0 },
                    history_max_file_kb: if on { 10240 } else { 256 },
                    recent_limit: if on { 15 } else { 5 },
                    save_before_build: on,
                    auto_save: AutoSave::ALL[usize::from(on)],
                    save_desktop: !on,
                    desktop_profile: if on { "wide".into() } else { "Default".into() },
                    recover_copies: !on,
                    recover_minutes: if on { 1 } else { 30 },
                    designer_grid: if on { 5 } else { 16 },
                    designer_snap: !on,
                    designer_show_grid: !on,
                    designer_show_tab_order: on,
                    designer_scale: if on { 1.1 } else { 1.0 },
                    ide_scale: if on { 1.25 } else { 1.0 },
                };
                assert_eq!(
                    preferences_from_text(&preferences_to_text(p.clone())),
                    p,
                    "preference did not survive file"
                );
            }
        }
    }
}

#[test]
fn a_grid_size_out_of_range_keeps_the_default() {
    for raw in [
        "designer-grid=1\n",
        "designer-grid=65\n",
        "designer-grid=x\n",
    ] {
        assert_eq!(preferences_from_text(raw).designer_grid, 8, "{raw}");
    }
    assert_eq!(preferences_from_text("designer-grid=4\n").designer_grid, 4);
}

#[test]
fn preference_history_range_checks() {
    let raw_invalid = "history-limit=0\nhistory-merge-seconds=4000\nhistory-max-file-kb=0\n";
    let p = preferences_from_text(raw_invalid);
    assert_eq!(p.history_limit, 10);
    assert_eq!(p.history_merge_seconds, 10);
    assert_eq!(p.history_max_file_kb, 5120);

    let raw_valid = "history-limit=50\nhistory-merge-seconds=60\nhistory-max-file-kb=10240\n";
    let p_valid = preferences_from_text(raw_valid);
    assert_eq!(p_valid.history_limit, 50);
    assert_eq!(p_valid.history_merge_seconds, 60);
    assert_eq!(p_valid.history_max_file_kb, 10240);
}

#[test]
fn preference_recent_limit_range_checks() {
    // Normal value
    assert_eq!(preferences_from_text("recent-limit=5\n").recent_limit, 5);
    // Boundary low (1) and high (50)
    assert_eq!(preferences_from_text("recent-limit=1\n").recent_limit, 1);
    assert_eq!(preferences_from_text("recent-limit=50\n").recent_limit, 50);
    // Missing (falls back to default 10)
    assert_eq!(preferences_from_text("").recent_limit, 10);
    // Zero (falls back to default 10)
    assert_eq!(preferences_from_text("recent-limit=0\n").recent_limit, 10);
    // Negative (falls back to default 10)
    assert_eq!(preferences_from_text("recent-limit=-5\n").recent_limit, 10);
    // Above ceiling (falls back to default 10)
    assert_eq!(preferences_from_text("recent-limit=51\n").recent_limit, 10);
    assert_eq!(
        preferences_from_text("recent-limit=999999\n").recent_limit,
        10
    );
    // Non-numeric
    assert_eq!(
        preferences_from_text("recent-limit=invalid\n").recent_limit,
        10
    );
    // Alternate key alias
    assert_eq!(
        preferences_from_text("recent-projects-limit=15\n").recent_limit,
        15
    );
    // Round-trip serialization
    let p = Preferences {
        recent_limit: 20,
        ..Default::default()
    };
    let serialized = preferences_to_text(p);
    assert!(serialized.contains("recent-limit=20"));
    assert_eq!(preferences_from_text(&serialized).recent_limit, 20);
}

#[test]
fn test_editor_color_scheme_hex_and_presets() {
    let custom = EditorCustomColors {
        keyword: "#112233".to_string(),
        typename: "#445566".to_string(),
        function: "#778899".to_string(),
        identifier: "#aabbcc".to_string(),
        comment: "#ddeeff".to_string(),
        number: "#123456".to_string(),
        string: "#654321".to_string(),
        delimiter: "#abcdef".to_string(),
        bg: "#000000".to_string(),
    };

    let modern = EditorColorScheme::ModernDark.colors(&custom);
    // Modern Dark default keyword is #ff7b72
    assert!((modern.branch_keyword[0] - 1.0).abs() < 0.01);

    let cust_colors = EditorColorScheme::Custom.colors(&custom);
    let kw_vec = hex_to_rgba("#112233").expect("valid hex");
    assert_eq!(cust_colors.branch_keyword, kw_vec);

    let parsed =
        preferences_from_text("editor-color-scheme=monokai\neditor-color-keyword=#ff00ff\n");
    assert_eq!(parsed.editor_color_scheme, EditorColorScheme::Monokai);
    assert_eq!(parsed.editor_custom_colors.keyword, "#ff00ff");
}

/// The UI framework left the preferences for the project: a file
/// still carrying the old `target-codegen` key reads without error, the key
/// ignored.
#[test]
fn the_old_target_key_is_read_and_ignored() {
    for old in ["target-codegen=egui\n", "target-codegen=flutter\n"] {
        assert_eq!(preferences_from_text(old), Preferences::default());
    }
}

#[test]
fn without_a_saved_preference_the_recent_list_opens() {
    // This test asserted the opposite until 27/11/2026, for a reason worth keeping
    // in view: an existing user should not have startup change under them because
    // of a box they never ticked. What it missed is that reopening was ALSO never
    // asked for — the IDE decided, on a machine it had never run on, to drop a
    // person straight into a file. Someone who did tick the box keeps reopening:
    // their preference is saved, and this default only answers an empty file.
    assert!(
        preferences_from_text("").welcome_on_startup,
        "with no saved preference the IDE reopened a project nobody asked it to"
    );
    assert!(
        !preferences_from_text(
            "welcome-on-startup=false
"
        )
        .welcome_on_startup,
        "a saved choice to reopen must still be honoured"
    );
    // And unknown key — from a future version, or manual file editing — does
    // not break reading the others.
    let p = preferences_from_text("cor-do-tema=verde\nwelcome-on-startup=true\nlixo\n");
    assert!(p.welcome_on_startup, "known key must be read");
}

#[test]
fn recent_counts_forms_by_same_scan_as_opening() {
    // A separate counter here could say 3 while opening finds 2 — and screen
    // would promise a form that doesn't open.
    let root = std::env::temp_dir().join("crawrad_recent_describe");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("app/src")).expect("create test folder");
    std::fs::write(root.join("Cargo.toml"), group_cargo_toml("app")).expect("group");
    std::fs::write(root.join("app/Cargo.toml"), "[package]\nname = \"app\"\n").expect("member");
    std::fs::write(root.join("app/src/form_main.crawform"), "{}").expect("design");
    std::fs::write(root.join("app/src/other.crawform"), "{}").expect("other design");

    let r = describe_recent(&root);
    assert_eq!(r.path, root);
    assert_eq!(
        r.forms,
        documents_in(&root).len(),
        "screen count and opening scan are the same"
    );
    assert_eq!(r.forms, 2);
    assert!(r.modified.is_some(), "date comes from file and exists");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn folder_without_form_still_has_age() {
    // Console project: no `.crawform`, and recent row cannot be left without
    // date because of that — the folder has an age.
    let root = std::env::temp_dir().join("crawrad_recent_no_form");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("create test folder");
    let r = describe_recent(&root);
    assert_eq!(r.forms, 0);
    assert!(
        r.modified.is_some(),
        "folder exists and has date, even without design"
    );
    let _ = std::fs::remove_dir_all(&root);
}

/// New Project's workspace manifest names its first project as the only
/// member AND the active one, and reads back through the same parser.
#[test]
fn the_new_workspace_lists_its_first_project_as_the_active_member() {
    let manifest = workspace_manifest("meu_app");
    assert_eq!(
        workspace_list(&manifest, "members"),
        vec!["meu_app".to_string()]
    );
    assert_eq!(
        workspace_list(&manifest, "default-members"),
        vec!["meu_app".to_string()]
    );
    assert!(
        !has_package(&manifest),
        "a virtual manifest: no [package] at the root"
    );
    let r = std::env::temp_dir().join(format!("crawrad_ws471_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&r);
    std::fs::create_dir_all(r.join("meu_app")).expect("member dir");
    std::fs::write(r.join(MANIFEST), &manifest).expect("root manifest");
    std::fs::write(
        r.join("meu_app").join(MANIFEST),
        "[package]\nname = \"meu_app\"\n",
    )
    .expect("member manifest");
    assert_eq!(members(&r), vec![r.join("meu_app")]);
    let _ = std::fs::remove_dir_all(&r);
}

/// **The diagnostic colours survive a save and stay apart from the scheme**:
/// they round-trip through the preferences file, a preset never touches
/// them, and an input that is not a colour keeps the old one.
#[test]
fn diagnostic_colors_round_trip_and_ignore_a_bad_input() {
    let mut p = Preferences::default();
    assert_eq!(p.editor_diagnostic_colors.warning, "#fab387");
    let inputs = [
        "#ff0000".to_string(),
        "orange".to_string(),
        String::new(),
        "#200000".to_string(),
        "#201000".to_string(),
        "#000020".to_string(),
    ];
    assert!(p.editor_diagnostic_colors.update_from_fields(&inputs));
    assert_eq!(p.editor_diagnostic_colors.error, "#ff0000");
    assert_eq!(
        p.editor_diagnostic_colors.warning, "#fab387",
        "not a colour: kept"
    );
    assert_eq!(p.editor_diagnostic_colors.info, "#89b4fa", "empty: kept");

    p.editor_color_scheme = EditorColorScheme::Monokai;
    let back = preferences_from_text(&preferences_to_text(p.clone()));
    assert_eq!(back.editor_diagnostic_colors, p.editor_diagnostic_colors);
    assert_eq!(back.editor_color_scheme, EditorColorScheme::Monokai);

    let rgba = back.editor_diagnostic_colors.rgba();
    assert_eq!(rgba[0], [1.0, 0.0, 0.0, 1.0]);
}

/// **The page's fifteen inputs**: a syntax colour edited turns the scheme
/// Custom, a diagnostic colour edited leaves the scheme alone, and Reset
/// Default brings both groups back.
#[test]
fn the_color_page_updates_syntax_and_diagnostics_apart() {
    let mut p = Preferences::default();
    let mut inputs: [String; 15] = p.color_fields().map(str::to_string);
    inputs[10] = "#ff8800".to_string();
    assert!(p.update_color_fields(&inputs));
    assert_eq!(p.editor_diagnostic_colors.warning, "#ff8800");
    assert_eq!(
        p.editor_color_scheme,
        EditorColorScheme::ModernDark,
        "a diagnostic colour keeps the scheme"
    );
    inputs[0] = "#123456".to_string();
    assert!(p.update_color_fields(&inputs));
    assert_eq!(p.editor_color_scheme, EditorColorScheme::Custom);
    assert!(!p.update_color_fields(&inputs), "nothing new: no change");
    p.reset_colors();
    assert_eq!(p.color_fields(), Preferences::default().color_fields());
}

/// The start form: the one that WAS open, falling back to the first form of the
/// most recent project — and nothing when the recent list opens instead, there is
/// no recent, or the recent has no form.
#[test]
fn the_last_project_form_comes_from_the_first_recent() {
    let recent = vec![PathBuf::from("/p/latest"), PathBuf::from("/p/older")];
    let docs = |root: &Path| -> Vec<PathBuf> {
        if root == Path::new("/p/latest") {
            vec![
                root.join("src/main.crawform"),
                root.join("src/other.crawform"),
            ]
        } else {
            vec![root.join("src/x.crawform")]
        }
    };
    // Reopening is now opt-in, so the test states it instead of leaning on the default.
    let on = Preferences {
        welcome_on_startup: false,
        ..Preferences::default()
    };
    assert_eq!(
        last_project_form(&on, &recent, docs),
        Some(PathBuf::from("/p/latest").join("src/main.crawform"))
    );
    let off = Preferences::default();
    assert!(off.welcome_on_startup, "the recent list is the default");
    assert_eq!(last_project_form(&off, &recent, docs), None);
    assert_eq!(last_project_form(&on, &[], docs), None);
    assert_eq!(last_project_form(&on, &recent, |_| Vec::new()), None);
}

#[test]
fn save_before_build_is_off_until_asked() {
    assert!(!Preferences::default().save_before_build);
    assert!(preferences_from_text("save-before-build=true\n").save_before_build);
}

#[test]
fn designer_scale_preference_round_trips_and_rejects_out_of_range() {
    let def = Preferences::default();
    assert_eq!(def.designer_scale, 1.0);

    let parsed = preferences_from_text("designer-zoom=2.0\n");
    assert_eq!(parsed.designer_scale, 2.0);

    let parsed_low = preferences_from_text("designer-zoom=0.25\n");
    assert_eq!(parsed_low.designer_scale, 0.25);

    let parsed_high = preferences_from_text("designer-zoom=4.0\n");
    assert_eq!(parsed_high.designer_scale, 4.0);

    // Out-of-range values: rejected on read, keeping default
    let rejected_too_low = preferences_from_text("designer-zoom=0.1\n");
    assert_eq!(rejected_too_low.designer_scale, 1.0);

    let rejected_too_high = preferences_from_text("designer-zoom=10.0\n");
    assert_eq!(rejected_too_high.designer_scale, 1.0);

    let rejected_negative = preferences_from_text("designer-zoom=-1.5\n");
    assert_eq!(rejected_negative.designer_scale, 1.0);

    // Round-trip
    let p = Preferences {
        designer_scale: 1.5,
        ..Default::default()
    };
    let serialized = preferences_to_text(p);
    assert!(serialized.contains("designer-zoom=1.5\n"));
    let restored = preferences_from_text(&serialized);
    assert_eq!(restored.designer_scale, 1.5);
}

/// A Welcome row is already in words: name, the folder above, the
/// forms counted in English, and the age — empty when the folder has no form.
#[test]
fn welcome_row_speaks_the_recent_in_words() {
    let now = days_ago(1_000);
    let folder = PathBuf::from("work").join("orders");
    for (forms, modified, said, age) in [
        (0, None, "no form", ""),
        (1, Some(days_ago(1_000)), "1 form", "today"),
        (3, Some(days_ago(995)), "3 forms", "5 days ago"),
    ] {
        let row = WelcomeRow::of(
            &Recent {
                path: folder.clone(),
                forms,
                modified,
            },
            now,
        );
        assert_eq!(row.name, "orders");
        assert_eq!(row.place, "work");
        assert_eq!(row.folder, folder);
        assert_eq!(row.forms, said);
        assert_eq!(row.age, age);
    }
}

/// Auto save: off until asked, the four choices round-trip through the
/// file, and anything unknown falls back to off.
#[test]
fn auto_save_round_trips_and_falls_back_to_off() {
    assert_eq!(Preferences::default().auto_save, AutoSave::Off);
    for a in AutoSave::ALL {
        let p = Preferences {
            auto_save: a,
            ..Preferences::default()
        };
        assert_eq!(preferences_from_text(&preferences_to_text(p)).auto_save, a);
        assert_eq!(AutoSave::ALL[a.position()], a);
    }
    assert_eq!(
        preferences_from_text("auto-save=afterDelay\n").auto_save,
        AutoSave::Off
    );
    assert_eq!(AutoSave::labels()[1], "After a delay (1000 ms)");
}

/// The invalid syntax colour is refused: update_from_fields ignores bad hex
/// and preserves the existing color.
#[test]
fn invalid_syntax_color_is_refused() {
    let mut colors = EditorCustomColors::default();
    let original_kw = colors.keyword.clone();
    let mut inputs: [String; 9] = colors.fields().map(str::to_string);
    inputs[0] = "#b5cea".to_string(); // Incomplete hex (5 digits)
    assert!(!colors.update_from_fields(&inputs));
    assert_eq!(colors.keyword, original_kw);

    inputs[0] = "#123456".to_string();
    assert!(colors.update_from_fields(&inputs));
    assert_eq!(colors.keyword, "#123456");
}

/// Color field validation returns error for invalid or empty inputs.
#[test]
fn color_field_errors_detects_invalid_and_empty_inputs() {
    let mut inputs: [String; 15] = Preferences::default().color_fields().map(str::to_string);
    let errs = Preferences::color_field_errors(&inputs);
    assert_eq!(errs, [None; 15]);

    inputs[0] = "#b5cea".to_string();
    inputs[2] = "".to_string();
    inputs[9] = "red".to_string();
    let errs = Preferences::color_field_errors(&inputs);
    assert_eq!(errs[0], Some(NOT_A_COLOUR));
    assert_eq!(errs[1], None);
    assert_eq!(errs[2], Some(NOT_A_COLOUR));
    assert_eq!(errs[9], Some(NOT_A_COLOUR));
}

/// Picking a preset records it as the base, and editing a color shows the base in Custom label.
#[test]
fn pick_scheme_and_preset_labels_show_base() {
    let mut p = Preferences::default();
    p.pick_scheme(EditorColorScheme::Monokai);
    assert_eq!(p.editor_color_scheme, EditorColorScheme::Monokai);
    assert_eq!(p.editor_color_base, EditorColorScheme::Monokai);
    assert_eq!(p.editor_custom_colors.keyword, "#f92672");

    let mut inputs: [String; 15] = p.color_fields().map(str::to_string);
    inputs[0] = "#ff0000".to_string();
    assert!(p.update_color_fields(&inputs));
    assert_eq!(p.editor_color_scheme, EditorColorScheme::Custom);
    assert_eq!(p.editor_color_base, EditorColorScheme::Monokai);

    let labels = EditorColorScheme::preset_labels(p.editor_color_base);
    assert_eq!(labels[3], "Custom (based on Vivid Dark)");

    p.reset_colors();
    assert_eq!(p.editor_color_scheme, EditorColorScheme::ModernDark);
    assert_eq!(p.editor_color_base, EditorColorScheme::ModernDark);
    let reset_labels = EditorColorScheme::preset_labels(p.editor_color_base);
    assert_eq!(reset_labels[3], "Custom (based on Modern Dark)");
}

/// Editor color base and recent colors round trip through the preferences text format.
#[test]
fn color_base_and_recent_round_trip_through_preferences_text() {
    let p = Preferences {
        editor_color_base: EditorColorScheme::ClassicDark,
        color_recent: vec!["#112233".into(), "#445566".into(), "#778899".into()],
        ..Preferences::default()
    };
    let text = preferences_to_text(p.clone());
    let back = preferences_from_text(&text);
    assert_eq!(back.editor_color_base, EditorColorScheme::ClassicDark);
    assert_eq!(back.color_recent, p.color_recent);

    // Reading drops invalid hex and caps at 8
    let raw = "editor-color-base=monokai\ncolor-recent=#111111,not-a-colour,#222222,#333333,,#444444,#555555,#666666,#777777,#888888,#999999\n";
    let loaded = preferences_from_text(raw);
    assert_eq!(loaded.editor_color_base, EditorColorScheme::Monokai);
    assert_eq!(
        loaded.color_recent,
        vec![
            "#111111", "#222222", "#333333", "#444444", "#555555", "#666666", "#777777", "#888888"
        ]
    );
}

#[test]
fn color_field_names_count_and_ninth_item() {
    assert_eq!(COLOR_FIELDS.len(), 15);
    assert_eq!(COLOR_FIELDS[8], ("bg", "Background"));
}

#[test]
fn cargo_version_reads_the_release_line() {
    assert_eq!(
        cargo_version("cargo 1.95.0 (f2d3ce0bd 2026-03-21)\n"),
        Some((1, 95))
    );
}

#[test]
fn cargo_version_reads_a_nightly() {
    assert_eq!(
        cargo_version("cargo 1.97.0-nightly (abc 2026-09-01)"),
        Some((1, 97))
    );
}

#[test]
fn cargo_version_ignores_other_text() {
    assert_eq!(cargo_version(""), None);
    assert_eq!(cargo_version("error: no such command"), None);
    assert_eq!(cargo_version("cargo x.y"), None);
}

#[test]
fn old_toolchain_is_refused_and_says_both_numbers() {
    let refusal = refusal_old_toolchain((1, 87), (1, 88)).expect("refusal expected");
    assert!(
        refusal.phrase.contains("needs Rust 1.88 or newer"),
        "{}",
        refusal.phrase
    );
    assert!(
        refusal.phrase.contains("this machine has 1.87"),
        "{}",
        refusal.phrase
    );
    assert!(refusal.phrase.contains("rustup update stable"));
    assert!(refusal.phrase.contains("Nothing was written"));
    assert!(!refusal.open_instead);
}

#[test]
fn equal_or_newer_toolchain_passes() {
    assert!(refusal_old_toolchain((1, 88), (1, 88)).is_none());
    assert!(refusal_old_toolchain((2, 0), (1, 95)).is_none());
    assert!(refusal_old_toolchain((1, 100), (1, 95)).is_none());
}

#[test]
fn cargo_version_skips_lines_before_its_own() {
    // rustup can print a note before cargo answers (a toolchain being synced).
    assert_eq!(
        cargo_version("info: syncing channel updates\ncargo 1.95.0 (f2d3ce0bd 2026-03-21)"),
        Some((1, 95))
    );
}

#[test]
fn shorten_middle_keeps_both_ends_within_max() {
    let long = "C:/Users/dev/AppData/Local/Temp/build/0a50886581f4270c/feb2bbf0/scratchpad";
    let short = super::shorten_middle(long, super::PLACE_MAX_CHARS);
    assert_eq!(short.chars().count(), super::PLACE_MAX_CHARS);
    assert!(short.starts_with("C:/Users/dev/"));
    assert!(short.ends_with("/scratchpad"));
    assert!(short.contains('…'));
    assert_eq!(
        super::shorten_middle("C:/crawrad-wt", super::PLACE_MAX_CHARS),
        "C:/crawrad-wt"
    );
}

/// The New Project dialog must not open pointing at a folder nobody can write to.
///
/// Installed from the Microsoft Store, the process working directory IS the
/// package's own `WindowsApps` folder, which Windows keeps read-only. The dialog
/// used to take the working directory unconditionally, so it opened pointing at
/// the one place a project can never live.
#[test]
fn the_default_location_is_never_inside_the_install_folder() {
    let root = std::env::temp_dir().join(format!("crawrad-default-{}", std::process::id()));
    let install = root.join("WindowsApps").join("GUARANA.CrawRAD_0.1.1.0");
    let home = root.join("home");
    std::fs::create_dir_all(&install).expect("install folder");
    std::fs::create_dir_all(&home).expect("home folder");

    // The working directory is inside the install folder: it is refused, and the
    // home folder answers instead.
    let chosen = default_project_parent(None, None, Some(&home), Some(&install), Some(&install))
        .expect("a default is found");
    assert_eq!(
        chosen,
        home.join("CrawRAD-Projects"),
        "the default fell back to the install folder, which is read-only"
    );

    // With no home to fall back to, it answers nothing rather than a folder that
    // cannot be written: an empty field is honest, a doomed path is not.
    assert_eq!(
        default_project_parent(None, None, None, Some(&install), Some(&install)),
        None,
        "with nowhere writable it still offered the install folder"
    );

    let _ = std::fs::remove_dir_all(&root);
}

/// Beside the last project opened, because a person keeps their projects together.
#[test]
fn the_default_location_prefers_where_the_last_project_lives() {
    let root = std::env::temp_dir().join(format!("crawrad-default-recent-{}", std::process::id()));
    let projects = root.join("CrawRAD-Projects");
    let last = projects.join("my_app");
    let home = root.join("home");
    std::fs::create_dir_all(&last).expect("recent project");
    std::fs::create_dir_all(&home).expect("home folder");

    assert_eq!(
        default_project_parent(None, Some(&last), Some(&home), None, None),
        Some(projects),
        "the folder holding the last project is the one a person expects"
    );

    let _ = std::fs::remove_dir_all(&root);
}

/// `Create` creates. A folder that does not exist yet is made, not answered with
/// a file picker — the picker is what the folder button beside the field is for.
#[test]
fn create_makes_the_folder_instead_of_asking_where() {
    let root = std::env::temp_dir().join(format!("crawrad-make-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let target = root.join("CrawRAD-Projects");
    assert!(!target.exists());

    make_project_parent(&target, None).expect("a missing folder is created, not refused");
    assert!(target.is_dir(), "Create did not create");

    // Running again on the folder it just made is not an error.
    make_project_parent(&target, None).expect("an existing folder is accepted");

    let _ = std::fs::remove_dir_all(&root);
}

/// A path inside the install folder gets its own phrase, because no amount of
/// retrying will make that folder writable.
#[test]
fn a_path_inside_the_install_folder_is_refused_by_name() {
    let root = std::env::temp_dir().join(format!("crawrad-refuse-{}", std::process::id()));
    let install = root.join("WindowsApps").join("GUARANA.CrawRAD_0.1.1.0");
    std::fs::create_dir_all(&install).expect("install folder");

    let refusal = make_project_parent(&install.join("my_app"), Some(&install))
        .expect_err("a project inside the install folder must be refused");
    assert!(
        refusal.phrase.contains("read-only"),
        "the phrase does not say why it cannot be written: {}",
        refusal.phrase
    );
    assert!(
        refusal.phrase.contains("Nothing was written"),
        "the phrase does not say the project was not half-created: {}",
        refusal.phrase
    );
    assert!(
        !install.join("my_app").exists(),
        "it refused and created the folder anyway"
    );

    let _ = std::fs::remove_dir_all(&root);
}

/// A file where a folder was asked for is refused, not silently replaced.
#[test]
fn a_file_in_the_folders_place_is_refused() {
    let root = std::env::temp_dir().join(format!("crawrad-file-{}", std::process::id()));
    std::fs::create_dir_all(&root).expect("test root");
    let file_in_the_way = root.join("not_a_folder");
    std::fs::write(&file_in_the_way, b"x").expect("the file");

    let refusal = make_project_parent(&file_in_the_way, None).expect_err("a file is not a folder");
    assert!(
        refusal.phrase.contains("is a file"),
        "the phrase does not name what is wrong: {}",
        refusal.phrase
    );

    let _ = std::fs::remove_dir_all(&root);
}

/// The folder chosen in Options wins over everything this function could infer.
///
/// `default_projects_dir` was already a preference, written and shown in Options —
/// and the New Project dialog never read it. A person could set where their
/// projects go and watch the dialog ignore it.
#[test]
fn the_folder_chosen_in_options_wins() {
    let root = std::env::temp_dir().join(format!("crawrad-options-{}", std::process::id()));
    let chosen_by_hand = root.join("Meus Projetos");
    let last = root.join("CrawRAD-Projects").join("my_app");
    let home = root.join("home");
    std::fs::create_dir_all(&chosen_by_hand).expect("chosen folder");
    std::fs::create_dir_all(&last).expect("recent project");
    std::fs::create_dir_all(&home).expect("home folder");

    assert_eq!(
        default_project_parent(Some(&chosen_by_hand), Some(&last), Some(&home), None, None),
        Some(chosen_by_hand.clone()),
        "the dialog ignored the folder the person chose in Options"
    );

    // Even a chosen folder is refused when it points inside the install folder:
    // the preference says where projects go, not that Windows will allow it.
    let install = root.join("WindowsApps").join("GUARANA.CrawRAD_0.1.1.0");
    std::fs::create_dir_all(&install).expect("install folder");
    assert_eq!(
        default_project_parent(Some(&install), None, Some(&home), None, Some(&install)),
        Some(home.join("CrawRAD-Projects")),
        "a chosen folder inside the install folder was offered anyway"
    );

    let _ = std::fs::remove_dir_all(&root);
}

/// Reopening brings back the form that WAS open, not whichever form sorts first.
///
/// Taking `documents(root).next()` meant the folder's listing order decided: add a
/// form and the newly added, empty one wins over the one being worked on. That is
/// what happened on the Store build — a fresh `form_1` reopened instead of the
/// project's real form.
#[test]
fn reopening_brings_back_the_form_that_was_open() {
    let root = std::env::temp_dir().join(format!("crawrad-lastform-{}", std::process::id()));
    let project = root.join("my_app");
    let src = project.join("src");
    std::fs::create_dir_all(&src).expect("project src");
    let first_listed = src.join("form_1.crawform");
    let was_open = src.join("form_main.crawform");
    std::fs::write(&first_listed, "").expect("first form");
    std::fs::write(&was_open, "").expect("open form");

    let recent = vec![project.clone()];
    let docs = |_: &Path| -> Vec<PathBuf> { vec![first_listed.clone(), was_open.clone()] };
    let reopening = |last: Option<PathBuf>| Preferences {
        welcome_on_startup: false,
        last_form: last,
        ..Preferences::default()
    };

    assert_eq!(
        last_project_form(&reopening(Some(was_open.clone())), &recent, docs),
        Some(was_open.clone()),
        "it reopened the first form the folder listed, not the one that was open"
    );

    // A form that was deleted, or that belongs to another project, must not be
    // reopened: the fallback is the project's own first document, never a stale path.
    assert_eq!(
        last_project_form(
            &reopening(Some(src.join("deleted.crawform"))),
            &recent,
            docs
        ),
        Some(first_listed.clone()),
        "a form that no longer exists was reopened"
    );
    assert_eq!(
        last_project_form(
            &reopening(Some(root.join("other_app/src/form_main.crawform"))),
            &recent,
            docs
        ),
        Some(first_listed.clone()),
        "a form from another project was reopened"
    );

    let _ = std::fs::remove_dir_all(&root);
}
