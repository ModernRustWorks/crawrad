# `source/` — Reference Source Code for CrawRAD Components

This directory contains the canonical reference source implementations for components in the CrawRAD ecosystem, written in **pure, idiomatic Rust** using the platform's **2D `Canvas`** drawing API.

It is also an autonomous, testable crate: **`crawrad-components-source`**.

---

## 1. Directory Structure and Palette Organization

Subdirectories under `source/` correspond 1:1 to the **Tool Palette categories** defined on components via `#[crawrad(palette = "...")]`:

```text
source/
│
├── Cargo.toml                 # Autonomous crate: crawrad-components-source
├── README.md                  # This documentation
├── src/
│   └── lib.rs                 # Crate root exporting all category modules
├── tests/
│   └── render_tests.rs        # Automated test exercising Paint on all 44 components
│
├── standard/                  # Palette category "Standard" (16 components)
│   ├── mod.rs
│   ├── action_list.rs         # ActionList and Action command manager
│   ├── button.rs              # Push button with theme roles and events
│   ├── check_box.rs           # Two-state toggle box
│   ├── combo_box.rs           # Dropdown selection box
│   ├── edit.rs                # Single-line text input
│   ├── group_box.rs           # Framed container with caption notch
│   ├── label.rs               # Text label with font and alignment
│   ├── list_box.rs            # Vertical item selection list
│   ├── main_menu.rs           # Window main menu bar
│   ├── memo.rs                # Multi-line text editor
│   ├── page_control.rs        # Multi-page tabbed container
│   ├── panel.rs               # Beveled rectangular container
│   ├── popup_menu.rs          # Contextual right-click popup menu
│   ├── radio_button.rs        # Mutually exclusive radio selector
│   ├── radio_group.rs         # Framed group of exclusive options (#500)
│   └── scroll_bar.rs          # Standalone scroll bar
│
├── additional/                # Palette category "Additional" (19 components)
│   ├── mod.rs
│   ├── badge.rs               # Short label in a pill (#500)
│   ├── bevel.rs               # Sunk or raised box, frame or line (#500)
│   ├── bit_btn.rs             # Bitmap glyph button with modal results
│   ├── breadcrumb.rs          # Navigation trail on one line (#500)
│   ├── card.rs                # Modern rounded container with header (#500)
│   ├── category_panel.rs      # Foldable container under a header (#500)
│   ├── chips.rs               # Group of rounded chips / tag pickers (#500)
│   ├── color_box.rs           # Drop-down of the standard colours (#500)
│   ├── drawer.rs              # Side panel overlay container with scrim (#500)
│   ├── image.rs               # Picture and graphic display
│   ├── link_label.rs          # Caption drawn as a link (#500)
│   ├── mask_edit.rs           # Edit shaped by an EditMask (#71)
│   ├── pagination.rs          # Row of page buttons (‹ 1 … 4 [5] 6 … 20 ›, #500)
│   ├── paint_box.rs           # Freehand canvas surface via OnPaint
│   ├── scroll_box.rs          # Scrollable viewport container
│   ├── shape.rs               # Vector geometric shapes
│   ├── speed_button.rs        # Flat toggle toolbar button
│   ├── spin_edit.rs           # Number field with step buttons (#500)
│   └── toggle.rs              # On/off switch (#500)
│
├── win32/                     # Palette category "Win32" (6 components)
│   ├── mod.rs
│   ├── image_list.rs          # Centralized indexed icon repository
│   ├── progress_bar.rs        # Continuous progress indicator
│   ├── status_bar.rs          # Multi-panel window status bar
│   ├── tool_bar.rs            # Command toolbar container
│   ├── track_bar.rs           # Slider track with tick marks
│   └── tree_view.rs           # Hierarchical expandable node tree
│
├── system/                    # Palette category "System" (1 component)
│   ├── mod.rs
│   └── timer.rs               # Periodic interval timer
│
└── dialogs/                   # Palette category "Dialogs" (2 components)
    ├── mod.rs
    ├── open_dialog.rs         # System file open dialog
    └── save_dialog.rs         # System file save dialog
```

---

## 2. Complete Reference Components Catalog (30 Components)

### Standard Palette (`source/standard/`) — 15 Components

