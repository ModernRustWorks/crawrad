# `datagrid` — Columnar In-Memory Store

`datagrid` is a zero-dependency, pure Rust columnar in-memory data store engineered for large-volume tables and interactive data grids. It provides ultra-fast multi-key sorting, predicate filtering, multi-level hash grouping with aggregate totals, and point/range lookups directly over row identifiers.

Like `portadb`, `datagrid` is completely decoupled and standalone: it has no knowledge of higher-level GUI shells or IDE layers.

---

## Key Architecture & Techniques

### 1. Columnar Memory Layout
Columns are stored as contiguous arrays (`Vec<i64>`, `Vec<f64>`, bitset `Bitmap`, or byte buffers with offsets) rather than row objects.
- Cache-friendly linear scans during predicate evaluation.
- Vectorizable loops without per-row pointer chasing.
- Optional validity bitmasks (`Bitmap`) isolate null tracking without wasting payload bytes.

### 2. Sorted Dictionary Encoding
Text columns with repetitive entries (`Column::Dict`) store string values once in a sorted dictionary `Vec<String>` and represent cell contents as dense `u32` codes.
- `codes[row]` indexes `dict`.
- Because `dict` is sorted lexicographically, string ordering strictly equals integer code ordering (`code_a < code_b <=> text_a < text_b`).
- Predicates like `Eq`, `In`, `Contains`, and `StartsWith` evaluate once per dictionary entry rather than once per row.
- `Column::texts_auto` dynamically picks `Dict` when distinct values $\le \text{rows} / 4$, falling back to plain offset-based text otherwise.

### 3. Row-ID Permutations (Zero Row Movement)
Sorting, filtering, and grouping never copy or move row contents.
- Filter operations produce dense `Bitmap` sets of matching row IDs.
- Sort operations take a slice `&[u32]` of row selections and return a new permutation `Vec<u32>` of the same identifiers.

### 4. LSD Radix Sort by Keys
Multi-column sorting executes an LSD (Least Significant Digit) radix sort over the key sequence (from the last key backwards to the first key):
- Each pass is an 8-pass stable radix sort over that key's `u64` ordinal, skipping byte passes that are constant for all rows (e.g. 500-entry dictionaries skip 6 out of 8 passes).
- **Null partitioning**: each pass partitions null rows apart from non-null values, sorting non-null values across the full `u64` range, and placing nulls first or last per `Nulls`.
- **Text prefix runs**: plain `Text` columns perform the radix pass on their 8-byte big-endian prefix, then fix contiguous runs of identical prefixes using a stable comparison sort on full text bytes.
- Stability across passes yields the complete multi-key lexicographic order with row-order stability on ties.

### 5. Dense Mixed-Radix Grouping
When all grouping columns are discrete types (`Dict` or `Bool`) and their combined cardinality $\le 4 \times \text{rows}$:
- Computes a mixed-radix group index `idx = ((k1 * card2 + k2) * card3 + ...)` mapping directly into flat vectors.
- Eliminates hash maps, hash computations, and collisions entirely.
- Groups and totals accumulate in a single pass.
- Row lists are assembled via a counting sort pass, preserving input row order.
- Automatically falls back to the hash grouping path when cardinality exceeds the threshold or continuous types (`Int`, `Float`) are grouped.

### 6. Bitmap Validity & Filtering
`Bitmap` implements a dense bitset over `Vec<u64>` supporting bitwise `and`, `or`, and `not`.
- Predicates evaluate directly against column vectors.
- Validity masks combine via bitwise operations in blocks of 64 rows, adhering strictly to SQL 3-valued comparison semantics (comparisons never match nulls; only `IsNull` does).

### 7. Lazy Indexes (`Locator`)
Indexes are computed on demand and cached per column:
- `find_eq`: builds an exact-match hash index on first access, returning the first matching row in $O(1)$. Float hashes canonicalize `-0.0` to `0.0` and treat all NaNs as a single matching key.
- `range`: builds a sorted non-null permutation on first access and resolves `[lo, hi]` queries using two binary searches (`partition_point`) in $O(\log N + K)$.
- `clear`: frees cached indexes when memory needs to be reclaimed.

