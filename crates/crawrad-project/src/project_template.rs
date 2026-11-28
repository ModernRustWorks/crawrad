//! The files of a new project, from the template chosen in **New Project…**
//! (gui, console, library, wasm, mobile, multi-window, IDE extension,
//! custom). Pure: no window — the dialog in the shell asks, this writes.

use std::path::{Path, PathBuf};

use crate::workspace::ProjectType;
use crawrad_note::note;

#[derive(Debug, Clone)]
pub struct TemplateDef {
    pub id: &'static str,
    pub name: &'static str,
    pub subtitle: &'static str,
    pub icon: &'static str,
    pub category_idx: usize,
    pub base_type: ProjectType,
    pub platforms_badge: &'static str,
    pub platforms_detail: &'static str,
    pub description: &'static str,
    pub supports_framework: bool,
    pub tree_format: &'static str,
    /// A desktop window app: it starts with the crab as its icon (`icon.ico`
    /// in the `.exe` via `build.rs` + `app.rc`, `icon.png` for the window).
    pub app_icon: bool,
}

pub const TEMPLATES: &[TemplateDef] = &[
    TemplateDef {
        id: "gui_app",
        name: "GUI Application",
        subtitle: "A desktop application with a window, built with Makepad and Rust.",
        icon: "💻",
        category_idx: 1, // Desktop
        base_type: ProjectType::Gui,
        platforms_badge: "⊞ 🐧 🍎",
        platforms_detail: "⊞ Windows    🐧 Linux    🍎 macOS",
        description: "Creates a new Rust application with a graphical user interface using the Makepad framework. It starts with a blank form, open in the Designer, and the code generated from it.",
        supports_framework: true,
        tree_format: "{name}/\n├── app.rc\n├── build.rs\n├── Cargo.toml\n├── icon.ico\n├── icon.png\n└── src/\n    ├── form_main.designer.rs\n    ├── form_main.crawform\n    ├── form_main.rs\n    └── main.rs",
        app_icon: true,
    },
    TemplateDef {
        id: "console_app",
        name: "Console Application",
        subtitle: "A command-line application. No UI, fast and lightweight.",
        icon: ">_",
        category_idx: 2, // Console
        base_type: ProjectType::Console,
        platforms_badge: "⊞ 🐧 🍎",
        platforms_detail: "⊞ Windows    🐧 Linux    🍎 macOS",
        description: "Creates a lightweight Rust command-line tool with clean argument parsing, error handling, and terminal output. No UI dependencies or GPU overhead.",
        supports_framework: false,
        tree_format: "{name}/\n├── Cargo.toml\n└── src/\n    └── main.rs",
        app_icon: false,
    },
    TemplateDef {
        id: "library_crate",
        name: "Library Crate",
        subtitle: "A reusable library crate to share code with other projects.",
        icon: "📦",
        category_idx: 5, // Library
        base_type: ProjectType::Library,
        platforms_badge: "⊞ 🐧 🍎",
        platforms_detail: "⊞ Windows    🐧 Linux    🍎 macOS",
        description: "Creates a reusable Rust crate structured for sharing components, models, and business logic across projects. Includes crawrad.toml manifest and unit tests.",
        supports_framework: true,
        tree_format: "{name}/\n├── Cargo.toml\n├── crawrad.toml\n└── src/\n    └── lib.rs",
        app_icon: false,
    },
    TemplateDef {
        id: "wasm_app",
        name: "Web App (WASM)",
        subtitle: "A web application compiled to WebAssembly, running in the browser.",
        icon: "🌐",
        category_idx: 3, // WebAssembly
        base_type: ProjectType::Gui,
        platforms_badge: "WA 🌐",
        platforms_detail: "🌐 WebAssembly (Browser)    ⊞ 🐧 🍎",
        description: "Compiles directly to WebAssembly and runs in any modern browser via Makepad WebGL/WebGPU backend. Zero client installation required.",
        supports_framework: true,
        tree_format: "{name}/\n├── Cargo.toml\n└── src/\n    ├── form_main.designer.rs\n    ├── form_main.crawform\n    ├── form_main.rs\n    └── main.rs",
        app_icon: false,
    },
    TemplateDef {
        id: "mobile_app",
        name: "Mobile App",
        subtitle: "A mobile application for Android and iOS using Makepad.",
        icon: "📱",
        category_idx: 4, // Mobile
        base_type: ProjectType::Gui,
        platforms_badge: "🤖 🍎",
        platforms_detail: "🤖 Android    🍎 iOS",
        description: "Cross-platform mobile application targeting Android and iOS with native touch gestures, responsive layout, and mobile window management.",
        supports_framework: true,
        tree_format: "{name}/\n├── Cargo.toml\n├── src/\n│   ├── main.rs\n│   └── app.rs\n└── res/\n    └── icon.png",
        app_icon: false,
    },
    TemplateDef {
        id: "multiwindow_app",
        name: "Multi-window Desktop",
        subtitle: "A desktop app with multiple windows and advanced window management.",
        icon: "🖥",
        category_idx: 1, // Desktop
        base_type: ProjectType::Gui,
        platforms_badge: "⊞ 🐧 🍎",
        platforms_detail: "⊞ Windows    🐧 Linux    🍎 macOS",
        description: "Advanced desktop application with support for multiple native OS windows, dockable viewports, RAD forms, and custom window chrome.",
        supports_framework: true,
        tree_format: "{name}/\n├── app.rc\n├── build.rs\n├── Cargo.toml\n├── forms/\n│   └── main.crawform\n├── icon.ico\n├── icon.png\n└── src/\n    ├── app.rs\n    └── main.rs",
        app_icon: true,
    },
    TemplateDef {
        id: "package_plugin",
        name: "Package / Plugin",
        subtitle: "A package, plugin or add-on for CrawRAD or other applications.",
        icon: "🧩",
        category_idx: 5, // Library
        base_type: ProjectType::Library,
        platforms_badge: "⊞ 🐧 🍎",
        platforms_detail: "⊞ Windows    🐧 Linux    🍎 macOS",
        description: "Extends CrawRAD IDE with custom component palettes, visual inspectors, code generator backends, and tooling integrations.",
        supports_framework: true,
        tree_format: "{name}/\n├── Cargo.toml\n├── plugin.toml\n└── src/\n    └── lib.rs",
        app_icon: false,
    },
    TemplateDef {
        id: "custom_template",
        name: "Custom Template",
        subtitle: "Start from a custom template or example.",
        icon: "📄",
        category_idx: 7, // Examples
        base_type: ProjectType::Gui,
        platforms_badge: "⚙",
        platforms_detail: "⊞ Windows    🐧 Linux    🍎 macOS    🌐 WASM",
        description: "Start from an empty or custom template scaffold with flexible Cargo configuration and customizable project directory layout.",
        supports_framework: true,
        tree_format: "{name}/\n├── app.rc\n├── build.rs\n├── Cargo.toml\n├── icon.ico\n├── icon.png\n└── src/\n    ├── app.rs\n    └── main.rs",
        app_icon: true,
    },
];

