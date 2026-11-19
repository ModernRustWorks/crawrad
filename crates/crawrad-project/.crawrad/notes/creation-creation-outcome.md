---
id: creation-creation-outcome
kind: doc
target_file: src/creation.rs
line_hint: 208
author: 
created: 
title: "creation_outcome"
tags: []
signature: "pub fn creation_outcome (project_root : & Path , done : Result < CreatedProject , workspace :: Refusal > , permission : Result < () , String > ,) -> Outcome"
---

Maps creation outcome to open status or explains on screen why not.

**A created project becomes an opened project** — or else the screen explains why not.

Pure, and it is the gateway for the defect that the owner measured on 08/09/2026.
Before this, `harvest_creation` did this:

```text
Ok(done) => { self.open_folder(&done.root); self.update_title(ctx); }
```

and `open_folder` **refuses on the first line** when there is unsaved work,
returning the phrase only via the status bar. With the form dirty, the project
was born in full on disk and the IDE did not switch: no new title, no entry in
`recent-folders.txt`, with the old project's Inspector on screen. The gesture had
no visible result, and the user repeated it.

The four outcomes, and none of them is silence:

| creation | open permission | outcome |
|---|---|---|
| `Ok`, without warning | `Ok` | opens, and the dialog disappears |
| `Ok`, with warning | `Ok` | opens, and the dialog STAYS with the warning |
| `Ok` | `Err` | **does not open**, and the dialog stays with the folder and the `Open` button |
| `Err` | — | did not create; stays, and offers to open if a project already existed there |

**The second line cost a revision.** It always said this, and the code was
sending the warning to `Outcome::Stay` — which means *did not open*. The
project was born in full and the IDE stayed silent on top of the old project, which is
word for word the bug this function exists to fix. Today the arm is
[`Outcome::OpenedWithWarning`], and both halves of the sentence are true at the same time.

**No warning prevents opening**, and the reason is that opening a folder is a read:
in the three warnings that exist (adoption by the workspace, `cargo add` without network,
and failed generation) the project is complete on disk, and the gesture was *create
and take me there*. What prevents opening is `permission`, and it has its own row.

The third line is the residue: `create_permission` already refused the gesture
*before* writing anything, so arriving here dirty requires dirtying the document
**during** the seconds of creation. It is rare, and not impossible — the dialog
does not block the main window —, and the way out cannot be discarding the work
nor hiding the folder that was just born.

`project_root` is `<folder chosen in picker>/<typed name>` — **the PROJECT folder,
and not the one the user pointed to** (ADR 0017, Amendment 3). It enters via parameter
because the fourth row of the table needs it and `Refusal` does not carry it: failed
creation has no `CreatedProject` from which to extract a path.

The parameter was called `escolhida`, and this doc claimed it was the picker's folder.
It was false, and revision measured how much it cost: implementing what the doc said
left the 312 tests green, and the `Open` of the fourth line started offering the PARENT
folder — the folder that CONTAINS someone else's project, instead of the project the
person wanted to open. Name and doc now state what arrives.

# Examples

```rust
use std::path::Path;
use crawrad_project::creation::{creation_outcome, CreatedProject};

let created = CreatedProject {
    root: Path::new(".").to_path_buf(),
    warning: None,
};
let outcome = creation_outcome(Path::new("."), Ok(created), Ok(()));
assert!(outcome.closes_automatically());
```