### 8. Hierarchical Views & Incremental Requery (`View`)
`View` models an interactive query result over a `Store`, integrating filtering, multi-key sorting, and multi-level group trees (`GroupNode`):
- **Group Hierarchy**: groups level by level, maintaining sorted row order inside groups. `SortKey` on a grouping column with `Order::Desc` reverses that level's group order.
- **Persistent Expand/Collapse State**: group expansion is tracked by key path (`Vec<Option<u64>>`) rather than volatile row indexes, surviving `requery` calls.
- **Incremental Requery**: compares new and old queries to reuse intermediate results (reuses filtered rows when filter is identical, reuses sorted rows when filter and sort are identical, and regroups only when grouping changes).
- **Execution Observability**: `view.reuse_stats()` tracks cumulative runs (`filter_runs`, `sort_runs`, `group_runs`).

### 9. Pluggable Row Sources (`RowSource`)
The `RowSource` trait decouples higher-level presentation layers from concrete data origins, establishing a uniform windowed contract across in-memory and external backends:
- **Windowed Retrieval**: Consuming layers request only a slice/window of rows (`fetch(&query, range)`) and matching totals (`count(&query)`), eliminating whole-dataset loading.
- **Off-Thread Synchronous Contract**: The trait is synchronous and `Send + Sync`; blocking calls run off the UI thread without forcing an async executor on callers.
- **In-Memory Caching (`VecSource`)**: Wraps a `Store`, memoizing filtered and sorted permutations across scrolling windows. Uses `Column::gather` to build each window column with one buffer (a dictionary column copies only the entries the window uses).
- **Latency & Telemetry Simulation (`FakeSlowSource`)**: Available under tests and `test-sources`, injecting configurable delay, asserting window size limits, and validating graceful failure on unsupported backend capabilities.

### 10. Query Model as Plain Data (`Query`, `Filter`, `GroupLevel`)
The query model is a complete, serializable, comparable, and hashable contract between the screen and data engines — containing no SQL strings and no closures:
- **Builder API**: Fluent composition for predicates, sort keys, and group hierarchies:
  ```rust
  let query = Query::new()
      .filter(Filter::gt("score", 30.0).and(!Filter::is_null("tag")))
      .sort_asc("score")
      .sort_desc_nulls_first("name")
      .group("region")
      .with_total(Total::sum("amount"))
      .group("city");
  ```
- **Per-Level Totals**: Grouping hierarchy is modeled as `Vec<GroupLevel>` where each level defines its own column and aggregate totals (`pub struct GroupLevel { pub column: String, pub totals: Vec<Total> }`).
- **Normalisation**: `Query::normalized(&self) -> Query` (and `Filter::normalized`) canonicalizes query representations idempotently:
  - Flattens nested `And` inside `And` and `Or` inside `Or`.
  - Simplifies single-child `And` / `Or` to the child itself.
  - Simplifies double negation `Not(Not(x))` to `x`.
  - Sorts `In` literal sets by canonical total order (`Owned::canonical_cmp`) and deduplicates values.
  - Drops exact duplicate children in `And` / `Or` while preserving first-occurrence order.
  - Keeps the first sort key for repeated columns and drops subsequent ones.
- **Serialization (`serde` Feature)**: With the optional `serde` feature enabled, all query types implement `Serialize` and `Deserialize` to/from readable JSON (e.g., `Owned::Int(5)` serializes as `{"Int": 5}`). JSON has no NaN or infinity: a query holding such a float literal does not round-trip through JSON.
- **Canonical Equality and Hashing**: Floats in `Owned` compare and hash by canonical bit patterns (`-0.0` maps to `0.0`, all NaNs map to a canonical NaN), ensuring `Owned`, `Filter`, and `Query` consistently satisfy `Eq` and `Hash`.

### 11. Flat View & Window Cache (`GroupedFlat`, `WindowedRows`)
To drive high-performance GUI virtual grids without whole-dataset materialization:
- **Unified Flat Sequence (`GroupedFlat`, `Line`)**: Projects hierarchical grouped views into a single flat sequence of lines (`Header`, `Row`, `Footer`).
  - Rows and groups carry stable keys (`LineKey::Row(u64)`, `LineKey::Group(Vec<Option<u64>>)`), enabling selection and scroll preservation across re-sorts via `line_of_key(&key)`.
  - Headers and footers expose group metadata (`row_count`, aggregate `totals`, `is_expanded`) by flat line index without per-line memory allocation.
  - Footers are optionally enabled via `show_footers` for expanded groups.
