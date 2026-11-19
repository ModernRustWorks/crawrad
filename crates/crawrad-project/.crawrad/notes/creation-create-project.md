---
id: creation-create-project
kind: doc
target_file: src/creation.rs
line_hint: 278
author: 
created: 
title: "create_project"
tags: []
signature: "pub fn create_project (parent : & Path , name : & str , project_type : workspace :: ProjectType , form : Option < Document > , report : & dyn Fn (CreationStep) ,) -> Result < CreatedProject , workspace :: Refusal >"
---

Creates a project in a folder with the typed name.

Creates the project: **one level, the folder with the typed name, like `cargo new`**.

Runs in a thread, hence a standalone function: it does not touch anything in `Ide`.

```text
Downloads/            <- `parent`, the folder the user chose
  test/               <- `name`, what they typed. This is what opens.
    Cargo.toml          name = "test"
    .gitignore          from `cargo new`
    src/
      main.rs
      form_main.rs            YOURS
      form_main.designer.rs   ours
      form_main.crawform      the design
```

**It used to create group+member, and stopped doing so** ([ADR 0017], Amendment 3). The
decision belongs to the owner — *"in Rust, on create, you name the project, and then it
creates a folder with the name. In the IDE it asks for folder. We'll need to review this"* — and
it explains the `test\test` from the report of 08/09/2026: the name came from the chosen
folder, and creation assembled two levels over it.

What left with the group, and each for its reason:

- **the virtual `Cargo.toml` and `.gitignore` at root**: there is no longer our root to write;
- **`git init`**: was *"the repository belongs to the group"*. Without a group, `cargo new`
  decides, and measured on 08/09/2026 it writes `.git` **and** a `.gitignore` with `/target` —
  exactly what was handwritten;
- **the order "root before `cargo new`"** (ADR 0018, Amendment 2): existed because `cargo new`
  searches for workspaces walking up the tree and we needed it to find OUR root. Without our
  root there is no workspace conflict to avoid — whatever it finds is the real workspace
  of that location, the same the user would get via terminal;
- **`undo`**: creation no longer writes anything before `cargo`, so there is no trace of ours
  to remove. `Nothing was written` changed from a promise backed by `remove_file` to a
  structural fact.

**What did NOT change is opening.** Every project created by the IDE to date is
group+member, and `members`, `documents_in`, and `document_member` continue honoring
both forms — a loose package is its own single member (ADR 0017, Amendment 1), which
is exactly the form born here now.

**Name collisions are refused BEFORE cargo** ([`workspace::refusal_occupied_folder`]):
with the typed name, *a folder called `test` already exists here* is a self-explanatory
sentence, and arrives without writing anything.

**The end is the first `Generate`**, without the button: whatever is born with a form
is born with its code in `src/` ([`generator::generate_for_new_project`]). Projects without
a form — `Console` and `Library` — have nothing to generate, and the step simply does not
happen: `form` is the authority.

**And it TELLS what it is doing**, via `report`. The owner asked for the progress dialog
in these words — *"Rust takes time to create the structure, so there must be a dlg with the
process on screen to announce that it is creating, that is valuable"* —, and real progress
only exists if the worker talks while working. The channel previously had only one value:
the verdict at the end.

`report` announces the **entire plan first** and then each step that starts. The plan
comes from [`workspace::steps_for`] from inside here, where `form` is already a fact —
calculating it in the shell, before the thread, would be the same question in two places.


# Examples

```rust,no_run
// Does not create files on disk during doctests.
use std::path::Path;
use crawrad_project::creation::create_project;
use crawrad_project::workspace::ProjectType;

let result = create_project(Path::new("."), "demo", ProjectType::Gui, None, &|_| {});
assert!(result.is_ok());
```

# Errors

Returns `Err` with a [`workspace::Refusal`] if toolchains are outdated, files already exist, or creation commands fail.
