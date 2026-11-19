use crawrad_binding::{
    BindingMode, CollectionBinding, DisplayFromStrConverter, Observable, ScalarBinding,
};

#[derive(Debug, Clone, PartialEq)]
struct Customer {
    name: String,
    credit_limit: f64,
    active: bool,
}

#[test]
fn whole_model_projection_test() {
    let initial_customer = Customer {
        name: "ACME Corp".to_string(),
        credit_limit: 5000.0,
        active: true,
    };

    let model = Observable::new(initial_customer.clone());
    assert_eq!(model.get(), initial_customer);

    model.set(Customer {
        name: "Initech LLC".to_string(),
        credit_limit: 7500.0,
        active: false,
    });
    assert_eq!(model.get().name, "Initech LLC");
}

#[test]
fn form_customer_registration_two_way_sync() {
    // 1. Data model (Source)
    let model_name = Observable::new("ACME Corp".to_string());
    let model_credit = Observable::new(5000.0);
    let model_active = Observable::new(true);

    // 2. UI Widgets (Target)
    let edit_name_text = Observable::new(String::new());
    let edit_credit_text = Observable::new(String::new());
    let checkbox_active = Observable::new(false);

    // 3. Setup bindings
    let bind_name = ScalarBinding::new_identity(
        model_name.clone(),
        edit_name_text.clone(),
        BindingMode::TwoWay,
    );
    bind_name.bind().unwrap();

    let bind_credit = ScalarBinding::new(
        model_credit.clone(),
        edit_credit_text.clone(),
        BindingMode::TwoWay,
        DisplayFromStrConverter::<f64>::new(),
    );
    bind_credit.bind().unwrap();

    let bind_active = ScalarBinding::new_identity(
        model_active.clone(),
        checkbox_active.clone(),
        BindingMode::TwoWay,
    );
    bind_active.bind().unwrap();

    // Verification: Initial state loaded into UI
    assert_eq!(edit_name_text.get(), "ACME Corp");
    assert_eq!(edit_credit_text.get(), "5000");
    assert!(checkbox_active.get());

    // User edits the form in the UI
    edit_name_text.set("Globex Corporation".to_string());
    assert_eq!(model_name.get(), "Globex Corporation");

    edit_credit_text.set("12500.5".to_string());
    assert_eq!(model_credit.get(), 12500.5);

    checkbox_active.set(false);
    assert!(!model_active.get());

    // External business logic updates the model in background
    model_name.set("Initech LLC".to_string());
    assert_eq!(edit_name_text.get(), "Initech LLC");

    model_credit.set(20000.0);
    assert_eq!(edit_credit_text.get(), "20000");
}

#[derive(Debug, Clone, PartialEq)]
struct InvoiceRow {
    id: u64,
    description: String,
    total: f64,
}

#[test]
fn grid_collection_virtual_window_and_navigation() {
    let mut rows = Vec::new();
    for i in 1..=10_000 {
        rows.push(InvoiceRow {
            id: i,
            description: format!("Item #{i}"),
            total: (i as f64) * 1.5,
        });
    }

    let dataset = CollectionBinding::from_vec(rows);
    assert_eq!(dataset.len(), 10_000);
    assert_eq!(dataset.cursor(), Some(0));

    // First visible viewport in Makepad Grid (e.g. 20 rows per screen)
    let viewport1 = dataset.window(0, 20);
    assert_eq!(viewport1.len(), 20);
    assert_eq!(viewport1[0].id, 1);
    assert_eq!(viewport1[19].id, 20);

    // User scrolls rapidly down to row 5000
    let viewport_scrolled = dataset.window(5000, 20);
    assert_eq!(viewport_scrolled.len(), 20);
    assert_eq!(viewport_scrolled[0].id, 5001);
    assert_eq!(viewport_scrolled[19].id, 5020);

    // User clicks row 5005
    assert!(dataset.set_cursor(Some(5004)));
    assert_eq!(
        dataset.current_item(),
        Some(InvoiceRow {
            id: 5005,
            description: "Item #5005".to_string(),
            total: 5005.0 * 1.5,
        })
    );

    // Navigation buttons (Next / Prev / First / Last)
    assert!(dataset.next());
    assert_eq!(dataset.cursor(), Some(5005));

    assert!(dataset.first());
    assert_eq!(dataset.cursor(), Some(0));

    assert!(dataset.last());
    assert_eq!(dataset.cursor(), Some(9999));
}
