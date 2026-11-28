//! The app icon is DATA, and a green build does not inspect data.
//!
//! `build.rs` now refuses to be silent when the resource cannot be embedded, but
//! a successful embed says nothing about WHAT was embedded. These assertions
//! cover the part the compiler cannot: that `app.rc` names files that exist, and
//! that the icon carries the sizes Windows actually asks for.

const RC: &str = include_str!("../../../app.rc");

/// `app.rc` must name files that are really there.
///
/// A renamed or moved `.ico` leaves `rc.exe` to fail, and the failure used to be
/// swallowed. Now it panics the build — but this test says WHICH file is missing,
/// at the place where a person is reading the resource script.
#[test]
fn app_rc_names_files_that_exist() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut named = 0usize;
    for linha in RC.lines() {
        let Some((_, resto)) = linha.split_once('"') else {
            continue;
        };
        let Some((arquivo, _)) = resto.split_once('"') else {
            continue;
        };
        if !(arquivo.ends_with(".ico") || arquivo.ends_with(".manifest")) {
            continue;
        }
        named += 1;
        assert!(
            dir.join(arquivo).exists(),
            "app.rc names `{arquivo}`, which is not beside it in {}",
            dir.display()
        );
    }
    assert_eq!(
        named, 2,
        "app.rc must name exactly the manifest and the icon; naming fewer means one \
         was dropped and this test would have passed over the absence"
    );
}

/// The icon must carry every size Windows asks for.
///
/// Windows picks the nearest size and scales it. An `.ico` missing 256x256 looks
/// soft or wrong in Explorer's large views and on the taskbar at high DPI, which
/// reads to a person exactly like "that is not our icon" — with the build green
/// and the file present.
#[test]
fn the_icon_carries_every_size_windows_asks_for() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let bytes = std::fs::read(dir.join("crawrad.ico")).expect("the icon must be readable");
    assert!(bytes.len() > 6, "the icon file is too short to be an icon");
    let count = u16::from_le_bytes([bytes[4], bytes[5]]) as usize;
    assert!(
        bytes.len() >= 6 + count * 16,
        "the icon's directory is truncated: {count} entries do not fit in {} bytes",
        bytes.len()
    );
    let mut sizes: Vec<u32> = (0..count)
        .map(|i| match bytes[6 + i * 16] {
            0 => 256,
            w => u32::from(w),
        })
        .collect();
    sizes.sort_unstable();
    for wanted in [16u32, 32, 48, 256] {
        assert!(
            sizes.contains(&wanted),
            "the icon has no {wanted}x{wanted} image, so Windows scales a nearby one \
             and it reads as the wrong icon; present sizes: {sizes:?}"
        );
    }
}
