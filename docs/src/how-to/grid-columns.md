# Configure the columns of a Grid

A Grid bound to a Dataset shows every column of the query, in query order, with
the field name as the title. To choose which columns appear, their titles,
widths and formats, give the Grid its own **Columns**.

## Open the columns editor

Select the Grid. In the **Object Inspector**, find the **Columns** row and click
its **···** button. Double-clicking the Grid on the form opens the same editor.

The editor lists only the columns. Click one and its properties open in the
Object Inspector.

## Add the columns

- **Add all fields** adds one column per field of the Grid's table. Numbers are
  right-aligned, a whole number gets a narrow column, a real number gets a
  thousands mask. Fields that already have a column are skipped, so pressing it
  again after the query changes adds only the new fields.
- **Add** adds one empty column. Set its **Field** to the name of a field of the
  query.

Use **Move up** and **Move down** to change the order, and **Delete** to remove
a column. As soon as the Grid has columns of its own, it shows only those.

## Set up one column

Click the column in the editor, then in the Object Inspector:

| To | Set |
|---|---|
| Change the header text | **Title** |
| Fix the width | **Width**, or **MinWidth** / **MaxWidth** |
| Align the values | **Alignment** |
| Format numbers or dates | **DisplayFormat** |
| Show text for an empty value | **NullText** |
| Hide it without deleting it | **Visible** = false |
| Stop the user from editing it | **ReadOnly** = true |
| Offer a fixed list of values | **PickList**, one value per line |

## Colour a cell by its value

Each column has its own **StyleRules** collection. Add a rule, set **When** to a
condition on the value and pick the **Color**, **TextColor** or font style to
use when it matches.

## Check it

The Designer shows real rows while you work (see
[Data at design time](../concepts/design-time-data.md)), so you see the titles,
widths and formats against real data before you run the app. The running app
uses the same widths.
