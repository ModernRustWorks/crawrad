# CrawRAD — Additional Permissions

**Version 1.0**

CrawRAD is free software: you can redistribute it and/or modify it under the terms
of the **GNU General Public License, version 3**, as published by the Free Software
Foundation. The full text is in [`LICENSE`](LICENSE).

This file grants an **additional permission** under **section 7 of the GNU General
Public License version 3**. It *widens* what you may do; it never narrows it. If
anything here were ever read as restricting a right the GPL gives you, the GPL wins
and that reading is void.

## 1. Output Exception — what CrawRAD writes for you is yours

**Grant.** Code, forms, project files, configuration and other material that CrawRAD
generates, scaffolds, completes, refactors or inserts into your own project (the
**"Output"**) is **not** covered by the GNU General Public License and is **not** a
derivative work of CrawRAD.

To the extent that any part of CrawRAD's own source — templates, snippets,
scaffolding, generated code — is reproduced in the Output, the copyright holder
grants you an unlimited, irrevocable, worldwide, royalty-free permission to use,
copy, modify and distribute that part **under any licence you choose**, including a
proprietary one.

**In practice.** Building software with CrawRAD places **no licensing obligation on
that software**. You may build closed-source, commercial products and distribute them
under whatever terms you like.

## 2. The runtime your app links is not GPL

The crates that end up **inside your application** are licensed separately under
**MIT OR Apache-2.0** ([`LICENSE-MIT`](LICENSE-MIT), [`LICENSE-APACHE`](LICENSE-APACHE)),
as stated in each crate's `Cargo.toml`: the runtime (`crawrad-runtime`,
`crawrad-runtime-makepad`, `crawrad-runtime-egui`), the binding and component SDK, and the
component suite in `components/` (PortaDB, the data grid, ...). Linking them into your
app never brings the GPL with it.

The GPL applies to **CrawRAD itself** — the IDE. If you modify the IDE and
distribute that modified IDE, you distribute it under the GPL v3, with its source.

*(The same spirit as the GCC Runtime Library Exception: a free tool must never make
the programs built with it unfree.)*
