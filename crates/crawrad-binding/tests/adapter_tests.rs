use crawrad_binding::{
    BindingMode, DatasetAdapter, DatasetState, ModelAdapter, Observable, PortaDbBridge,
    ScalarBinding,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
struct Product {
    sku: String,
    price: f64,
    in_stock: bool,
}

#[test]
fn test_model_adapter_with_scalar_bindings() {
    let adapter = ModelAdapter::new(Product {
        sku: "PROD-001".to_string(),
        price: 199.99,
        in_stock: true,
    });

    // 1. Projections from model
    let model_sku = adapter.bind_field(|p| p.sku.clone(), |p, v| p.sku = v);
    let model_in_stock = adapter.bind_field(|p| p.in_stock, |p, v| p.in_stock = v);

    // 2. Simulated UI controls
    let edit_sku = Observable::new(String::new());
    let checkbox_in_stock = Observable::new(false);

    // 3. Connect UI controls via TwoWay ScalarBinding
    let bind_sku = ScalarBinding::new_identity(model_sku, edit_sku.clone(), BindingMode::TwoWay);
    bind_sku.bind().unwrap();

    let bind_stock = ScalarBinding::new_identity(
        model_in_stock,
        checkbox_in_stock.clone(),
        BindingMode::TwoWay,
    );
    bind_stock.bind().unwrap();

    // UI loaded initial values
    assert_eq!(edit_sku.get(), "PROD-001");
    assert!(checkbox_in_stock.get());

    // User types in edit
    edit_sku.set("PROD-002-V2".to_string());
    assert_eq!(adapter.get().sku, "PROD-002-V2");

    // User clicks checkbox
    checkbox_in_stock.set(false);
    assert!(!adapter.get().in_stock);
}

#[test]
fn test_dataset_adapter_full_lifecycle_and_auto_edit() {
    let mut row1 = BTreeMap::new();
    row1.insert("code".to_string(), "C10".to_string());
    row1.insert("desc".to_string(), "Monitor 4K".to_string());

    let mut row2 = BTreeMap::new();
    row2.insert("code".to_string(), "C20".to_string());
    row2.insert("desc".to_string(), "Mechanical Keyboard".to_string());

    let dataset = DatasetAdapter::new(&["code", "desc"], vec![row1, row2]);

    assert_eq!(dataset.state().get(), DatasetState::Browse);
    assert_eq!(dataset.collection().len(), 2);

    let code_field = dataset.field("code").unwrap();
    let desc_field = dataset.field("desc").unwrap();

    assert_eq!(code_field.get(), "C10");
    assert_eq!(desc_field.get(), "Monitor 4K");

    // 1. Auto-edit test: editing desc_field in UI triggers Edit state
    desc_field.set("Monitor 4K OLED 144Hz".to_string());
    assert_eq!(dataset.state().get(), DatasetState::Edit);

    // 2. Cancel test: cancel reverts field to original snapshot
    dataset.cancel().unwrap();
    assert_eq!(dataset.state().get(), DatasetState::Browse);
    assert_eq!(desc_field.get(), "Monitor 4K");

    // 3. Edit and Post test
    desc_field.set("Monitor 5K Retina".to_string());
    assert_eq!(dataset.state().get(), DatasetState::Edit);
    dataset.post().unwrap();
    assert_eq!(dataset.state().get(), DatasetState::Browse);
    assert_eq!(
        dataset
            .collection()
            .current_item()
            .unwrap()
            .get("desc")
            .unwrap(),
        "Monitor 5K Retina"
    );

    // 4. Cursor movement in Grid updates Form Fields
    assert!(dataset.collection().next());
    assert_eq!(code_field.get(), "C20");
    assert_eq!(desc_field.get(), "Mechanical Keyboard");

    // 5. Insert test
    dataset.insert().unwrap();
    assert_eq!(dataset.state().get(), DatasetState::Insert);
    assert_eq!(code_field.get(), "");

    code_field.set("C30".to_string());
    desc_field.set("Ergonomic Mouse".to_string());
    dataset.post().unwrap();

    assert_eq!(dataset.state().get(), DatasetState::Browse);
    assert_eq!(dataset.collection().len(), 3);
    assert_eq!(code_field.get(), "C30");

    // 6. Delete test
    dataset.delete().unwrap();
    assert_eq!(dataset.collection().len(), 2);
}

#[cfg(feature = "portadb")]
#[test]
fn test_portadb_bridge_two_way_cursor_and_fields() {
    use portadb::{ColumnDef, DataSource, DataType, Row, TableSchema, Value};

    let schema = TableSchema::new()
        .with_column(ColumnDef::new("id", DataType::Integer, false))
        .with_column(ColumnDef::new("name", DataType::Text, false));

    let cols = vec!["id".to_string(), "name".to_string()];
    let r1 = Row::new(
        cols.clone(),
        vec![
            Value::Integer(101),
            Value::Text("Server Primary".to_string()),
        ],
    );
    let r2 = Row::new(
        cols,
        vec![
            Value::Integer(102),
            Value::Text("Server Backup".to_string()),
        ],
    );

    let datasource = DataSource::new(schema, vec![r1, r2]);
    let adapter = PortaDbBridge::from_datasource(&datasource);

    let name_field = adapter.field("name").unwrap();
    assert_eq!(name_field.get(), "Server Primary");

    // Grid row navigation on DataSource propagates to adapter fields
    datasource.select(1);
    assert_eq!(name_field.get(), "Server Backup");

    // Navigation on adapter collection propagates back to DataSource
    adapter.collection().set_cursor(Some(0));
    assert_eq!(datasource.position(), Some(0));
    assert_eq!(name_field.get(), "Server Primary");
}
