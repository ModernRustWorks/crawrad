# Projects and project groups

The folder you open in CrawRAD is a **project group**. A group holds one or
more projects: usually an app and, when you need them, libraries it uses.

```text
MyGroup/                      the folder you open: the group
  Cargo.toml                    lists the projects of the group
  app/
    Cargo.toml                  the app
    src/
      form_main.crawform        the form you design
      form_main.designer.rs     written by the IDE
      form_main.rs              yours
```

A group is a normal Cargo workspace, and every project in it is a normal
Cargo package. You can build any of them with `cargo` outside the IDE, and the
IDE never moves or restructures your folders.

## The active project

One project of the group is **active**: it is the one Build and Run work on.
CrawRAD keeps that choice in the group's `Cargo.toml` (`default-members`),
so `cargo run` in a terminal runs the same project the IDE does.

See [Forms: the designer file and your file](forms-and-files.md) for what each
file inside a project is for.
