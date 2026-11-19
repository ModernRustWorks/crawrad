---
id: targets-egui-components-scroll-box-fecha
kind: doc
target_file: src/targets/egui/components/scroll_box.rs
line_hint: 55
author: 
created: 
title: "FECHA"
tags: []
signature: "const FECHA : & str = \"\\\n\\x20                           });\\n\\\n\\x20                   });\\n\\\n\\x20               });\\n\\\n\\x20           });\\n\\\n\\x20       }\\n\""
---

What closes the ScrollBox block after its children.

What closes the `ScrollBox` block after its children: the four closings
that [`emitir`] opened, in reverse order.

It lives next to what opens it, rather than in `form.rs`, because opening and closing are the
same decision: whoever modifies one has both on screen.

**SINGLE brace, not doubled.** [`emitir`] is a `format!` where `}}` is
how one writes a `}`; here it is a raw `&str`, and `}}` would come out literal — which is
what happened on the first attempt, and the generated project did not compile.
