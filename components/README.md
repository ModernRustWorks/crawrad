# `components/` — the component suite

What the classic RAD IDE calls the **classic component library**: the component library that comes in the box,
with open source, and installs into the IDE.

Here it uses the vocabulary that Rust developers already know — **crates**, one per
responsibility, publishable on crates.io. No proprietary format.

## The TWO uses, and both must remain open

In the classic RAD IDE, a component can be **installed in the IDE** and **used at runtime**, and
they are two separate things: the *design-time package* puts the component on the palette and
in the Object Inspector; the *runtime package* is the code the application actually
runs.

A connection engine like `portadb` is, in practice, mostly used at runtime —
someone writes `cargo add portadb` and never opens the IDE. **That must
keep working.** That is what this separation ensures:

```
portadb                  ← PURE. It does not know CrawRAD exists.
      ▲                        `cargo add portadb` and you are done.
      │
portadb-crawrad          ← Knows both: wraps the engine for the IDE.
      ▲                        Palette, Object Inspector, code generation.
      │
crawrad-component (the SDK)
```

The dependency points in only one direction, and never the reverse. From that come three
properties, and the third is the most valuable:

1. **Whoever uses the engine does not carry the IDE.** The generated application depends on
   `portadb`, and nothing else.
2. **Whoever uses the IDE does not lose the engine.** The component is a layer on top,
   not a rewrite.
3. **You can write a component for an existing crate whose author
   never heard of CrawRAD.** Someone can publish `sqlx-crawrad` without
   `sqlx` knowing. That is what allows the ecosystem to grow without asking permission.

## THE BOX — what you use to write a component

What the classic RAD IDE delivers as the classic component library source. You do not write a `TComponent` from
scratch: you inherit from it and use the `TStrings`, `TCanvas`, `TCollection` that already
came in the box.

Here the box is the crate **`crawrad-component`**, and it is the only dependency that a
component needs to have:

```toml
[dependencies]
crawrad-component = "0.1"
```

| goes into the box | why |
|---|---|
| property types (`Strings`, and future parts) | you declare and receive the ready type |
| drawing arithmetic (`fraction`) | the SAME calculation that the canvas and the generated app use |

| does **not** go in | why |
|---|---|
| `Document`, `Registry`, `Designer` | sending them would turn the user's app into an interpreter |
| Makepad, egui, no toolkit | the component **does not paint**: it declares verbs, and the IDE paints |
| **any dependency** | it would enter the binary of someone who only dragged an icon — there is a gate |

**It goes into the binary of whoever uses your component**, and that is a settled decision
(ADR 0038):
a classic RAD IDE app links `rtl.bpl` for the same reason. A form with only native controls
still has not a single line of ours in `Cargo.toml`.

### What is still missing from the box, and is written down so as not to be forgotten

The **`#[derive(Component)]`** that writes `crawrad.toml` from the `struct` —
the classic RAD IDE's `published` in the language of Rust. Until it exists, the manifest is
written by hand, and that is a legitimate complaint from whoever writes their first one.

## The shape of a component

Every component folder follows the same pattern — which is what allows the IDE to open and
compile any of them.

```
my-component/
├── Cargo.toml          a plain crate. No `cdylib`, and NO dependency on us.
├── crawrad.toml        ← THE MANIFEST: what it is, and how it draws itself
└── src/
    └── lib.rs          the TYPE the user's app will hold
```

**There is no `register`, no `descriptor.rs`, no `design.rs`.** This form
was once written here, with `crate-type = ["cdylib", "rlib"]` and five files, and
it was what ADR 0036
**rejected** after measuring the cost to those who paid it: in classic RAD IDEs, installing a
design-time package means linking into the IDE, and a well-known form designer was rewritten
to a separate process because of that.

What remains is simpler and stronger:

| question | who answers |
|---|---|
| what it IS (palette, properties, events) | the `crawrad.toml` |
| how it DRAWS itself | the verbs in `crawrad.toml` — the IDE runs none of its code |
| what it WRITES to the application | the same manifest, through the four emission verbs |
| what it DOES | the `src/lib.rs`, which is regular Rust and runs in the user's application |

The entire `Gauge` is three files, and its `lib.rs` does not know the IDE
exists: `cargo add gauge-crawrad` and it works in any program.

## What installs with the IDE

The classic RAD IDE delivers `bin/`, `lib/`, and `source/`. In Rust vocabulary, and with what
ADR 0034 decides regarding the
bundled toolchain:

