# Components and the palette

Everything you place on a form is a **component**: a button, an edit box, a
grid, a data connection. The **palette** lists the components you can use, and
the **Object Inspector** shows the properties and events of the one you select.

## Visual and non-visual components

Most components are visible in the running app. Some are not — a data
connection, an action list, an image list. They still sit on the form at design
time, drawn as a small icon, so you can select them and set their properties.

## Installing more components

A component can come as a **package**: a Rust crate with a `manifest.json` that
tells the IDE what it brings before anything is compiled.

- **Component ▸ Install Package…** installs a package from its folder.
- **Component ▸ Installed Packages…** shows what the palette loaded, and what
  it refused and why.

The IDE checks a package before it installs it. It refuses a package that
brings a component name already in use, or a property that would break the
forms that use it. You get the reason before waiting for a build.
