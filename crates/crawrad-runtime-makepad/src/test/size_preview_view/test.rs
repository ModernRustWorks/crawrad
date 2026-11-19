//! Unit tests for SizePreviewView and its action model.

use super::*;
use crawrad_designer::size_preview::{SizePlatform, SizePreview};

#[test]
fn combo_label_formats_shown_count_and_names() {
    assert_eq!(combo_label(&[]), "None shown");

    let items_none = vec![
        SizeItem {
            name: "Desktop".to_string(),
            width: 1280,
            height: 800,
            shown: false,
            custom: false,
        },
        SizeItem {
            name: "Phone".to_string(),
            width: 390,
            height: 844,
            shown: false,
            custom: false,
        },
    ];
    assert_eq!(combo_label(&items_none), "None shown");

    let items_one = vec![SizeItem {
        name: "Desktop".to_string(),
        width: 1280,
        height: 800,
        shown: true,
        custom: false,
    }];
    assert_eq!(combo_label(&items_one), "1 shown — Desktop");

    let items_three = vec![
        SizeItem {
            name: "Desktop".to_string(),
            width: 1280,
            height: 800,
            shown: true,
            custom: false,
        },
        SizeItem {
            name: "Tablet".to_string(),
            width: 820,
            height: 1180,
            shown: true,
            custom: false,
        },
        SizeItem {
            name: "Phone".to_string(),
            width: 390,
            height: 844,
            shown: true,
            custom: false,
        },
    ];
    assert_eq!(
        combo_label(&items_three),
        "3 shown — Desktop · Tablet · Phone"
    );

    let items_mixed = vec![
        SizeItem {
            name: "Desktop".to_string(),
            width: 1280,
            height: 800,
            shown: true,
            custom: false,
        },
        SizeItem {
            name: "Tablet".to_string(),
            width: 820,
            height: 1180,
            shown: false,
            custom: false,
        },
        SizeItem {
            name: "Phone".to_string(),
            width: 390,
            height: 844,
            shown: true,
            custom: false,
        },
        SizeItem {
            name: "Laptop".to_string(),
            width: 1024,
            height: 600,
            shown: true,
            custom: true,
        },
    ];
    assert_eq!(
        combo_label(&items_mixed),
        "3 shown — Desktop · Phone · Laptop"
    );
}

#[test]
fn device_picture_proportions_and_corner_radius() {
    // Desktop 1280x800: width > height and width >= 1024 -> desktop-like (radius = 0.5)
    let (w, h, r) = device_picture(1280, 800);
    assert_eq!(w, 30.0);
    assert_eq!(h, 18.75);
    assert_eq!(r, 0.5);

    // Full HD 1920x1080: desktop-like
    let (w, h, r) = device_picture(1920, 1080);
    assert_eq!(w, 30.0);
    assert_eq!(h, 16.875);
    assert_eq!(r, 0.5);

    // Phone 390x844: height > width, longer side is height = 30.0
    let (w, h, r) = device_picture(390, 844);
    assert_eq!(h, 30.0);
    let expected_w = 30.0 * 390.0 / 844.0;
    assert!((w - expected_w).abs() < 1e-6);
    assert!((r - (expected_w / 10.0)).abs() < 1e-6);

    // Tablet 820x1180: height > width
    let (w, h, r) = device_picture(820, 1180);
    assert_eq!(h, 30.0);
    let expected_w = 30.0 * 820.0 / 1180.0;
    assert!((w - expected_w).abs() < 1e-6);
    assert!((r - (expected_w / 10.0)).abs() < 1e-6);

    // Minimum side 8 px enforcement
    let (w, h, _) = device_picture(3000, 100);
    assert_eq!(w, 30.0);
    assert_eq!(h, 8.0);
}

#[test]
fn size_preview_action_variants() {
    let a1 = SizePreviewAction::Toggle(0);
    let a2 = SizePreviewAction::Toggle(0);
    assert_eq!(a1, a2);

    let add = SizePreviewAction::AddCustom {
        name: "Custom 1".to_string(),
        width: 400,
        height: 600,
    };
    assert_eq!(
        add,
        SizePreviewAction::AddCustom {
            name: "Custom 1".to_string(),
            width: 400,
            height: 600,
        }
    );

    assert_eq!(SizePreviewAction::Fit, SizePreviewAction::Fit);
    assert_eq!(
        SizePreviewAction::ToggleCustomCard,
        SizePreviewAction::ToggleCustomCard
    );
    assert_eq!(
        SizePreviewAction::CloseCustomCard,
        SizePreviewAction::CloseCustomCard
    );
}

#[test]
fn size_preview_model_integration_with_desktop_presets() {
    let mut preview = SizePreview::new(SizePlatform::Desktop);
    assert_eq!(preview.sizes.len(), 5);
    assert_eq!(preview.sizes[0].0.name, "Desktop");
    assert_eq!(preview.sizes[1].0.name, "Tablet");
    assert_eq!(preview.sizes[2].0.name, "Phone");
    assert_eq!(preview.sizes[3].0.name, "Laptop");
    assert_eq!(preview.sizes[4].0.name, "Full HD");

    let initial_shown = preview.shown();
    assert_eq!(initial_shown.len(), 3);

    preview.toggle(1);
    let after_toggle = preview.shown();
    assert_eq!(after_toggle.len(), 2);
    assert_eq!(after_toggle[0].name, "Desktop");
    assert_eq!(after_toggle[1].name, "Phone");

    let add_res = preview.add_custom("Mini Watch", 200, 200);
    assert!(add_res.is_ok());
    let with_custom = preview.shown();
    assert_eq!(with_custom.len(), 3);
    assert_eq!(with_custom[2].name, "Mini Watch");
}

#[test]
fn combo_label_lists_names_while_they_fit_and_counts_the_rest() {
    let item = |name: &str| SizeItem {
        name: name.to_string(),
        width: 100,
        height: 100,
        shown: true,
        custom: false,
    };
    let four = [
        item("Desktop"),
        item("Tablet"),
        item("Phone"),
        item("Laptop"),
    ];
    assert_eq!(combo_label(&four), "4 shown — Desktop · Tablet · Phone +1");
    let long = [item("A very long custom size name"), item("Phone")];
    assert_eq!(
        combo_label(&long),
        "2 shown — A very long custom size name +1"
    );
}
