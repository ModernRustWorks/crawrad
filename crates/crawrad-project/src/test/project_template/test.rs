//! The New Project name: what an empty field means; the drawn structure.

use super::*;

#[test]
fn an_empty_or_blank_name_is_the_default() {
    assert_eq!(name_or_default(""), DEFAULT_PROJECT_NAME);
    assert_eq!(name_or_default("   "), DEFAULT_PROJECT_NAME);
}

#[test]
fn a_typed_name_is_kept_trimmed() {
    assert_eq!(name_or_default("  calc  "), "calc");
}

/// The paths a drawn tree names (`a`, `a/b.rs`), its notes after the name dropped.
fn drawn_paths(tree: &str) -> Vec<String> {
    let mut parents: Vec<String> = Vec::new();
    let mut paths = Vec::new();
    for line in tree.lines() {
        let (depth, entry) = match line.find("── ") {
            Some(at) => (
                line[..at].chars().count() / 4 + 1,
                &line[at + "── ".len()..],
            ),
            None => (0, line),
        };
        let entry = entry.split("   ").next().unwrap_or(entry);
        parents.truncate(depth);
        parents.push(entry.trim_end_matches('/').to_string());
        paths.push(parents.join("/"));
    }
    paths.sort();
    paths
}

/// What Create wrote: `root`'s name and every path under it, sorted.
fn written_paths(root: &std::path::Path) -> Vec<String> {
    fn walk(dir: &std::path::Path, prefix: &str, out: &mut Vec<String>) {
        for e in std::fs::read_dir(dir).unwrap().flatten() {
            let path = format!("{prefix}/{}", e.file_name().to_string_lossy());
            out.push(path.clone());
            if e.path().is_dir() {
                walk(&e.path(), &path, out);
            }
        }
    }
    let name = root.file_name().unwrap().to_string_lossy().to_string();
    let mut out = vec![name.clone()];
    walk(root, &name, &mut out);
    out.sort();
    out
}