#[note("project-template-projecttemplateoptions")]
#[derive(Debug, Clone, Default)]
pub struct ProjectTemplateOptions {
    pub framework: crawrad_codegen::AlvoGerador,
    pub use_binding: bool,
    pub extra_dependencies: Vec<crawrad_codegen::dependencias::Dependencia>,
}

#[note("project-template-create-project-template")]
pub fn create_project_template(
    folder: &std::path::Path,
    name: &str,
    template_id: &str,
    git_init: bool,
    framework: crawrad_codegen::AlvoGerador,
) -> Result<PathBuf, String> {
    create_project_template_with_options(
        folder,
        name,
        template_id,
        git_init,
        ProjectTemplateOptions {
            framework,
            use_binding: false,
            extra_dependencies: Vec::new(),
        },
    )
}

#[note("project-template-create-project-template-with-options")]
pub fn create_project_template_with_options(
    folder: &std::path::Path,
    name: &str,
    template_id: &str,
    git_init: bool,
    options: ProjectTemplateOptions,
) -> Result<PathBuf, String> {
    // `<folder>/<name>/` is a Cargo workspace (its `Cargo.toml` first), and
    // the project goes inside it, in `<folder>/<name>/<name>/` — one `target/` and one
    // `Cargo.lock` for every project added later.
    let workspace_root = folder.join(name);
    if workspace_root.exists()
        && let Ok(entries) = std::fs::read_dir(&workspace_root)
        && entries.count() > 0
    {
        return Err(format!(
            "The folder '{}' already exists and is not empty.",
            workspace_root.display()
        ));
    }
    refuse_a_platform_the_framework_cannot_reach(template_id, &options)?;
    refuse_old_toolchain(folder, template_id, &options)?;
    std::fs::create_dir_all(&workspace_root).map_err(|e| {
        format!(
            "Could not create directory '{}': {e}",
            workspace_root.display()
        )
    })?;
    let manifest = crate::workspace::workspace_manifest(name);
    std::fs::write(workspace_root.join(crate::workspace::MANIFEST), manifest)
        .map_err(|e| format!("Failed to write the workspace's Cargo.toml: {e}"))?;
    write_file(&workspace_root, ".gitignore", "/target\n")?;
    let root = workspace_root.join(name);
    write_package(&root, name, template_id, &options)?;

    if git_init {
        let _ = crawrad_os::process::no_console("git")
            .arg("init")
            .current_dir(&workspace_root)
            .output();
    }

    Ok(root)
}

