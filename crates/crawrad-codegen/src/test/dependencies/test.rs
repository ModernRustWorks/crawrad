use super::*;

#[test]
fn makepad_com_formulario_inclui_widgets_e_runtime() {
    let deps = dependencias_do_projeto(AlvoGerador::Makepad, true, false);
    assert_eq!(deps.len(), 3);
    assert_eq!(deps[0].nome(), "makepad-widgets");
    assert_eq!(deps[1].nome(), "crawrad-runtime");
    assert_eq!(deps[2].nome(), "crawrad-runtime-makepad");
    assert!(matches!(&deps[1], Dependencia::Caminho { .. }));
    assert!(matches!(&deps[2], Dependencia::Caminho { .. }));
}

#[test]
fn makepad_com_formulario_e_binding_inclui_tres_deps() {
    let deps = dependencias_do_projeto(AlvoGerador::Makepad, true, true);
    assert_eq!(deps.len(), 4);
    assert_eq!(deps[0].nome(), "makepad-widgets");
    assert_eq!(deps[1].nome(), "crawrad-runtime");
    assert_eq!(deps[2].nome(), "crawrad-runtime-makepad");
    assert_eq!(deps[3].nome(), "crawrad-binding");
}

#[test]
fn egui_com_formulario_inclui_eframe() {
    let deps = dependencias_do_projeto(AlvoGerador::Egui, true, false);
    assert_eq!(deps.len(), 2);
    assert_eq!(deps[0].nome(), "eframe");
    assert_eq!(deps[1].nome(), "crawrad-runtime");
}

#[test]
fn egui_com_binding_inclui_crawrad_binding() {
    let deps = dependencias_do_projeto(AlvoGerador::Egui, true, true);
    assert_eq!(deps.len(), 3);
    assert_eq!(deps[0].nome(), "eframe");
    assert_eq!(deps[1].nome(), "crawrad-runtime");
    assert_eq!(deps[2].nome(), "crawrad-binding");
}

#[test]
fn projeto_sem_formulario_nao_tem_deps_de_ui() {
    let deps = dependencias_do_projeto(AlvoGerador::Makepad, false, false);
    assert!(deps.is_empty());
}

#[test]
fn args_cargo_add_formata_corretamente() {
    let reg = Dependencia::registro("eframe", "0.36");
    assert_eq!(reg.args_cargo_add(), vec!["eframe@0.36"]);

    let cam = Dependencia::caminho("crawrad-runtime", PathBuf::from("/path/to/crawrad-runtime"));
    assert_eq!(
        cam.args_cargo_add(),
        vec!["--path", "/path/to/crawrad-runtime"]
    );
}

#[test]
fn gerar_cargo_toml_makepad_e_egui() {
    let toml_makepad = gerar_cargo_toml("demo_mk", AlvoGerador::Makepad, true, false, &[]);
    assert!(toml_makepad.contains("name = \"demo_mk\""));
    assert!(toml_makepad.contains("makepad-widgets = \"1.0\""));
    assert!(toml_makepad.contains("crawrad-runtime = { path = "));
    assert!(toml_makepad.contains("crawrad-runtime-makepad = { path = "));

    let toml_egui = gerar_cargo_toml("demo_egui", AlvoGerador::Egui, true, false, &[]);
    assert!(toml_egui.contains("name = \"demo_egui\""));
    assert!(toml_egui.contains("eframe = \"0.36\""));
    assert!(toml_egui.contains("crawrad-runtime = { path = "));
    assert!(!toml_egui.contains("makepad-widgets"));

    // The generated code uses let chains, which need the 2024 edition:
    // a 2021 manifest made every template project fail to build.
    for toml in [&toml_makepad, &toml_egui] {
        assert!(toml.contains("edition = \"2024\""), "{toml}");
    }
}

#[test]
fn a_path_dependency_carries_its_features_to_cargo_add_and_to_the_manifest() {
    let dep =
        Dependencia::caminho("crawrad-binding", "/x/crawrad-binding").with_features(&["rusqlite"]);
    let args = dep.args_cargo_add();
    assert_eq!(args[2..], ["--features", "rusqlite"]);
    let toml = gerar_cargo_toml("app", AlvoGerador::Egui, false, false, &[dep]);
    assert!(
        toml.contains(
            r#"crawrad-binding = { path = "/x/crawrad-binding", features = ["rusqlite"] }"#
        ),
        "{toml}"
    );
}

#[test]
fn egui_floor_is_above_makepad_floor() {
    assert_eq!(minimum_rustc(AlvoGerador::Egui, true), (1, 95));
    assert_eq!(minimum_rustc(AlvoGerador::Makepad, true), (1, 88));
    assert_eq!(minimum_rustc(AlvoGerador::Egui, false), (1, 85));
    assert_eq!(minimum_rustc(AlvoGerador::Makepad, false), (1, 85));
}

