# Reference extension: CrawRAD Code Formatter

An example extension that shows how CrawRAD extensions are put together.

It walks the whole circuit, with no code inside the IDE:

1. **Declarative manifest (`manifest.json`)**: metadata, commands, shortcuts, menus,
   AI tools (MCP) and project templates, all declared in one file.
2. **Menu contribution**: adds a `Format Code` entry to the IDE's `Tools` menu.
3. **Editor command**: runs `crawrad_format.format_file` on the active editor buffer or
   on the file given.
4. **MCP tool for AI**: exposes the command as an agent tool through a JSON Schema
   (`crawrad_format_format_file`), so AI agents and MCP clients can format code in the
   editor with `tools/call`.
5. **Project template**: adds an extension template to the *New Project* gallery.

## Manifest fields

- `id`: the extension's unique id (`crawrad.formatter`).
- `activation_events`: activation on demand (`["onCommand:crawrad_format.format_file"]`).
- `contributions.actions`: keyboard and command-palette actions (`Ctrl+Shift+F`).
- `contributions.menus`: where the entry goes in the IDE menu (`target: "Tools"`).
- `contributions.tools`: the tool exposed to the IDE's MCP server, with its JSON schema.
- `contributions.templates`: the project template offered in the New Project dialog.