#[note("project-template-template-floor")]
fn template_floor(template_id: &str, options: &ProjectTemplateOptions) -> (u32, u32) {
    match template_id {
        "console_app" | "library_crate" | "package_plugin" => {
            crawrad_codegen::minimum_rustc(options.framework, false)
        }
        "wasm_app" | "mobile_app" | "multiwindow_app" => {
            crawrad_codegen::minimum_rustc(options.framework, true)
        }
        _ => crawrad_codegen::minimum_rustc(options.framework, true),
    }
}

/// Refuses a template the chosen framework cannot build for, naming which.
///
/// Three templates used to hardcode `AlvoGerador::Makepad` and ignore
/// `options.framework` entirely, so choosing **egui + WASM** quietly produced a
/// MAKEPAD project - the dialog offered a combination it did not honour, and
/// nothing said so. Handing the person another framework's project is worse than
/// refusing: they find out when the code does not look like what they asked for.
///
/// The decision is the framework card's own `web` and `mobile`, not a name
/// written here, so a framework added later is covered without touching this.
///
/// # Errors
///
/// A phrase naming the framework and the platform it cannot reach, for the
/// dialog to show.
#[note("project-template-refuse-a-platform-the-framework-cannot-reach")]
fn refuse_a_platform_the_framework_cannot_reach(
    template_id: &str,
    options: &ProjectTemplateOptions,
) -> Result<(), String> {
    let card = crate::frameworks::of(options.framework);
    let missing = match template_id {
        "wasm_app" if !card.web => "the browser",
        "mobile_app" if !card.mobile => "phones",
        _ => return Ok(()),
    };
    Err(format!(
        "{} cannot build for {missing}, so this template has nothing to produce. \
         Pick another framework, or another template.",
        card.name
    ))
}

#[note("project-template-refuse-old-toolchain")]
fn refuse_old_toolchain(
    at: &std::path::Path,
    template_id: &str,
    options: &ProjectTemplateOptions,
) -> Result<(), String> {
    // `cargo` honours a `rust-toolchain.toml` up the tree, so it is asked where the
    // project will live; a folder not created yet falls back to the temp dir.
    let dir = if at.is_dir() {
        at.to_path_buf()
    } else {
        std::env::temp_dir()
    };
    if let Ok(output) = crate::creation::cargo_with_output(&["--version".to_string()], &dir)
        && let Some(found) = crate::workspace::cargo_version(&output)
        && let Some(refusal) =
            crate::workspace::refusal_old_toolchain(found, template_floor(template_id, options))
    {
        return Err(refusal.phrase);
    }
    Ok(())
}