#[test]
fn the_manifest_declares_the_floor() {
    let toml_egui = gerar_cargo_toml("x", AlvoGerador::Egui, true, false, &[]);
    assert!(toml_egui.contains("rust-version = \"1.95\""), "{toml_egui}");
    let toml_makepad = gerar_cargo_toml("x", AlvoGerador::Makepad, false, false, &[]);
    assert!(
        toml_makepad.contains("rust-version = \"1.85\""),
        "{toml_makepad}"
    );
}

#[test]
fn the_platform_patch_points_at_the_vendored_copy() {
    let patch = patch_makepad_platform();
    assert!(
        patch.starts_with("\n[patch.crates-io]\nmakepad-platform = { path = \""),
        "{patch}"
    );
    assert!(patch.ends_with("vendor/makepad-platform\" }\n"), "{patch}");
    let member = gerar_cargo_toml("app", AlvoGerador::Makepad, true, false, &[]);
    assert!(
        !member.contains("[patch"),
        "a member's patch is ignored by cargo: {member}"
    );
}

#[test]
fn locale_dependencies_returns_sys_locale() {
    let deps = locale_dependencies();
    assert_eq!(deps.len(), 1);
    assert_eq!(deps[0].nome(), "sys-locale");
    assert_eq!(deps[0].args_cargo_add(), vec!["sys-locale@0.3"]);
}

#[test]
fn an_inheriting_component_crate_is_a_project_dependency() {
    let mut reg = crawrad_registry::Registry::builtin();
    reg.register_external(crawrad_registry::sample_price_edit())
        .expect("PriceEdit registers");

    let mut doc = crawrad_ir::Document::new("Form1", "Form");
    let mut ped1 = reg
        .instantiate(
            "PriceEdit",
            "ped1".to_string(),
            crawrad_ir::Rect::new(0.0, 0.0, 100.0, 24.0),
        )
        .expect("PriceEdit instantiates");
    ped1.set_name("ped1");
    doc.add_child("Form1", ped1).unwrap();

    // `crawrad-project` runs `cargo add --path` for every class listed here.
    assert_eq!(crate::classes_de_terceiro(&doc, &reg), vec!["PriceEdit"]);
    assert_eq!(
        reg.get("PriceEdit").map(|d| d.crate_module),
        Some("finance_crawrad")
    );
}

#[test]
fn manifest_path_inside_the_checkout_is_relative() {
    let root = std::env::temp_dir().join("crawrad-relpaths-checkout");
    let member = root.join("examples").join("calculator").join("calculator");
    let runtime = root.join("crates").join("crawrad-runtime");
    assert_eq!(
        manifest_path(&member, &runtime),
        Path::new("..")
            .join("..")
            .join("..")
            .join("crates")
            .join("crawrad-runtime")
    );
}

#[test]
fn manifest_path_to_a_sibling_folder_is_relative() {
    let base = std::env::temp_dir().join("crawrad-relpaths-sib");
    let member = base.join("projects").join("app").join("app");
    let runtime = base.join("CrawRAD").join("crates").join("crawrad-runtime");
    let got = manifest_path(&member, &runtime);
    assert!(got.is_relative(), "{}", got.display());
    assert_eq!(normalize(&member.join(&got)), runtime);
}

#[cfg(windows)]
#[test]
fn manifest_path_across_drives_stays_absolute() {
    let got = manifest_path(
        Path::new(r"D:\work\app\app"),
        Path::new(r"C:\crawrad\crates\crawrad-runtime"),
    );
    assert_eq!(got, Path::new(r"C:\crawrad\crates\crawrad-runtime"));
    // The drive letter's case is not a different drive.
    let same = manifest_path(
        Path::new(r"c:\crawrad\examples\app"),
        Path::new(r"C:\crawrad\crates\x"),
    );
    assert_eq!(same, Path::new(r"..\..\crates\x"));
}

#[test]
fn a_project_manifest_carries_no_absolute_runtime_path() {
    let runtime = caminho_local_crate("crawrad-runtime");
    let checkout = runtime
        .parent()
        .and_then(Path::parent)
        .unwrap()
        .to_path_buf();
    let member = checkout.join("examples").join("x").join("x");
    let toml = cargo_toml_at(&member, "x", AlvoGerador::Makepad, true, false, &[]);
    assert!(
        toml.contains("crawrad-runtime = { path = \"../../../crates/crawrad-runtime\" }"),
        "{toml}"
    );
    let patch = patch_makepad_platform_at(&checkout.join("examples").join("x"));
    assert!(
        patch.is_empty() || patch.contains("path = \"../../vendor/makepad-platform\""),
        "{patch}"
    );
}
