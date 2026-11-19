# Design view and Code view

Every form opens in two views. The tabs at the bottom of the editor switch
between them:

- **Design** — the form as you draw it. You place components from the palette,
  move and resize them, and set their properties in the Object Inspector.
- **Code** — your Rust file for the form, where you write the event handlers.
- **History** — the local copies of the file, compared with what is in the editor now.

The two views are the same form. A property you set in Design ends up in the
generated code; a handler you link in the Object Inspector appears in Code,
ready for you to fill in.

## Where state belongs

What the form looks like when it opens — texts, fonts, colours, sizes,
positions — belongs in **Design**, as properties. Set them in the Object
Inspector, not in code at start-up. Code is for what changes while the app
runs: answering a click, loading data, reacting to the user.
