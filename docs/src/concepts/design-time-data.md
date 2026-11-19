# Data at design time

A `Grid` bound to a data source shows **real rows from your database while you
design**, not placeholders. You see the real columns and the first rows of
the table, so you can size and format columns against real data.

## How it works

- The Grid's `DataSource` points at a `Dataset`, whose `Connection` reaches
  the database and whose `SQL` is the query. The Grid shows that query's
  columns and rows.
- The `DesignRows` property, in the Data category, sets how many rows the
  Designer shows: **Off**, **8**, **20** or **50**. The default is 8.
- The rows are read-only in the Designer. They are a preview: nothing is
  written to the database.
- A badge at the foot of the Grid says what you are looking at, for example
  *design-time rows · 8 of 1,240 · customers (app.db)*.

## When the database is not reachable

The form never waits for the database. If it cannot be reached, the Grid keeps
the last columns it knew, the badge turns amber and says why, and you keep
working.

## At run time

Your app reads its own data when it runs. The design-time rows exist only in
the IDE.
