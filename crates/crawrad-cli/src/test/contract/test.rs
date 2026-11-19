//! **THE CONTRACT GATE**: the CLI and the macro write the same file.
//!
//! # Why it exists
//!
//! [ADR 0039] left **two** writers of `manifest.json`, on purpose:
//!
//! | who | when | what only it can do |
//! |---|---|---|
//! | the test that `manifest!` emits | on `cargo test`, with the crate compiled | **prove** that `Default::default()` matches the declared defaults |
//! | `crawrad manifest` | on request, without compiling anything | **write** the file, and it is what `New Project ▸ Package` calls |
//!
//! Two writers are two chances to disagree, and the disagreement would show up
//! in the worst way: the author's `cargo test` green, the CI's
//! `crawrad manifest --check` red, both looking at the same file.
//!
//! This is what prevents it. It runs the CLI's assembly over the **real**
//! packages of this tree and compares byte for byte with the file the macro's
//! test blessed.
//!
//! # What it catches that neither of the two catches alone
//!
//! The **order**. The macro uses the order the author wrote in `manifest!(A, B)`;
//! the CLI uses the order of the files. While there is one component per
//! package the two agree by luck, and on the day the second one arrives, this
//! gate is what warns, instead of the user finding out.
//!

use std::path::{Path, PathBuf};

use crate::{MANIFEST_FILE, assemble, first_difference, same_file};

/// The `components/` folder of this repository.
fn suite() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("components")
}

/// The packages that have a manifest: the ones the IDE would install.
fn packages() -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = std::fs::read_dir(suite())
        .expect("the `components/` folder exists")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.join(MANIFEST_FILE).is_file())
        .collect();
    found.sort();
    found
}

/// **The suite is not empty**, otherwise the gate would pass without measuring anything.
///
/// It is the trap this house has already paid for: a gate that scans a folder
/// and finds nothing stays green forever, and nobody finds out it stopped looking.
#[test]
fn there_are_packages_to_measure() {
    let found = packages();
    assert!(
        found.len() >= 2,
        "expected at least the suite's two packages, found {}: {found:?}",
        found.len()
    );
}

/// **The CLI writes the same file that is in the repository.**
///
/// The file in the repository is the one the macro's test blessed, so this is
/// the contract between the two writers: measured, not agreed upon.
///
/// `portadb-crawrad` is in on purpose and is the most interesting case: its
/// manifest is **hand-written** and its crate has a `struct` without
/// `#[derive(Component)]`. The CLI must refuse to touch it instead of rewriting
/// it with whatever it managed to guess.
#[test]
fn the_cli_writes_the_same_as_the_macro() {
    for package in packages() {
        let name = package.file_name().unwrap_or_default().to_string_lossy();
        let on_disk = std::fs::read_to_string(package.join(MANIFEST_FILE))
            .unwrap_or_else(|e| panic!("{name}: {e}"));

        match assemble(&package) {
            Ok(from_cli) => assert!(
                same_file(&on_disk, &from_cli),
                "{name}: the CLI and the macro write DIFFERENT files\n{}",
                first_difference(&on_disk, &from_cli).unwrap_or_default()
            ),
            // **Refusing is the right answer for a hand-written manifest.**
            //
            // Without `#[derive(Component)]` in the source, the CLI has nothing
            // to read, and writing anyway would erase the work of whoever wrote
            // the file. See section 4 of ADR 0039.
            Err(complaint) => assert!(
                complaint.contains("no `#[derive(Component)]` found"),
                "{name}: expected the refusal for having nothing to read, got: {complaint}"
            ),
        }
    }
}

/// **At least one package goes through the real contract path.**
///
/// Without this, the test above would stay green in a world where EVERY
/// package were hand-written, that is, without the CLI ever having been
/// compared with anything. It is the same family as
/// `there_are_packages_to_measure`, one step further.
#[test]
fn at_least_one_package_is_generated_and_matches() {
    let checked = packages()
        .into_iter()
        .filter(|p| assemble(p).is_ok())
        .count();
    assert!(
        checked >= 1,
        "no package of the suite is generated: the contract was measured nowhere"
    );
}