#[note("project-template-write-package")]
fn write_package(
    folder: &std::path::Path,
    name: &str,
    template_id: &str,
    options: &ProjectTemplateOptions,
) -> Result<(), String> {
    let src = folder.join("src");
    std::fs::create_dir_all(&src)
        .map_err(|e| format!("Could not create directory '{}: {e}'", src.display()))?;
    // The icon BEFORE the starter form: generating it reads `icon.png` (egui's window).
    let app_icon = TEMPLATES.iter().any(|t| t.id == template_id && t.app_icon);
    if app_icon {
        crate::project_options::apply_icon(folder, crate::project_options::IconSource::Default)?;
    }

    // Every path dependency relative to the member's own folder (see
    // `crawrad_codegen::dependencies::manifest_path`).
    let manifest = |target, with_form, uses_binding| {
        crawrad_codegen::dependencias::cargo_toml_at(
            folder,
            name,
            target,
            with_form,
            uses_binding,
            &options.extra_dependencies,
        )
    };
    match template_id {
        "console_app" => {
            let toml = manifest(options.framework, false, false);
            write_file(folder, "Cargo.toml", toml)?;
            let main_rs = format!("fn main() {{\n    println!(\"Hello from {name}!\");\n}}\n");
            write_file(folder, "src/main.rs", main_rs)?;
        }
        "library_crate" => {
            let toml = manifest(options.framework, false, false);
            write_file(folder, "Cargo.toml", toml)?;
            let crawrad_toml =
                format!("[package]\nname = \"{name}\"\nkind = \"library\"\nversion = \"0.1.0\"\n");
            write_file(folder, "crawrad.toml", crawrad_toml)?;
            let header = format!(
                "//! {name}: a CrawRAD library.\n\n#![warn(missing_docs)]\n\n/// Adds two numbers.\n"
            );
            let body = concat!(
                "pub fn add(left: u64, right: u64) -> u64 {\n    left + right\n}\n\n",
                "#[cfg(test)]\nmod tests {\n    use super::*;\n\n",
                "    #[",
                "test]\n",
                "    fn it_works() {\n        assert_eq!(add(2, 2), 4);\n    }\n}\n"
            );
            write_file(folder, "src/lib.rs", format!("{header}{body}"))?;
        }
        "package_plugin" => {
            let toml = manifest(options.framework, false, false);
            write_file(folder, "Cargo.toml", toml)?;
            let plugin_toml = format!(
                "[plugin]\nname = \"{name}\"\nversion = \"0.1.0\"\nauthor = \"CrawRAD Developer\"\n"
            );
            write_file(folder, "plugin.toml", plugin_toml)?;
            let lib_rs = format!(
                "//! {name}: a CrawRAD plugin.\n\n#![warn(missing_docs)]\n\n/// Called by the IDE when the plugin loads.\npub fn plugin_init() -> &'static str {{\n    \"{name} plugin initialized\"\n}}\n"
            );
            write_file(folder, "src/lib.rs", lib_rs)?;
        }
        "wasm_app" => {
            let mut toml = manifest(options.framework, true, options.use_binding);
            toml.push_str("\n[package.metadata.crawrad]\nplatform = \"web\"\n");
            write_file(folder, "Cargo.toml", toml)?;
            let main_rs =
                crawrad_codegen::scaffold::scaffold_main_rs(options.framework, name, template_id);
            write_file(folder, "src/main.rs", main_rs)?;
            // A form, like the desktop app: the generated main shows it in the browser.
            write_starter_form(folder, options.framework)?;
        }
        "mobile_app" => {
            let toml = manifest(options.framework, true, options.use_binding);
            write_file(folder, "Cargo.toml", toml)?;
            let res_dir = folder.join("res");
            let _ = std::fs::create_dir_all(&res_dir);
            let _ = std::fs::write(
                res_dir.join("icon.png"),
                [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A],
            );
            let main_rs =
                crawrad_codegen::scaffold::scaffold_main_rs(options.framework, name, template_id);
            write_file(folder, "src/main.rs", main_rs)?;
            if let Some(app_rs) =
                crawrad_codegen::scaffold::scaffold_app_rs(options.framework, name, template_id)
            {
                write_file(folder, "src/app.rs", app_rs)?;
            }
        }
        "multiwindow_app" => {
            let toml = manifest(options.framework, true, options.use_binding);
            write_file(folder, "Cargo.toml", toml)?;
            let forms_dir = folder.join("forms");
            let _ = std::fs::create_dir_all(&forms_dir);
            let _ = std::fs::write(
                forms_dir.join("main.crawform"),
                "{\n  \"name\": \"main\",\n  \"components\": []\n}\n",
            );
            let main_rs =
                crawrad_codegen::scaffold::scaffold_main_rs(options.framework, name, template_id);
            write_file(folder, "src/main.rs", main_rs)?;
            if let Some(app_rs) =
                crawrad_codegen::scaffold::scaffold_app_rs(options.framework, name, template_id)
            {
                write_file(folder, "src/app.rs", app_rs)?;
            }
        }
        _ => {
            // gui_app, custom_template, and any fallback
            let toml = manifest(options.framework, true, options.use_binding);
            write_file(folder, "Cargo.toml", toml)?;

            let main_rs =
                crawrad_codegen::scaffold::scaffold_main_rs(options.framework, name, template_id);
            write_file(folder, "src/main.rs", main_rs)?;
            if template_id == "gui_app" {
                write_starter_form(folder, options.framework)?;
            } else if let Some(app_rs) =
                crawrad_codegen::scaffold::scaffold_app_rs(options.framework, name, template_id)
            {
                write_file(folder, "src/app.rs", app_rs)?;
            }
        }
    }

    if app_icon {
        write_app_icon(folder)?;
    }
    patch_group_platform(folder)
}

