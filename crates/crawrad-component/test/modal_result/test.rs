use crate::ModalResult;
use std::str::FromStr;

#[test]
fn default_modal_result_is_none() {
    assert_eq!(ModalResult::default(), ModalResult::None);
    assert_eq!(ModalResult::None.as_str(), "none");
    assert!(ModalResult::None.is_none());
}

#[test]
fn modal_result_as_str_and_display() {
    assert_eq!(ModalResult::Ok.as_str(), "ok");
    assert_eq!(format!("{}", ModalResult::Ok), "ok");
    assert_eq!(ModalResult::Cancel.as_str(), "cancel");
    assert_eq!(ModalResult::Abort.as_str(), "abort");
    assert_eq!(ModalResult::Retry.as_str(), "retry");
    assert_eq!(ModalResult::Ignore.as_str(), "ignore");
    assert_eq!(ModalResult::Yes.as_str(), "yes");
    assert_eq!(ModalResult::No.as_str(), "no");
    assert_eq!(ModalResult::Close.as_str(), "close");
}

#[test]
fn modal_result_from_str_and_aliases() {
    assert_eq!(ModalResult::from_str("none"), Ok(ModalResult::None));
    assert_eq!(ModalResult::from_str("mrnone"), Ok(ModalResult::None));
    assert_eq!(ModalResult::from_str("ok"), Ok(ModalResult::Ok));
    assert_eq!(ModalResult::from_str("mrok"), Ok(ModalResult::Ok));
    assert_eq!(ModalResult::from_str("cancel"), Ok(ModalResult::Cancel));
    assert_eq!(ModalResult::from_str("mrcancel"), Ok(ModalResult::Cancel));
    assert_eq!(ModalResult::from_str("yes"), Ok(ModalResult::Yes));
    assert_eq!(ModalResult::from_str("no"), Ok(ModalResult::No));
    assert_eq!(ModalResult::from_str("invalid"), Err(()));
}

#[test]
fn modal_result_integer_conversions() {
    assert_eq!(ModalResult::None.as_i32(), 0);
    assert_eq!(ModalResult::Ok.as_i32(), 1);
    assert_eq!(ModalResult::Cancel.as_i32(), 2);
    assert_eq!(ModalResult::from_i32(0), Some(ModalResult::None));
    assert_eq!(ModalResult::from_i32(1), Some(ModalResult::Ok));
    assert_eq!(ModalResult::from_i32(2), Some(ModalResult::Cancel));
    assert_eq!(ModalResult::from_i32(99), None);
}

#[test]
fn modal_result_helpers() {
    assert!(ModalResult::Ok.is_affirmative());
    assert!(ModalResult::Yes.is_affirmative());
    assert!(!ModalResult::Cancel.is_affirmative());
    assert!(ModalResult::Cancel.is_dismissive());
    assert!(ModalResult::Abort.is_dismissive());
    assert!(ModalResult::Close.is_dismissive());
    assert!(!ModalResult::Ok.is_dismissive());
}
