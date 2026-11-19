---
id: file-dialog-pick-open
kind: doc
target_file: src/file_dialog.rs
line_hint: 125
author: 
created: 
title: "pick_open"
tags: []
signature: "pub fn pick_open (req : & FileDialogRequest) -> Option < Vec < PathBuf > >"
---

Opens a native file selection dialog to choose one or more files to open.

If `CRAWRAD_FILE_DIALOG_ANSWER` is present in the environment:
- `""` simulates cancelling the dialog (`None`).
- Pipe-separated paths (e.g. `"file1.txt|file2.txt"`) are returned as `PathBuf` entries.
- If `req.multi` is `false`, only the first path is returned.

# Examples

```no_run
// Opens an interactive native file dialog, which cannot run in automated tests.
use crawrad_runtime::file_dialog::{pick_open, FileDialogRequest};

let req = FileDialogRequest::default();
let _ = pick_open(&req);
```
