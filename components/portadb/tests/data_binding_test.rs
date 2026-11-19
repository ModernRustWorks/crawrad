use portadb::{DataSource, FieldDef, FieldType, Row, SchemaDescriptor, Value};
use std::sync::{Arc, Mutex};

/// Fake visual Grid component observing a shared DataSource.
struct FakeGrid {
    source: DataSource,
    last_rendered_version: u64,
    rendered_row_count: usize,
}

impl FakeGrid {
    fn new(source: DataSource) -> Self {
        let version = source.version();
        let count = source.row_count();
        Self {
            source,
            last_rendered_version: version,
            rendered_row_count: count,
        }
    }

    fn select_row(&self, index: usize) -> bool {
        self.source.move_to(index)
    }

    fn current_selected_index(&self) -> Option<usize> {
        self.source.position()
    }

    fn current_row(&self) -> Option<Row> {
        self.source.current_row()
    }

    fn render(&mut self) {
        // Immediate-mode pull check
        if self.source.version() != self.last_rendered_version {
            self.last_rendered_version = self.source.version();
            self.rendered_row_count = self.source.row_count();
        }
    }
}

/// Fake visual Detail Form observing the same DataSource.
struct FakeDetailForm {
    source: DataSource,
    current_title: Arc<Mutex<String>>,
    current_price: Arc<Mutex<f64>>,
}

impl FakeDetailForm {
    fn new(source: DataSource) -> Self {
        let initial_title = source.get_field::<String>("title").unwrap_or_default();
        let initial_price = source.get_field::<f64>("price").unwrap_or_default();

        let current_title = Arc::new(Mutex::new(initial_title));
        let current_price = Arc::new(Mutex::new(initial_price));

        let title_clone = Arc::clone(&current_title);
        let price_clone = Arc::clone(&current_price);
        let src_clone = source.clone();

        // Push-based listener registration
        source.add_listener(move |_signal| {
            if let Ok(title) = src_clone.get_field::<String>("title") {
                *title_clone.lock().unwrap() = title;
            }
            if let Ok(price) = src_clone.get_field::<f64>("price") {
                *price_clone.lock().unwrap() = price;
            }
        });

        Self {
            source,
            current_title,
            current_price,
        }
    }

    fn title(&self) -> String {
        self.current_title.lock().unwrap().clone()
    }

    fn price(&self) -> f64 {
        *self.current_price.lock().unwrap()
    }

    fn source(&self) -> &DataSource {
        &self.source
    }
}

#[test]
fn test_fake_grid_and_detail_form_stay_in_sync() {
    // 1. Setup design-time schema descriptor
    let schema = SchemaDescriptor::new()
        .with_field(FieldDef::new("id", FieldType::Integer, false))
        .with_field(FieldDef::new("title", FieldType::Text, false))
        .with_field(FieldDef::new("price", FieldType::Real, false));

    // 2. Populate dataset rows
    let columns = vec!["id".to_string(), "title".to_string(), "price".to_string()];
    let rows = vec![
        Row::new(
            columns.clone(),
            vec![
                Value::Integer(1),
                Value::Text("Rust Book".into()),
                Value::Real(49.99),
            ],
        ),
        Row::new(
            columns.clone(),
            vec![
                Value::Integer(2),
                Value::Text("Wireless Mouse".into()),
                Value::Real(29.95),
            ],
        ),
        Row::new(
            columns.clone(),
            vec![
                Value::Integer(3),
                Value::Text("Mechanical Keyboard".into()),
                Value::Real(119.00),
            ],
        ),
    ];

    // 3. Create shared observable data source
    let source = DataSource::new(schema, rows);

    // 4. Attach two distinct fake UI controls to the SAME data source
    let mut grid = FakeGrid::new(source.clone());
    let detail_form = FakeDetailForm::new(source.clone());

    // Initially, both controls must observe row 0
    assert_eq!(detail_form.source().row_count(), 3);
    assert_eq!(grid.current_selected_index(), Some(0));
    assert_eq!(detail_form.title(), "Rust Book");
    assert!((detail_form.price() - 49.99).abs() < f64::EPSILON);

    // Action A: Moving row in Grid moves the Detail Form in lockstep
    assert!(grid.select_row(2));
    grid.render();

    assert_eq!(grid.current_selected_index(), Some(2));
    assert_eq!(
        grid.current_row().unwrap().get::<String>("title").unwrap(),
        "Mechanical Keyboard"
    );
    // Detail Form updated automatically via change signal!
    assert_eq!(detail_form.title(), "Mechanical Keyboard");
    assert!((detail_form.price() - 119.00).abs() < f64::EPSILON);

    // Action B: Navigating backward via source
    assert!(source.prev());
    grid.render();

    assert_eq!(grid.current_selected_index(), Some(1));
    assert_eq!(detail_form.title(), "Wireless Mouse");
    assert!((detail_form.price() - 29.95).abs() < f64::EPSILON);

    // Out of bounds navigation returns false
    assert!(!grid.select_row(99));
    assert_eq!(grid.current_selected_index(), Some(1));
}