```
<CrawRAD install>/
├── bin/                the IDE, and the toolchain that built it
├── components/         the SOURCES of the components in the box
│   └── portadb-crawrad/
└── lib/                the libraries already built FOR THIS IDE
    └── <fingerprint>/
        └── portadb_crawrad.dll
```

The `lib/<fingerprint>/` is the pin of
ADR 0033
§2b: a library only loads in the IDE that compiled it. Source is forward-compatible;
the binary is pinned — just like in the classic RAD IDE, and for the same reason.

## What is here today

| folder | what it is | status |
|---|---|---|
| `portadb/` | the connection engine, agnostic of driver and IDE | **source brought in**, green suite |
| `portadb-crawrad/engines.toml` | the **five engine components** | enter the catalog and palette |
| `portadb-crawrad/crawrad.toml` | the `Connection`, the abstraction | **becomes CODE that COMPILES** |
| `portadb-crawrad/src/lib.rs` | the type that the application loads | created on 2026-09-18 — see below |
| `gauge-crawrad/` | the **`Gauge`** — a meter | **does BOTH halves**, and RESPONDS to click |

### `Connection` generated code that did not compile, and no one had asked

It was the first third-party component here and proved the entire
chain — palette, Object Inspector, `.crawform`, Rust code. And the package was
**manifest only**: the generated code wrote `portadb_crawrad::Connection` and the
crate did not exist.

What caught it was the first installation gate test, with the most
obvious question there was: *"does what we deliver pass through our own gate?"*. The crate
was written, and the gate that caught it stays.

**What it does not do yet is written in its `lib.rs`, in full**: it
stores what the designer declared and connects to no database. Linking it to the
`portadb/` engine is the next step, and contains a decision — the engine enables the
driver via a Cargo *feature*, so the `database` chosen in the Inspector must
become a feature in the generated project's `Cargo.toml`.

### `Gauge` is the first to close both halves

It **draws itself** in the designer from verbs in `crawrad.toml`, without a
line of Rust for it, and **becomes the type** `gauge_crawrad::Gauge` in the application.

Until 2026-09-18, no component did both. `Connection` became code and
did not draw — it is non-visual. `Gauge` drew and did not become code — it
had no crate.

| half | who answers |
|---|---|
| drawing in the designer | manifest verbs, painted by the IDE's `Brush` |
| type in the application | the `struct` from the `gauge-crawrad` crate |

And both read **the same properties**, because they come from the same manifest. That
is what prevents the designer from showing one thing and the application delivering another.

**The IDE does not execute a line of its code to draw.** The manifest declares
VERBS — `box`, `frame`, `text`, `dashed`, `check` — and the canvas paints them with the
same closed vocabulary that the twenty natives use. No `Cx2d`, no
shader, no library loaded into the IDE process.

And **there is no literal color**: the verb declares a role (`surface`, `border`, `text`,
`text_dim`, `accent`) and the theme resolves it. A component published on crates.io with
`#ff0000` inside would be immune to the theme and beyond the reach of whoever maintains the
IDE.

#### And its drawing CROSSES OVER to the application — on both targets

For one day it did not cross over, and the user's `cargo build` said this:

```text
warning: unused variable: `r`
warning: struct `Widget` is never constructed
```

The generator calculated the meter's rectangle and threw it away. The application had the
type and properties, and **drew nothing** — the designer showed one thing
and the application showed another, which is worse than not supporting it.

On the **Makepad** target, which is the default, the component did not even exist: the generator filtered
nodes by the class list it registered itself, and `Gauge` was dropped without
a word.

Today both draw, and what makes all three drawings agree is what
**crosses over compiled**, not copied:

| what | where it comes from |
|---|---|
| the bar calculation | the `fraction` from `shared_geometry.rs` — canvas calls it, generator emits the text |
| the five colors | the roles from `shared_theme.rs` — `[f32; 4]` on canvas, `#RRGGBB` in `live_design!` |

A meter showing 65% in the designer shows 65%, in the same color, in the application —
because it is the same calculation, not because two calculations happen to agree today.

#### And it NOTIFIES: a third party's `OnClick` reaches the handler

Every visual control in the classic RAD IDE has `OnClick`, and here it is the same. The
surface of a declared component is the **rectangle itself**, and what the
framework can report about a rectangle is what the pointer did to it.

```toml
[[component.event]]
name = "on_click"
display = "OnClick"
signature = "fn(&mut self)"
```

