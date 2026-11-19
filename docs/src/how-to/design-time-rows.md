# Show real rows in the Designer

A Grid bound to a Dataset shows the first rows of its query while you design.
This guide sets that up and controls how many rows you see.

## Bind the Grid

You need the chain the app uses at run time:

1. A provider, for example **SQLite Provider (rusqlite)** from the **Data**
   section of the Toolbox.
2. A **Connection** whose **Provider** is that provider and whose **Url** points
   at the database, for example `sqlite:contacts.db`. A relative path is read
   from the project folder.
3. A **Dataset** whose **Connection** is that connection and whose **SQL** is
   your query.
4. The Grid's **DataSource**, in the **Data** category, set to the Dataset.

The Grid fills with the columns and rows of the query. The tutorial
[A form that reads a database](../tutorials/a-form-that-reads-a-database.md)
walks through the same steps with pictures.

## Choose how many rows

Set the Grid's **DesignRows**, in the **Data** category, to **Off**, **8**,
**20** or **50**. The default is 8. **Off** shows the columns without rows.

## Read the badge

The foot of the Grid says what you are looking at, for example
**design-time rows · 8 of 8 · dst1 (contacts.db)**. When the database cannot
be reached, the badge turns amber and says why; the Grid keeps the last columns
it knew and you keep working.

## Good to know

- The rows are a preview. Nothing you do in the Designer is written to the
  database.
- If the Inspector's **Field** rows say *Columns not listed*, the IDE could not
  run its small column reader (`crawrad-dbprobe`) from its own folder. Build the IDE
  with `cargo ide` so the reader is built too.