/// A Makepad member makes its group (the workspace root, one folder up) build
/// against the IDE's `makepad-platform`: the window then shows the app's icon. Once.
fn patch_group_platform(member: &Path) -> Result<(), String> {
    let uses_makepad = std::fs::read_to_string(member.join(crate::workspace::MANIFEST))
        .is_ok_and(|t| t.contains("makepad-widgets"));
    let Some(group_dir) = member.parent() else {
        return Ok(());
    };
    let root = group_dir.join(crate::workspace::MANIFEST);
    let Ok(mut group) = std::fs::read_to_string(&root) else {
        return Ok(());
    };
    let patch = crawrad_codegen::dependencias::patch_makepad_platform_at(group_dir);
    if !uses_makepad
        || patch.is_empty()
        || !group.contains("[workspace]")
        || group.contains("makepad-platform")
    {
        return Ok(());
    }
    group.push_str(&patch);
    std::fs::write(&root, group).map_err(|e| format!("cannot write the group's Cargo.toml: {e}"))
}

/// What the generated `build.rs` does: the icon into the `.exe` on Windows.
const APP_BUILD_RS: &str = "fn main() {
    // The app's icon in the .exe (Explorer, taskbar, Alt+Tab): resource 1 (CrawRAD).
    #[cfg(target_os = \"windows\")]
    {
        let _ = embed_resource::compile(\"app.rc\", embed_resource::NONE);
    }
}
";

