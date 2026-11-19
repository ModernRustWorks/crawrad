//! Tests for `crawrad_project::new_component` pure model.

use super::*;
use crate::fixture;
use crate::installed::ComponentScope;

#[test]
fn validate_refuses_invalid_component_names() {
    let existing = vec!["Button".to_string(), "Edit".to_string()];

    // Empty name
    let draft = ComponentDraft::new("", "SpinEdit", ComponentScope::Project);
    assert!(validate(&draft, &existing).is_err());

    // Non-PascalCase (snake_case)
    let draft = ComponentDraft::new("price_edit", "SpinEdit", ComponentScope::Project);
    assert!(validate(&draft, &existing).is_err());

    // Non-PascalCase (starts with digit)
    let draft = ComponentDraft::new("123Edit", "SpinEdit", ComponentScope::Project);
    assert!(validate(&draft, &existing).is_err());

    // Non-PascalCase (camelCase)
    let draft = ComponentDraft::new("priceEdit", "SpinEdit", ComponentScope::Project);
    assert!(validate(&draft, &existing).is_err());

    // Existing class name
    let draft = ComponentDraft::new("Button", "SpinEdit", ComponentScope::Project);
    assert!(validate(&draft, &existing).is_err());
}

#[test]
fn validate_refuses_non_native_base_class() {
    let existing = vec![];
    let draft = ComponentDraft::new("MyWidget", "NonExistentBase", ComponentScope::Project);
    let err = validate(&draft, &existing).unwrap_err();
    assert!(err.contains("not a native component"), "{err}");
}

#[test]
fn validate_refuses_invalid_props() {
    let existing = vec![];

    // Property name not snake_case
    let mut draft = ComponentDraft::new("MySpin", "SpinEdit", ComponentScope::Project);
    draft
        .props
        .push(PropDraft::new("NotSnake", "i64", "0", "behavior", ""));
    assert!(validate(&draft, &existing).is_err());

    // Duplicate property name
    let mut draft = ComponentDraft::new("MySpin", "SpinEdit", ComponentScope::Project);
    draft
        .props
        .push(PropDraft::new("step_val", "i64", "1", "behavior", ""));
    draft
        .props
        .push(PropDraft::new("step_val", "i64", "2", "behavior", ""));
    let err = validate(&draft, &existing).unwrap_err();
    assert!(err.contains("Duplicate property"), "{err}");

    // Clashing with base component prop (SpinEdit has `value`)
    let mut draft = ComponentDraft::new("MySpin", "SpinEdit", ComponentScope::Project);
    draft
        .props
        .push(PropDraft::new("value", "i64", "10", "behavior", ""));
    let err = validate(&draft, &existing).unwrap_err();
    assert!(err.contains("clashes with base class"), "{err}");

    // Clashing with universal name or rect
    let mut draft = ComponentDraft::new("MySpin", "SpinEdit", ComponentScope::Project);
    draft
        .props
        .push(PropDraft::new("name", "String", "my_name", "identity", ""));
    assert!(validate(&draft, &existing).is_err());

    let mut draft = ComponentDraft::new("MySpin", "SpinEdit", ComponentScope::Project);
    draft
        .props
        .push(PropDraft::new("rect", "String", "", "layout", ""));
    assert!(validate(&draft, &existing).is_err());
}

#[test]
fn validate_refuses_mismatched_defaults() {
    let existing = vec![];

    // i64 with text
    let mut draft = ComponentDraft::new("MySpin", "SpinEdit", ComponentScope::Project);
    draft
        .props
        .push(PropDraft::new("extra_step", "i64", "one", "behavior", ""));
    assert!(validate(&draft, &existing).is_err());

    // f64 with text
    let mut draft = ComponentDraft::new("MySpin", "SpinEdit", ComponentScope::Project);
    draft
        .props
        .push(PropDraft::new("ratio", "f64", "fast", "behavior", ""));
    assert!(validate(&draft, &existing).is_err());

    // bool with numeric
    let mut draft = ComponentDraft::new("MySpin", "SpinEdit", ComponentScope::Project);
    draft
        .props
        .push(PropDraft::new("active", "bool", "1", "behavior", ""));
    assert!(validate(&draft, &existing).is_err());
}

#[test]
fn validate_accepts_clean_draft() {
    let existing = vec!["Button".to_string()];
    let mut draft = ComponentDraft::new("PriceEdit", "SpinEdit", ComponentScope::Project);
    draft.props.push(PropDraft::new(
        "currency",
        "String",
        "BRL",
        "data",
        "Currency code",
    ));
    draft.props.push(PropDraft::new(
        "decimals",
        "i64",
        "2",
        "data",
        "Places after the mark",
    ));
    draft.props.push(PropDraft::new(
        "rate",
        "f64",
        "1.25",
        "data",
        "Conversion rate",
    ));
    draft.props.push(PropDraft::new(
        "auto_format",
        "bool",
        "true",
        "behavior",
        "Auto format",
    ));

    assert!(validate(&draft, &existing).is_ok());
}

