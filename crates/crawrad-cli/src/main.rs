//! # `crawrad`: the CrawRAD command line
//!
//! Provides CLI commands for packages and documentation notes without compiling anything.
//!
//! ## Commands
//!
//! - `crawrad manifest [--check] [PATH]`: Reads Rust source code and writes or checks `manifest.json`.
//! - `crawrad note [--check] [--missing] [--links] [PATH]`: Reads `.crawrad/notes/*.md` and source code anchors, reporting
//!   every note, orphan notes, dangling anchors, kind mismatches, and stale signatures, or lists undocumented
//!   public items when `--missing` is given. When `--links` is given, also runs cargo doc and reports broken
//!   intra-doc links mapped to their notes.
//! - `crawrad note inline [--check] [PATH]`: Inlines doc notes back into the Rust source as `///` lines and
//!   removes their anchors.
//! - `crawrad note extract [--dry-run] [--all] [PATH]`: Turns `///` doc comments under `PATH/src` into doc notes
//!   under `.crawrad/notes/` and leaves `#[note]` anchors in the source.

// No `unsafe`, and the compiler guarantees it, as in every crate of this house.
#![forbid(unsafe_code)]

use std::path::PathBuf;
use std::process::ExitCode;

use crawrad_cli::{
    MANIFEST_FILE, assemble, broken_links, extract_notes, first_difference, inline_notes,
    missing_docs, same_file, scan_notes,
};

const USAGE: &str = "\
crawrad — the CrawRAD command line

USAGE:
    crawrad manifest [--check] [PATH]
    crawrad note [--check] [--missing] [--links] [PATH]
    crawrad note inline [--check] [PATH]
    crawrad note extract [--dry-run] [--all] [PATH]

