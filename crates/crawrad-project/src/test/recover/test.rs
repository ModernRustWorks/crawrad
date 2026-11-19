use super::*;
use std::time::Duration;

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("crawrad-recover-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("src")).unwrap();
    dir
}

fn set_modified(p: &Path, t: SystemTime) {
    std::fs::File::options()
        .write(true)
        .open(p)
        .unwrap()
        .set_modified(t)
        .unwrap();
}

#[test]
fn the_folder_mirrors_the_unsaved_edits() {
    let root = scratch("mirror");
    let (a, b) = (root.join("src/a.rs"), root.join("Cargo.toml"));
    let outside = std::env::temp_dir().join("elsewhere.rs");
    let n = sync(
        &root,
        [
            (a.as_path(), "fn a() {}"),
            (b.as_path(), "[package]"),
            (outside.as_path(), "x"),
        ],
        &[],
    );
    assert_eq!(n.unwrap(), 2, "a file outside the project gets no copy");
    assert_eq!(
        read(&root, Path::new("src/a.rs")).as_deref(),
        Some("fn a() {}")
    );
    // `b` was saved: its copy goes on the next pass.
    sync(&root, [(a.as_path(), "fn a() { 1 }")], &[]).unwrap();
    assert_eq!(
        read(&root, Path::new("src/a.rs")).as_deref(),
        Some("fn a() { 1 }")
    );
    assert_eq!(read(&root, Path::new("Cargo.toml")), None);
    sync(&root, [], &[]).unwrap();
    assert_eq!(read(&root, Path::new("src/a.rs")), None);
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn only_copies_newer_than_the_disk_are_offered() {
    let root = scratch("pending");
    let now = SystemTime::now();
    let files = ["src/newer.rs", "src/older.rs", "src/gone.rs"].map(|f| root.join(f));
    sync(&root, files.iter().map(|f| (f.as_path(), "copy")), &[]).unwrap();
    // `gone.rs` never reached the disk; the other two did, before and after the copy.
    for (f, disk_age, copy_age) in [("src/newer.rs", 60, 10), ("src/older.rs", 10, 60)] {
        std::fs::write(root.join(f), "disk").unwrap();
        set_modified(&root.join(f), now - Duration::from_secs(disk_age));
        set_modified(&folder(&root).join(f), now - Duration::from_secs(copy_age));
    }
    let files: Vec<_> = pending(&root).into_iter().map(|p| p.file).collect();
    let expected: Vec<PathBuf> = ["src/gone.rs", "src/newer.rs"]
        .iter()
        .map(PathBuf::from)
        .collect();
    assert_eq!(files, expected);
    discard(&root, &[PathBuf::from("src/gone.rs")]);
    assert_eq!(pending(&root).len(), 1);
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_time_reads_as_the_clock_today_then_yesterday_then_the_date() {
    let day = |d: u64, h: u64, m: u64| {
        SystemTime::UNIX_EPOCH + Duration::from_secs(d * 86_400 + h * 3600 + m * 60)
    };
    let now = day(20_000, 15, 0);
    assert_eq!(when(day(20_000, 9, 42), now, 0), "09:42");
    assert_eq!(when(day(19_999, 23, 59), now, 0), "yesterday");
    assert_eq!(when(day(19_990, 9, 0), now, 0), "2024-09-24");
    // 01:30 UTC is still the day before at UTC-3.
    assert_eq!(when(day(20_000, 1, 30), now, -3 * 3600), "yesterday");
}

#[test]
fn a_crash_s_copies_wait_for_an_answer_across_passes_and_exits() {
    let root = scratch("mirror-wait");
    let (left, new) = (root.join("src/left.rs"), root.join("src/new.rs"));
    sync(&root, [(left.as_path(), "from the crash")], &[]).unwrap();
    let mut m = Mirror::default();
    let found = m.follow(Some(&root)).unwrap();
    assert_eq!(
        found.len(),
        1,
        "the file never reached the disk, so its copy is newer"
    );
    assert_eq!(
        m.follow(Some(&root)),
        None,
        "the same project is no arrival"
    );
    // A pass with other edits does not wipe it, and neither does a clean exit.
    assert_eq!(m.sync([(new.as_path(), "typed now")]).unwrap(), 2);
    m.close();
    assert_eq!(
        read(&root, Path::new("src/left.rs")).as_deref(),
        Some("from the crash")
    );
    assert_eq!(read(&root, Path::new("src/new.rs")), None);
    // Answered, it goes like any other.
    m.settle(&[PathBuf::from("src/left.rs")]);
    m.sync([]).unwrap();
    assert_eq!(read(&root, Path::new("src/left.rs")), None);
    assert!(m.kept().is_empty());
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn an_answer_brings_back_or_deletes_and_never_leaves_the_project() {
    let root = scratch("answer");
    let (a, b) = (root.join("src/a.rs"), root.join("src/b.rs"));
    sync(&root, [(a.as_path(), "aa"), (b.as_path(), "bb")], &[]).unwrap();
    std::fs::write(root.join("secret.txt"), "no").unwrap();
    let mut m = Mirror::default();
    m.follow(Some(&root));
    let escape = m.answer(
        vec![
            PathBuf::from("../secret.txt"),
            PathBuf::from("../../secret.txt"),
        ],
        false,
    );
    assert!(escape.is_empty(), "a path out of the folder reads nothing");
    assert_eq!(
        m.answer(vec![PathBuf::from("src/a.rs")], true),
        vec![(a.clone(), "aa".to_string())]
    );
    assert_eq!(
        read(&root, Path::new("src/a.rs")),
        None,
        "discarded is deleted"
    );
    assert_eq!(
        m.answer(Vec::new(), false),
        vec![(b.clone(), "bb".to_string())],
        "none named: every one waiting"
    );
    assert!(m.kept().is_empty());
    assert!(root.join("secret.txt").exists());
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn the_offer_lists_every_copy_checked_and_says_when() {
    let at = |s: u64| SystemTime::UNIX_EPOCH + Duration::from_secs(20_000 * 86_400 + s);
    let found = [
        Pending {
            file: PathBuf::from("src/a.rs"),
            copy: at(9 * 3600 + 42 * 60),
            disk: Some(at(0) - Duration::from_secs(60)),
        },
        Pending {
            file: PathBuf::from("src/gone.rs"),
            copy: at(10 * 3600),
            disk: None,
        },
    ];
    let mut offer = Offer::new(&found, at(15 * 3600), 0);
    let shown: Vec<_> = offer
        .rows
        .iter()
        .map(|r| (r.copy.as_str(), r.disk.as_str(), r.checked))
        .collect();
    assert_eq!(
        shown,
        [("09:42", "yesterday", true), ("10:00", "missing", true)]
    );
    offer.check(0, false);
    offer.check(9, false);
    assert_eq!(offer.checked(), [PathBuf::from("src/gone.rs")]);
    assert_eq!(offer.files().len(), 2);
}
