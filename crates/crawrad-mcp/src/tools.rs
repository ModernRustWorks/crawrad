//! Catalogue adapter mapping crawrad-actions to MCP ToolDefinitions.

use super::protocol::ToolDefinition;
use crawrad_actions::ViewAffinity;
use crawrad_actions::capability::{CapabilityRegistry, CapabilitySource};
use crawrad_note::note;
use serde_json::{Value, json};

#[note("tools-build-tools-list")]
pub fn build_tools_list() -> Vec<ToolDefinition> {
    build_tools_list_with_registry(None)
}

#[note("tools-build-tools-list-with-registry")]
pub fn build_tools_list_with_registry(
    registry: Option<&CapabilityRegistry>,
) -> Vec<ToolDefinition> {
    let mut list = Vec::new();

    // 1. Map all actions from the catalogue
    for action in crawrad_actions::catalogue() {
        let mut desc = action.hint.to_string();
        match action.view {
            ViewAffinity::ViewSwitch => {
                desc.push_str(" (View switch tool: exempt from view guards)");
            }
            ViewAffinity::Design => {
                desc.push_str(
                    " (Requires form designer view; call 'view.toggle_code' if in code editor)",
                );
            }
            ViewAffinity::Code => {
                desc.push_str(
                    " (Requires code editor view; call 'view.toggle_code' if in form designer)",
                );
            }
            ViewAffinity::Any => {}
        }

        list.push(ToolDefinition {
            name: action.id.to_string(),
            description: desc,
            input_schema: action.input_schema(),
        });
    }

    // 2. Code reading tool with content_hash and pagination
    list.push(ToolDefinition {
        name: "code.read".to_string(),
        description: "Read file content from live editor buffer or disk with content_hash and pagination (Requires code editor view; call 'view.toggle_code' if in form designer)".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "File path relative to project workspace"
                },
                "offset": {
                    "type": "integer",
                    "description": "Optional start byte offset"
                },
                "limit": {
                    "type": "integer",
                    "description": "Optional maximum bytes to read (budget limit)"
                }
            },
            "required": ["path"]
        }),
    });

    // 3. Code editing tool requiring matching base_hash
    list.push(ToolDefinition {
        name: "code.edit".to_string(),
        description: "Edit file content in live editor buffer requiring matching base_hash from prior code.read (Requires code editor view; call 'view.toggle_code' if in form designer)".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "File path relative to project workspace"
                },
                "base_hash": {
                    "type": "string",
                    "description": "SHA-256 hash returned by prior code.read in this session"
                },
                "content": {
                    "type": "string",
                    "description": "New file content to write into live buffer"
                }
            },
            "required": ["path", "base_hash", "content"]
        }),
    });

    // 4. Form reading tool: inspects the active visual designer form
    list.push(ToolDefinition {
        name: "form.read".to_string(),
        description: "Inspect the active visual form document in designer: returns root, nodes, geometry rects, anchors, properties, selection, revision, and view".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    // 5. Project workspace info tool
    list.push(ToolDefinition {
        name: "project.info".to_string(),
        description: "Get current project workspace root path, active file, open editor buffers, center view, and dirty status".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    // 6. Project creation tool
    list.push(ToolDefinition {
        name: "project.create".to_string(),
        description: "Create a new CrawRAD project from template: gui_app, console_app, library_crate, wasm_app, mobile_app, multiwindow_app, package_plugin, custom_template".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "Project name (e.g. 'my_gui_app')"
                },
                "folder": {
                    "type": "string",
                    "description": "Parent folder where project directory will be created"
                },
                "template": {
                    "type": "string",
                    "description": "Template ID: 'gui_app', 'console_app', 'library_crate', 'wasm_app', 'mobile_app', 'multiwindow_app', 'package_plugin', or 'custom_template'"
                },
                "git_init": {
                    "type": "boolean",
                    "description": "Whether to initialize a git repository"
                },
                "framework": {
                    "type": "string",
                    "description": "UI framework for GUI projects: 'makepad' (default) or 'egui'"
                }
            },
            "required": ["name", "folder"]
        }),
    });

    // 7. Project open tool
    list.push(ToolDefinition {
        name: "project.open".to_string(),
        description: "Open a project directory or file in the IDE".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute or relative path to project directory or form file"
                }
            },
            "required": ["path"]
        }),
    });

    // 8. Project build / check tool
    list.push(ToolDefinition {
        name: "project.build".to_string(),
        description: "[Operation: Execute] Run cargo check, build, or test in the active workspace and return output, errors, and status".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "'check', 'build', or 'test' (default: 'check')"
                },
                "release": {
                    "type": "boolean",
                    "description": "Build with --release flag"
                }
            }
        }),
    });

    // --- 1. Form Designer & Visual Tree (designer.*) ---
    list.push(ToolDefinition {
        name: "designer.get_active_form".to_string(),
        description: "[Operation: Read] Returns the complete structure of the active .crawform (root, properties, rects, selection, revision, hierarchy)".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "designer.list_components".to_string(),
        description: "[Operation: Read] Lists all visual components of the active form with their IDs, names, classes, parent IDs, and layout rects".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "designer.get_selection".to_string(),
        description: "[Operation: Read] Returns the components currently selected in the form designer canvas".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "view.set_toolbox_filter".to_string(),
        description: "[Operation: Write] Types into the Toolbox filter, as the user does: the palette shows only the components that match. An empty text clears it. Answers with the Toolbox as view.get_toolbox reads it".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "text": {
                    "type": "string",
                    "description": "The filter text; empty clears the filter"
                }
            },
            "required": ["text"]
        }),
    });

    list.push(ToolDefinition {
        name: "designer.select".to_string(),
        description: "[Operation: Read] Selects that component in the active form by ID or name, as a click on the canvas does: the tree and the Object Inspector follow. Changes the selection, never the form".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "id": {
                    "type": "string",
                    "description": "Component ID or name to select"
                }
            },
            "required": ["id"]
        }),
    });

    list.push(ToolDefinition {
        name: "designer.get_component_properties".to_string(),
        description: "[Operation: Read] Gets the detailed properties, layout rect, and events of a specific component in the active form".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "id": {
                    "type": "string",
                    "description": "Component ID or name"
                }
            },
            "required": ["id"]
        }),
    });

    list.push(ToolDefinition {
        name: "designer.add_component".to_string(),
        description: "[Operation: Write] Adds a new visual component to the active form at the specified coordinates relative to parent".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "class": {
                    "type": "string",
                    "description": "Component class name (e.g. Button, Edit, Label, Panel, CheckBox, ListBox)"
                },
                "x": {
                    "type": "number",
                    "description": "X coordinate relative to parent (default: 20)"
                },
                "y": {
                    "type": "number",
                    "description": "Y coordinate relative to parent (default: 20)"
                },
                "w": {
                    "type": "number",
                    "description": "Width in pixels (optional, defaults to class standard)"
                },
                "h": {
                    "type": "number",
                    "description": "Height in pixels (optional, defaults to class standard)"
                },
                "parent": {
                    "type": "string",
                    "description": "Parent container component ID (defaults to form root)"
                },
                "name": {
                    "type": "string",
                    "description": "Optional component name"
                },
                "properties": {
                    "type": "object",
                    "description": "Optional map of initial property key/values"
                }
            },
            "required": ["class"]
        }),
    });

    list.push(ToolDefinition {
        name: "designer.set_component_properties".to_string(),
        description: "[Operation: Write] Sets one or more properties on an existing component in the active form in a single transaction".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "id": {
                    "type": "string",
                    "description": "Component ID"
                },
                "properties": {
                    "type": "object",
                    "description": "Key-value map of properties to update"
                }
            },
            "required": ["id", "properties"]
        }),
    });

    list.push(ToolDefinition {
        name: "designer.delete_component".to_string(),
        description: "[Operation: Write] Deletes a component from the active form by ID"
            .to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "id": {
                    "type": "string",
                    "description": "Component ID to delete"
                }
            },
            "required": ["id"]
        }),
    });

    list.push(ToolDefinition {
        name: "designer.wire_event".to_string(),
        description: "[Operation: Write] Binds an event (e.g. on_click, on_change) to an event handler stub in the form and code file".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "id": {
                    "type": "string",
                    "description": "Component ID"
                },
                "event": {
                    "type": "string",
                    "description": "Event name (e.g. on_click, on_change)"
                },
                "handler": {
                    "type": "string",
                    "description": "Optional handler name (defaults to {component_name}_{event})"
                }
            },
            "required": ["id", "event"]
        }),
    });

    // --- 2. Code Editor & Buffer (editor.*) ---
    list.push(ToolDefinition {
        name: "editor.get_active_file".to_string(),
        description: "[Operation: Read] Identifies the active file and tab in the code editor, along with dirty/editable status; `tab_file` is the tab in front, `read_only` whether it takes typing (toolchain and downloaded crates do not), `caret` its line (1-based) and byte, `status` the status bar's sentence (e.g. a refused Format Document)".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "editor.get_buffer_content".to_string(),
        description: "[Operation: Read] Reads live buffer content in memory (preserving unsaved edits) or falls back to disk, with content_hash and pagination".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "File path (if omitted, reads the active editor buffer)"
                },
                "offset": {
                    "type": "integer",
                    "description": "Optional start byte offset"
                },
                "limit": {
                    "type": "integer",
                    "description": "Optional maximum bytes to read"
                }
            }
        }),
    });

    list.push(ToolDefinition {
        name: "editor.apply_edit".to_string(),
        description: "[Operation: Write] Applies surgical line replacement or full buffer replacement to live editor buffer".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "File path (if omitted, edits the active editor buffer)"
                },
                "start_line": {
                    "type": "integer",
                    "description": "1-based starting line number for surgical edit"
                },
                "end_line": {
                    "type": "integer",
                    "description": "1-based ending line number (inclusive) for surgical edit"
                },
                "new_text": {
                    "type": "string",
                    "description": "Replacement text for the specified line range"
                },
                "content": {
                    "type": "string",
                    "description": "Full new file content (used if start_line/end_line omitted)"
                },
                "base_hash": {
                    "type": "string",
                    "description": "Optional SHA-256 base hash from previous read for optimistic concurrency check"
                }
            }
        }),
    });

    list.push(ToolDefinition {
        name: "editor.get_selection".to_string(),
        description: "[Operation: Read] The text selected in the code editor (`text`) and where it is (`range`: `start`/`end`, each a 1-based `line` and `column`, the numbering editor.apply_edit takes); both null without a selection. Also the active file and its length".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "editor.get_completion".to_string(),
        description: "[Operation: Read] The completion list open in the code editor: its items (label, kind, detail), the first row shown (`top`), where the box is (`at`, window coordinates) and whether rust-analyzer's answer is still on its way. Does not change the view".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "editor.get_choices".to_string(),
        description: "[Operation: Read] When Go to Definition found several places, the list offered to choose from: whether it is open and each row (`file.rs:line  folder`); while the note dialog's Link item list is up (#659), that list instead, each row `Type::item  file.rs:line`. Does not change the view".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "view.get_inspector".to_string(),
        description: "[Operation: Read] The Object Inspector's rows as on screen: each category (and whether it is open), each property (key, label, depth — 1 for a sub-row of a folded one such as Font or Margins —, template, and its cells: slot, kind field/check/choice/caption/button/swatch with text, on, options and current, or rgba), each event and its handler, or the notice an empty panel shows. Does not change the view".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "view.get_open_menu".to_string(),
        description: "[Operation: Read] The menu open on screen, if any (the bar's, a panel's, the Explorer's or the code editor's context menu): which one, the row the arrow keys highlighted, each row's action, label, shortcut (the active key scheme's), submenu, whether it can run, open_submenu (name and rows) if a submenu is open, and rects (each bar button, the menu panel and the submenu panel, as [x, y, w, h]). Does not change the view".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "editor.get_rename".to_string(),
        description: "[Operation: Read] The Rename Symbol dialog (`editor.rename_symbol`): whether it is open, its `stage` (preparing, naming, asking, preview), the old and new names, the `message` it shows, and in preview the `files` with each one's `count`. Does not change the view".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "editor.get_references".to_string(),
        description: "[Operation: Read] The References panel (Find All References, `editor.find_references`): the symbol, the title, `waiting` while rust-analyzer looks, every file with `count`, `generated` (the generated form source), `read_only` and its occurrences (1-based `line`, `word`, `text`), and `rows`, each drawn row's rect [x, y, w, h] in window pixels. Does not change the view".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "editor.get_hover".to_string(),
        description: "[Operation: Read] The hover card over the code editor: whether it is open, its text (rust-analyzer's signature and docs, made plain), `note` (the id of the note it shows over a noted item, #659) and `anchor`, the name it points at on screen as [x, y, line height] in window pixels. Does not change the view".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "view.get_explorer".to_string(),
        description: "[Operation: Read] The Explorer's rows as drawn: each one's `template` (which names its colour — `FileError`, `FileWarn`, `MemberError`… when rust-analyzer published problems for it, #412), `icon`, `text`, `badge` (the problems' count goes before the owner word), `indent`, and its `rect` [x, y, w, h] in window pixels. Does not change the view".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "view.get_notes".to_string(),
        description: "[Operation: Read] The Notes side panel rows as drawn: chips, file dropdown, in-alert toggle, footer, and each drawn row with kind, text, badge, right line, group, faded status, and its rect [x, y, w, h] in window pixels. Does not change the view".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "view.get_tests".to_string(),
        description: "[Operation: Read] The Tests side panel as drawn (#927): the summary line, the Passed/Failed/Ignored chips, and each tree row (package, file, test) with its depth, mark (0 failed, 1 passed, 2 ignored, 3 not run), text, note, open and selected state and its rect [x, y, w, h] in window pixels. Does not change the view".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "form.check".to_string(),
        description: "[Operation: Read] The form checker's findings for the open form (#928): `summary` (\"2 errors · 5 warnings\") and `findings`, each `{ number, severity: \"error\"|\"warning\", rule, controls: [names, the marked one first], property, text, fix }` — the same list, numbers and fixes as the Form check panel and the pins on the canvas. Rules: overlapping siblings and controls outside their parent (errors); text contrast under WCAG AA, tied or unused TabOrder, and a Button or MenuItem with no OnClick (warnings). Does not change the form".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "view.sizes".to_string(),
        description: "[Operation: Read] Inspects the Sizes preview tab (#929), returning each shown size with its name, width, height, note, warning, and overflowing controls: `{ sizes: [{ name, width, height, note, warning, overflows: [{ control, edge, px }] }] }`. Does not change the form".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "strings.list".to_string(),
        description: "[Operation: Read] Lists translatable strings across project forms and their translations in each configured locale: `{ languages: [{code, name, done, total}], rows: [{key, label, source, cells: {code: {state, text}}}] }`. Optional `form` filters to one form, `lang` to one locale code".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "form": { "type": "string", "description": "Optional form name to filter by" },
                "lang": { "type": "string", "description": "Optional language code to filter by" }
            }
        }),
    });

    list.push(ToolDefinition {
        name: "strings.set".to_string(),
        description: "[Operation: Write] Sets a translation for a specific string key and language code, saving to `locales/<code>.toml` and repainting the view".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "key": { "type": "string", "description": "The persistent translation key (e.g. 'form_main.btn_ok.text')" },
                "lang": { "type": "string", "description": "The BCP 47 language code (e.g. 'pt-BR', 'de')" },
                "text": { "type": "string", "description": "The translated text" }
            },
            "required": ["key", "lang", "text"]
        }),
    });

    list.push(ToolDefinition {
        name: "view.get_rail".to_string(),
        description: "[Operation: Read] The Activity Rail as drawn: `panels`: each `{ \"panel\": <Panel label as the code names it>, \"rect\": [x, y, w, h] }` in window pixels. Does not change the view".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "view.get_toolbox".to_string(),
        description: "[Operation: Read] The Toolbox side panel empty state: `{ \"empty_state\": null }` when forms exist, or `{ \"empty_state\": { \"title\", \"body\", \"hint\", \"button\": { \"text\", \"rect\": [x, y, w, h] } } }` when the project has no form. Does not change the view".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "view.get_dialog".to_string(),
        description: "[Operation: Read] The open dialog (`open`: its name — `new_project`, `build`, `options`, `items`, `lines`, `rename`, `actions`, `imagelist`, `quick_open`, `quit`… — or null when none) and every control in it as drawn: `name`, `kind` (button, check, text, label, row), `text`, `checked` for a check box, `visible` and its `rect` [x, y, w, h] in window pixels, to click or type in for real; `rows`: a list drawn row by row (the Options page tree), each row's `text` and `rect`. `visible` is the control's own flag: one in a scrolled list may lie outside the list's viewport, so scroll it into view first. Does not change the view".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "dialog.press".to_string(),
        description: "[Operation: Write] Presses a button, toggles a check box or clicks a card in the open dialog through the same path as a real click. Call 'view.get_dialog' first to read the open dialog and its control names".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "control": {
                    "type": "string",
                    "description": "The name of the button or check box control to press"
                }
            },
            "required": ["control"]
        }),
    });

    list.push(ToolDefinition {
        name: "dialog.set_field".to_string(),
        description: "[Operation: Write] Sets the text of an input field, the state of a check box, or the selected item of a drop-down (by the item's text) in the open dialog through the same path as a real click and keystrokes. Call 'view.get_dialog' first to read the open dialog and its control names".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "control": {
                    "type": "string",
                    "description": "The name of the text input or check box control to set"
                },
                "value": {
                    "description": "The value to set: string for a text field or a drop-down item, boolean for a check box"
                }
            },
            "required": ["control", "value"]
        }),
    });

    list.push(ToolDefinition {
        name: "view.get_status_bar".to_string(),
        description: "[Operation: Read] The status bar's cells as drawn: `errors` and `warnings` (rust-analyzer's live count over every file, the same the Explorer colours by; the last build's before the server speaks; `—` before either), `lsp`, `target`, `message`, `selection`, `nodes`, `rev`, `zoom`, `framework`. Does not change the view".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "view.get_history".to_string(),
        description: "[Operation: Read] The local history view of the file in the centre: whether it is open, the file path, mode (differences, information, contents), entries, selected copy index, change blocks count and active block, and clickable control rects [x, y, w, h]".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "view.get_designer".to_string(),
        description: "[Operation: Read] The Form Designer bar (#564): `snap`, `show_grid`, `grid_size` (the same value as Options > Form Designer > Grid size), whether the Snap tooltip is on screen, `canvas` [origin_x, origin_y, scale] (window pixel = origin + design unit x scale), and clickable control rects [x, y, w, h] of `btn_designer_grid`, `btn_designer_snap`, `btn_grid_out`, `btn_grid_in`".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "designer.set_grid_size".to_string(),
        description: "[Operation: Write] Sets the Form Designer grid size, 2 to 64 (the bar's and the Options' one value); kept in the preferences".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "size": { "type": "number", "description": "Grid size in design units, 2 to 64" }
            },
            "required": ["size"]
        }),
    });

    list.push(ToolDefinition {
        name: "designer.set_language".to_string(),
        description: "[Operation: Write] Sets the Form Designer language preview ('en' for English source, or a project locale code like 'de', 'pt'). Unknown code returns an error listing valid languages".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "code": { "type": "string", "description": "Language code (e.g. 'en', 'de', 'pt')" }
            },
            "required": ["code"]
        }),
    });

    list.push(ToolDefinition {
        name: "view.get_welcome".to_string(),
        description: "[Operation: Read] The Welcome page (#555): whether it is the centre, the `Reopen the last project` box, the recent project folders in the order drawn, and clickable control rects [x, y, w, h] (`welcome_recent_N` opens row N, `welcome_forget_N` takes it out of the list)".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "view.show_panel".to_string(),
        description: "[Operation: Read] Switches the side panel, as its rail button does (e.g. 'Explorer', 'Inspector', 'Palette', 'Structure', 'Packages', 'Debug'). Changes the view, never the project".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "Name or identifier of the side panel to switch to"
                }
            },
            "required": ["name"]
        }),
    });

    list.push(ToolDefinition {
        name: "view.capture".to_string(),
        description: "[Operation: Read] Captures the IDE window, or a region of it, as a PNG image. The region uses the same logical [x, y, width, height] rects the view.get_* reads report. Optionally also saves the PNG to an absolute path. Changes nothing".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "x": {
                    "type": "number",
                    "description": "Optional left coordinate of the region in logical pixels"
                },
                "y": {
                    "type": "number",
                    "description": "Optional top coordinate of the region in logical pixels"
                },
                "width": {
                    "type": "number",
                    "description": "Optional width of the region in logical pixels (must be greater than 0)"
                },
                "height": {
                    "type": "number",
                    "description": "Optional height of the region in logical pixels (must be greater than 0)"
                },
                "path": {
                    "type": "string",
                    "description": "Optional absolute path to save the captured PNG file to disk"
                }
            }
        }),
    });

    list.push(ToolDefinition {
        name: "ide.run_action".to_string(),
        description: "[Operation: Execute] Run an IDE action by its catalogue identifier (e.g. 'tools.options', 'file.save'). An action that is greyed right now (no form open, nothing being debugged…) is refused with 'action-disabled' and the reason".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "id": {
                    "type": "string",
                    "description": "Catalogue identifier of the action to execute"
                },
                "args": {
                    "type": "object",
                    "description": "The action's parameters, e.g. {\"path\": \"src/main.rs\"} for 'explorer.copy_path'"
                }
            },
            "required": ["id"]
        }),
    });

    list.push(ToolDefinition {
        name: "acp.list_agents".to_string(),
        description: "[Operation: Read] Lists all ACP agents in the registry for this platform with installation status and action".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "acp.install_agent".to_string(),
        description: "[Operation: Execute] Downloads and installs a binary ACP agent into the local agents folder".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "id": {
                    "type": "string",
                    "description": "Identifier of the agent to install"
                }
            },
            "required": ["id"]
        }),
    });

    list.push(ToolDefinition {
        name: "acp.connect_agent".to_string(),
        description: "[Operation: Execute] Connects an ACP agent to the active IDE configuration and saves it".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "id": {
                    "type": "string",
                    "description": "Identifier of the agent to connect"
                }
            },
            "required": ["id"]
        }),
    });

    list.push(ToolDefinition {
        name: "view.get_chat".to_string(),
        description: "[Operation: Read] The chat turns displayed in the AI chat panel: each turn has 'user' ({text, meta} or null) and 'reply' ({text, meta, notice} or null). Does not change the view".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    for (name, description, properties) in [
        (
            "chat.send",
            "[Operation: Execute] Types `text` in the chat's prompt box and sends it, as Enter does (queued while the agent works). Returns `view.get_chat`",
            json!({ "text": { "type": "string" } }),
        ),
        (
            "chat.stop",
            "[Operation: Execute] Stops the agent's turn, as the stop button does. Returns `view.get_chat`",
            json!({}),
        ),
        (
            "chat.new_session",
            "[Operation: Execute] A new conversation and a new ACP session, as New Session does. Returns `view.get_chat`",
            json!({}),
        ),
        (
            "chat.context_menu",
            "[Operation: Execute] One item of the chat's `+` menu, as its click: `media`, `mentions` (adds `@<open file>`), `actions` (the slash commands), `memory`, `mcp`, `agent`. Returns `view.get_chat`",
            json!({ "item": { "type": "string", "enum": ["media", "mentions", "actions", "memory", "mcp", "agent"] } }),
        ),
        (
            "chat.permission",
            "[Operation: Execute] Answers the permission card the agent raised (`view.get_chat.permission`): `allow_once`, `allow_always`, `reject_once`, `reject_always`. Returns `view.get_chat`",
            json!({ "choice": { "type": "string", "enum": ["allow_once", "allow_always", "reject_once", "reject_always"] } }),
        ),
        (
            "chat.queue",
            "[Operation: Execute] A message waiting in the queue (`view.get_chat.queue`, while the agent works): `send` it now, `edit` it back into the prompt, or `delete` it. Returns `view.get_chat`",
            json!({ "action": { "type": "string", "enum": ["send", "edit", "delete"] }, "index": { "type": "integer", "minimum": 0 } }),
        ),
        (
            "chat.history",
            "[Operation: Execute] The past conversations (#478), newest first: each `id`, `title` (its first message), `agent`, `turns`, `group` (Today/Yesterday/Earlier), `when`, and `resumes` (the agent can continue the same ACP session). `query` filters by title or agent. `open: <id>` reopens one, as a click on it: its turns come back and, when `resumes`, its session goes on. Returns `conversation` (the open one's id) and `history`",
            json!({ "open": { "type": "string" }, "query": { "type": "string" } }),
        ),
        (
            "chat.set_context",
            "[Operation: Execute] The `Form` / `Sel` switches: whether the open file or form, and its selection, go with each message. Returns `view.get_chat`",
            json!({ "form": { "type": "boolean" }, "selection": { "type": "boolean" } }),
        ),
    ] {
        list.push(ToolDefinition {
            name: name.to_string(),
            description: description.to_string(),
            input_schema: json!({ "type": "object", "properties": properties }),
        });
    }

    list.push(ToolDefinition {
        name: "chat.copy_message".to_string(),
        description: "[Operation: Execute] Copies a chat message to the clipboard, as its Copy button does: `turn` (0 = the oldest) and `part` `reply` (default, the whole markdown) or `user`. Returns the text copied".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "turn": { "type": "integer", "minimum": 0 },
                "part": { "type": "string", "enum": ["reply", "user"] }
            }
        }),
    });

    list.push(ToolDefinition {
        name: "chat.get_config_options".to_string(),
        description:
            "[Operation: Read] Active ACP agent configuration options, categories, and choices"
                .to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "chat.set_config_option".to_string(),
        description: "[Operation: Execute] Selects an ACP agent configuration option (e.g. model or thought_level)".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "config_id": {
                    "type": "string",
                    "description": "Identifier of the configuration option"
                },
                "value": {
                    "type": "string",
                    "description": "Choice value to select"
                }
            },
            "required": ["config_id", "value"]
        }),
    });

    list.push(ToolDefinition {
        name: "view.get_extension_views".to_string(),
        description: "[Operation: Read] The Extensions panel (#398): every view an extension declared with the tree it sent (`views`: extension, id, title, items), the `filter` typed and its box's `filter_rect`, and `rows`, each drawn row (`kind` header or item, `text`, `depth`) with its rect [x, y, w, h] in window pixels (a click on an item is sent to its extension and opens its place). Does not change the view".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "editor.get_line_marks".to_string(),
        description: "[Operation: Read] The marks extensions painted on the open code file (#398) that are drawn now: each with its `owner` extension, its `id` and its gutter `icon` rect [x, y, w, h] in window pixels (a click there is sent to the extension). Does not change the view".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "editor.get_semantic_tokens".to_string(),
        description: "[Operation: Read] The spans of the active code file that rust-analyzer classified and the editor colours over its tokenizer (#362): each with its 1-based `line`, `start` and `end` bytes, `kind` (Macro, Trait, Mutable, Parameter, Field, EnumMember, Unsafe; null for a deprecated name the tokenizer colours), `struck` (deprecated, struck through while typing, #730) and `text`. Does not change the view".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "editor.get_code_lenses".to_string(),
        description: "[Operation: Read] The code lenses shown in the active code editor: each with its 1-based `line`, `title`, `kind` (show_locations, run, other) and `locations` count. Does not change the view".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "editor.get_code_actions".to_string(),
        description: "[Operation: Read] The code actions and quick fixes rust-analyzer offers at the caret (#359): the 1-based `line`, `waiting` while it is asked, `actions` (each `title`, `kind`, `preferred`, in the order `editor.apply_code_action` takes by `index`) and whether the light `bulb` is shown. `editor.code_actions` opens the list. Does not change the view".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "editor.get_signature_help".to_string(),
        description: "[Operation: Read] The parameter card over the code editor (signature help): whether it is open, the signature's `label` and `active_parameter`, the parameter the caret is on (empty when none is marked). Typing ( or , in a call opens it; `editor.parameter_hints` asks at the caret. Does not change the view".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "editor.get_decorations".to_string(),
        description: "[Operation: Read] The squiggles drawn on the active code editor (rust-analyzer's diagnostics): each with its kind and its start and end as [line, byte] (0-based, byte in the line). Does not change the view".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "editor.open_file".to_string(),
        description:
            "[Operation: Read] Opens a specific file in a code editor tab and switches view to Code"
                .to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "File path to open"
                }
            },
            "required": ["path"]
        }),
    });

    list.push(ToolDefinition {
        name: "editor.list_open_files".to_string(),
        description: "[Operation: Read] Lists all open editor tabs and buffers with their paths and modified/dirty status".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    // --- 3. Project & Workspace (project.*) ---
    list.push(ToolDefinition {
        name: "project.get_tree".to_string(),
        description:
            "[Operation: Read] Returns the directory and file tree of the current project workspace"
                .to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "max_depth": {
                    "type": "integer",
                    "description": "Maximum directory traversal depth (default: 4)"
                }
            }
        }),
    });

    list.push(ToolDefinition {
        name: "project.create_file".to_string(),
        description: "[Operation: Write] Creates a new file in the workspace directory with optional initial content".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Relative file path inside the project"
                },
                "content": {
                    "type": "string",
                    "description": "Initial content for the file"
                },
                "open_in_editor": {
                    "type": "boolean",
                    "description": "Whether to open the new file in an editor tab (default: true)"
                }
            },
            "required": ["path"]
        }),
    });

    list.push(ToolDefinition {
        name: "project.create_form".to_string(),
        description: "[Operation: Write] Creates a new visual form pair (.crawform and .rs) in the project and opens it in the designer".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "Form name (e.g. 'form_settings' or 'FormSettings')"
                },
                "folder": {
                    "type": "string",
                    "description": "Folder relative to project root (default: 'src')"
                }
            },
            "required": ["name"]
        }),
    });

    list.push(ToolDefinition {
        name: "project.delete_file".to_string(),
        description: "[Operation: Write] Deletes a file from the workspace project and closes any open editor buffers for it".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Relative or absolute file path to delete"
                }
            },
            "required": ["path"]
        }),
    });

    list.push(ToolDefinition {
        name: "project.get_dependencies".to_string(),
        description: "[Operation: Read] Reads and parses dependencies configured in Cargo.toml of the active project".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "project.install_extension".to_string(),
        description: "[Operation: Execute] Installs the extension in a folder (its manifest.json and files) into the user's extension store (`%APPDATA%/CrawRAD/extensions/<id>@<version>/`, installed once for every project), registers it in the project's `.crawrad/extensions.json`, and starts it now, no restart: its actions, cards, line marks and views are there at once (#398). Refused when that id is already in the project list".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "The folder holding the extension's manifest.json" }
            },
            "required": ["path"]
        }),
    });

    list.push(ToolDefinition {
        name: "project.install_component".to_string(),
        description: "[Operation: Execute] Installs a component package (a folder with its manifest.json) into the IDE's components folder and reloads the catalog now, no restart: its components are in the palette and the Inspector at once. Refused, with nothing copied, when the package is already installed, when its manifest is refused, or when one of its classes is already taken".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "The folder holding the component package's manifest.json" }
            },
            "required": ["path"]
        }),
    });

    list.push(ToolDefinition {
        name: "project.uninstall_extension".to_string(),
        description: "[Operation: Write] Uninstalls an extension now, no restart: its process stops, its actions, cards, line marks and views go, its entry is removed from `.crawrad/extensions.json`, and its user store folder is preserved for other projects (legacy `.crawrad/extensions/<id>/` folders are deleted) (#398)".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "id": { "type": "string", "description": "The extension's id, as its manifest says" }
            },
            "required": ["id"]
        }),
    });

    list.push(ToolDefinition {
        name: "project.add_dependency".to_string(),
        description:
            "[Operation: Execute] Adds a crate dependency to the project using 'cargo add'"
                .to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "crate_name": {
                    "type": "string",
                    "description": "Name of the crate to add (e.g. 'serde', 'tokio')"
                },
                "features": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Optional list of features to enable"
                },
                "dev": {
                    "type": "boolean",
                    "description": "Whether to add as a dev-dependency (--dev)"
                }
            },
            "required": ["crate_name"]
        }),
    });

    // --- 4. Build, Run & Diagnostics (build.* and run.*) ---
    list.push(ToolDefinition {
        name: "build.check".to_string(),
        description: "[Operation: Execute] Runs 'cargo check --message-format=json' in the active project and returns structured compiler messages".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "build.build".to_string(),
        description: "[Operation: Execute] Runs 'cargo build' in the active project returning compiler status and output".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "release": {
                    "type": "boolean",
                    "description": "Whether to build with --release"
                }
            }
        }),
    });

    list.push(ToolDefinition {
        name: "build.get_diagnostics".to_string(),
        description: "[Operation: Execute] Runs static analysis via rustc / cargo check and returns list of errors and warnings with file/line/column locations".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    list.push(ToolDefinition {
        name: "run.start".to_string(),
        description: "[Operation: Execute] Starts the application process using 'cargo run'"
            .to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "args": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Optional command-line arguments to pass to the running program"
                }
            }
        }),
    });

    list.push(ToolDefinition {
        name: "run.stop".to_string(),
        description: "[Operation: Execute] Stops the currently running application process"
            .to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    });

    // The debugger. Its gestures (`project.run`, `debug.step_over`,
    // `debug.stop`…) are catalogue actions and are listed above; these two read
    // the session and place breakpoints by file and line.
    list.push(ToolDefinition {
        name: "debug.state".to_string(),
        description: "[Operation: Read] The debug session: phase (idle, starting, running, stopped, or ended with how), where the program stopped with its stack and its locals as Rust `{:?}` text, the program's last output, and every breakpoint by file".to_string(),
        input_schema: json!({ "type": "object", "properties": {} }),
    });
    list.push(ToolDefinition {
        name: "debug.set_breakpoints".to_string(),
        description: "[Operation: Write] Replaces the breakpoints of one file; an empty list clears them. The editor shows them and a program being debugged gets them at once".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "The file, absolute or relative to the workspace root" },
                "lines": { "type": "array", "items": { "type": "integer", "minimum": 1 }, "description": "One-based line numbers" }
            },
            "required": ["path", "lines"]
        }),
    });

    // The diff review. A write tool answers `status: "pending_approval"` and the
    // edit waits in the editor: without these three, the caller is told to wait
    // for a human and has no way to see the review or finish it.
    list.push(ToolDefinition {
        name: "review.status".to_string(),
        description: "[Operation: Read] The diff review waiting in the editor, if any: which file, which tool proposed it, how many additions and deletions, and how many separate changes it holds".to_string(),
        input_schema: json!({ "type": "object", "properties": {} }),
    });
    list.push(ToolDefinition {
        name: "review.accept".to_string(),
        description: "[Operation: Write] Accepts the whole diff review waiting in the editor, the same as the 'Accept all' button. The proposed content becomes the file's. Refused when no review is pending".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "The reviewed file; omit for the one shown in the editor" }
            }
        }),
    });
    list.push(ToolDefinition {
        name: "review.reject".to_string(),
        description: "[Operation: Write] Rejects the whole diff review waiting in the editor, the same as the 'Reject all' button. The file is left byte for byte as it was. Refused when no review is pending".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "The reviewed file; omit for the one shown in the editor" }
            }
        }),
    });

    // 5. Dynamic capabilities and extension contributions from CapabilityRegistry
    if let Some(reg) = registry {
        let mut existing_names: std::collections::HashSet<String> =
            list.iter().map(|t| t.name.clone()).collect();

        for cap in reg.list() {
            if !existing_names.contains(&cap.id) {
                let mut desc = if !cap.description.is_empty() {
                    cap.description.clone()
                } else {
                    cap.label.clone()
                };

                match cap.view_affinity {
                    ViewAffinity::ViewSwitch => {
                        desc.push_str(" (View switch tool: exempt from view guards)");
                    }
                    ViewAffinity::Design => {
                        desc.push_str(
                            " (Requires form designer view; call 'view.toggle_code' if in code editor)",
                        );
                    }
                    ViewAffinity::Code => {
                        desc.push_str(
                            " (Requires code editor view; call 'view.toggle_code' if in form designer)",
                        );
                    }
                    ViewAffinity::Any => {}
                }

                existing_names.insert(cap.id.clone());
                list.push(ToolDefinition {
                    name: cap.id.clone(),
                    description: desc,
                    input_schema: cap.input_schema(),
                });
            }
        }
    }

    list
}

