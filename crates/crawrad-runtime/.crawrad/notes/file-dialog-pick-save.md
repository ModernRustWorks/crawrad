---
id: file-dialog-pick-save
kind: doc
target_file: src/file_dialog.rs
line_hint: 139
author: 
created: 
title: "pick_save"
tags: []
signature: "pub fn pick_save (req : & FileDialogRequest) -> Option < PathBuf >"
---

Opens a native file dialog to choose a file path to save.

If `CRAWRAD_FILE_DIALOG_ANSWER` is present in the environment:
- `""` simulates cancelling the dialog (`None`).
- Non-empty string is treated as the chosen path, appending `req.default_ext`
  via [`with_default_ext`] if no extension was provided.

# Examples

```no_run
// Opens an interactive native file dialog, which cannot run in automated tests.
use crawrad_runtime::file_dialog::{pick_save, FileDialogRequest};

let req = FileDialogRequest::default();
let _ = pick_save(&req);
```
