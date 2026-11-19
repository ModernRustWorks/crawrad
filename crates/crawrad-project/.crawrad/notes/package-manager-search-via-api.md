---
id: package-manager-search-via-api
kind: doc
target_file: src/package_manager.rs
line_hint: 128
author: 
created: 
title: "search_via_api"
tags: []
signature: "pub fn search_via_api (query : & str , limite : u32) -> Result < Vec < PackageItem > , String >"
---

Searches packages in the public crates.io REST API.

Returns up to `limite` results with all metadata (downloads, links,
update date). Runs synchronously — should be called from a
separate thread to avoid blocking the UI.

# Errors

Returns an error message if the network request or response parsing fails.

# Examples

```rust,no_run
use crawrad_project::package_manager::search_via_api;

// Network request is disabled in doctests.
let res = search_via_api("serde", 1).unwrap();
assert_eq!(res.len(), 1);
```
