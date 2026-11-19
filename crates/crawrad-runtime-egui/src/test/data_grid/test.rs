use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use crawrad_runtime::grid::{ColumnAlign, GridColumn};
use datagrid::{
    Capabilities, Column, ColumnInfo, ColumnKind, DisplayHint, Query, RowSource, Rows, SourceError,
    Store, VecSource,
};

use crate::data_grid::{
    DataGrid, PAGE_SIZE, compute_column_ranges, compute_column_widths, compute_fetch_request,
    compute_grid_column_widths, compute_group_indent, compute_loading_card_rect,
    compute_spinner_arc, rows_to_show,
};

const FLOAT_TOLERANCE: f32 = 1e-4;

/// A gate the test opens: until then every query waits, so "pending" lasts
/// as long as the test needs and no timing decides what it sees.
#[derive(Default)]
struct Gate {
    open: Mutex<bool>,
    opened: Condvar,
}

impl Gate {
    fn wait(&self) {
        let mut open = self.open.lock().expect("gate lock");
        while !*open {
            open = self.opened.wait(open).expect("gate wait");
        }
    }

    fn open(&self) {
        *self.open.lock().expect("gate lock") = true;
        self.opened.notify_all();
    }
}

struct GatedSource {
    inner: VecSource,
    gate: Arc<Gate>,
}

impl RowSource for GatedSource {
    fn schema(&self) -> &[ColumnInfo] {
        self.inner.schema()
    }

    fn capabilities(&self) -> Capabilities {
        self.inner.capabilities()
    }

    fn count(&self, query: &Query) -> Result<usize, SourceError> {
        self.gate.wait();
        self.inner.count(query)
    }

    fn fetch(&self, query: &Query, range: std::ops::Range<usize>) -> Result<Rows, SourceError> {
        self.gate.wait();
        self.inner.fetch(query, range)
    }
}

fn make_gated_source(num_rows: usize, gate: Arc<Gate>) -> Arc<dyn RowSource> {
    let mut store = Store::new();
    let id_vals: Vec<Option<i64>> = (0..num_rows as i64).map(Some).collect();
    let val_vals: Vec<Option<i64>> = (0..num_rows as i64).map(|i| Some(i * 10)).collect();
    store.add("id", Column::ints(id_vals)).expect("add id");
    store.add("val", Column::ints(val_vals)).expect("add val");

    let schema = vec![
        ColumnInfo {
            name: "id".into(),
            kind: ColumnKind::Int,
            nullable: false,
            hint: DisplayHint::Plain,
        },
        ColumnInfo {
            name: "val".into(),
            kind: ColumnKind::Int,
            nullable: false,
            hint: DisplayHint::Plain,
        },
    ];

    let vec_source = VecSource::new(store, schema).expect("VecSource");
    Arc::new(GatedSource {
        inner: vec_source,
        gate,
    })
}

#[test]
fn test_compute_column_widths_empty() {
    let widths = compute_column_widths(300.0, &[]);
    assert!(widths.is_empty());
}

#[test]
fn test_compute_column_widths_single() {
    let widths = compute_column_widths(320.0, &[None]);
    assert_eq!(widths.len(), 1);
    assert!((widths[0] - 320.0).abs() < FLOAT_TOLERANCE);
}

#[test]
fn test_compute_column_widths_multiple() {
    let total_w = 900.0;
    let widths = compute_column_widths(total_w, &[None, None, None]);
    assert_eq!(widths.len(), 3);

    let sum: f32 = widths.iter().sum();
    assert!(
        (sum - total_w).abs() < FLOAT_TOLERANCE,
        "sum was {}, expected ~{}",
        sum,
        total_w
    );

    assert!((widths[0] - 300.0).abs() < FLOAT_TOLERANCE);
    assert!((widths[1] - 300.0).abs() < FLOAT_TOLERANCE);
    assert!((widths[2] - 300.0).abs() < FLOAT_TOLERANCE);
}

#[test]
fn test_compute_column_widths_fixed_and_none() {
    let widths = compute_column_widths(300.0, &[Some(100.0), None, None]);
    assert_eq!(widths, vec![100.0, 100.0, 100.0]);

    // When fixed takes more than available, remainder is zero (never negative)
    let widths_overflow = compute_column_widths(50.0, &[Some(100.0), None]);
    assert_eq!(widths_overflow, vec![100.0, 0.0]);
}

#[test]
fn test_compute_grid_column_widths_with_min_max() {
    let mut c1 = GridColumn::new("c1", "C1", ColumnAlign::Left);
    c1.min_width = 120.0;
    let c2 = GridColumn::new("c2", "C2", ColumnAlign::Left);
    let c3 = GridColumn::new("c3", "C3", ColumnAlign::Left);
    let widths = compute_grid_column_widths(300.0, &[c1, c2, c3]);
    assert_eq!(widths.len(), 3);
    assert!((widths[0] - 120.0).abs() < FLOAT_TOLERANCE);
    assert!((widths[1] - 90.0).abs() < FLOAT_TOLERANCE);
    assert!((widths[2] - 90.0).abs() < FLOAT_TOLERANCE);
}

