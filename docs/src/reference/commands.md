# Menus and commands

This page is generated from the command catalogue: every command the IDE offers, grouped by where you find it. The id is the name an agent uses to run the command over MCP. Shortcuts are shown for both keymap schemes, Modern (the default) and Classic, chosen in Tools > Options.

## File

| Command | Shortcut | Classic keymap | What it does | Id |
|---|---|---|---|---|
| New Project... | — | — | Create a Cargo project — cargo does it, CrawRAD asks the questions | `project.new` |
| Add Project... | — | — | Create a new project inside the open workspace, and list it as a member | `project.add` |
| Add Existing Project... | — | — | List a project folder inside the workspace as one of its members | `project.add_existing` |
| Open Project... | — | — | Open a project folder — the one named, or the folder picker | `project.open_folder` |
| Open Form... | — | — | Open a form (.crawform) — the one named, or the file picker | `file.open_form` |
| Remove from Recent | — | — | Take a folder out of the recent projects — the list, never the disk | `project.forget_recent` |
| New Form... | — | — | Create a new form (.crawform) in the current project | `file.new_form` |
| New Inherited Form... | — | — | Create a form that inherits the components and handlers of another | `file.new_inherited_form` |
| New Component... | — | — | Create a user component that inherits an existing one or composes several | `file.new_component` |
| New File... | — | — | Create a new Rust file (.rs) in the current project | `file.new_file` |
| Quick Open... | `Ctrl+P` | `Ctrl+O` | Quickly open any file in the workspace | `file.quick_open` |
| Save | `Ctrl+S` | `Ctrl+S` | Save what is in front — the edited file, or the form | `file.save` |
| Save All | — | `Ctrl+Shift+S` | Save every changed file: the code in every tab, then the form | `file.save_all` |
| Reload | — | — | Discard changes and reload from disk | `file.reload` |
| Close | — | — | Close the form — the folder stays open | `file.close` |
| Recover Unsaved Work | — | — | Bring recovery copies back as unsaved edits — the file on disk stays as it is | `file.recover_unsaved` |
| Discard Recovery Copies | — | — | Delete recovery copies left by a crash | `file.discard_recovery` |
| Keep My Version | — | — | Keep your version of the file and overwrite disk on next save | `editor.keep_mine` |
| Reload from Disk | — | — | Discard your edits and reload the file from disk | `editor.reload_from_disk` |
| Compare with Disk | — | — | Open diff review comparing your edits with the disk copy | `editor.compare_with_disk` |

## Explorer

