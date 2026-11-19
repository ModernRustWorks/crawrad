//! The shape of the shell's `main.rs`: no console in release, and the panic
//! hook handing failures to `crawrad_os::crash_report`.

/// The shell's `main`, read as text: what these two proofs guard is the
/// **shape** of `main`, which cannot be called from a test without opening a
/// window.
const MAIN_RS: &str = include_str!("../../main.rs");

#[test]
fn the_release_build_has_no_console() {
    // the attribute did not come across from the egui shell, and
    // the release opened a black console before the window.
    let attribute = format!(
        "#![cfg_attr(not(debug_assertions), {} = \"windows\")]",
        "windows_subsystem"
    );
    assert_eq!(
        MAIN_RS.matches(&attribute).count(),
        1,
        "`main.rs` must declare the windows subsystem for release, exactly once"
    );
}

#[test]
fn the_panic_hook_sends_the_failure_to_the_reporter() {
    // Without a console, a panic that does not reach the reporter is silence.
    assert!(
        MAIN_RS.contains("std::panic::set_hook"),
        "`main` no longer installs the panic hook"
    );
    assert_eq!(
        MAIN_RS.matches("crash_report::report(").count(),
        1,
        "the panic hook stopped sending the failure to the crash reporter"
    );
}
