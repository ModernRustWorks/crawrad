---
id: types-c-primitives
kind: doc
target_file: src/types.rs
line_hint: 204
author: 
created: 
title: "C_PRIMITIVES"
tags: []
signature: "const C_PRIMITIVES : & [(& str , & str)] = & [(\"unsigned long long\" , \"u64\") , (\"unsigned __int128\" , \"u128\") , (\"unsigned short\" , \"u16\") , (\"unsigned char\" , \"u8\") , (\"unsigned int\" , \"u32\") , (\"signed char\" , \"i8\") , (\"long long\" , \"i64\") , (\"__int128\" , \"i128\") , (\"char32_t\" , \"char\") , (\"double\" , \"f64\") , (\"float\" , \"f32\") , (\"short\" , \"i16\") , (\"char\" , \"i8\") , (\"int\" , \"i32\") ,]"
---

LLDB C spellings of primitive types mapped to Rust names.

LLDB's C spelling of the primitives, longest first, with the Rust name
DWARF keeps (measured on 30/09/2026, -gnu). Only the top-level generic
arguments come out in C: inside a tuple LLDB keeps `(u32, i32)`.
