//! Unit tests for pure disk change action decisions.

use super::{DiskAction, disk_change_action};

#[test]
fn disk_change_action_rules() {
    // None -> Deleted
    assert_eq!(
        disk_change_action(None, "code", false, None),
        DiskAction::Deleted
    );
    // equal -> Same
    assert_eq!(
        disk_change_action(Some("abc"), "abc", false, None),
        DiskAction::Same
    );
    assert_eq!(
        disk_change_action(Some("abc"), "abc", true, Some("xyz")),
        DiskAction::Same
    );
    // no buffer & differs -> Reload
    assert_eq!(
        disk_change_action(Some("new"), "old", false, Some("old")),
        DiskAction::Reload
    );
    // buffer & differs & base None -> Conflict
    assert_eq!(
        disk_change_action(Some("disk"), "ide", true, None),
        DiskAction::Conflict
    );
    // buffer & disk==base -> Same (the user's edit, disk untouched)
    assert_eq!(
        disk_change_action(Some("orig"), "modified", true, Some("orig")),
        DiskAction::Same
    );
}

#[test]
fn disk_change_conflict_requires_buffer_and_differs_from_base() {
    assert_eq!(
        disk_change_action(Some("disk_v2"), "ide_v2", true, Some("base_v1")),
        DiskAction::Conflict
    );
    assert_eq!(
        disk_change_action(Some("disk_v2"), "base_v1", false, Some("base_v1")),
        DiskAction::Reload
    );
}