That is how a drawing becomes a component: the handler modifies properties, and the
declared drawing updates in the next frame — because the verbs read the fields.
Proven by clicking: two clicks on a 20% meter wrote `60` on the label and the
bar moved along, without a single line of hand-written drawing code.

The list of what a declared component can respond to is **one**, applies to every
class arriving from outside, and today has one item. An additional event enters when
both targets know how to deliver it — adding it earlier means putting an event in the Object Inspector
that is never called.

#### It was the component that BROKE the vocabulary, and that is why it is here

The first version had five verbs and all painted the entire rectangle. A
meter displays a VALUE, and that requires painting a **fraction** — which did not fit.

The fix was not a sixth verb: it was separating **where** you paint from **what** you
paint. Today a verb has an `area` (`tudo`, `inset`, or a fraction that follows a
property), and the five remain five.

```toml
[[component.draw]]
verb = "box"
fill = "accent"
area = { fraction_of = "value", axis = "x", min = 0.0, max = 100.0 }
```

### The two tiers, and why both exist

| | what it solves | the trade-off |
|---|---|---|
| **`Connection`** (abstraction) | the user selects the engine by property and **switches databases without rewriting repositories** | requires SOME engine to exist in the binary |
| **`SqliteProvider`, `PostgresProvider`, …** | specifies **which driver will exist in the binary** | whoever uses an engine directly is tied to it |

The engine component is the ***driver link*** of the classic RAD IDEs: in them,
dropping it onto the form is what causes the driver to be *linked* into the application. In Rust
the exact equivalent is the **Cargo feature** — without `portadb/rusqlite`, the
driver module is not even compiled (`drivers/mod.rs` is entirely `#[cfg(feature = …)]`).

That is why they are not redundant: the abstraction component says *"I want to connect"*, and the
engine component says *"and THIS is the driver that must be in the binary"*.

**And they are the ones that prove the gate first.** An engine component declares
only `name` and `rect`, which belong to the common core — it does not run into the missing
drawer. The five are read from a file, validated, and registered in a catalog alongside
the twenty natives, appearing on the palette in the `Data` group.

### `Connection` began installing, and the gap was closed inside out

Properties that belong **to the component** — `url`, `database`,
`max_connections`, `connect_timeout` — had no drawer in the contract of
ADR 0002: a key was either in the common
core (which *every* runtime honors) or had a *runtime* prefix.

**The solution was not to open a third drawer.** It was realizing the first should not
be a gatekeeper: the list of forty names was the second copy of what the component
already declares, and maintaining two places is the flaw this repository has already paid for.

The evaluator of the unprefixed key became the **class**, and the rule became
stronger, not weaker — it catches what the global list never caught: a `Button`
with the `interval` property, which belongs to `Timer`.

This is in Amendment 1 of ADR 0031.

## What was fixed when bringing in `portadb`

The crate arrived with **four** failing tests, and all suffered from the same issue: the
in-memory driver **responded instead of rejecting**.

| defect | what happened |
|---|---|
| `ORDER BY`/`LIMIT` never applied | `ORDER BY version DESC LIMIT 1` returned the FIRST row |
| `DELETE` and `UPDATE` did not exist | fell into `Ok(0)` — *"zero rows affected"*, indistinguishable from *"found nothing"* |
| `SELECT` on non-existent table | returned empty, as an empty table would |
| `PRAGMA TABLE_INFO` | answered hardcoded `TEXT` for every column, `notnull` and `pk` always zero |

An in-memory test double does not need to be a full database. It needs **not to lie**: what
it does not support now fails with the command mentioned, instead of quietly slipping
away. The `PRAGMA` now answers what `CREATE TABLE` declared — which was already
being read and discarded one line later.

## Crates

| Crate | What it is |
|---|---|
| `portadb-core` | Types and contracts: values, rows, parameters, dialects, the driver SPI. Depends on nothing of CrawRAD. |
| `portadb-memory` | Provider: in-memory database, for forms that run without a server. |
| `portadb-rusqlite` | Provider: SQLite through `rusqlite`. |
| `portadb-sqlite` | Provider: SQLite through `sqlx`. |
| `portadb-postgres` | Provider: PostgreSQL through `sqlx`. |
| `portadb-redis` | Provider: Redis. |
| `portadb` | The engine applications use (`Engine`, `Connection`, pool, migrations, data views). Turns providers on by cargo feature and re-exports them. |
| `portadb-crawrad` | The CrawRAD glue: palette components (`Connection`, `DataSet`, engines). |
