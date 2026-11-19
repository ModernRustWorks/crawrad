//! Tests for #[derive(ComponentItem)] and typed Collection<T> #[derive(Component)].

use crawrad_component::Collection;
use crawrad_component_derive::{Component, ComponentItem};
use serde_json::Value;

#[derive(ComponentItem, Debug, Clone, PartialEq)]
pub struct GridColumn {
    #[prop(display = "Title", category = "appearance", default = "Column")]
    pub title: String,

    #[prop(
        display = "Width",
        category = "layout",
        default = 80,
        min = 10,
        max = 2000
    )]
    pub width: i64,

    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,
}

impl Default for GridColumn {
    fn default() -> Self {
        Self {
            title: "Column".to_string(),
            width: 80,
            visible: true,
        }
    }
}

#[derive(Component, Debug, Clone, PartialEq, Default)]
#[crawrad(
    palette = "Grids",
    prefix = "grd",
    size = (200.0, 150.0),
    targets = ["*"]
)]
pub struct CustomGrid {
    #[prop(display = "Columns", category = "appearance")]
    pub columns: Collection<GridColumn>,
}

#[test]
fn component_item_emits_valid_json_with_props_and_class() {
    let raw = GridColumn::CRAWRAD_ITEM_JSON;
    let json: Value = serde_json::from_str(raw).expect("CRAWRAD_ITEM_JSON must be valid JSON");

    assert_eq!(json["class"], "GridColumn");

    let props = json["props"].as_array().expect("props must be an array");
    assert_eq!(props.len(), 3);

    let title_prop = &props[0];
    assert_eq!(title_prop["key"], "title");
    assert_eq!(title_prop["display"], "Title");
    assert_eq!(title_prop["category"], "appearance");
    assert_eq!(title_prop["kind"], "text");
    assert_eq!(title_prop["default"], "Column");

    let width_prop = &props[1];
    assert_eq!(width_prop["key"], "width");
    assert_eq!(width_prop["display"], "Width");
    assert_eq!(width_prop["category"], "layout");
    assert_eq!(width_prop["kind"], "int");
    assert_eq!(width_prop["default"], 80);
    assert_eq!(width_prop["min"], 10);
    assert_eq!(width_prop["max"], 2000);

    let visible_prop = &props[2];
    assert_eq!(visible_prop["key"], "visible");
    assert_eq!(visible_prop["display"], "Visible");
    assert_eq!(visible_prop["category"], "behavior");
    assert_eq!(visible_prop["kind"], "bool");
    assert_eq!(visible_prop["default"], true);
}

#[test]
fn component_with_typed_collection_infers_collection_kind_and_item_class() {
    let raw = CustomGrid::CRAWRAD_COMPONENT_JSON;
    let json: Value = serde_json::from_str(raw).expect("CRAWRAD_COMPONENT_JSON must be valid JSON");

    assert_eq!(json["class"], "CustomGrid");
    assert_eq!(json["palette"], "Grids");

    let props = json["props"].as_array().expect("props must be an array");
    let col_prop = props
        .iter()
        .find(|p| p["key"] == "columns")
        .expect("columns property must be present");

    assert_eq!(col_prop["display"], "Columns");
    assert_eq!(col_prop["category"], "appearance");
    assert_eq!(col_prop["kind"], "collection");
    assert_eq!(col_prop["item_class"], "GridColumn");
    assert_eq!(col_prop["default"], serde_json::json!([]));
}

#[test]
fn typed_collection_runtime_usage() {
    let mut grid = CustomGrid::default();
    assert!(grid.columns.is_empty());
    assert_eq!(grid.columns.len(), 0);

    grid.columns.push(GridColumn {
        title: "ID".to_string(),
        width: 50,
        visible: true,
    });
    grid.columns.push(GridColumn {
        title: "Name".to_string(),
        width: 150,
        visible: true,
    });

    assert_eq!(grid.columns.len(), 2);
    assert_eq!(grid.columns[0].title, "ID");
    assert_eq!(grid.columns[1].width, 150);

    let titles: Vec<&str> = grid.columns.iter().map(|c| c.title.as_str()).collect();
    assert_eq!(titles, vec!["ID", "Name"]);
}
