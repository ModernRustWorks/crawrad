# crawrad-note

Turns external Markdown notes into rustdoc documentation at compile time.
Annotating an item with `#[crawrad_note::note("id")]` reads `.crawrad/notes/<id>.md`, injects its body as doc comments, and tracks rebuilds.

## Minimal Example

Create `.crawrad/notes/sum.md`:

~~~markdown
---
id: sum
kind: doc
title: "Sum"
---

Computes the arithmetic sum of two 32-bit signed integers.

# Examples

```rust
use my_crate::sum;

assert_eq!(sum(2, 3), 5);
```
~~~

Attach the note to your function:

```rust
#[crawrad_note::note("sum")]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}
```

Run rustdoc:

```text
cargo doc --open
```

## Details

- **Doc vs Internal notes**: Only notes with `kind: doc` become documentation. Notes with `kind: internal` are rejected at compile time when targeted by an anchor.
- **Rebuild tracking**: Editing a note file rebuilds the crate automatically because the macro embeds tracking via `include_str!`.
  `cargo doc` is different: rustdoc writes no dependency info, so cargo refreshes the docs only when the note is in the
  package's file list. A note committed to git is; a hidden `.crawrad/` outside git is not — then list it:
  `include = ["src/**", ".crawrad/notes/*.md", "Cargo.toml"]`.
- **Packaging and docs.rs**: `cargo package` ships `.crawrad/notes/*.md` under the same rule (in git, or in `include`), so docs.rs renders the notes too. Check with `cargo package --list`.
- **Signature staleness**: When a note's front matter records a `signature`, the build warns if the item's signature in source code changes without updating the note.
- **CI validation**: Run `crawrad note --check` (the `crawrad` binary of the `crawrad-cli` crate) in CI to detect orphan notes, dangling anchors, kind mismatches, or stale signatures without compiling anything.