- **Prefix-Sum Tree Indexing**:
  - `len()` runs in $O(1)$ by reading the cached root visible size.
  - `line(i)` descends using `partition_point` on prefix-sum arrays at each level in $O(\text{depth} \cdot \log \text{siblings})$, jumping directly to line 900,000 without touching earlier lines 0..899,999.
  - Zero heap allocation on the data row read path.
- **Path-Only Recomputation**:
  - Expanding or collapsing a group via `set_expanded(path, bool)` or `toggle(line_index)` updates the node and recomputes only prefix sums along the path to the root. The cost is $O(\sum \text{sibling list sizes on path})$ rather than $O(N)$ for the whole tree.
- **Windowed Page Cache (`WindowedRows`)**:
  - Wraps any `RowSource` with an LRU cache of `max_pages` pages (each of `page_size` lines).
  - `ensure(range)` fetches missing pages overlapping `range` plus one prefetch page before and after when available (blocking, intended for off-UI threads).
  - `peek(i)` borrows from cached pages without fetching; `line(i)` ensures the target page and peeks.
  - Never evicts the page currently being returned or read.
  - Requerying via `set_query` clears pages and invalidates the cached count.
  - `len()` and `line(i)` return `Result`: a failing source is reported, never shown as an empty grid.

---

## Usage Example

```rust
use datagrid::{
    Aggregate, Column, Filter, GroupLevel, GroupedFlat, LineKind, Locator, Nulls, Order, Owned,
    Query, SortKey, Store, Total, View,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut store = Store::new();

    // 1. Add columns
    store.add(
        "id",
        Column::ints(vec![Some(101), Some(102), Some(103), Some(104)]),
    )?;
    store.add(
        "city",
        Column::dict_from(vec![
            Some("Berlin".into()),
            Some("Tokyo".into()),
            Some("Berlin".into()),
            Some("Paris".into()),
        ]),
    )?;
    store.add(
        "amount",
        Column::floats(vec![Some(250.0), Some(120.5), Some(80.0), None]),
    )?;

    // 2. Filter rows (amount > 100.0)
    let predicate = Filter::gt("amount", 100.0);
    let matching_bitmap = store.filter(&predicate)?;
    let filtered_rows: Vec<u32> = matching_bitmap.ones().collect();

    // 3. Sort matching rows by city Ascending, then amount Descending
    let sorted_rows = store.sort(
        &filtered_rows,
        &[
            SortKey {
                column: "city".into(),
                order: Order::Asc,
                nulls: Nulls::Last,
            },
            SortKey {
                column: "amount".into(),
                order: Order::Desc,
                nulls: Nulls::Last,
            },
        ],
    )?;

    // 4. Group by city with Sum and Count totals
    let all_rows: Vec<u32> = (0..store.rows() as u32).collect();
    let totals = vec![
        Total::sum("amount"),
        Total::count("amount"),
    ];
    let groups = store.group(&all_rows, &["city".into()], &totals)?;

    for grp in groups {
        let city_name = store.key_value("city", grp.key[0])?;
        println!(
            "Group: {:?}, Rows: {:?}, Sum: {:?}, Count: {:?}",
            city_name, grp.rows, grp.totals[0], grp.totals[1]
        );
    }

    // 5. Lazy locate
    let mut locator = Locator::new();
    let first_berlin = locator.find_eq(&store, "city", &Owned::Text("Berlin".into()))?;
    assert_eq!(first_berlin, Some(0));

    // 6. Interactive View and GroupedFlat for screen rendering
    let query = Query::new()
        .filter(Filter::gt("amount", 100.0))
        .sort_desc("amount")
        .group("city")
        .with_total(Total::sum("amount"));
    let view = View::new(&store, query)?;
    let mut flat = GroupedFlat::new(view, true);
    flat.expand_all();

    for i in 0..flat.len() {
        let line = flat.line(i).unwrap();
        match line.kind {
            LineKind::Header => println!("Header at line {}: depth {}", i, line.depth),
            LineKind::Row => println!("Data row at line {}: key {:?}", i, line.key),
            LineKind::Footer => println!("Footer at line {}: depth {}", i, line.depth),
        }
    }

    Ok(())
}
```
