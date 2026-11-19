---
id: ide-app-constants-align-zones
kind: doc
target_file: src/ide_app/constants.rs
line_hint: 32
author: 
created: 
title: "ALIGN_ZONES"
tags: []
signature: "pub (super) const ALIGN_ZONES : [(ZonePath , & str) ; 5] = [(| | id ! (align_pad . align_paper . zone_top) , \"Top\") , (| | id ! (align_pad . align_paper . zone_bottom) , \"Bottom\") , (| | id ! (align_pad . align_paper . align_middle . zone_left) , \"Left\") , (| | id ! (align_pad . align_paper . align_middle . zone_right) , \"Right\" ,) , (| | id ! (align_pad . align_paper . align_middle . zone_client) , \"Client\" ,) ,]"
---

The five regions of the `Align` selector and their values.

The five regions of the `Align` selector, and the catalog value each one
picks.

**No alignment name is invented here**: all five are in
`crawrad_registry::ALIGN_OPTIONS`, and the test
`toda_regiao_do_seletor_de_align_e_um_valor_do_catalogo` checks all five
against it — a region naming a value that the catalog does not have
would write an alignment into the document that nothing draws.

The path is a FUNCTION and not a `&[LiveId]` because `id!` is a macro that
produces a `&'static [LiveId]` per invocation, and a `const` of references to
it does not survive `const` promotion in a readable way.