#[test]
fn test_compute_column_ranges() {
    let total_w = 300.0;
    let ranges = compute_column_ranges(total_w, &[None, None, None]);
    assert_eq!(ranges.len(), 3);

    assert!((ranges[0].0 - 0.0).abs() < FLOAT_TOLERANCE);
    assert!((ranges[0].1 - 100.0).abs() < FLOAT_TOLERANCE);

    assert!((ranges[1].0 - 100.0).abs() < FLOAT_TOLERANCE);
    assert!((ranges[1].1 - 200.0).abs() < FLOAT_TOLERANCE);

    assert!((ranges[2].0 - 200.0).abs() < FLOAT_TOLERANCE);
    assert!((ranges[2].1 - 300.0).abs() < FLOAT_TOLERANCE);
}

#[test]
fn test_compute_fetch_request() {
    // Unchanged window returns None
    assert_eq!(compute_fetch_request(0, 0, PAGE_SIZE), None);
    assert_eq!(compute_fetch_request(150, 150, PAGE_SIZE), None);

    // Changed window returns Some with new first and standard page count
    assert_eq!(
        compute_fetch_request(50, 0, PAGE_SIZE),
        Some((50, PAGE_SIZE))
    );
    assert_eq!(
        compute_fetch_request(200, 50, PAGE_SIZE),
        Some((200, PAGE_SIZE))
    );
}

#[test]
fn test_compute_group_indent() {
    assert!((compute_group_indent(0) - 8.0).abs() < FLOAT_TOLERANCE);
    assert!((compute_group_indent(1) - 24.0).abs() < FLOAT_TOLERANCE);
    assert!((compute_group_indent(2) - 40.0).abs() < FLOAT_TOLERANCE);
}

#[test]
fn test_compute_loading_card_rect() {
    let rows_rect = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(800.0, 600.0));
    let text_size = egui::vec2(120.0, 30.0);
    let spinner_size = 20.0;
    let padding = egui::vec2(18.0, 12.0);
    let gap = 12.0;

    let card_rect = compute_loading_card_rect(rows_rect, text_size, spinner_size, padding, gap);
    let expected_w = 18.0 * 2.0 + 20.0 + 12.0 + 120.0;
    let expected_h = 12.0 * 2.0 + 30.0;

    assert!((card_rect.width() - expected_w).abs() < FLOAT_TOLERANCE);
    assert!((card_rect.height() - expected_h).abs() < FLOAT_TOLERANCE);
    assert!((card_rect.center().x - rows_rect.center().x).abs() < FLOAT_TOLERANCE);
    assert!((card_rect.center().y - rows_rect.center().y).abs() < FLOAT_TOLERANCE);
}

#[test]
fn test_compute_spinner_arc() {
    let center = egui::pos2(100.0, 100.0);
    let radius = 10.0;
    let points = compute_spinner_arc(center, radius, 0.0, 16);
    assert_eq!(points.len(), 17);

    for p in points {
        let dist = p.distance(center);
        assert!(
            (dist - radius).abs() < FLOAT_TOLERANCE,
            "dist was {}, expected radius {}",
            dist,
            radius
        );
    }
}

#[test]
fn test_data_grid_headless_run() {
    let gate = Arc::new(Gate::default());
    let source = make_gated_source(1_000, Arc::clone(&gate));
    let columns = vec![
        GridColumn::new("id", "ID", ColumnAlign::Left),
        GridColumn::new("val", "Value", ColumnAlign::Right),
    ];
    let mut grid = DataGrid::new(source, columns);

    let ctx = egui::Context::default();
    // One frame, answering whether it asked to be repainted at once.
    let frame = |grid: &mut DataGrid| {
        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| grid.show(ui));
        // No renderer here: the font atlas upload is dropped on purpose.
        output.textures_delta.clear();
        output
            .viewport_output
            .get(&egui::ViewportId::ROOT)
            .is_some_and(|v| v.repaint_delay.is_zero())
    };

    // The gate is shut: the query the first frame sends stays pending. egui
    // asks for repaints of its own in the first frames, so only a frame well
    // past them tells whether the grid keeps asking.
    for _ in 0..10 {
        frame(&mut grid);
    }
    assert!(
        grid.is_pending(),
        "the query must still be waiting at the gate"
    );
    assert!(
        frame(&mut grid),
        "repainting must be requested while the worker query is pending"
    );

    gate.open();
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(5)
        && (grid.is_pending() || grid.total() != Some(1_000))
    {
        frame(&mut grid);
        std::thread::sleep(Duration::from_millis(5));
    }
    assert_eq!(grid.rec_no(), 0);
    assert_eq!(grid.total(), Some(1_000));
}

#[test]
fn rows_to_show_keeps_the_old_rows_while_a_new_query_is_counted() {
    assert_eq!(rows_to_show(None, 1_000_000), 1_000_000);
    assert_eq!(rows_to_show(Some(42), 1_000_000), 42);
    assert_eq!(rows_to_show(Some(0), 7), 0);
    assert_eq!(rows_to_show(None, 0), 0);
}

#[test]
fn test_data_grid_column_properties_defaults() {
    let col = GridColumn::new("field1", "Field 1", ColumnAlign::Left);
    assert!(!col.auto_size);
    assert!(col.resizable);
    assert!(col.movable);
}