#[note("tools-tools-list-result")]
pub fn tools_list_result(registry: Option<&CapabilityRegistry>) -> serde_json::Value {
    let extension_tools: Vec<&str> = registry
        .map(|reg| reg.list())
        .unwrap_or_default()
        .into_iter()
        .filter(|cap| matches!(cap.source, CapabilitySource::Extension { .. }))
        .map(|cap| cap.id.as_str())
        .collect();
    json!({
        "tools": build_tools_list_with_registry(registry),
        "_meta": { "version": super::TOOLS_VERSION, "extensionTools": extension_tools },
    })
}

#[note("tools-code-writing-tool-names")]
pub fn code_writing_tool_names() -> Vec<&'static str> {
    vec!["code.edit", "editor.apply_edit", "project.create_file"]
}

#[note("tools-is-code-writing-tool")]
pub fn is_code_writing_tool(name: &str) -> bool {
    matches!(
        name,
        "code.edit" | "editor.edit_file" | "editor.apply_edit" | "project.create_file"
    )
}

#[note("tools-validate-arguments")]
pub fn validate_arguments(
    action: &crawrad_actions::Action,
    args: &serde_json::Value,
) -> Result<(), String> {
    let obj = match args.as_object() {
        Some(o) => o,
        None => {
            return Err("Arguments must be a JSON object".to_string());
        }
    };

    for param in action.params {
        let val = obj.get(param.name);
        if param.required && val.is_none() {
            return Err(format!(
                "Missing required argument '{}' for action '{}'",
                param.name, action.id
            ));
        }

        if let Some(v) = val {
            if param.list {
                if !v.is_array() {
                    return Err(format!(
                        "Argument '{}' must be an array, got {:?}",
                        param.name, v
                    ));
                }
            } else {
                match param.kind {
                    crawrad_actions::ParamKind::String
                    | crawrad_actions::ParamKind::ComponentId => {
                        if !v.is_string() {
                            return Err(format!(
                                "Argument '{}' must be a string, got {:?}",
                                param.name, v
                            ));
                        }
                    }
                    crawrad_actions::ParamKind::Number => {
                        if !v.is_number() {
                            return Err(format!(
                                "Argument '{}' must be a number, got {:?}",
                                param.name, v
                            ));
                        }
                    }
                    crawrad_actions::ParamKind::Integer => {
                        if !v.is_i64() && !v.is_u64() {
                            return Err(format!(
                                "Argument '{}' must be an integer, got {:?}",
                                param.name, v
                            ));
                        }
                    }
                    crawrad_actions::ParamKind::Boolean => {
                        if !v.is_boolean() {
                            return Err(format!(
                                "Argument '{}' must be a boolean, got {:?}",
                                param.name, v
                            ));
                        }
                    }
                    crawrad_actions::ParamKind::Enum(variants) => {
                        let s = v.as_str().ok_or_else(|| {
                            format!("Argument '{}' must be a string enum", param.name)
                        })?;
                        if param.domain.is_none() && !variants.contains(&s) {
                            return Err(format!(
                                "Argument '{}' has invalid value '{s}'. Allowed values: {:?}",
                                param.name, variants
                            ));
                        }
                    }
                    crawrad_actions::ParamKind::Any => {}
                }
            }

            if let Some(ref dom) = param.domain
                && let Some(s) = v.as_str()
            {
                let choices = (dom.values)();
                if !choices.iter().any(|c| c.id == s) {
                    let allowed: Vec<&str> = choices.iter().map(|c| c.id).collect();
                    return Err(format!(
                        "Argument '{}' has invalid value '{s}'. Allowed values: {:?}",
                        param.name, allowed
                    ));
                }
            }
        }
    }

    Ok(())
}

#[note("tools-unwrap-action-execute")]
pub fn unwrap_action_execute<'a>(name: &'a str, args: &'a Value) -> (&'a str, &'a Value) {
    if name != "action.execute" {
        return (name, args);
    }
    let target = args.get("name").and_then(Value::as_str).unwrap_or(name);
    let inner = args
        .get("arguments")
        .or_else(|| args.get("params"))
        .unwrap_or(args);
    (target, inner)
}

#[note("tools-is-save-tool")]
pub fn is_save_tool(name: &str) -> bool {
    matches!(
        name,
        "file.save" | "arquivo.salvar" | "save" | "editor.save"
    ) || name.ends_with(".salvar")
        || name.ends_with(".save")
}