#[test]
fn the_structure_preview_is_what_create_writes() {
    let base = std::env::temp_dir().join(format!("crawrad-preview-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let makepad = crawrad_codegen::AlvoGerador::Makepad;
    for tpl in TEMPLATES {
        let folder = base.join(tpl.id);
        let name = format!("p_{}", tpl.id);
        create_or_add(None, &folder, &name, tpl.id, false, makepad).unwrap();
        let workspace = folder.join(&name);
        let drawn = drawn_paths(&structure_preview(tpl, &name, None));
        assert_eq!(drawn, written_paths(&workspace), "New Project, {}", tpl.id);

        // Add Project: the template drawn inside that workspace, its manifest edited
        let added = format!("q_{}", tpl.id);
        create_or_add(Some(&workspace), &folder, &added, tpl.id, false, makepad).unwrap();
        let drawn = drawn_paths(&structure_preview(tpl, &added, Some(&workspace)));
        let mut want: Vec<String> = written_paths(&workspace.join(&added))
            .into_iter()
            .map(|p| format!("{name}/{p}"))
            .chain([name.clone(), format!("{name}/Cargo.toml")])
            .collect();
        want.sort();
        assert_eq!(drawn, want, "Add Project, {}", tpl.id);
    }
    let _ = std::fs::remove_dir_all(&base);
}

/// A `gui_app` is born with its form on disk and the
/// code generated from it: `main.rs` shows the form, not the template's demo.
#[test]
fn a_gui_app_is_born_with_its_form_and_its_code() {
    let base = std::env::temp_dir().join(format!("crawrad-gui-form-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let member = create_or_add(
        None,
        &base,
        "calc",
        "gui_app",
        false,
        crawrad_codegen::AlvoGerador::Makepad,
    )
    .unwrap();
    let form = member.join("src").join("form_main.crawform");
    let doc = crawrad_ir::Document::load(&form).expect("the form is on disk and opens");
    assert_eq!(doc.root, "form_main");
    assert!(
        member.join("src").join("form_main.designer.rs").is_file(),
        "the code was generated with the form"
    );
    let main_rs = std::fs::read_to_string(member.join("src").join("main.rs")).unwrap();
    assert!(
        main_rs.contains("form_main"),
        "main.rs still runs the template demo:\n{main_rs}"
    );
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn template_floor_follows_what_the_template_writes() {
    use crawrad_codegen::AlvoGerador;

    let egui_options = ProjectTemplateOptions {
        framework: AlvoGerador::Egui,
        ..Default::default()
    };
    assert_eq!(template_floor("console_app", &egui_options), (1, 85));
    // THIS ASSERTION USED TO READ `(1, 88)`, which is MAKEPAD's floor, because
    // `wasm_app` hardcoded `AlvoGerador::Makepad` and threw away the framework the
    // person chose. The test froze the defect instead of catching it: choosing
    // egui + WASM produced a Makepad project, and the floor agreed with the
    // project nobody asked for. egui needs 1.95 for a graphical project, which is
    // the same number `gui_app` asserts two lines down.
    assert_eq!(template_floor("wasm_app", &egui_options), (1, 95));
    assert_eq!(template_floor("gui_app", &egui_options), (1, 95));
    assert_eq!(
        template_floor("gui_app", &ProjectTemplateOptions::default()),
        crawrad_codegen::minimum_rustc(AlvoGerador::default(), true)
    );
}

#[test]
fn library_crate_born_with_missing_docs_lint_and_documented() {
    let base = std::env::temp_dir().join(format!("crawrad-lib-template-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let member = create_or_add(
        None,
        &base,
        "mylib",
        "library_crate",
        false,
        crawrad_codegen::AlvoGerador::Makepad,
    )
    .unwrap();
    let lib_rs = std::fs::read_to_string(member.join("src").join("lib.rs")).unwrap();
    assert!(lib_rs.contains("//! mylib: a CrawRAD library."));
    assert!(lib_rs.contains("#![warn(missing_docs)]"));
    assert!(lib_rs.contains("/// Adds two numbers."));
    assert!(lib_rs.contains("pub fn add("));
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn package_plugin_born_with_missing_docs_lint_and_documented() {
    let base = std::env::temp_dir().join(format!("crawrad-plugin-template-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let member = create_or_add(
        None,
        &base,
        "myplugin",
        "package_plugin",
        false,
        crawrad_codegen::AlvoGerador::Makepad,
    )
    .unwrap();
    let lib_rs = std::fs::read_to_string(member.join("src").join("lib.rs")).unwrap();
    assert!(lib_rs.contains("//! myplugin: a CrawRAD plugin."));
    assert!(lib_rs.contains("#![warn(missing_docs)]"));
    assert!(lib_rs.contains("/// Called by the IDE when the plugin loads."));
    assert!(lib_rs.contains("pub fn plugin_init("));
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn desktop_templates_born_with_icon_build_script_and_resource() {
    let base = std::env::temp_dir().join(format!("crawrad-desktop-icon-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let member = create_or_add(
        None,
        &base,
        "desk_app",
        "gui_app",
        false,
        crawrad_codegen::AlvoGerador::Makepad,
    )
    .unwrap();

    assert!(member.join("icon.ico").is_file());
    assert!(member.join("icon.png").is_file());
    assert!(member.join("build.rs").is_file());
    assert!(member.join("app.rc").is_file());

    let build_rs = std::fs::read_to_string(member.join("build.rs")).unwrap();
    assert!(build_rs.contains("embed_resource::compile(\"app.rc\", embed_resource::NONE);"));

    let app_rc = std::fs::read_to_string(member.join("app.rc")).unwrap();
    assert_eq!(app_rc, crate::project_options::app_rc("desk_app"));

    let toml = std::fs::read_to_string(member.join("Cargo.toml")).unwrap();
    assert!(toml.contains("[build-dependencies]"));
    assert!(toml.contains("embed-resource = \"3.0\""));

    let non_desktop = create_or_add(
        None,
        &base,
        "web_app",
        "wasm_app",
        false,
        crawrad_codegen::AlvoGerador::Makepad,
    )
    .unwrap();
    assert!(!non_desktop.join("icon.ico").exists());
    assert!(!non_desktop.join("build.rs").exists());
    assert!(!non_desktop.join("app.rc").exists());

    let _ = std::fs::remove_dir_all(&base);
}

/// THE DIALOG MUST NOT OFFER WHAT IT WILL NOT HONOUR.
///
/// Before this, `wasm_app`, `mobile_app` and `multiwindow_app` hardcoded Makepad
/// and ignored `options.framework`, so **egui + WASM produced a Makepad project**
/// and nothing said so. Handing somebody another framework's project is worse
/// than refusing: they find out when the code does not look like what they asked
/// for.
#[test]
fn a_framework_that_cannot_reach_the_platform_is_refused_by_name() {
    use crawrad_codegen::AlvoGerador;

    let egui = ProjectTemplateOptions {
        framework: AlvoGerador::Egui,
        ..Default::default()
    };
    let refusal = refuse_a_platform_the_framework_cannot_reach("mobile_app", &egui)
        .expect_err("egui's card does not claim phones, so this cannot be built");
    assert!(
        refusal.contains("egui") || refusal.contains("Egui"),
        "the refusal must name the framework, not just say no: {refusal}"
    );
    assert!(
        refusal.contains("phones"),
        "and name the platform it cannot reach: {refusal}"
    );
}

#[test]
fn a_framework_that_does_reach_the_platform_is_let_through() {
    use crawrad_codegen::AlvoGerador;

    let egui = ProjectTemplateOptions {
        framework: AlvoGerador::Egui,
        ..Default::default()
    };
    // egui's card DOES claim the browser, so WASM is not refused.
    assert!(refuse_a_platform_the_framework_cannot_reach("wasm_app", &egui).is_ok());
    // And a template with no platform of its own never asks the question.
    assert!(refuse_a_platform_the_framework_cannot_reach("console_app", &egui).is_ok());
    let makepad = ProjectTemplateOptions::default();
    assert!(refuse_a_platform_the_framework_cannot_reach("mobile_app", &makepad).is_ok());
}

/// The refusal reads the CARD, not a list of names written in the refusal itself.
///
/// That is what makes a framework added later covered without touching this: the
/// card says what it can reach, and the decision follows the card.
#[test]
fn the_refusal_follows_the_card_and_not_a_hardcoded_name() {
    use crawrad_codegen::AlvoGerador;

    // TODOS, not a list written here: a target registered later is covered by
    // this test the moment it joins the generator, which is how the Slint card
    // got caught missing on 28/11/2026.
    for &alvo in AlvoGerador::TODOS {
        let card = crate::frameworks::of(alvo);
        let options = ProjectTemplateOptions {
            framework: alvo,
            ..Default::default()
        };
        assert_eq!(
            refuse_a_platform_the_framework_cannot_reach("wasm_app", &options).is_ok(),
            card.web,
            "{}: the browser answer must be the card's `web`",
            card.name
        );
        assert_eq!(
            refuse_a_platform_the_framework_cannot_reach("mobile_app", &options).is_ok(),
            card.mobile,
            "{}: the phone answer must be the card's `mobile`",
            card.name
        );
    }
}

/// THE STARTER FORM MUST BE THE CHOSEN FRAMEWORK'S, not Makepad's by default.
///
/// Found by mutation on 28/11/2026: putting `AlvoGerador::Makepad` back into
/// `write_starter_form`'s call left the whole suite green, so a `wasm_app`
/// created with egui could be born with a Makepad form and nothing would notice.
/// The generated code is what the person opens first, so this is the most
/// visible place the wrong framework could hide.
#[test]
fn a_wasm_project_is_born_with_the_chosen_frameworks_form() {
    use crawrad_codegen::AlvoGerador;

    let base = std::env::temp_dir().join(format!("crawrad-wasm-framework-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let member = create_or_add(None, &base, "wapp", "wasm_app", false, AlvoGerador::Egui)
        .expect("egui reaches the browser, so this template must be produced");

    let generated = member
        .join("src")
        .join(format!("form_main{}", crawrad_codegen::SUFIXO_GERADO));
    let code = std::fs::read_to_string(&generated).unwrap_or_else(|e| {
        panic!(
            "the generated form must be readable at {}: {e}",
            generated.display()
        )
    });

    // Makepad's generated form is built around its `live_design!` macro, which
    // egui's has no equivalent of — so the macro's presence is the framework's
    // own fingerprint in the file the person opens first.
    assert!(
        !code.contains("live_design!"),
        "the form was generated for Makepad although egui was chosen:
{}",
        &code[..code.len().min(400)]
    );
    assert!(
        code.contains("egui") || code.contains("eframe"),
        "the form names neither egui nor eframe, so it was not generated for egui"
    );
    let _ = std::fs::remove_dir_all(&base);
}
