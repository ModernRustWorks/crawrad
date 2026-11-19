//! # Visual DataGrid Demo with 1,000,000 rows
//!
//! Demonstrates `CrawradDataGrid` rendering 1,000,000 rows virtualized with Makepad's `PortalList`.
//! A slow background source wrapper sleeps 300 ms on every `fetch` and `count`
//! to demonstrate the translucent loading card with its animated spinner arc.
//!
//! To run:
//! ```powershell
//! cargo run -p crawrad-runtime-makepad --example data_grid_demo
//! ```

use crawrad_runtime::grid::{ColumnAlign, GridColumn};
use crawrad_runtime_makepad::data_grid::*;
use datagrid::{
    Column, ColumnInfo, ColumnKind, DisplayHint, RowSource, Rows, SourceError, Store, VecSource,
};
pub use makepad_widgets;
use makepad_widgets::*;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

struct SlowSource {
    inner: VecSource,
    delay_ms: u64,
}

impl RowSource for SlowSource {
    fn schema(&self) -> &[ColumnInfo] {
        self.inner.schema()
    }

    fn capabilities(&self) -> datagrid::Capabilities {
        self.inner.capabilities()
    }

    fn count(&self, query: &datagrid::Query) -> Result<usize, SourceError> {
        sleep(Duration::from_millis(self.delay_ms));
        self.inner.count(query)
    }

    fn fetch(
        &self,
        query: &datagrid::Query,
        range: std::ops::Range<usize>,
    ) -> Result<Rows, SourceError> {
        sleep(Duration::from_millis(self.delay_ms));
        self.inner.fetch(query, range)
    }

    fn position_of(&self, query: &datagrid::Query, id: u64) -> Result<Option<usize>, SourceError> {
        self.inner.position_of(query, id)
    }

    fn locate(
        &self,
        query: &datagrid::Query,
        filter: &datagrid::Filter,
    ) -> Result<Option<(usize, u64)>, SourceError> {
        self.inner.locate(query, filter)
    }

    fn groups(
        &self,
        query: &datagrid::Query,
        parent: &[Option<datagrid::Owned>],
    ) -> Result<Vec<datagrid::SourceGroup>, SourceError> {
        self.inner.groups(query, parent)
    }
}

fn build_million_row_source() -> Arc<dyn RowSource> {
    const ROW_COUNT: usize = 1_000_000;
    let mut ids = Vec::with_capacity(ROW_COUNT);
    let mut customers = Vec::with_capacity(ROW_COUNT);
    let mut cities = Vec::with_capacity(ROW_COUNT);
    let mut totals = Vec::with_capacity(ROW_COUNT);

    let customer_list = [
        "Acme Corp",
        "Global Logistics",
        "Apex Solutions",
        "Starlight Industries",
        "Vertex Media",
        "Nexus Retail",
        "Blue Horizon Inc",
        "Omega Energy",
    ];

    let city_list = [
        "Curitiba",
        "São Paulo",
        "Rio de Janeiro",
        "Belo Horizonte",
        "Porto Alegre",
        "Florianópolis",
        "Salvador",
        "Brasília",
    ];

    for i in 0..ROW_COUNT {
        ids.push(Some((i + 1) as i64));
        customers.push(Some(customer_list[i % customer_list.len()].to_string()));
        cities.push(Some(city_list[(i / 7) % city_list.len()].to_string()));
        let total = ((i * 37 + 100) % 50_000) as f64 / 10.0 + 10.5;
        totals.push(Some(total));
    }

    let mut store = Store::new();
    store.add("id", Column::ints(ids)).expect("add id column");
    store
        .add("customer", Column::texts_auto(customers))
        .expect("add customer column");
    store
        .add("city", Column::texts_auto(cities))
        .expect("add city column");
    store
        .add("total", Column::floats(totals))
        .expect("add total column");

    let schema = vec![
        ColumnInfo {
            name: "id".to_string(),
            kind: ColumnKind::Int,
            nullable: false,
            hint: DisplayHint::Plain,
        },
        ColumnInfo {
            name: "customer".to_string(),
            kind: ColumnKind::Text,
            nullable: false,
            hint: DisplayHint::Plain,
        },
        ColumnInfo {
            name: "city".to_string(),
            kind: ColumnKind::Text,
            nullable: false,
            hint: DisplayHint::Plain,
        },
        ColumnInfo {
            name: "total".to_string(),
            kind: ColumnKind::Float,
            nullable: false,
            hint: DisplayHint::Plain,
        },
    ];

    let vec_source = VecSource::new(store, schema).expect("create VecSource");
    Arc::new(SlowSource {
        inner: vec_source,
        delay_ms: 300,
    })
}

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use crawrad_runtime_makepad::data_grid::CrawradDataGrid;

    App = {{App}} {
        ui: <Root> {
            window = <Window> {
                window: { inner_size: vec2(960, 720) },
                pass: { clear_color: #1e1f22 },
                body = <View> {
                    width: Fill, height: Fill,
                    flow: Down,
                    grid = <CrawradDataGrid> {}
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crawrad_runtime_makepad::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions) {}
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Startup = event {
            let source = build_million_row_source();
            let columns = vec![
                GridColumn::new("id", "Id", ColumnAlign::Left),
                GridColumn::new("customer", "Customer", ColumnAlign::Left),
                GridColumn::new("city", "City", ColumnAlign::Left),
                GridColumn::new("total", "Total", ColumnAlign::Right),
            ];
            self.ui
                .crawrad_data_grid(id!(grid))
                .set_source(cx, source, columns);
        }
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

fn main() {
    app_main();
}
