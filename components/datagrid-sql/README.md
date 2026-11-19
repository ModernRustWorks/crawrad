# datagrid-sql

A high-performance bridge crate connecting the [`datagrid`](../datagrid) columnar grid engine with the [`portadb`](../portadb) database abstraction layer.

`datagrid` and `portadb` remain completely decoupled and unaware of each other; `datagrid-sql` serves as the sole bridge implementing `datagrid::RowSource` over relational SQL databases (starting with SQLite via `rusqlite`).

---

## Architecture & Concurrency

- **Synchronous by Contract**: Following ADR 0069 and architecture decision #669, the `RowSource` trait is synchronous and `Send + Sync`. Data operations may block, and callers (such as virtual viewports and UI components) run them off the UI thread.
- **Dedicated Runtime**: `SqlSource` initializes a dedicated single-threaded `tokio::runtime::Runtime` upon instantiation and drives asynchronous `portadb::Connection` queries synchronously via `runtime.block_on(...)`.
- **Zero Per-Row Allocation Fetch Path**: Database result rows are decoded on-the-fly using `portadb::row_ref::RowVisitor` directly into `datagrid::column::Column` contiguous buffers (primitive vectors, bitsets, and text byte/offset arrays). No intermediate `Row` structs or heap values are allocated per row.

---

## SQL Translation & Injection Safety

All SQL translation in `datagrid-sql::translate` is pure and unit-tested without database dependencies:
- **Zero Literal Interpolation**: Every literal operand goes through a bound named parameter (`:p0`, `:p1`, etc.).
- **Identifier Quoting**: All table names, column names, and unique keys are escaped through `dialect.quote_identifier(...)`.
- **Malicious Text Literals**: Injection payloads such as `x'); DROP TABLE t; --` safely round-trip as data parameters without altering SQL statement syntax.

---

## Semantics & Engine Equivalence

The SQL translation is mathematically aligned with `datagrid`'s in-memory columnar engine:

| In-Memory Engine Feature | SQLite / SQL Translation | Semantic Details |
| :--- | :--- | :--- |
| **`Eq`, `Lt`, `Le`, `Gt`, `Ge`, `Between`, `In`** | `= :p`, `< :p`, `<= :p`, `> :p`, `>= :p`, `BETWEEN`, `IN` | Null cells never match, following both SQL three-valued logic and the engine. |
| **`Filter::Not(x)`** | `NOT COALESCE((x), 0)` | In the engine, `Not` performs a bitwise complement of the match bitmap, matching rows where the operand column is `NULL`. In SQL, `NOT (col = 5)` evaluates to `UNKNOWN` (falsy) for NULL cells. Wrapping with `COALESCE((x), 0)` maps NULL to 0 (false), which `NOT` flips to 1 (true), matching the engine. |
| **`Contains` / `StartsWith`** | `instr(col, :p) > 0` / `substr(col, 1, length(:p)) = :p` | Avoids SQL `LIKE` wildcard character escaping (`%`, `_`). |
| **`EqIgnoreCase`** | `lower(col) = lower(:p)` | Exact case-insensitive equality. See **Limits** below regarding Unicode vs ASCII case folding. |
| **Case Insensitive Search** | `instr(lower(col), lower(:p)) > 0` | See **Limits** below regarding Unicode vs ASCII case folding. |
| **Text Collation** | Default `BINARY` | The in-memory engine compares text slices byte-by-byte (`as_bytes().cmp`). SQLite's default `BINARY` collation performs standard byte-by-byte `memcmp`, providing identical sorting order. |
| **Boolean Columns** | `0` / `1` | Booleans are represented as `0` (false) and `1` (true) integers in SQLite. |
| **`ORDER BY` & Null Placement** | `(col IS NULL) DESC` (First)<br>`(col IS NULL) ASC` (Last) | Null placement is explicitly and portably written. The unique table key (e.g. `rowid`) is appended with `ASC` order as a unique tail to preserve deterministic store order for ties. |
| **Aggregate Totals** | `COUNT(*)`, `SUM(col)`, `AVG(col)`, `MIN(col)`, `MAX(col)` | `Count` translates to `COUNT(*)` to count every row including nulls (matching `update_accumulators`). Numerical aggregates sum non-null values and return `NULL` when all values are null. |

---

## Keyset Paging