| File | Component | Kind | Key Features & Canvas Rendering |
|---|---|---|---|
| [`button.rs`](standard/button.rs) | `Button` | Visual | Rounded border, 12 events, semantic `ThemeRole` colors, centered text |
| [`label.rs`](standard/label.rs) | `Label` | Visual | Graphic text, `fill_text_prop`, alignment, transparent background |
| [`panel.rs`](standard/panel.rs) | `Panel` | Container | `children = "area"`, configurable `border_width`, beveled surface |
| [`edit.rs`](standard/edit.rs) | `Edit` | Visual | Single-line text input, `password_char`, `readonly`, `text`, `on_change` |
| [`check_box.rs`](standard/check_box.rs) | `CheckBox` | Visual | Two-state toggle box, vector checkmark, caption text, `on_click` |
| [`radio_button.rs`](standard/radio_button.rs) | `RadioButton` | Visual | Mutually exclusive circular selector, inner dot indicator, `on_click` |
| [`radio_group.rs`](standard/radio_group.rs) | `RadioGroup` | Visual | Framed group of exclusive options: `Items`, `ItemIndex`, `Columns` |
| [`group_box.rs`](standard/group_box.rs) | `GroupBox` | Container | `children = "area"`, framed border with clipped top caption notch |
| [`memo.rs`](standard/memo.rs) | `Memo` | Visual | Multiline text area, simulated text lines, `readonly`, `on_change` |
| [`combo_box.rs`](standard/combo_box.rs) | `ComboBox` | Visual | Dropdown list, `Strings` items, `item_index`, chevron arrow icon |
| [`list_box.rs`](standard/list_box.rs) | `ListBox` | Visual | Vertical list, `Strings` items, highlighted active selection row |
| [`page_control.rs`](standard/page_control.rs) | `PageControl` | Container | Tabbed multi-page container, `active_page`, `children = "area"` |
| [`scroll_bar.rs`](standard/scroll_bar.rs) | `ScrollBar` | Visual | Horizontal/vertical scroll track, min/max range, step arrows, thumb |
| [`action_list.rs`](standard/action_list.rs) | `ActionList` / `Action` | Non-visual | Centralized action management, `ShortCut`, vector lightning icon |
| [`main_menu.rs`](standard/main_menu.rs) | `MainMenu` | Non-visual | Primary horizontal window menu bar and drop-downs, 28x28 menu icon |
| [`popup_menu.rs`](standard/popup_menu.rs) | `PopupMenu` | Non-visual | Floating contextual right-click menu, 28x28 popup icon |

### Additional Palette (`source/additional/`) — 6 Components

| File | Component | Kind | Key Features & Canvas Rendering |
|---|---|---|---|
| [`bit_btn.rs`](additional/bit_btn.rs) | `BitBtn` | Visual | Push button with bitmap glyph presets (`Kind`), `ModalResult`, action cues |
| [`speed_button.rs`](additional/speed_button.rs) | `SpeedButton` | Visual | Flat tool button, toggle states (`Down`), `GroupIndex`, `AllowAllUp` |
| [`toggle.rs`](additional/toggle.rs) | `Toggle` | Visual | On/off switch with caption: `Checked` |
| [`color_box.rs`](additional/color_box.rs) | `ColorBox` | Visual | Drop-down of the standard colours with swatches: `Selected` |
| [`bevel.rs`](additional/bevel.rs) | `Bevel` | Visual | Sunk or raised `Box`, `Frame` or edge line: `Shape`, `Style` |
| [`badge.rs`](additional/badge.rs) | `Badge` | Visual | Short label in a pill: `Kind`, `Outline`, `Caption`, `OnClick` |
| [`card.rs`](additional/card.rs) | `Card` | Container | Modern rounded container: `CornerRadius`, `Elevation`, `ShowHeader`, `children = "area"` |
| [`drawer.rs`](additional/drawer.rs) | `Drawer` | Container | Side panel overlay container with scrim: `Side`, `Opened`, `ShowHeader`, `Overlay`, `children = "area"` |
| [`category_panel.rs`](additional/category_panel.rs) | `CategoryPanel` | Container | Header that folds it: `Collapsed`, `OnCollapsed`/`OnExpanded`, `children = "area"` |
| [`chips.rs`](additional/chips.rs) | `Chips` | Visual | Group of rounded chips: `Items`, `ItemIndex`, `Removable`, `OnClick`, `OnChange` |
| [`breadcrumb.rs`](additional/breadcrumb.rs) | `Breadcrumb` | Visual | Navigation trail on one line: `Items`, `Separator`, `ItemIndex`, `OnClick`, `OnChange` |
| [`pagination.rs`](additional/pagination.rs) | `Pagination` | Visual | Row of page buttons: `PageCount`, `PageIndex`, `MaxButtons`, `ShowArrows`, `OnChange` |
| [`link_label.rs`](additional/link_label.rs) | `LinkLabel` | Visual | Caption drawn as a link: `URL` (web or mail), `OnClick` |
| [`mask_edit.rs`](additional/mask_edit.rs) | `MaskEdit` | Visual | Edit shaped by an `EditMask` (`!99/99/0000;1;_`), `OnChange` |
| [`spin_edit.rs`](additional/spin_edit.rs) | `SpinEdit` | Visual | Whole number with step buttons: `Value`, `MinValue`/`MaxValue`, `Increment`, `EditorEnabled` |
| [`image.rs`](additional/image.rs) | `Image` | Visual | Vector & raster display, `Picture`, `stretch`, `proportional`, `center` |
| [`shape.rs`](additional/shape.rs) | `Shape` | Visual | Vector geometric shapes (`Rectangle`, `Square`, `RoundRect`, `Ellipse`, `Circle`) |
| [`paint_box.rs`](additional/paint_box.rs) | `PaintBox` | Visual | Custom procedural drawing canvas surface via `on_paint` event |
| [`scroll_box.rs`](additional/scroll_box.rs) | `ScrollBox` | Container | Scrollable windowed container, `children = "area"`, dual scrollbars |
| [`splitter.rs`](additional/splitter.rs) | `Splitter` | Visual | Sash resize control with centered grip dots for adjacent docked panels |


