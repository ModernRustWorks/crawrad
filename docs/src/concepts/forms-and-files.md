# Forms: the designer file and your file

A form is what you draw in the Designer: a window with its components and
their properties. CrawRAD keeps each form in three files, and each file has an
owner.

| File | Owner | What it holds |
|---|---|---|
| `form_main.crawform` | the IDE | The form itself: components, properties and event links, as JSON. |
| `form_main.designer.rs` | the IDE | The Rust code that builds the form. |
| `form_main.rs` | you | Your code: the event handlers and anything else you write. |

The Explorer shows the owner as a badge next to each file:

- **Rewritten** — rewritten in full on every Generate. Anything you type there is lost.
- **Yours** — written once and never touched again.
- **Yours now** — written by the IDE the first time; from then on it is yours,
  and Generate only adds what is missing.

## Your code is never overwritten

When you link an event in the Object Inspector (for example `OnClick` on a
button), the IDE adds an empty handler to your file. It never changes or
removes code you wrote.

The one thing Generate cleans up is a handler the IDE created that you left
**empty**: Generate removes it, together with its event link, so the form does
not keep links to code that does nothing. As soon as you write anything in the
handler, it is yours like the rest of the file.

## The project's `Cargo.toml` is yours too

The project manifest is created by Cargo, not by the IDE, and it belongs to
you. You add dependencies to it the normal Cargo way. CrawRAD reads it, for
example to find out which [target](targets.md) the project uses.
