use super::*;

#[test]
fn read_defaults_from_simple_manifest() {
    let tmp = std::env::temp_dir().join(format!("crawrad-opt-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(&tmp);

    let cargo = r#"[package]
name = "my_app"
version = "0.1.0"
"#;
    std::fs::write(tmp.join("Cargo.toml"), cargo).unwrap();

    let opts = read(&tmp);
    assert_eq!(opts.title, "my_app");
    assert_eq!(opts.icon, IconChoice::Default);

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn write_creates_table_when_missing() {
    let tmp = std::env::temp_dir().join(format!("crawrad-opt-create-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(&tmp);

    let cargo = r#"[package]
name = "demo"
version = "0.1.0"
"#;
    std::fs::write(tmp.join("Cargo.toml"), cargo).unwrap();

    let opts = ApplicationOptions {
        title: "Demo App".to_string(),
        icon: IconChoice::Custom,
    };
    write(&tmp, &opts).unwrap();

    let read_back = read(&tmp);
    assert_eq!(read_back.title, "Demo App");
    assert_eq!(read_back.icon, IconChoice::Custom);

    let written = std::fs::read_to_string(tmp.join("Cargo.toml")).unwrap();
    assert!(written.contains("[package.metadata.crawrad]"));
    assert!(written.contains("title = \"Demo App\""));
    assert!(written.contains("icon = \"custom\""));
    assert!(written.contains("name = \"demo\""));

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn write_keeps_other_lines_and_platform_web() {
    let tmp = std::env::temp_dir().join(format!("crawrad-opt-web-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(&tmp);

    let cargo = r#"[package]
name = "web_demo"
version = "0.1.0"

[package.metadata.crawrad]
platform = "web"
"#;
    std::fs::write(tmp.join("Cargo.toml"), cargo).unwrap();

    let opts = ApplicationOptions {
        title: "Web Title".to_string(),
        icon: IconChoice::Default,
    };
    write(&tmp, &opts).unwrap();

    let content = std::fs::read_to_string(tmp.join("Cargo.toml")).unwrap();
    assert!(content.contains("platform = \"web\""));
    assert!(content.contains("title = \"Web Title\""));
    assert!(content.contains("icon = \"default\""));
    assert!(content.contains("name = \"web_demo\""));

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn round_trip_options() {
    let tmp = std::env::temp_dir().join(format!("crawrad-opt-rt-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(&tmp);

    let cargo = r#"[package]
name = "calc"
version = "0.1.0"

[dependencies]
serde = "1.0"
"#;
    std::fs::write(tmp.join("Cargo.toml"), cargo).unwrap();

    let initial = ApplicationOptions {
        title: "Super Calculator".to_string(),
        icon: IconChoice::Custom,
    };
    write(&tmp, &initial).unwrap();
    let r1 = read(&tmp);
    assert_eq!(r1, initial);

    let updated = ApplicationOptions {
        title: "Calculator v2".to_string(),
        icon: IconChoice::Default,
    };
    write(&tmp, &updated).unwrap();
    let r2 = read(&tmp);
    assert_eq!(r2, updated);

    let content = std::fs::read_to_string(tmp.join("Cargo.toml")).unwrap();
    assert!(content.contains("serde = \"1.0\""));

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn ico_from_png_produces_four_frames_and_rgba_buffer() {
    // Create a 32x32 test PNG image
    let mut raw_rgba = vec![0u8; 32 * 32 * 4];
    for chunk in raw_rgba.chunks_exact_mut(4) {
        chunk[0] = 255; // R
        chunk[1] = 128; // G
        chunk[2] = 0; // B
        chunk[3] = 255; // A
    }

    let png_bytes = encode_png(32, 32, &raw_rgba).expect("encode test PNG");

    let ico_bytes = ico_from_png(&png_bytes).expect("convert PNG to ICO");
    let frames = ico_frames(&ico_bytes);
    assert_eq!(frames.len(), 4);

    let sizes: Vec<u32> = frames.iter().map(|(s, _)| *s).collect();
    assert_eq!(sizes, vec![16, 32, 48, 256]);

    let frame_16 = ico_frame_rgba(&ico_bytes, 16).expect("get 16x16 frame RGBA");
    assert_eq!(frame_16.0, 16);
    assert_eq!(frame_16.1, 16);
    assert_eq!(frame_16.2.len(), 16 * 16 * 4); // 1024 bytes!
}

#[test]
fn apply_icon_rejects_unsupported_extensions() {
    let tmp = std::env::temp_dir().join(format!("crawrad-opt-apply-err-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(&tmp);

    let bmp_path = tmp.join("x.bmp");
    std::fs::write(&bmp_path, [0u8; 64]).unwrap();

    let res = apply_icon(&tmp, IconSource::File(bmp_path));
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "only .ico or .png");

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn apply_icon_default_writes_crab_files() {
    let tmp = std::env::temp_dir().join(format!("crawrad-opt-apply-def-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(&tmp);

    let res = apply_icon(&tmp, IconSource::Default).unwrap();
    assert_eq!(res, IconChoice::Default);

    assert!(tmp.join("icon.ico").is_file());
    let ico = std::fs::read(tmp.join("icon.ico")).unwrap();
    assert_eq!(ico, DEFAULT_ICON);

    assert!(tmp.join("icon.png").is_file());
    let png = std::fs::read(tmp.join("icon.png")).unwrap();
    assert!(png.starts_with(&[0x89, b'P', b'N', b'G']));

    let _ = std::fs::remove_dir_all(&tmp);
}

/// A fresh folder with a minimal `Cargo.toml` in `eol` line endings.
fn project(tag: &str, eol: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("crawrad-opt-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp folder");
    let toml = ["[package]", "name = \"calc\"", "version = \"0.1.0\"", ""].join(eol);
    std::fs::write(dir.join("Cargo.toml"), toml).expect("manifest");
    dir
}

#[test]
fn write_keeps_a_crlf_manifest_crlf() {
    let dir = project("crlf", "\r\n");
    let opts = ApplicationOptions {
        title: "Calc".into(),
        icon: IconChoice::Default,
    };
    write(&dir, &opts).expect("writes");
    let text = std::fs::read_to_string(dir.join("Cargo.toml")).expect("reads");
    assert!(
        !text.replace("\r\n", "").contains('\n'),
        "a bare LF slipped in: {text:?}"
    );
    assert_eq!(read(&dir).title, "Calc");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn save_writes_title_and_icon_and_a_failed_icon_writes_nothing() {
    let dir = project("save", "\n");
    let said = save(&dir, "Calculator", Some(IconSource::Default)).expect("saves");
    assert!(said.contains("Calculator"), "{said}");
    assert!(dir.join(ICON_ICO).is_file() && dir.join(ICON_PNG).is_file());
    assert_eq!(read(&dir).title, "Calculator");
    let before = std::fs::read_to_string(dir.join("Cargo.toml")).expect("reads");
    let bad = save(&dir, "Other", Some(IconSource::File(dir.join("x.bmp"))));
    assert_eq!(bad, Err("only .ico or .png".to_string()));
    assert_eq!(
        std::fs::read_to_string(dir.join("Cargo.toml")).expect("reads"),
        before
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_crab_previews_at_every_size() {
    for size in [16, 32, 48, 256] {
        assert!(preview_png(DEFAULT_ICON, size).is_some(), "{size}");
    }
    assert_eq!(
        current_icon(std::path::Path::new("nowhere"), IconChoice::Custom),
        DEFAULT_ICON
    );
}

#[test]
fn the_title_goes_into_the_exe_description_when_the_project_embeds_its_icon() {
    let dir = project("rc", "\n");
    save(&dir, "Calc", None).expect("saves");
    assert!(
        !dir.join(APP_RC).exists(),
        "no app.rc is created for a project without one"
    );
    std::fs::write(dir.join(APP_RC), app_rc("old")).expect("rc");
    save(&dir, "Say \"hi\" Calc", None).expect("saves");
    let rc = std::fs::read_to_string(dir.join(APP_RC)).expect("reads");
    assert!(
        rc.contains("VALUE \"FileDescription\", \"Say \"\"hi\"\" Calc\""),
        "{rc}"
    );
    assert!(rc.contains("1 ICON \"icon.ico\""), "{rc}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_project_without_a_log_section_reads_the_defaults() {
    let dir = project("log-none", "\n");
    assert_eq!(read_log(&dir), LogOptions::default());
}

#[test]
fn the_log_settings_round_trip_beside_the_application_ones() {
    let dir = project("log-rt", "\r\n");
    let app = ApplicationOptions {
        title: "Calc".into(),
        icon: IconChoice::Default,
    };
    write(&dir, &app).unwrap();
    let log = LogOptions {
        enabled: false,
        file_name: "{app}_{date}_{time}.log".into(),
        folder: LogFolder::Custom("D:/logs/calc".into()),
        debug_level: LogLevel::Debug,
        release_level: LogLevel::Error,
        new_file: NewFile::EveryRun,
        keep: 7,
    };
    write_log(&dir, &log).unwrap();
    assert_eq!(read_log(&dir), log);
    assert_eq!(read(&dir).title, "Calc");
    write(&dir, &app).unwrap();
    assert_eq!(
        read_log(&dir),
        log,
        "saving the Application page keeps the log"
    );
    let text = std::fs::read_to_string(dir.join("Cargo.toml")).unwrap();
    assert!(
        !text.replace("\r\n", "").contains('\n'),
        "CRLF kept: {text:?}"
    );
    assert_eq!(text.matches(LOG_SECTION).count(), 1);
}

#[test]
fn a_refused_log_setting_leaves_the_manifest_untouched() {
    let dir = project("log-bad", "\n");
    let before = std::fs::read_to_string(dir.join("Cargo.toml")).unwrap();
    let log = LogOptions {
        keep: 0,
        ..LogOptions::default()
    };
    assert!(write_log(&dir, &log).is_err());
    assert_eq!(
        std::fs::read_to_string(dir.join("Cargo.toml")).unwrap(),
        before
    );
}

#[test]
fn the_sample_override_lands_in_the_project_folder() {
    let dir = project("log-sample", "\n");
    let path = write_sample_override(&dir, "calc", &LogOptions::default()).unwrap();
    assert_eq!(path, dir.join("calc.log.toml"));
    let parsed: toml::Table = std::fs::read_to_string(&path).unwrap().parse().unwrap();
    assert_eq!(parsed["level"].as_str(), Some("warn"));
}

#[test]
fn custom_sizes_round_trip() {
    let dir = project("sizes-rt", "\n");
    let sizes = vec![
        ("Small laptop".to_string(), 1024, 600),
        ("Big monitor".to_string(), 2560, 1440),
    ];
    write_sizes(&dir, &sizes).unwrap();

    let read_back = read_sizes(&dir);
    assert_eq!(read_back, sizes);

    let text = std::fs::read_to_string(dir.join("Cargo.toml")).unwrap();
    assert!(text.contains(SIZES_SECTION));
    assert!(text.contains("custom = [\"Small laptop:1024x600\", \"Big monitor:2560x1440\"]"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn custom_sizes_skip_bad_entries() {
    let dir = project("sizes-bad", "\n");
    let cargo = r#"[package]
name = "calc"
version = "0.1.0"

[package.metadata.crawrad.sizes]
custom = [
    "Small laptop:1024x600",
    "bad_no_colon",
    ":1024x600",
    "bad_dims:1024",
    "bad_width:ax600",
    "bad_height:1024xb",
    "zero_width:0x600",
    "zero_height:800x0",
    "Big monitor:2560x1440",
]
"#;
    std::fs::write(dir.join("Cargo.toml"), cargo).unwrap();

    let read_back = read_sizes(&dir);
    assert_eq!(
        read_back,
        vec![
            ("Small laptop".to_string(), 1024, 600),
            ("Big monitor".to_string(), 2560, 1440),
        ]
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn custom_sizes_keep_other_sections_byte_for_byte() {
    let dir = project("sizes-keep", "\n");
    let initial = r#"[package]
name = "calc"
version = "0.1.0"

[dependencies]
serde = "1.0"

[package.metadata.crawrad]
title = "Original Title"
icon = "default"
"#;
    std::fs::write(dir.join("Cargo.toml"), initial).unwrap();

    let sizes = vec![("Laptop".to_string(), 1024, 600)];
    write_sizes(&dir, &sizes).unwrap();

    let content = std::fs::read_to_string(dir.join("Cargo.toml")).unwrap();
    assert!(content.starts_with(initial));
    assert!(content.contains(SIZES_SECTION));
    assert!(content.contains("custom = [\"Laptop:1024x600\"]"));
    let _ = std::fs::remove_dir_all(&dir);
}
