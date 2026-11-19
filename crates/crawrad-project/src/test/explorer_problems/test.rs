//! A row's problems: by file, summed under a folder, the tone and the badge.

use super::{ProblemMap, Problems, key, state_icon};
use crate::explorer_rows::icon::{ERROR, UNSAVED, WARNING};
use std::path::Path;

fn map() -> ProblemMap {
    let mut m = ProblemMap::new();
    m.insert(
        key(Path::new("C:/p/app/src/main.rs")),
        Problems {
            errors: 1,
            warnings: 2,
        },
    );
    m.insert(
        key(Path::new("C:/p/app/src/lib.rs")),
        Problems {
            errors: 0,
            warnings: 3,
        },
    );
    m.insert(
        key(Path::new("C:/p/app2/src/x.rs")),
        Problems {
            errors: 4,
            warnings: 0,
        },
    );
    m
}

#[test]
fn a_file_has_its_own_and_a_folder_sums_what_is_under_it() {
    let m = map();
    assert_eq!(
        Problems::of(&m, Path::new("C:\\p\\app\\src\\lib.rs")).warnings,
        3
    );
    assert_eq!(
        Problems::of(&m, Path::new("C:/p/app/Cargo.toml")),
        Problems::default()
    );
    let app = Problems::under(&m, Path::new("C:/p/app"));
    assert_eq!((app.errors, app.warnings), (1, 5));
    // `app2` is not under `app`: a prefix of the name is not a folder
    assert_eq!(Problems::under(&m, Path::new("C:/p/app2/")).errors, 4);
}

#[test]
fn an_error_wins_the_tone_and_the_count_goes_before_the_badge() {
    let both = Problems {
        errors: 1,
        warnings: 2,
    };
    let warn = Problems {
        errors: 0,
        warnings: 3,
    };
    assert_eq!(both.tone(), Some("Error"));
    assert_eq!(warn.tone(), Some("Warn"));
    assert_eq!(Problems::default().tone(), None);
    assert_eq!(both.badge("Yours"), "3 \u{b7} Yours");
    assert_eq!(warn.badge(""), "3");
    assert_eq!(Problems::default().badge("Rewritten"), "Rewritten");
}

#[test]
fn state_icon_precedence() {
    let err_and_warn = Problems {
        errors: 1,
        warnings: 2,
    };
    let warn_only = Problems {
        errors: 0,
        warnings: 3,
    };
    let clean = Problems::default();

    // error beats warning and unsaved
    assert_eq!(state_icon(err_and_warn, true), Some(ERROR));
    assert_eq!(state_icon(err_and_warn, false), Some(ERROR));

    // warning beats unsaved
    assert_eq!(state_icon(warn_only, true), Some(WARNING));
    assert_eq!(state_icon(warn_only, false), Some(WARNING));

    // unsaved alone gives UNSAVED
    assert_eq!(state_icon(clean, true), Some(UNSAVED));

    // clean and saved gives None
    assert_eq!(state_icon(clean, false), None);
}