/// What embeds the crab (already written as `icon.ico` + `icon.png`): `app.rc`, `build.rs`
/// and its `embed-resource` build dependency (the version the IDE itself uses).
fn write_app_icon(folder: &Path) -> Result<(), String> {
    write_file(folder, "build.rs", APP_BUILD_RS)?;
    let title = crate::project_options::read(folder).title;
    write_file(
        folder,
        crate::project_options::APP_RC,
        crate::project_options::app_rc(&title),
    )?;
    let manifest = folder.join(crate::workspace::MANIFEST);
    let mut toml =
        std::fs::read_to_string(&manifest).map_err(|e| format!("cannot read Cargo.toml: {e}"))?;
    toml.push_str("\n[build-dependencies]\nembed-resource = \"3.0\"\n");
    std::fs::write(&manifest, toml).map_err(|e| format!("cannot write Cargo.toml: {e}"))
}

#[note("project-template-write-starter-form")]
fn write_starter_form(
    member: &std::path::Path,
    framework: crawrad_codegen::AlvoGerador,
) -> Result<(), String> {
    let doc = crate::templates::Template::default_template()
        .document(&crawrad_registry::Registry::builtin())
        .ok_or("the blank form template is not in the catalogue")?;
    doc.save(member.join("src").join("form_main.crawform"))
        .map_err(|e| format!("Failed to write src/form_main.crawform: {e}"))?;
    crate::generator::generate_for_new_project_with(member, &doc, framework)
}

#[note("project-template-create-or-add")]
pub fn create_or_add(
    add_to: Option<&std::path::Path>,
    folder: &std::path::Path,
    name: &str,
    template_id: &str,
    git_init: bool,
    framework: crawrad_codegen::AlvoGerador,
) -> Result<PathBuf, String> {
    match add_to {
        Some(root) => add_project(root, name, template_id, framework),
        None => create_project_template(folder, name, template_id, git_init, framework),
    }
}

#[note("project-template-add-project")]
pub fn add_project(
    root: &std::path::Path,
    name: &str,
    template_id: &str,
    framework: crawrad_codegen::AlvoGerador,
) -> Result<PathBuf, String> {
    let folder = root.join(name);
    if std::fs::read_dir(&folder).is_ok_and(|mut d| d.next().is_some()) {
        return Err(format!(
            "The folder '{}' already exists and is not empty.",
            folder.display()
        ));
    }
    let options = ProjectTemplateOptions {
        framework,
        use_binding: false,
        extra_dependencies: Vec::new(),
    };
    refuse_old_toolchain(root, template_id, &options)?;
    write_package(&folder, name, template_id, &options)?;
    crate::workspace_edit::add_member(root, &folder)?;
    Ok(folder)
}

#[note("project-template-structure-preview")]
pub fn structure_preview(
    tpl: &TemplateDef,
    name: &str,
    add_to: Option<&std::path::Path>,
) -> String {
    let head = match add_to.and_then(|root| root.file_name()) {
        Some(ws) => format!(
            "{}/\n├── Cargo.toml   (members += \"{name}\")\n",
            ws.to_string_lossy()
        ),
        None => format!("{name}/\n├── Cargo.toml   (workspace)\n├── .gitignore\n"),
    };
    let project = tpl
        .tree_format
        .replace("{name}", name)
        .replace("{nome}", name);
    let nested: Vec<String> = project
        .lines()
        .enumerate()
        .map(|(i, line)| match i {
            0 => format!("└── {line}"),
            _ => format!("    {line}"),
        })
        .collect();
    head + &nested.join("\n")
}

#[note("project-template-default-project-name")]
pub const DEFAULT_PROJECT_NAME: &str = "my_app";

#[note("project-template-name-or-default")]
pub fn name_or_default(typed: &str) -> String {
    match typed.trim() {
        "" => DEFAULT_PROJECT_NAME.to_string(),
        n => n.to_string(),
    }
}

#[note("project-template-write-file")]
fn write_file(folder: &Path, name: &str, content: impl AsRef<[u8]>) -> Result<(), String> {
    std::fs::write(folder.join(name), content).map_err(|e| format!("Failed to write {name}: {e}"))
}

#[cfg(test)]
#[path = "test/project_template/test.rs"]
mod tests;
