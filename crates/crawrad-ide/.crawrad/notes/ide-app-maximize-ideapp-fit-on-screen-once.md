---
id: ide-app-maximize-ideapp-fit-on-screen-once
kind: doc
target_file: src/ide_app/maximize.rs
line_hint: 112
author: 
created: 
title: "IdeApp::fit_on_screen_once"
tags: []
signature: "pub (super) fn fit_on_screen_once (& mut self , cx : & mut Cx)"
---

Constrains the initial window size to the usable monitor work area.

**The window never opens bigger than the screen it opens on.**

# What the owner saw

> *"maximized screen bigger than the monitor, ate the entire footer"*

Measured on his machine at the time: a 1024x768 screen, a 1024x720 work
area, and the IDE opening at 1044x788 from the origin — 68 pixels below
the usable bottom, with the status bar under the taskbar.

The cause is in the DSL, and it is not a mistake there: the window asks
for `inner_size: 1440x900`, which is the right size to ask for and the
wrong size to insist on. Neither Makepad nor this shell can know the
monitor is smaller — `WindowGeom` carries no screen size
(`makepad-platform-1.0.0/src/event/window.rs:11-20`). The system knows,
and [`crawrad_os::monitor_at`] is how it is asked.

# Why it runs on an event and not at startup

At `Event::Startup` the window may not have been described yet, and an
undescribed `WindowGeom` is `Default` — a zero DPI and a zero size. Read
then, it would report a window that fits nothing and shrink a perfectly
good window on every machine. So this waits for a geometry that says
something, and then runs once.

# It opens MAXIMIZED, through the title-bar button's own door

The owner asked for it on 30/09/2026. It goes through
[`IdeApp::toggle_maximized`] and not a platform op, so it stops at the
taskbar like the button does, sets [`IdeApp::maximizada`], and keeps the
size the window opened at as the one `Restore` goes back to.

#### At `if !crawrad_os::place_main_window(area.work) {`

NOT `toggle_maximized` here, and it was measured: on the first
described event the window is not VISIBLE yet, `place_main_window`
cannot find it (`crawrad-os` `main_window` filters on
`IsWindowVisible`), and the toggle's fallback — the framework's
maximize — covered the taskbar: `(-7,-7)-(1927,1087)` on a 1920x1032
work area. So a miss waits for the next event instead.

## Inside `fit_on_screen_once`: `self.fit_on_screen_pending = false;`

No work area to ask for (not Windows): the framework's maximize
already stops at the usable area there.