COMMANDS:
    manifest    Writes PATH/manifest.json from the Rust source of the component package
                at PATH (default: the current directory). Nothing is compiled.

    note        Scans PATH/.crawrad/notes/*.md and the .rs files under PATH/src, tests,
                examples and benches (default: .),
                reporting anchors, orphan notes, dangling anchors, kind mismatches,
                and stale signatures without compiling anything.

    note inline Inlines doc notes back into the Rust source under PATH (default: .)
                as `///` doc comments and removes their `#[note]` anchors.

    note extract Turns /// doc comments under PATH/src (default: .) into doc notes
                 under .crawrad/notes/ and leaves #[note] anchors in the source.
                 Private items need a one-sentence summary; public items also the
                 rustdoc sections. Run `cargo fmt` afterwards: the added
                 `use crawrad_note::note;` follows the file's last `use`.

OPTIONS:
    --all       Extract docs even when they fail rustdoc style conventions.
    --check     Do not write or modify files. Exit 1 on discrepancy or error.
                With --links, broken links in notes fail; links in /// do not (not a note problem).
    --dry-run   Do not write or modify files; report what would be done.
    --links     Also run cargo doc and report broken intra-doc links, mapped to their notes.
    --missing   List public items without documentation under PATH/src.
";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let refs: Vec<&str> = args.iter().map(String::as_str).collect();

    match refs.as_slice() {
        [] | ["-h"] | ["--help"] | ["help"] => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        ["manifest", rest @ ..] => manifest(rest),
        ["note", rest @ ..] => note(rest),
        [other, ..] => {
            eprintln!("crawrad: `{other}` is not a command\n\n{USAGE}");
            ExitCode::FAILURE
        }
    }
}

fn manifest(args: &[&str]) -> ExitCode {
    let mut check = false;
    let mut path: Option<&str> = None;
    for a in args {
        match *a {
            "--check" => check = true,
            other if other.starts_with('-') => {
                eprintln!("crawrad manifest: unknown option `{other}`\n\n{USAGE}");
                return ExitCode::FAILURE;
            }
            other => path = Some(other),
        }
    }
    let root = PathBuf::from(path.unwrap_or("."));

    let written = match assemble(&root) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("crawrad manifest: {e}");
            return ExitCode::FAILURE;
        }
    };

    let target = root.join(MANIFEST_FILE);
    if !check {
        return match std::fs::write(&target, &written) {
            Ok(()) => {
                println!("crawrad: wrote {}", target.display());
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("crawrad manifest: cannot write {}: {e}", target.display());
                ExitCode::FAILURE
            }
        };
    }

    match std::fs::read_to_string(&target) {
        Err(e) => {
            eprintln!("crawrad manifest: cannot read {}: {e}", target.display());
            ExitCode::FAILURE
        }
        Ok(found) if same_file(&found, &written) => {
            println!("crawrad: {} is up to date", target.display());
            ExitCode::SUCCESS
        }
        Ok(found) => {
            eprintln!(
                "crawrad manifest: {} does not match the Rust source.",
                target.display()
            );
            if let Some(line) = first_difference(&found, &written) {
                eprintln!("{line}");
            }
            eprintln!("Run `crawrad manifest` to rewrite it.");
            ExitCode::FAILURE
        }
    }
}

fn note(args: &[&str]) -> ExitCode {
    if let Some(&"inline") = args.first() {
        return note_inline(&args[1..]);
    }
    if let Some(&"extract") = args.first() {
        return note_extract(&args[1..]);
    }

    let mut check = false;
    let mut missing = false;
    let mut links = false;
    let mut path: Option<&str> = None;
    for a in args {
        match *a {
            "--check" => check = true,
            "--missing" => missing = true,
            "--links" => links = true,
            other if other.starts_with('-') => {
                eprintln!("crawrad note: unknown option `{other}`\n\n{USAGE}");
                return ExitCode::FAILURE;
            }
            other => path = Some(other),
        }
    }

    if missing && links {
        eprintln!("crawrad note: --links cannot be used with --missing\n\n{USAGE}");
        return ExitCode::FAILURE;
    }

    let root = PathBuf::from(path.unwrap_or("."));

    if missing {
        let items = match missing_docs(&root) {
            Ok(items) => items,
            Err(e) => {
                eprintln!("crawrad note: {e}");
                return ExitCode::FAILURE;
            }
        };

        for item in &items {
            let rel = item.file.to_string_lossy().replace('\\', "/");
            println!(
                "{rel}:{}: missing docs for {} {}",
                item.line, item.kind, item.name
            );
        }

        if items.is_empty() {
            println!("All public items are documented.");
        } else {
            println!("{} public items without documentation", items.len());
        }

        if check && !items.is_empty() {
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        }
    } else {
        let report = match scan_notes(&root) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("crawrad note: {e}");
                return ExitCode::FAILURE;
            }
        };

        print!("{report}");

        let note_links_failed = if links {
            let broken = match broken_links(&root) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("crawrad note: {e}");
                    return ExitCode::FAILURE;
                }
            };

            if !broken.is_empty() {
                println!("\nbroken intra-doc links:");
                for b in &broken {
                    if let Some(id) = &b.note {
                        println!("  {}:{}: {} (note {id})", b.file, b.line, b.message);
                    } else {
                        println!("  {}:{}: {}", b.file, b.line, b.message);
                    }
                }
            }

            broken.iter().any(|b| b.note.is_some())
        } else {
            false
        };

        if check && (report.has_problems() || note_links_failed) {
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        }
    }
}

fn note_inline(args: &[&str]) -> ExitCode {
    let mut check = false;
    let mut path: Option<&str> = None;
    for a in args {
        match *a {
            "--check" => check = true,
            other if other.starts_with('-') => {
                eprintln!("crawrad note inline: unknown option `{other}`\n\n{USAGE}");
                return ExitCode::FAILURE;
            }
            other => path = Some(other),
        }
    }
    let root = PathBuf::from(path.unwrap_or("."));

    let report = match inline_notes(&root, !check) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("crawrad note inline: {e}");
            return ExitCode::FAILURE;
        }
    };

    for i in &report.inlined {
        println!("{}:{}: inlined {}", i.file, i.line, i.id);
    }
    for s in &report.skipped {
        println!("{}:{}: skipped {}: {}", s.file, s.line, s.id, s.reason);
    }

    let n = report.inlined.len();
    let m = report.skipped.len();
    if check {
        println!("{n} notes would be inlined, {m} skipped");
    } else {
        println!("{n} notes inlined, {m} skipped");
        if n > 0 {
            println!("The crate may now drop its crawrad-note dependency if no #[note] remains.");
        }
    }

    ExitCode::SUCCESS
}

fn note_extract(args: &[&str]) -> ExitCode {
    let mut dry_run = false;
    let mut all = false;
    let mut path: Option<&str> = None;
    for a in args {
        match *a {
            "--dry-run" => dry_run = true,
            "--all" => all = true,
            other if other.starts_with('-') => {
                eprintln!("crawrad note extract: unknown option `{other}`\n\n{USAGE}");
                return ExitCode::FAILURE;
            }
            other => path = Some(other),
        }
    }
    let root = PathBuf::from(path.unwrap_or("."));

    let report = match extract_notes(&root, !dry_run, all) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("crawrad note extract: {e}");
            return ExitCode::FAILURE;
        }
    };

    for e in &report.extracted {
        println!(
            "{}:{}: extracted {} -> .crawrad/notes/{}.md",
            e.file, e.line, e.item, e.id
        );
    }
    for s in &report.skipped {
        println!(
            "{}:{}: kept /// on {}: {}",
            s.file, s.line, s.item, s.reason
        );
    }

    let n = report.extracted.len();
    let m = report.skipped.len();
    if dry_run {
        println!("would have {n} extracted, {m} kept");
    } else {
        println!("{n} extracted, {m} kept");
    }
    if report.cargo_toml_changed && !dry_run {
        println!("Cargo.toml: added crawrad-note");
    }

    ExitCode::SUCCESS
}
