//! How the designer paints a form's page: a web project is a white
//! browser page; a desktop one keeps the dark window, unless its form says
//! `thLight`. The generated app makes the same choice (`crawrad-codegen`,
//! `targets/makepad/form.rs`): `thLight` always light, `thSystem` light in the
//! browser build, `thDark` never.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

thread_local! {
    /// The last answer per member, with the `Cargo.toml` time it was read at:
    /// the designer asks on every sync, and only a stat is paid for it.
    static WEB: RefCell<Vec<(PathBuf, Option<SystemTime>, bool)>> = const { RefCell::new(Vec::new()) };
}

/// Whether the package at `member` says `[package.metadata.crawrad] platform = "web"`.
pub fn is_web_package(member: &Path) -> bool {
    let manifest = member.join("Cargo.toml");
    let stamp = std::fs::metadata(&manifest).and_then(|m| m.modified()).ok();
    WEB.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some((_, at, web)) = cache.iter().find(|(m, _, _)| m == member)
            && *at == stamp
        {
            return *web;
        }
        let web = crate::web_run::is_web_member(member);
        cache.retain(|(m, _, _)| m != member);
        cache.push((member.to_path_buf(), stamp, web));
        web
    })
}

/// Whether a form with `theme` (`thSystem`, `thLight`, `thDark`) is painted light.
pub fn is_light(theme: Option<&str>, web: bool) -> bool {
    match theme {
        Some("thLight") => true,
        Some("thDark") => false,
        _ => web,
    }
}

/// `(light, web)` for the form `doc` of the package at `member`.
pub fn of_form(member: Option<&Path>, doc: Option<&crawrad_ir::Document>) -> (bool, bool) {
    let web = member.is_some_and(is_web_package);
    let theme = doc
        .and_then(|d| d.node(&d.root))
        .and_then(|root| match root.props.get("theme") {
            Some(crawrad_ir::Value::Enum(t)) => Some(t.as_str()),
            _ => None,
        });
    (is_light(theme, web), web)
}

#[cfg(test)]
#[path = "test/page_look/test.rs"]
mod tests;
