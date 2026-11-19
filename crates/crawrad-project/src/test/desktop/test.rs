use super::*;

fn sample() -> Desktop {
    Desktop {
        window: Some(WindowPlace {
            x: -8.0,
            y: 10.5,
            width: 1600.0,
            height: 900.0,
            maximized: true,
        }),
        tabs: vec![
            TabPlace {
                file: PathBuf::from("src/main.rs"),
                line: 12,
                byte: 4,
            },
            TabPlace {
                file: PathBuf::from("src/my form.rs"),
                line: 0,
                byte: 0,
            },
        ],
        active: Some(1),
        dock: "dock v1\nline two".into(),
    }
}

/// The text round-trips whole, a path with a space and the dock's own lines
/// included.
#[test]
fn desktop_round_trips() {
    let d = sample();
    assert_eq!(Desktop::decode(&d.encode()), Some(d));
    assert_eq!(
        Desktop::decode(&Desktop::default().encode()),
        Some(Desktop::default())
    );
}

/// A foreign header throws the file away; an unknown or broken line is only
/// skipped, and an active index past the tabs is no tab.
#[test]
fn desktop_skips_what_it_cannot_read() {
    assert_eq!(Desktop::decode("crawrad-desktop 2\n"), None);
    assert_eq!(Desktop::decode(""), None);
    let d = Desktop::decode(
        "crawrad-desktop 1\nwindow 1 2 0 4 0\nzoom 3\ntab x 1 a.rs\ntab 1 2\ntab 3 4 b.rs\nactive 5\n",
    )
    .unwrap();
    assert_eq!(d.window, None);
    assert_eq!(d.tabs.len(), 1);
    assert_eq!(d.tabs[0].file, PathBuf::from("b.rs"));
    assert_eq!(d.active, None);
}

/// Only the project's files become tabs, relative to it, with their caret,
/// and the front follows its tab.
#[test]
fn desktop_takes_the_tabs_of_the_project() {
    let root = Path::new("/p");
    let files = [
        PathBuf::from("/elsewhere/x.rs"),
        PathBuf::from("/p/src/a.rs"),
        PathBuf::from("/p/src/b.rs"),
    ];
    let caret = |f: &Path| f.ends_with("b.rs").then_some((7, 2));
    let d = Desktop::default().with_tabs(root, &files, Some(2), caret);
    assert_eq!(d.tabs.len(), 2);
    assert_eq!(d.tabs[0].file, PathBuf::from("src/a.rs"));
    assert_eq!((d.tabs[0].line, d.tabs[0].byte), (0, 0));
    assert_eq!((d.tabs[1].line, d.tabs[1].byte), (7, 2));
    assert_eq!(d.active, Some(1));
    let outside = Desktop::default().with_tabs(root, &files, Some(0), caret);
    assert_eq!(outside.active, None);
}

/// Read from the project: a tab whose file is gone is dropped, and the
/// front stays on its own tab.
#[test]
fn desktop_read_drops_gone_files() {
    let root = std::env::temp_dir().join(format!("crawrad-desktop-{}", std::process::id()));
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(root.join("src/my form.rs"), "").unwrap();
    sample().write(&root).unwrap();
    let d = Desktop::read(&root).unwrap();
    assert_eq!(d.tabs.len(), 1);
    assert_eq!(d.active, Some(0));
    assert_eq!(d.dock, sample().dock);
    std::fs::remove_dir_all(&root).unwrap();
}