To avoid expensive $O(N)$ linear table scans when scrolling deep into large datasets (`LIMIT ... OFFSET ...`), `SqlSource` implements transparent **keyset pagination**:
1. Maintains a bounded bookmark map: `(normalized filter + sort) -> BTreeMap<position, bookmark_tuple>`.
2. When a window fetch completes, the last row's sort keys plus unique table key are recorded for `range.start + rows.len`.
3. Subsequent fetches starting at a bookmarked position expand into a multi-column inequality:
   $$(C_0 \succ v_0) \lor (C_0 \equiv v_0 \land C_1 \succ v_1) \lor \dots$$
   respecting each column's `ASC`/`DESC` direction and `Nulls::First`/`Nulls::Last` rule.
4. Bookmarks are capped at 4,096 entries per normalized query; `position_of` and `locate` also leave one at the row they found.
5. When no NULL of the first sort column can come after the bookmark (a NOT NULL column, or nulls first), a redundant leading range `C0 >= v0` (`<=` for DESC) is added so SQLite seeks the index instead of walking it.

### Jumping anywhere without an OFFSET cliff (#674)

A window at position `p` with no bookmark exactly there is reached, cheapest first:
1. a bookmark `b <= p` at most one **stride** away: keyset from `b`, `OFFSET p - b`;
2. `p` within one stride of the **end**: the same rows read in the reversed order with a short OFFSET, then turned back — `last()` costs a first window plus the (cached) count;
3. `p` within one stride of the top: a short OFFSET;
4. otherwise the query's **checkpoints**: one ordered scan (`ROW_NUMBER() ... % stride`) keeps the key tuple of every stride-th row, built once per query, after which every jump is case 1.

stride = max(1024, count / 4096), so at most ~4,096 tuples per query. "Rows before a tuple" (for `position_of`) is the keyset predicate over the reversed sort keys. Counts are cached per filter for these decisions — like the bookmarks, this assumes the data does not change under the source (read-only, #674); `clear_cache()` forgets all three.

### Keeping the index usable

`ORDER BY` writes the `(col IS NULL)` placement term only when SQLite would not place nulls that way by itself (it sorts NULL first in ASC, last in DESC) and the column is nullable. That expression term stops SQLite from walking an index, so **declare NOT NULL columns as `nullable: false`** in the `Table`; ASC with nulls last on a nullable column costs a sort of the filtered rows.

### Measured (1,000,000 rows, file SQLite, index on the sort column, `benches/server.rs`)

| Operation | Time |
|---|---|
| `count` | 12.1 ms |
| first window (100 rows, sorted) | 0.12 ms |
| window at row 900,000 by keyset | 0.13 ms |
| `groups` level 0 with a sum | 482 ms |
| window at row 900,000, no bookmark (was 9.7 s by OFFSET) | 0.17 ms |
| `Cursor::last()`, cold | 14.6 ms |
| `Cursor::set_rec_no(500_000)`, cold (builds the checkpoints) | 556 ms |
| `set_rec_no` at random, checkpoints built | 4.6 ms |
| `locate` on an indexed column | 0.06 ms |
| `position_of` / `goto_bookmark` after a re-sort | 44 / 39 ms |

---

## Hierarchy Grouping

`SqlSource` implements `RowSource::groups(&self, query: &Query, parent: &[Option<Owned>])`:
- Evaluates `GROUP BY` at level `parent.len()` filtered by equality to the parent path (mapping `None` to `IS NULL`).
- Computes requested aggregate totals for that level.
- Orders groups by key with nulls first, matching the in-memory engine order.
- Group data rows are fetched on-demand using `Query::within_group(&parent_path)`.

---

## Limits & Differences

1. **Case-Folding Differences**:
   SQLite's built-in `lower()` function only folds ASCII characters (`A-Z` to `a-z`). In contrast, `datagrid`'s in-memory engine uses Rust's Unicode-aware `to_lowercase()`. For datasets with non-ASCII case conversions, `ignore_case` string matching may differ unless a custom SQLite ICU extension is loaded.
2. **Floating-Point NaN**:
   SQLite does not natively preserve IEEE `NaN` values (storing them as `NULL` or integers). Datasets intended for SQLite storage should replace `NaN` with `NULL` or valid real numbers.
3. **`position_of` is a COUNT**:
   the row's position is `COUNT(*)` of the rows before it, a range walk over the index (tens of ms at 1M rows). Fine for a bookmark or a locate; not for per-row use.