| Command | Shortcut | Classic keymap | What it does | Id |
|---|---|---|---|---|
| Set as Active Project | — | — | The project Run and Generate work on (the workspace's default-members) | `project.set_active` |
| Toggle folder | — | — | Open the member node in the Explorer, or close it when it is open | `explorer.toggle_folder` |
| Toggle badge legend | — | — | Show or hide `What the badges mean` at the foot of the Explorer | `explorer.toggle_legend` |
| Toggle open editors | — | — | Show or hide open editors section in Explorer | `explorer.toggle_open_editors` |
| Toggle outline | — | — | Show or hide outline section in Explorer | `explorer.toggle_outline` |
| Toggle timeline | — | — | Show or hide timeline section in Explorer | `explorer.toggle_timeline` |
| Open timeline entry | — | — | Open local history view comparing the selected entry with current buffer | `explorer.open_timeline_entry` |
| Toggle dependencies | — | — | Show or hide dependencies section in Explorer | `explorer.toggle_dependencies` |
| New File... | — | — | Create a new file in the selected folder | `explorer.new_file` |
| New Form... | — | — | Create a new form (.crawform) in the selected folder | `explorer.new_form` |
| New Folder... | — | — | Create a new directory in the selected path | `explorer.new_folder` |
| Open | — | — | Open selected file in editor or designer | `explorer.open` |
| Reveal in File Explorer | — | — | Open system file manager showing this file or folder | `explorer.reveal` |
| Copy Path | — | — | Copy absolute file path to clipboard | `explorer.copy_path` |
| Copy Relative Path | — | — | Copy relative file path to clipboard | `explorer.copy_rel_path` |
| Rename... | — | — | Rename selected file or folder | `explorer.rename` |
| Delete | — | — | Delete selected file or folder | `explorer.delete` |
| Refresh Explorer | — | — | Refresh explorer file list | `explorer.refresh` |
| Collapse Folders in Explorer | — | — | Collapse all expanded folders in the explorer tree | `explorer.collapse_all` |

## Edit

| Command | Shortcut | Classic keymap | What it does | Id |
|---|---|---|---|---|
| Undo | `Ctrl+Z` | `Ctrl+Z` | Undo the last change | `edit.undo` |
| Redo | `Ctrl+Y` | `Ctrl+Shift+Z` | Redo the last undone change | `edit.redo` |
| Cut | `Ctrl+X` | `Ctrl+X` | Cut the selected components | `edit.cut` |
| Copy | `Ctrl+C` | `Ctrl+C` | Copy the selected components | `edit.copy` |
| Paste | `Ctrl+V` | `Ctrl+V` | Paste components from the clipboard | `edit.paste` |
| Delete | `Del` | `Del` | Delete the selected components | `edit.delete` |
| Select all | `Ctrl+A` | `Ctrl+A` | Select every component on the form | `edit.select_all` |
| Cut | `Ctrl+X` | `Ctrl+X` | Cut the selected text to the clipboard | `editor.cut` |
| Copy | `Ctrl+C` | `Ctrl+C` | Copy the selected text to the clipboard | `editor.copy` |
| Paste | `Ctrl+V` | `Ctrl+V` | Paste the clipboard's text at the caret | `editor.paste` |
| Select all | `Ctrl+A` | `Ctrl+A` | Select all the text of the file | `editor.select_all` |
| Duplicate | `Ctrl+D` | `Ctrl+Shift+D` | Duplicate the selected components | `edit.duplicate` |
| New action | — | — | Add an action to the selected ActionList | `action.new` |
| New page | — | — | Add a page to the selected PageControl, and open it | `page.new` |
| Standard actions... | — | — | Add standard actions to the selected ActionList | `action.standard` |
| Add all fields | — | — | Add one column per field of the grid's table | `collection.add_all_fields` |
| Lock controls | `Ctrl+Shift+L` | `Ctrl+Shift+L` | Stop components from being moved or resized in the designer — the Inspector still edits them | `edit.lock_controls` |
| Go to Line and Column | — | — | Put the caret on a line and column of the file open in the editor | `editor.go_to` |
| Trigger Suggest | — | — | Open the completion list at the caret: the component's fields at once, rust-analyzer's items when they arrive | `editor.complete` |
| Code Templates | — | — | Insert a ready snippet at the caret: Tab walks its stops | `editor.templates` |
| Toggle Bookmark | — | — | Numbered bookmark 1–9 on the caret's line, or off it (Ctrl+Shift+1…9) | `editor.toggle_bookmark` |
| Go to Bookmark | — | — | The caret to numbered bookmark 1–9 of the file in the editor (Ctrl+1…9) | `editor.go_to_bookmark` |
| Add documentation | `Ctrl+Alt+D` | `Ctrl+Alt+D` | Public docs of this item, in the rustdoc format. Shows in cargo doc and docs.rs. | `editor.add_documentation` |
| Add note | `Ctrl+Alt+N` | `Ctrl+Alt+N` | Private context for developers: why, history, decisions. Never becomes docs. | `editor.add_note` |
| Add comment | `Ctrl+Alt+M` | `Ctrl+Alt+M` | A short remark on this line, like a review comment. Open or resolved. | `editor.add_comment` |

## View

| Command | Shortcut | Classic keymap | What it does | Id |
|---|---|---|---|---|
| Toggle Design/Code | `F12` | `F12` | Switch between the form and the code behind it | `view.toggle_code` |
| Toggle form source | `Alt+F12` | `Alt+F12` | Show the form as text, in place of the drawing | `view.toggle_form_source` |
| Preview | — | — | Run the form inside the IDE — no ruler, no handles, no grid | `view.toggle_live` |
| Problems | `Ctrl+Shift+M` | `Ctrl+Shift+M` | Show what the last build reported, and click to go there | `view.toggle_problems` |
| Inlay Hints | — | — | Show or hide the types rust-analyzer infers, in grey inside the lines | `editor.toggle_inlay_hints` |
| Toggle panel | — | — | Show the panel, or hide it when it is already the visible tab | `layout.toggle_panel` |
| Reset Layout | — | — | Put every panel back where the IDE ships it | `layout.reset` |
| Show panel | — | — | Open the panel if it was closed, and focus it | `layout.show_panel` |
| Hide panel | — | — | Close the panel's tab — the rail brings it back | `layout.hide_panel` |
| Float panel | — | — | Detach the panel over the window — it does not leave the IDE | `layout.float_panel` |
| Move panel to side | — | — | Dock the panel against the left, right or bottom edge | `layout.move_panel` |
| Stack panel onto another | — | — | Put the panel in the same tab group as another panel | `layout.stack_panel` |
| Apply Desktop Profile | — | — | Put the window, docks, Inspector split, font and zoom as the profile has them | `layout.apply_profile` |
| Save Desktop Profile | — | — | Keep the window, docks, Inspector split, font and zoom under a name | `layout.save_profile` |
| Delete Desktop Profile | — | — | Forget a saved desktop profile | `layout.delete_profile` |
| Zoom in | `Ctrl+Shift+.` | `Ctrl+Shift+.` | Enlarge the form on the canvas — the form itself does not change | `view.zoom_in` |
| Zoom out | `Ctrl+Shift+,` | `Ctrl+Shift+,` | Shrink the form on the canvas — the form itself does not change | `view.zoom_out` |
| Zoom 100% | `Ctrl+1` | `Ctrl+1` | Back to actual size — the only zoom where the canvas measures what the app will | `view.zoom_reset` |
| Zoom in (interface) | — | — | Enlarge the whole IDE — menus, panels and the canvas together | `view.ui_zoom_in` |
| Zoom out (interface) | — | — | Shrink the whole IDE — menus, panels and the canvas together | `view.ui_zoom_out` |
| Zoom 100% (interface) | — | — | Back to the system size — the IDE at the scale the desktop asked for | `view.ui_zoom_reset` |
| Command Palette | `Ctrl+Shift+P` | `Ctrl+Shift+P` | Search and run any command or capability across the IDE | `view.command_palette` |

## Tools

| Command | Shortcut | Classic keymap | What it does | Id |
|---|---|---|---|---|
| Manage Code Templates… | — | — | Add, change, import and export your code templates | `editor.manage_templates` |
| Options... | `Ctrl+,` | — | Configure IDE settings, designer grid, build targets and ACP agents | `tools.options` |
| MCP Servers... | — | — | Configure connected Model Context Protocol servers and tool integrations | `tools.mcp` |
| AI Custom Instructions... | — | — | Global instructions and context shared with all AI assistant sessions | `tools.ai_memory` |
| Notes... | `Ctrl+Shift+N` | `Ctrl+Shift+N` | Add or edit documentation, a note or a comment at the caret | `tools.sidecar_note` |

## Go

| Command | Shortcut | Classic keymap | What it does | Id |
|---|---|---|---|---|
| Go to Definition | `Ctrl+F12` | `Alt+Up` | Ask rust-analyzer where the symbol at the caret is defined and go there; Ctrl+Click does the same | `editor.goto_definition` |
| Go Back | `Alt+Left` | `Alt+Left` | Back to where the last Go to Definition left | `editor.navigate_back` |
| Go Forward | `Alt+Right` | `Alt+Right` | Forward again, to where Go Back left | `editor.navigate_forward` |
| Find References | `Shift+Alt+F12` | `Ctrl+Shift+Enter` | Every use of the name at the caret, its declaration included, in the References panel; the generated form source is marked | `editor.find_references` |
| Show Hover | `Ctrl+K` | `Ctrl+K` | rust-analyzer's signature and docs for the name at the caret; resting the mouse on a name does the same | `editor.show_hover` |
| Param Hints | `Ctrl+Shift+Space` | `Ctrl+Shift+Space` | rust-analyzer's signature of the call around the caret, its current parameter marked; typing ( or , opens it, Esc closes it | `editor.parameter_hints` |
| Context Menu | `Shift+F10` | `Shift+F10` | The code editor's context menu at the caret, as a right click opens it at the pointer | `editor.context_menu` |
| Go to base form | — | — | Open the form this one inherits from | `form.open_base` |

## Refactor

| Command | Shortcut | Classic keymap | What it does | Id |
|---|---|---|---|---|
| Format Document | `Shift+Alt+F` | `Ctrl+D` | Format the whole file with rust-analyzer (rustfmt); one Ctrl+Z undoes it. Not the generated form source | `editor.format_document` |
| Rename Symbol | — | `Ctrl+Shift+E` | Rename the name at the caret in every file, after a preview, in memory (F2 in the code editor). A component or a handler is renamed from the Object Inspector | `editor.rename_symbol` |
| Quick Fix... | `Ctrl+.` | `Ctrl+.` | rust-analyzer's code actions and quick fixes at the caret (an import, a qualified path, a rewrite); the light bulb on the line says there are some | `editor.code_actions` |
| Apply Code Action | — | — | Apply one of the code actions offered at the caret (editor.code_actions lists them), by its index from 0; the generated form source is refused | `editor.apply_code_action` |
| Fix and Explain | — | — | Fix the compiler diagnostic on the caret's line and explain why in chat | `chat.fix_explain` |
| Add File to Chat | — | — | Put @file in the chat message, without sending it | `ai.add_file_to_chat` |
| Add Selection to Chat | — | — | Attach the selected code to the chat message, without sending it | `ai.add_selection_to_chat` |
| Explain Selection | — | — | Attach the selected code and ask the chat to explain it | `ai.explain_selection` |
| Review Selection | — | — | Attach the selected code and ask the chat for a read-only review | `ai.review_selection` |

## rust-analyzer

| Command | Shortcut | Classic keymap | What it does | Id |
|---|---|---|---|---|
| Reload Workspace | — | — | Make rust-analyzer read the Cargo.toml files again | `lsp.reload_workspace` |
| Restart Server | — | — | Stop rust-analyzer and start it again | `lsp.restart` |
| Stop Server | — | — | Stop rust-analyzer; opening a Rust file starts it again | `lsp.stop` |

## Run

| Command | Shortcut | Classic keymap | What it does | Id |
|---|---|---|---|---|
| Run | `F5` | `F9` | Generate, build and run — with the PROJECT's generator, not the surface selector | `project.run` |
| Toggle Breakpoint | `F9` | `F5` | Add or remove the breakpoint on the caret's line of the file in the editor | `debug.toggle_breakpoint` |
| Add Watch | — | `Ctrl+F5` | Watch an expression in the Run and Debug panel — the selection or the word under the caret when none is given | `debug.add_watch` |
| Remove Watch | — | — | Remove a watch from the Run and Debug panel by its position (0 is the first) | `debug.remove_watch` |
| Run Without Debugging | `Ctrl+F5` | `Ctrl+Shift+F9` | Generate, build and run with no debugger attached — breakpoints are ignored | `project.run_without_debugging` |
| Step Over | `F10` | `F8` | Run the stopped program to the next line, over any call | `debug.step_over` |
| Step Into | `F11` | `F7` | Run the stopped program into the call on this line | `debug.step_into` |
| Step Out | `Shift+F11` | `Shift+F8` | Run the stopped program until the current function returns | `debug.step_out` |
| Pause | `F6` | — | Stop the running program where it is | `debug.pause` |
| Stop Debugging | `Shift+F5` | `Ctrl+F2` | End the program under the debugger | `debug.stop` |

## Project

| Command | Shortcut | Classic keymap | What it does | Id |
|---|---|---|---|---|
| Open code | — | — | Open the generated project in your editor | `project.open_editor` |
| Open in editor | — | — | Open a file of the project in a tab, next to the design | `editor.open` |
| Generate | — | `Ctrl+F9` | Generate the Cargo project — with the PROJECT's generator, not the surface selector | `project.generate` |
| Build... | `Ctrl+Shift+B` | `Shift+F9` | Configure targets, profiles and build project | `project.build_manager` |
| Build All | — | — | Build all workspace members with cargo build --workspace | `project.build_all` |
| Clean All | — | — | Remove what every workspace member built, with cargo clean | `project.clean_all` |
| Options... | — | — | Configure project title and icon | `project.options` |
| Run Language | — | — | Select language for project execution (empty for system language) | `project.run_language` |

## Help

| Command | Shortcut | Classic keymap | What it does | Id |
|---|---|---|---|---|
| Welcome Page | — | — | Show the welcome page — templates and recent folders | `help.welcome` |
| Check Environment... | — | — | Check the Rust toolchain and build tools this IDE needs | `help.environment` |

## Component

| Command | Shortcut | Classic keymap | What it does | Id |
|---|---|---|---|---|
| Install Package... | — | — | Install a component package — the folder with its manifest.json | `component.install` |
| Installed Packages... | — | — | What the catalogue loaded — and what it refused, with the reason | `component.installed` |
| Add component | — | — | Create a component of a class inside a parent, at a given rectangle | `component.add` |
| Set property | — | — | Change a property on one or more components, in a single undo step | `component.set_property` |
| Add item | — | — | Create an item inside a collection, of the class it declares | `component.add_item` |
| Reorder | — | — | Move a component up or down among its siblings | `component.reorder` |
| Rename | — | — | Change the Name of a component — it is unique in the form | `component.rename` |
| Bind event | — | — | Point an event of a component at a handler function — empty unbinds it | `event.bind` |
| Override inherited event | — | — | Give an inherited component's event a handler of this form that calls the base's first | `event.override` |
| Go to handler | — | — | Open this form's handler of the event in the code | `event.go_to_handler` |
| Go to inherited handler | — | — | Open the base form's handler of the event in the code | `event.go_to_inherited` |
| Revert to inherited | — | — | Point the event back at the base's handler; the method you wrote stays in the code | `event.revert_inherited` |
| Rename event handler | — | — | Give the handler of an event a new name, keeping its code — every event bound to it follows | `event.rename` |
| Bind property | — | — | Bind a component property to a data source — empty source unbinds it | `binding.set` |

## Align

| Command | Shortcut | Classic keymap | What it does | Id |
|---|---|---|---|---|
| Align left | — | — | Align left | `align.left` |
| Align centers | — | — | Align centers | `align.centers` |
| Align right | — | — | Align right | `align.right` |
| Align top | — | — | Align top | `align.top` |
| Align middles | — | — | Align middles | `align.middles` |
| Align bottom | — | — | Align bottom | `align.bottom` |
| Space equally, horizontally | — | — | Same gap between components, horizontally | `align.space_horizontally` |
| Space equally, vertically | — | — | Same gap between components, vertically | `align.space_vertically` |

## Size

| Command | Shortcut | Classic keymap | What it does | Id |
|---|---|---|---|---|
| Same width | — | — | Match the width of the first selected | `align.same_width` |
| Same height | — | — | Match the height of the first selected | `align.same_height` |
| Same size | — | — | Match the size of the first selected | `align.same_size` |

## In form

| Command | Shortcut | Classic keymap | What it does | Id |
|---|---|---|---|---|
| Center horizontally in form | — | — | Center the selection in the form, horizontally | `align.center_in_form_h` |
| Center vertically in form | — | — | Center the selection in the form, vertically | `align.center_in_form_v` |

## Order

| Command | Shortcut | Classic keymap | What it does | Id |
|---|---|---|---|---|
| Bring to front | — | — | Bring to front | `order.bring_to_front` |
| Bring forward | — | — | Bring forward | `order.bring_forward` |
| Send backward | — | — | Send backward | `order.send_backward` |
| Send to back | — | — | Send to back | `order.send_to_back` |
| Renumber tab order | — | — | Renumber the tab sequence from 0, removing gaps | `order.renumber_tab_order` |
| Earlier in tab order | — | — | Move the selected component earlier in the tab sequence | `order.tab_earlier` |
| Later in tab order | — | — | Move the selected component later in the tab sequence | `order.tab_later` |

## Designer

| Command | Shortcut | Classic keymap | What it does | Id |
|---|---|---|---|---|
| Designer Language | — | — | Switch visual designer language preview (empty for source English) | `designer.language` |
| Select Component | — | — | Select a component of the form by id (the Object Inspector picker) | `component.select` |

## Inspector

| Command | Shortcut | Classic keymap | What it does | Id |
|---|---|---|---|---|
| Open property editor | — | — | Open whatever edits this property — the lines editor, the collection editor, the align picker | `inspector.open_editor` |
| Open collection editor | — | — | Open the dialog of a component's child collection — the Columns of a Grid, the Panels of a StatusBar | `inspector.open_collection` |
| Close property editor | — | — | Close the property editor without writing anything | `inspector.close_editor` |
| Edit events… | — | — | Show the Events tab of the selected component | `inspector.show_events` |
| Rename | — | — | Type a new Name for the selected component in the Inspector | `inspector.rename` |
| Toggle property group | — | — | Unfold the sub-rows of a property — `Rect`, `Margins`, `Anchors` — or fold them away | `inspector.toggle_group` |
| Toggle property category | — | — | Fold a category of the Object Inspector — `Layout`, `Advanced` — or open it back | `inspector.toggle_category` |
| Pick picture file | — | — | Choose the image file a `Picture` property shows — the file dialog, or `file` to skip it | `inspector.pick_picture` |
| Pick font | — | — | Choose the font a `Font` property shows — the native font dialog, or `font` to skip it | `inspector.pick_font` |
| Set property name width | — | — | Move the splitter between the names and the values of the Object Inspector — the width of the name column, in pixels | `inspector.set_name_width` |
