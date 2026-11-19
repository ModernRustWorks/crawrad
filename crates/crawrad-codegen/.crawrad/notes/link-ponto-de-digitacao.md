---
id: link-ponto-de-digitacao
kind: doc
target_file: src/link.rs
line_hint: 139
author: 
created: 
title: "ponto_de_digitacao"
tags: []
signature: "pub fn ponto_de_digitacao (fonte : & str , nome : & str) -> Option < Local >"
---

Point where the cursor should be positioned after double-clicking the component.

Where the cursor lands after the double click: **inside** the body, on the column
where the user will type.

# Examples

```rust
use crawrad_codegen::link::ponto_de_digitacao;

let src = "fn test() {\n    \n}";
let pt = ponto_de_digitacao(src, "test");
assert!(pt.is_some());
```

# Why it is not [`localizar_handler`] under another name

The two questions are different, and each has its own client:

- *"does this handler exist, and where is its definition?"* — this is
  [`localizar_handler`], and the generator's `crescido` is what asks it, before
  deciding whether to add the stub. Changing the meaning of what it receives
  would make "does it already exist?" depend on where the body begins;
- *"where will the first keystroke type?"* — this is it, and the shell
  asks it on double click.

The second was born on 08/09/2026, from the owner's classic RAD IDE:

```text
procedure TForm5.Button1Click(Sender: TObject);
begin
                { <- o cursor pousa aqui: dentro, e já indentado }
end;
```

# The rule, and why it READS the file instead of recalculating the stub

The landing point is the first line **after** the `{`, right after its indentation.
The stub is born with this line ready, but the file belongs to the user and three
things undo it — all three measured on 08/09/2026, and all three come for
free because the calculation is derived from the text that is there:

| what happened to the body | where the cursor lands |
|---|---|
| untouched, `{\n        \n    }` | at column 9, which is what is wanted |
| `cargo fmt` collapsed it to `{}` | between the braces |
| an editor trimmed the end of the line | at column 1, inside the body |
| the user typed | at the beginning of their code, which is where one wants to return |

A landing point calculated from the stub's **format** would point to the middle of
nowhere on the first `cargo fmt`.

`None` for the same reasons as the two it depends on: the `fn` is not
here, or its body does not close.
