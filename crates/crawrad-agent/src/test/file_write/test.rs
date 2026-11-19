//! Tests for FileWriteRequest lifecycle and responses.

use super::FileWriteRequest;
use std::path::PathBuf;
use std::sync::mpsc::channel;

#[test]
fn respond_is_called_once_second_respond_ignored() {
    let (tx, rx) = channel();
    let req = FileWriteRequest::new(PathBuf::from("test.txt"), "hello".to_string(), move |res| {
        let _ = tx.send(res);
    });
    req.respond(Ok(()));
    req.respond(Err("second response".to_string()));
    assert_eq!(rx.recv().unwrap(), Ok(()));
    assert!(rx.try_recv().is_err());
}

#[test]
fn dropping_unanswered_request_answers_err() {
    let (tx, rx) = channel();
    {
        let _req =
            FileWriteRequest::new(PathBuf::from("test.txt"), "hello".to_string(), move |res| {
                let _ = tx.send(res);
            });
    }
    assert_eq!(rx.recv().unwrap(), Err("The edit was not applied.".into()));
}
