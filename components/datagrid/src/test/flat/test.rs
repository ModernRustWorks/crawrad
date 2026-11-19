use crate::column::Column;
use crate::filter::{Filter, Owned};
use crate::flat::{FlatView, GroupedFlat, Line, LineKey, LineKind, WindowedRows};
use crate::group::Total;
use crate::query::Query;
use crate::source::{Capabilities, FakeSlowSource, RowSource, SourceError, VecSource};
use crate::store::Store;
use crate::view::{GroupNode, View};
use std::sync::Arc;
use std::time::Duration;

struct XorShift64(u64);

impl XorShift64 {
    fn new(seed: u64) -> Self {
        let s = if seed == 0 { 0x853c49e6748fea9b } else { seed };
        Self(s)
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    fn next_range(&mut self, min: u64, max: u64) -> u64 {
        assert!(min <= max);
        let range = max - min + 1;
        min + (self.next_u64() % range)
    }

    fn next_bool(&mut self) -> bool {
        (self.next_u64() & 1) == 1
    }
}

fn naive_flatten(view: &View, show_footers: bool) -> Vec<Line> {
    if view.groups().is_empty() {
        return view
            .rows()
            .iter()
            .map(|&r| Line {
                kind: LineKind::Row,
                depth: 0,
                key: LineKey::Row(r as u64),
            })
            .collect();
    }

    let mut out = Vec::new();
    let num_levels = view.query().group_by.len();
    flatten_group_level(view.groups(), view, show_footers, num_levels, &mut out);
    out
}

fn flatten_group_level(
    nodes: &[GroupNode],
    view: &View,
    show_footers: bool,
    num_levels: usize,
    out: &mut Vec<Line>,
) {
    for node in nodes {
        out.push(Line {
            kind: LineKind::Header,
            depth: node.level,
            key: LineKey::Group(node.key.clone()),
        });

        if view.is_key_expanded(&node.key) {
            if !node.children.is_empty() {
                flatten_group_level(&node.children, view, show_footers, num_levels, out);
            } else {
                for &r in &node.rows {
                    out.push(Line {
                        kind: LineKind::Row,
                        depth: num_levels,
                        key: LineKey::Row(r as u64),
                    });
                }
            }

            if show_footers {
                out.push(Line {
                    kind: LineKind::Footer,
                    depth: node.level,
                    key: LineKey::Group(node.key.clone()),
                });
            }
        }
    }
}

fn create_test_store(rows: usize) -> Store {
    let mut store = Store::new();
    let depts = ["Engineering", "Marketing", "Sales", "Support"];
    let regions = ["US-East", "US-West", "EU-Central", "APAC"];

    let dept_col: Vec<Option<String>> = (0..rows)
        .map(|i| Some(depts[i % depts.len()].to_string()))
        .collect();
    let region_col: Vec<Option<String>> = (0..rows)
        .map(|i| Some(regions[(i / 2) % regions.len()].to_string()))
        .collect();
    let salary_col: Vec<Option<f64>> = (0..rows)
        .map(|i| Some(50_000.0 + ((i * 100) % 50_000) as f64))
        .collect();
    let id_col: Vec<Option<i64>> = (0..rows).map(|i| Some(i as i64)).collect();

    store.add("id", Column::ints(id_col)).unwrap();
    store.add("dept", Column::dict_from(dept_col)).unwrap();
    store.add("region", Column::dict_from(region_col)).unwrap();
    store.add("salary", Column::floats(salary_col)).unwrap();

    store
}

#[test]
fn test_grouped_flat_equals_naive_full_flattening_randomized() {
    let store = create_test_store(200);

    let base_query = Query::new()
        .group("dept")
        .with_total(Total::sum("salary"))
        .with_total(Total::count("salary"))
        .group("region")
        .with_total(Total::sum("salary"))
        .sort_asc("dept");

    for seed in 0..101 {
        let mut rng = XorShift64::new(seed);
        let show_footers = rng.next_bool();
        let view = View::new(&store, base_query.clone()).unwrap();
        let mut flat = GroupedFlat::new(view, show_footers);

        // Perform random expand/collapse actions
        let num_actions = rng.next_range(5, 20);
        for _ in 0..num_actions {
            let dept_idx = rng.next_range(0, 3) as usize;
            let target_subgroup = rng.next_bool();
            if target_subgroup {
                let reg_idx = rng.next_range(0, 3) as usize;
                let path = [dept_idx, reg_idx];
                let expand = rng.next_bool();
                flat.set_expanded(&path, expand);
            } else {
                let path = [dept_idx];
                let expand = rng.next_bool();
                flat.set_expanded(&path, expand);
            }
        }

        // Validate flat sequence against naive flattening
        let expected = naive_flatten(flat.view(), show_footers);
        assert_eq!(
            flat.len(),
            expected.len(),
            "Length mismatch at seed {}",
            seed
        );

        for (i, exp_line) in expected.iter().enumerate() {
            let actual = flat
                .line(i)
                .unwrap_or_else(|| panic!("flat.line({}) failed at seed {}", i, seed));
            assert_eq!(
                &actual, exp_line,
                "Line mismatch at index {} for seed {}",
                i, seed
            );

            match actual.kind {
                LineKind::Header | LineKind::Footer => {
                    assert!(flat.group_row_count(i).is_some());
                    assert!(flat.group_totals(i).is_some());
                    assert!(flat.is_expanded(i).is_some());
                }
                LineKind::Row => {
                    assert!(flat.group_row_count(i).is_none());
                    assert!(flat.group_totals(i).is_none());
                    assert!(flat.is_expanded(i).is_none());
                }
            }
        }

        // Test after re-sort
        let resort_query = base_query.clone().sort_desc("dept");
        flat.requery(&store, resort_query).unwrap();

        let expected_resort = naive_flatten(flat.view(), show_footers);
        assert_eq!(
            flat.len(),
            expected_resort.len(),
            "Length mismatch after re-sort at seed {}",
            seed
        );
        for (i, exp_line) in expected_resort.iter().enumerate() {
            let actual = flat.line(i).unwrap();
            assert_eq!(
                &actual, exp_line,
                "Line mismatch after re-sort at index {} for seed {}",
                i, seed
            );
        }
    }
}

#[test]
fn test_line_at_last_row_of_one_million_rows() {
    let n = 1_000_000;

    // 1. Ungrouped 1_000_000-row view
    let mut store = Store::new();
    let ids: Vec<Option<i64>> = (0..n).map(|i| Some(i as i64)).collect();
    store.add("id", Column::ints(ids)).unwrap();

    let view = View::new(&store, Query::new()).unwrap();
    let flat_ungrouped = GroupedFlat::new(view, false);
    assert_eq!(flat_ungrouped.len(), n);

    flat_ungrouped.reset_visited_nodes();
    let last_ungrouped = flat_ungrouped.line(n - 1).unwrap();
    assert_eq!(last_ungrouped.kind, LineKind::Row);
    assert_eq!(last_ungrouped.depth, 0);
    assert_eq!(last_ungrouped.key, LineKey::Row((n - 1) as u64));
    assert_eq!(flat_ungrouped.visited_nodes(), 0);

    // 2. Grouped 1_000_000-row view (2 levels, 8 siblings per level)
    let mut grouped_store = Store::new();
    let cat1: Vec<Option<String>> = (0..n).map(|i| Some(format!("c{}", i % 8))).collect();
    let cat2: Vec<Option<String>> = (0..n).map(|i| Some(format!("s{}", (i / 8) % 8))).collect();
    let val: Vec<Option<i64>> = (0..n).map(|i| Some(i as i64)).collect();

    grouped_store.add("cat1", Column::dict_from(cat1)).unwrap();
    grouped_store.add("cat2", Column::dict_from(cat2)).unwrap();
    grouped_store.add("val", Column::ints(val)).unwrap();

    let query = Query::new().group("cat1").group("cat2");
    let grouped_view = View::new(&grouped_store, query.clone()).unwrap();
    let mut flat_grouped = FlatView::new(grouped_view, true);
    flat_grouped.expand_all();

    let last_idx = flat_grouped.len() - 1;
    flat_grouped.reset_visited_nodes();
    let last_line = flat_grouped.line(last_idx).unwrap();
    assert_eq!(last_line.kind, LineKind::Footer);

    // Bounded by depth * (log2(siblings) + 2)
    // depth = 2, siblings = 8, log2(8) = 3 => bound = 2 * (3 + 2) = 10
    let bound = 2 * (3 + 2);
    assert!(
        flat_grouped.visited_nodes() <= bound,
        "visited {} > bound {}",
        flat_grouped.visited_nodes(),
        bound
    );

    // Also test with footers off (last line is a data row)
    let grouped_view_no_footers = View::new(&grouped_store, query).unwrap();
    let mut flat_no_footers = FlatView::new(grouped_view_no_footers, false);
    flat_no_footers.expand_all();

    let last_row_idx = flat_no_footers.len() - 1;
    flat_no_footers.reset_visited_nodes();
    let last_row_line = flat_no_footers.line(last_row_idx).unwrap();
    assert_eq!(last_row_line.kind, LineKind::Row);
    assert_eq!(last_row_line.depth, 2);
    assert!(
        flat_no_footers.visited_nodes() <= bound,
        "visited {} > bound {}",
        flat_no_footers.visited_nodes(),
        bound
    );
}

#[test]
fn test_collapse_expand_recomputes_only_path() {
    let rows = 1000;
    let mut store = Store::new();
    let cat1: Vec<Option<String>> = (0..rows).map(|i| Some(format!("c{}", i % 6))).collect();
    let cat2: Vec<Option<String>> = (0..rows)
        .map(|i| Some(format!("s{}", (i / 6) % 5)))
        .collect();
    let val: Vec<Option<i64>> = (0..rows).map(|i| Some(i as i64)).collect();

    store.add("cat1", Column::dict_from(cat1)).unwrap();
    store.add("cat2", Column::dict_from(cat2)).unwrap();
    store.add("val", Column::ints(val)).unwrap();

    let query = Query::new().group("cat1").group("cat2");
    let view = View::new(&store, query).unwrap();
    let mut flat = GroupedFlat::new(view, true);
    flat.expand_all();

    // 1. Collapse top-level group 0
    // Sibling list size on path = root nodes count (6)
    flat.reset_prefix_sums_rewritten();
    flat.set_expanded(&[0], false);
    let rewritten_top = flat.prefix_sums_rewritten();
    assert!(
        rewritten_top <= 6,
        "rewritten {} > 6 for top-level collapse",
        rewritten_top
    );

    // Re-expand top-level group 0
    flat.reset_prefix_sums_rewritten();
    flat.set_expanded(&[0], true);
    assert!(flat.prefix_sums_rewritten() <= 6);

    // 2. Collapse subgroup [1, 2]
    // Sibling list sizes on path = group 1's children (5) + root nodes (6) = 11
    flat.reset_prefix_sums_rewritten();
    flat.set_expanded(&[1, 2], false);
    let rewritten_sub = flat.prefix_sums_rewritten();
    assert!(
        rewritten_sub <= 11,
        "rewritten {} > 11 for subgroup collapse",
        rewritten_sub
    );

    // Toggle back
    let line_of_sub = flat
        .line_of_key(&LineKey::Group(
            flat.view().groups()[1].children[2].key.clone(),
        ))
        .unwrap();
    flat.reset_prefix_sums_rewritten();
    assert!(flat.toggle(line_of_sub));
    assert!(flat.prefix_sums_rewritten() <= 11);
}

#[test]
fn test_line_of_key_stable_after_resort_and_expand_collapse() {
    let store = create_test_store(100);
    let query = Query::new().group("dept").group("region").sort_asc("dept");

    let view = View::new(&store, query.clone()).unwrap();
    let mut flat = GroupedFlat::new(view, true);
    flat.expand_all();

    let target_row_id = 42;
    let target_group_key = LineKey::Group(flat.view().groups()[2].key.clone());

    let row_idx_initial = flat.line_of_key(&LineKey::Row(target_row_id)).unwrap();
    let grp_idx_initial = flat.line_of_key(&target_group_key).unwrap();

    assert_eq!(
        flat.line(row_idx_initial).unwrap().key,
        LineKey::Row(target_row_id)
    );
    assert_eq!(flat.line(grp_idx_initial).unwrap().key, target_group_key);

    // Collapse group 0 (preceding group 2 and row 42)
    flat.set_expanded(&[0], false);
    let row_idx_after_collapse = flat.line_of_key(&LineKey::Row(target_row_id)).unwrap();
    let grp_idx_after_collapse = flat.line_of_key(&target_group_key).unwrap();

    assert!(row_idx_after_collapse < row_idx_initial);
    assert!(grp_idx_after_collapse < grp_idx_initial);
    assert_eq!(
        flat.line(row_idx_after_collapse).unwrap().key,
        LineKey::Row(target_row_id)
    );
    assert_eq!(
        flat.line(grp_idx_after_collapse).unwrap().key,
        target_group_key
    );

    // Re-sort
    let resort_query = Query::new().group("dept").group("region").sort_desc("dept");
    flat.requery(&store, resort_query).unwrap();
    flat.expand_all();

    let row_idx_resort = flat.line_of_key(&LineKey::Row(target_row_id)).unwrap();
    let grp_idx_resort = flat.line_of_key(&target_group_key).unwrap();

    assert_eq!(
        flat.line(row_idx_resort).unwrap().key,
        LineKey::Row(target_row_id)
    );
    assert_eq!(flat.line(grp_idx_resort).unwrap().key, target_group_key);
}

#[test]
fn test_windowed_rows_over_fake_slow_source() {
    let total_rows = 10_000;
    let mut store = Store::new();
    let ids: Vec<Option<i64>> = (0..total_rows).map(|i| Some(i as i64)).collect();
    let vals: Vec<Option<f64>> = (0..total_rows).map(|i| Some(i as f64 * 1.5)).collect();

    store.add("id", Column::ints(ids)).unwrap();
    store.add("val", Column::floats(vals)).unwrap();

    let vec_source = VecSource::from_store(store);
    let slow_source = FakeSlowSource::new(Box::new(vec_source), Duration::from_millis(0));

    let page_size = 100;
    let max_pages = 5;
    let query = Query::new();

    let slow_arc = Arc::new(slow_source);
    let mut windowed = WindowedRows::new(
        slow_arc.clone() as Arc<dyn RowSource>,
        query,
        page_size,
        max_pages,
    );

    assert_eq!(windowed.len().unwrap(), total_rows);
    let total_pages = total_rows / page_size;
    assert_eq!(total_pages, 100);

    // 1. Scroll page by page through 10 000 rows
    for p in 0..total_pages {
        let line_idx = p * page_size;
        let (line, rows, offset) = windowed.line(line_idx).unwrap().unwrap();
        assert_eq!(line.kind, LineKind::Row);
        assert_eq!(line.depth, 0);
        assert_eq!(offset, 0);
        assert_eq!(line.key, LineKey::Row(line_idx as u64));
        assert_eq!(rows.ids[offset], line_idx as u64);
    }

    let fetches = windowed.fetch_calls();
    assert!(
        fetches <= total_pages + 2,
        "fetches {} > total_pages + 2 ({})",
        fetches,
        total_pages + 2
    );

    // 2. Re-reading a visible slice performs 0 fetches
    let fetches_before_reread = windowed.fetch_calls();
    for i in 9900..10_000 {
        let (line, _, _) = windowed.line(i).unwrap().unwrap();
        assert_eq!(line.key, LineKey::Row(i as u64));
    }
    assert_eq!(windowed.fetch_calls(), fetches_before_reread);

    // 3. Jumping to the end performs at most 3 fetches
    windowed.invalidate();
    let _ = windowed.line(0).unwrap().unwrap();
    let fetches_before_jump = windowed.fetch_calls();
    let (end_line, _, _) = windowed.line(9999).unwrap().unwrap();
    assert_eq!(end_line.key, LineKey::Row(9999));
    let jump_fetches = windowed.fetch_calls() - fetches_before_jump;
    assert!(
        jump_fetches <= 3,
        "jump fetches {} exceeded bound 3",
        jump_fetches
    );

    // 4. max_pages is never exceeded
    assert!(windowed.cached_pages_count() <= max_pages);
    let _ = windowed.ensure(0..500);
    assert!(windowed.cached_pages_count() <= max_pages);
    // 5. Every fetch is at most page_size long
    assert!(slow_arc.stats().max_window_requested <= page_size);

    // 6. set_query makes the next read re-count once
    let initial_counts = slow_arc.stats().count_calls;
    windowed.set_query(Query::new().sort_asc("id"));
    assert_eq!(slow_arc.stats().count_calls, initial_counts);
    let count = windowed.len().unwrap();
    assert_eq!(count, total_rows);
    assert_eq!(slow_arc.stats().count_calls, initial_counts + 1);
}

#[test]
fn test_vec_source_ids_equal_store_row_ids() {
    let mut store = Store::new();
    let n = 20;
    let ids: Vec<Option<i64>> = (0..n).map(|i| Some(i as i64)).collect();
    let scores: Vec<Option<f64>> = (0..n).map(|i| Some((n - i) as f64)).collect();

    store.add("id", Column::ints(ids)).unwrap();
    store.add("score", Column::floats(scores)).unwrap();

    let source = VecSource::from_store(store);
    let query = Query::new().sort_asc("score");

    let rows = source.fetch(&query, 5..15).unwrap();
    assert_eq!(rows.len, 10);

    // Store row ids corresponding to ascending score (lowest score is at store index n-1)
    let expected_store_ids: Vec<u64> = (5..15).map(|rank| (n - 1 - rank) as u64).collect();
    assert_eq!(rows.ids, expected_store_ids);
}

#[test]
fn test_windowed_rows_reports_a_failing_source_instead_of_an_empty_grid() {
    let store = create_test_store(50);
    let no_filter = FakeSlowSource::with_capabilities(
        Box::new(VecSource::from_store(store)),
        Duration::from_millis(0),
        Capabilities {
            filter: false,
            sort: true,
            group: true,
            totals: true,
            keyset: false,
        },
    );
    let query = Query::new().filter(Filter::Gt("id".into(), Owned::Int(10)));
    let mut windowed = WindowedRows::new(Arc::new(no_filter) as Arc<dyn RowSource>, query, 10, 4);

    assert!(matches!(
        windowed.len(),
        Err(SourceError::Unsupported("filter"))
    ));
    assert!(windowed.line(0).is_err());
    assert!(windowed.peek(0).is_none());
}

#[test]
fn test_windowed_rows_prefetches_the_page_before_a_jump() {
    let store = create_test_store(1_000);
    let source = Arc::new(VecSource::from_store(store)) as Arc<dyn RowSource>;
    let mut windowed = WindowedRows::new(source, Query::new().sort_asc("id"), 100, 8);

    // Jump to page 5: pages 4 (before) and 6 (after) come with it
    windowed.ensure(550..560).unwrap();
    let calls = windowed.fetch_calls();
    assert_eq!(calls, 3);
    assert!(windowed.peek(450).is_some());
    assert!(windowed.peek(650).is_some());
    assert!(windowed.peek(350).is_none());

    // Scrolling back one page is already served
    windowed.ensure(450..460).unwrap();
    assert!(windowed.fetch_calls() <= calls + 1);
}
