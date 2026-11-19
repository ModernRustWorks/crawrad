//! **The manifest becomes an artifact** — `manifest!` writes `manifest.json`.
//!
//! `#[derive(Component)]` gives each struct its own component object. This is
//! what turns those objects into the file the IDE reads, and what makes the
//! file stop being something a person maintains.
//!
//! ```text
//! crawrad_component::manifest!(Gauge);
//! ```
//!
//! One line at the bottom of `lib.rs`, and the package has a manifest.
//!
//! # It is a test, and that is the whole trick
//!
//! The file has to exist on disk as plain text, because the IDE reads it
//! **without executing anything** — a component is data until the user's app is
//! built ([ADR 0036]). So the macro cannot simply *be* the manifest.
//!
//! What it does instead is emit a test that compares the file against what the
//! structs say, and rewrites it when asked:
//!
//! ```text
//! cargo test                      # the file disagrees -> red, with the line
//! CRAWRAD_BLESS=1 cargo test      # the file is rewritten from the structs
//! ```
//!
//! That is the same bargain a classic RAD IDE makes, and the same one its form file makes: the
//! metadata is generated, it lives in the repository so tools can read it
//! without running anything, and it is never the place you edit.
//!
//! # No dependency, and that is not an accident
//!
//! Everything here is `macro_rules!`, `format!` and `std::fs` — not even a JSON
//! crate, which is why [`json_string`] exists. The box is linked into the
//! binary of somebody who only dragged an icon onto a form, so it has an empty
//! `[dependencies]` and a gate that fails if it ever gains a line. A macro
//! costs that person nothing.
//!

/// The manifest contract version this box speaks.
///
/// **The manifest contract this box speaks.**
///
/// It lives here, at the bottom of the stack, because both halves need it and
/// two copies would be one copy too many: `crawrad-package` refuses a file whose
/// `sdk` it does not know, and `manifest!` writes that number into the file it
/// generates. If they ever disagreed, every package this box built would be
/// rejected by the IDE that shipped with it.
///
/// **2 since 18/09/2026**, when the format moved from TOML to JSON so a
/// component's drawing could be a tree you can see. That is exactly the change
/// this number exists for: the format stopped accepting what it used to accept.
///
/// # Examples
///
/// ```rust
/// use crawrad_component::SDK;
///
/// assert_eq!(SDK, 2);
/// ```
pub const SDK: u32 = 2;

/// The SDK a manifest declares when one of its components `inherits` a native one.
/// An IDE that only knows [`SDK`] refuses it, instead of installing the component without
/// its base.
pub const SDK_INHERITS: u32 = 3;

/// The notice every generated manifest carries, under the `$doc` key.
///
/// It is addressed to the person who opens the file expecting to edit it, which
/// is what everybody did until this macro existed. JSON has no comments, so it
/// is a field — and a field is better than a comment anyway, because a tool can
/// read it.
///
/// # Examples
///
/// ```rust
/// use crawrad_component::GENERATED_NOTICE;
///
/// assert!(GENERATED_NOTICE.contains("generated"));
/// ```
pub const GENERATED_NOTICE: &str = "This file is generated from the Rust source. \
Change the struct that carries #[derive(Component)], then run \
`CRAWRAD_BLESS=1 cargo test` to rewrite it. It stays in the repository because \
the IDE reads it WITHOUT running anything.";

/// Escapes a string as JSON according to the specification.
///
/// **A JSON string, escaped by the specification and not by eye.**
///
/// Rust's `{:?}` is *almost* this, and that is the trap: it writes a control
/// character the way Rust spells it -- with braces -- and JSON accepts only the
/// four-hex-digit form without them.
/// manifest a parser refuses is worse than one that merely looks wrong.
///
/// It is public because the assembled file quotes the package's own name,
/// version and description, and those come from `Cargo.toml` — text this crate
/// does not control.
///
/// # Examples
///
/// ```rust
/// use crawrad_component::manifest::json_string;
///
/// assert_eq!(json_string("hello"), "\"hello\"");
/// ```
pub fn json_string(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len() + 2);
    out.push('"');
    for c in raw.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Assembles the complete manifest JSON from package metadata and components.