#[test]
fn lib_rs_round_trips_through_from_source() {
    let mut draft = ComponentDraft::new("PriceEdit", "SpinEdit", ComponentScope::Project);
    draft.props.push(PropDraft::new(
        "currency",
        "String",
        "BRL",
        "data",
        "Shown before the number",
    ));
    draft.props.push(PropDraft::new(
        "decimals",
        "i64",
        "2",
        "data",
        "Places after the mark",
    ));
    draft.props.push(PropDraft::new(
        "rate",
        "f64",
        "1.5",
        "data",
        "Exchange rate",
    ));
    draft.props.push(PropDraft::new(
        "show_symbol",
        "bool",
        "true",
        "appearance",
        "Whether symbol is shown",
    ));

    let src = lib_rs(&draft);
    assert!(src.contains("#[crawrad(inherits = \"SpinEdit\", palette = \"My components\""));
    assert!(src.contains("pub struct PriceEdit"));

    let specs = crawrad_manifest_gen::from_source(&src).expect("source must parse");
    assert_eq!(specs.len(), 1);
    let spec = &specs[0];

    assert_eq!(spec.class, "PriceEdit");
    assert_eq!(spec.inherits.as_deref(), Some("SpinEdit"));
    assert_eq!(spec.palette, "My components");
    assert_eq!(spec.prefix, "priceedit");

    // Inherited components inherit `name` and `rect` from base, so only declared props appear
    let prop_keys: Vec<&str> = spec.props.iter().map(|p| p.key.as_str()).collect();
    assert_eq!(
        prop_keys,
        vec!["currency", "decimals", "rate", "show_symbol"]
    );

    let currency = spec.props.iter().find(|p| p.key == "currency").unwrap();
    assert_eq!(currency.kind, "text");
    assert_eq!(currency.default_json, "\"BRL\"");
    assert_eq!(currency.category, "data");
    assert_eq!(currency.docs, vec![" Shown before the number".to_string()]);

    let decimals = spec.props.iter().find(|p| p.key == "decimals").unwrap();
    assert_eq!(decimals.kind, "int");
    assert_eq!(decimals.default_json, "2");

    let rate = spec.props.iter().find(|p| p.key == "rate").unwrap();
    assert_eq!(rate.kind, "float");
    assert_eq!(rate.default_json, "1.5");

    let show_symbol = spec.props.iter().find(|p| p.key == "show_symbol").unwrap();
    assert_eq!(show_symbol.kind, "bool");
    assert_eq!(show_symbol.default_json, "true");
}

#[test]
fn cargo_toml_generates_correct_contents() {
    let draft = ComponentDraft::new("PriceEdit", "SpinEdit", ComponentScope::Project);
    let cargo = cargo_toml(&draft);
    assert!(cargo.contains("name = \"price_edit\""));
    assert!(cargo.contains("edition = \"2024\""));
    assert!(cargo.contains("crawrad-component = { path = \""));
    assert!(cargo.contains("crawrad-component-derive = { path = \""));
}

#[test]
fn write_writes_all_files_and_refuses_existing_dir() {
    let temp_root = fixture::folder("new_comp_test");
    let target = temp_root.join("price_edit");

    let mut draft = ComponentDraft::new("PriceEdit", "SpinEdit", ComponentScope::Project);
    draft.props.push(PropDraft::new(
        "factor",
        "i64",
        "10",
        "behavior",
        "Scale factor",
    ));

    let files = write(&draft, &target).expect("write must succeed");
    assert_eq!(files.len(), 3);

    let cargo_path = target.join("Cargo.toml");
    let lib_path = target.join("src").join("lib.rs");
    let manifest_path = target.join("manifest.json");

    assert!(cargo_path.is_file());
    assert!(lib_path.is_file());
    assert!(manifest_path.is_file());

    // Verify manifest is valid JSON and parses as a Manifest
    let manifest_text = std::fs::read_to_string(&manifest_path).expect("read manifest");
    let manifest = crawrad_package::manifest::Manifest::parse(&manifest_text)
        .expect("manifest must parse via Manifest::parse");
    assert_eq!(manifest.package.name, "price_edit");
    assert_eq!(manifest.components.len(), 1);
    assert_eq!(manifest.components[0].class, "PriceEdit");
    assert_eq!(manifest.components[0].inherits.as_deref(), Some("SpinEdit"));

    // Writing again to the same directory must be refused
    let second_write = write(&draft, &target);
    assert!(second_write.is_err());
    let err = second_write.unwrap_err();
    assert!(err.contains("already exists"), "{err}");
}

#[test]
fn target_dir_resolution_for_scopes() {
    let draft_project = ComponentDraft::new("PriceEdit", "SpinEdit", ComponentScope::Project);
    let root = Path::new("C:/my_project");
    let target = target_dir(&draft_project, Some(root)).expect("must resolve");
    assert_eq!(target, root.join("components").join("price_edit"));

    let no_root = target_dir(&draft_project, None);
    assert!(no_root.is_none());
}

#[test]
fn wizard_step_flow_and_navigation() {
    let existing = vec!["Button".to_string()];
    let mut wizard = NewComponentWizard::new(Some(Path::new("C:/test_proj")));
    assert_eq!(wizard.step, 1);
    assert_eq!(wizard.draft.name, "PriceEdit");

    // Advance to step 2
    let ok = wizard.next_step(&existing);
    assert!(ok);
    assert_eq!(wizard.step, 2);

    // Add and remove props
    assert!(wizard.draft.props.is_empty());
    wizard.add_prop();
    assert_eq!(wizard.draft.props.len(), 1);
    assert_eq!(wizard.draft.props[0].name, "prop_1");

    wizard.add_prop();
    assert_eq!(wizard.draft.props.len(), 2);
    assert_eq!(wizard.draft.props[1].name, "prop_2");

    wizard.remove_prop(0);
    assert_eq!(wizard.draft.props.len(), 1);
    assert_eq!(wizard.draft.props[0].name, "prop_2");

    // Go back to step 1
    wizard.prev_step();
    assert_eq!(wizard.step, 1);
}

#[test]
fn scope_hint_names_the_folder_and_where_it_lives() {
    let mut d = ComponentDraft::new("PriceEdit", "SpinEdit", ComponentScope::Project);
    assert_eq!(
        scope_hint(&d),
        "components/price_edit/ — goes to git with the project"
    );
    d.scope = ComponentScope::Mine;
    assert!(scope_hint(&d).starts_with("price_edit/ in your CrawRAD folder"));
}
