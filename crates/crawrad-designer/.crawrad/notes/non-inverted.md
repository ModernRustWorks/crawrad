---
id: non-inverted
kind: doc
target_file: src/lib.rs
line_hint: 1364
author: 
created: 
title: "non_inverted"
tags: []
signature: "fn non_inverted (r : Rect) -> Rect"
---

The rectangle without negative sides, for marquee selection.

Since 2026-09-09 anchors **may** produce negative width on purpose:
it stopped clamping at the floor so that shrinking the form past zero and
growing back returns the exact component, which is what the owner measured in
the classic RAD IDE. This places an inverted rectangle in the document, and
`Rect::intersects` reads inverted literally.

## The error is LACK, not excess — it was measured, contrary to what
it seemed

Intuition suggests an inverted rectangle "turns into a strip where it is
not" and starts being picked up by marquee from afar. **That is not what happens.** A
component at `x = 300` with `w = -120` only satisfies the predicate when the
marquee covers the ENTIRE `[180, 300]` strip — and every marquee covering that strip
also covers point `300`, where the component actually is. The inverted
condition is **more restrictive**, never looser.

Scanned on 2026-09-09 over 6912 marquees against `(300, 250, -120 × -90)`:

| marquees that hit | with negative `w` | with zeroed `w` |
|---|---|---|
| in both | 1520 | 1520 |
| **only** in this | **0** | **5472** |

Zero false positives, 5472 false **negatives**. The defect is the marquee
`(0, 161, 301 × 116)` — passing right over `(300, 250)` — failing to
hit the component. Without this, a collapsed component would only return via
Structure, and the owner resizes the form on the canvas, not in the tree.

The cutoff is at **zero** because zero is the honest area of what collapsed, and is the
same number that `apply` in `layout.rs` uses for margins — a single
floor, not two. A zero-area rectangle at `(300, 250)` still satisfies the
predicate for the marquee passing over it, which keeps the
component reachable.

**The test limits the floor from above, not exactly at zero**:
`marquee_beside_collapse_point_hits_nothing` fails any floor from
~11 pt upward, and a `max(1.0)` here would pass it. It is measured and stated
deliberately, so nobody reads the test as proving more than it
proves — between 0 and 1 there is no observable difference in marquee.

The durable fix is for `crawrad-ir`'s `Rect::intersects` to normalize on its own —
two workarounds for the same predicate (here and the margins `max(0.0)` in
`layout.rs`) show the predicate itself is what is wrong. Noted for the record;
`crawrad-ir` is not touched without notice.
