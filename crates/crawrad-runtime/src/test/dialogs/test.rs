use crate::dialogs::{
    DialogButtons, DialogKind, ModalResult, PendingAnswer, dialog_manager, resolve_dialog,
};

#[test]
fn test_pending_answer_open_resolve_and_take() {
    let _guard = crate::tests::dialogs_alone();
    dialog_manager().lock().unwrap().clear();

    let pending = PendingAnswer::default();
    assert_eq!(pending.take(), None);

    pending.open(
        "Confirm Action",
        "Are you sure?",
        DialogKind::Confirmation,
        DialogButtons::YesNo,
    );
    assert_eq!(pending.take(), None);

    let resolved = resolve_dialog(ModalResult::Yes);
    assert_eq!(resolved, Some(ModalResult::Yes));

    assert_eq!(pending.take(), Some(ModalResult::Yes));
    assert_eq!(pending.take(), None);
}

#[test]
fn test_pending_answer_open_default_title() {
    let _guard = crate::tests::dialogs_alone();
    dialog_manager().lock().unwrap().clear();

    let pending = PendingAnswer::default();
    pending.open(
        "",
        "Informational notice",
        DialogKind::Information,
        DialogButtons::Ok,
    );

    let snap = crate::dialogs::active_dialog().expect("active dialog");
    assert_eq!(snap.title, "Information");

    resolve_dialog(ModalResult::Ok);
    assert_eq!(pending.take(), Some(ModalResult::Ok));
    assert_eq!(pending.take(), None);
}

#[test]
fn test_dialog_kind_from_name_and_as_name_round_trip() {
    for (name, expected) in [
        ("mtInformation", DialogKind::Information),
        ("Information", DialogKind::Information),
        ("mtWarning", DialogKind::Warning),
        ("Warning", DialogKind::Warning),
        ("mtError", DialogKind::Error),
        ("Error", DialogKind::Error),
        ("mtConfirmation", DialogKind::Confirmation),
        ("Confirmation", DialogKind::Confirmation),
        ("Custom", DialogKind::Custom),
        ("unknown_fallback", DialogKind::Information),
    ] {
        assert_eq!(DialogKind::from_name(name), expected);
    }

    assert_eq!(DialogKind::Information.as_name(), "mtInformation");
    assert_eq!(DialogKind::Warning.as_name(), "mtWarning");
    assert_eq!(DialogKind::Error.as_name(), "mtError");
    assert_eq!(DialogKind::Confirmation.as_name(), "mtConfirmation");
    assert_eq!(DialogKind::Custom.as_name(), "Custom");
}

#[test]
fn test_dialog_buttons_from_name_and_as_name_round_trip() {
    for (name, expected) in [
        ("mbOK", DialogButtons::Ok),
        ("Ok", DialogButtons::Ok),
        ("mbOKCancel", DialogButtons::OkCancel),
        ("OkCancel", DialogButtons::OkCancel),
        ("mbYesNo", DialogButtons::YesNo),
        ("YesNo", DialogButtons::YesNo),
        ("mbYesNoCancel", DialogButtons::YesNoCancel),
        ("YesNoCancel", DialogButtons::YesNoCancel),
        ("unknown_fallback", DialogButtons::Ok),
    ] {
        assert_eq!(DialogButtons::from_name(name), expected);
    }

    assert_eq!(DialogButtons::Ok.as_name(), "mbOK");
    assert_eq!(DialogButtons::OkCancel.as_name(), "mbOKCancel");
    assert_eq!(DialogButtons::YesNo.as_name(), "mbYesNo");
    assert_eq!(DialogButtons::YesNoCancel.as_name(), "mbYesNoCancel");
}