///
/// **The whole file, from a package's identity and its component objects.**
///
/// This is the ONE assembler, and it lives here for a reason that the rest of
/// this crate is also about: two things write this file, and they have to write
/// the same bytes.
///
/// | who | when |
/// |---|---|
/// | `manifest!`, through the test it emits | on `cargo test`, from the compiled constants |
/// | `crawrad manifest`, the CLI | when somebody asks, **without compiling anything** |
///
/// If each had its own assembler, they would agree until the first person
/// touched one of them — and then a package would carry a file that its own
/// test says is right and the tool says is wrong.
///
/// `components` are the objects `#[derive(Component)]` produced, each already
/// indented to sit at column four.
///
/// # Examples
///
/// ```rust
/// use crawrad_component::manifest::assemble;
///
/// let json = assemble("demo", "0.1.0", "A demo", &[]);
/// assert!(json.contains("\"name\": \"demo\""));
/// ```
pub fn assemble(name: &str, version: &str, description: &str, components: &[&str]) -> String {
    let mut out = String::from("{\n");
    out.push_str(&format!("  \"$doc\": {},\n", json_string(GENERATED_NOTICE)));
    // `"inherits":` unescaped can only be a key: inside a JSON string the quotes are `\"`.
    let sdk = if components.iter().any(|c| c.contains("\"inherits\":")) {
        SDK_INHERITS
    } else {
        SDK
    };
    out.push_str(&format!("  \"sdk\": {sdk},\n"));
    out.push_str("  \"package\": {\n");
    out.push_str(&format!("    \"name\": {},\n", json_string(name)));
    out.push_str(&format!("    \"version\": {},\n", json_string(version)));
    out.push_str(&format!(
        "    \"description\": {}\n",
        json_string(description)
    ));
    out.push_str("  },\n");
    out.push_str("  \"components\": [\n");
    let bodies: Vec<String> = components.iter().map(|b| format!("    {b}")).collect();
    out.push_str(&bodies.join(",\n"));
    out.push_str("\n  ]\n}\n");
    out
}

/// Emit the test that keeps `manifest.json` equal to the structs.
///
/// Pass every type in the package that derives `Component`:
///
/// ```text
/// crawrad_component::manifest!(Gauge, Dial);
/// ```
///
/// The order is the order they appear in the file, and therefore the order the
/// palette shows them.
#[macro_export]
macro_rules! manifest {
    ($($component:ty),+ $(,)?) => {
        #[cfg(test)]
        mod crawrad_manifest {
            use super::*;

            /// Where the file lives: next to the `Cargo.toml` of THIS package.
            const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/manifest.json");

            /// The file as the structs describe it, byte for byte.
            ///
            /// The assembling itself is [`assemble`], and not written here: the
            /// `crawrad manifest` CLI writes the same file without compiling
            /// anything, and two assemblers would be two files that agree only
            /// until somebody touches one.
            fn from_the_structs() -> String {
                use $crate::canvas::{CanvasDetector, CanvasExtract, CanvasFallback, merge_manifest_draw};
                let bodies = [
                    $({
                        let raw = <$component>::CRAWRAD_COMPONENT_JSON;
                        let detector = CanvasDetector::<$component>::new();
                        let draw_json = (&&detector).extract_draw_json();
                        merge_manifest_draw(raw, draw_json.as_deref())
                    }),+
                ];
                let refs: Vec<&str> = bodies.iter().map(|s| s.as_str()).collect();
                $crate::manifest::assemble(
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION"),
                    env!("CARGO_PKG_DESCRIPTION"),
                    &refs,
                )
            }

            /// **The file on disk says what the structs say.**
            ///
            /// Red here means somebody changed a struct and the manifest did
            /// not follow -- the exact drift this macro exists to make
            /// impossible. Rerun with `CRAWRAD_BLESS=1` to rewrite the file.
            #[test]
            fn the_manifest_on_disk_is_what_the_structs_say() {
                let wanted = from_the_structs();

                if std::env::var_os("CRAWRAD_BLESS").is_some() {
                    std::fs::write(PATH, &wanted)
                        .unwrap_or_else(|e| panic!("could not write {PATH}: {e}"));
                    return;
                }

                let found = std::fs::read_to_string(PATH).unwrap_or_else(|e| {
                    panic!("could not read {PATH}: {e}\nrun `CRAWRAD_BLESS=1 cargo test` to create it")
                });

                // Line endings are the repository's business, not the
                // manifest's: this tree checks out CRLF on Windows and LF
                // elsewhere, and a manifest that failed on one platform would
                // be a gate that measures the checkout instead of the content.
                let normalize = |text: &str| text.replace("\r\n", "\n");
                let (found, wanted) = (normalize(&found), normalize(&wanted));
                if found == wanted {
                    return;
                }

                let mut report = String::from(
                    "manifest.json no longer matches the structs.\n\
                     Rerun with `CRAWRAD_BLESS=1 cargo test` to rewrite it.\n\n",
                );
                for (line, (a, b)) in found.lines().zip(wanted.lines()).enumerate() {
                    if a != b {
                        report.push_str(&format!(
                            "first difference, line {}:\n  on disk:      {a}\n  from structs: {b}\n",
                            line + 1
                        ));
                        break;
                    }
                }
                if found.lines().count() != wanted.lines().count() {
                    report.push_str(&format!(
                        "the file has {} lines and the structs say {}\n",
                        found.lines().count(),
                        wanted.lines().count()
                    ));
                }
                panic!("{report}");
            }
        }
    };
}