### Win32 / Common Controls (`source/win32/`) — 6 Components

| File | Component | Kind | Key Features & Canvas Rendering |
|---|---|---|---|
| [`tree_view.rs`](win32/tree_view.rs) | `TreeView` | Visual | Hierarchical tree, expandable `[+]`/`[-]` nodes, selection, connecting lines |
| [`image_list.rs`](win32/image_list.rs) | `ImageList` | Non-visual | Centralized indexed icon repository, `image_width`/`image_height`, 2x2 grid icon |
| [`progress_bar.rs`](win32/progress_bar.rs) | `ProgressBar` | Visual | Operation progress indicator bar, min/max range, continuous track fill |
| [`track_bar.rs`](win32/track_bar.rs) | `TrackBar` | Visual | Slider track control, numeric adjustment thumb, scale tick marks |
| [`status_bar.rs`](win32/status_bar.rs) | `StatusBar` | Visual | Window footer status bar, sunken bevel panels, resize grip |
| [`tool_bar.rs`](win32/tool_bar.rs) | `ToolBar` | Container | Top command toolbar container (`children = "area"`), separators, tool buttons |

### System Palette (`source/system/`) — 1 Component

| File | Component | Kind | Key Features & Canvas Rendering |
|---|---|---|---|
| [`timer.rs`](system/timer.rs) | `Timer` | Non-visual | Interval timer, `interval: i64`, `enabled`, `on_timer`, 28x28 clock icon |

### Dialogs Palette (`source/dialogs/`) — 2 Components

| File | Component | Kind | Key Features & Canvas Rendering |
|---|---|---|---|
| [`open_dialog.rs`](dialogs/open_dialog.rs) | `OpenDialog` | Non-visual | Operating system file open dialog, file filters, 28x28 folder icon |
| [`save_dialog.rs`](dialogs/save_dialog.rs) | `SaveDialog` | Non-visual | Operating system file save/export dialog, file filters, 28x28 disk icon |

---

## 3. Autonomous Crate and Testability

`source/` is structured as the `crawrad-components-source` crate. All components can be compiled and validated independently:

```bash
# Run the integration paint test across all 30 components:
cargo test -p crawrad-components-source

# Verify compiler warnings and clippy:
cargo clippy -p crawrad-components-source --all-targets -- -D warnings
```

---

## 4. Why Factory Controls Are Protected in the Engine

The core factory components of the IDE (`Button`, `Label`, `Panel`, `Edit`, `Memo`, etc.) are precompiled and protected directly inside the CrawRAD core engine (`Registry::builtin()`).

This separation serves two essential architectural purposes:
1. **IDE Stability and Reliability**: Keeping factory controls integrated into the core engine prevents accidental edits from corrupting the development environment or breaking form rendering.
2. **Universal Visual Fidelity**: Guarantees that any form designed in CrawRAD renders with identical visual precision and dimensional fidelity across all installations and target operating systems.

---

## 5. Purpose of Reference Source Files

**The files in this directory serve as the official canonical reference for developers**:
- They demonstrate how to declare component structs and attributes (`#[derive(Component)]`, `#[crawrad]`, `#[prop]`, `#[event]`).
- They show how to implement vector rendering using the 2D `Canvas` API (`Paint` trait).
- They provide practical patterns for building third-party component packages under `components/`.

---

## 6. How to Create a New Component

1. Create a package directory under `components/<package-name>-crawrad`.
2. Configure `Cargo.toml` with dependencies on `crawrad-component` and `crawrad-component-derive`.
3. Define the component struct:
   ```rust
   use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
   use crawrad_component_derive::Component;

   #[derive(Component, Debug, Clone, PartialEq)]
   #[crawrad(
       palette = "Standard",
       prefix = "btn",
       size = (96.0, 32.0),
       targets = ["*"]
   )]
   #[event(name = "on_click")]
   pub struct Button {
       #[prop(display = "Caption", category = "appearance", default = "Button")]
       pub text: String,
   }

   impl Paint for Button {
       fn paint(&self, canvas: &mut Canvas) {
           canvas.set_fill_style(ThemeRole::Surface);
           canvas.fill_round_rect(0.0, 0.0, 96.0, 32.0, 4.0);
           canvas.set_fill_style(ThemeRole::Text);
           canvas.fill_text_prop("text", 0.0, 0.0, 96.0, 32.0, TextAlign::Center);
       }
   }
   ```
4. Emit metadata in `src/lib.rs` using `crawrad_component::manifest!(Button);`.
5. Run `CRAWRAD_BLESS=1 cargo test` to generate `manifest.json`.
6. The IDE automatically discovers and installs the package into the Tool Palette without recompiling the IDE binary!
