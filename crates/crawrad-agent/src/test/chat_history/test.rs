//! The conversation store: what is kept, the order, titles, dates, search.

use super::*;

fn store() -> HistoryStore {
    let dir = std::env::temp_dir().join(format!(
        "crawrad-history-{}-{}",
        std::process::id(),
        Conversation::new_id()
    ));
    HistoryStore::at(dir)
}

fn talk(id: &str, agent: &str, updated: &str, said: &[&str]) -> Conversation {
    let mut messages = Vec::new();
    for (i, text) in said.iter().enumerate() {
        messages.push(ChatMessage::user(format!("u{i}"), *text));
        messages.push(ChatMessage::assistant(format!("a{i}"), "ok"));
    }
    Conversation {
        id: id.to_string(),
        agent: agent.to_string(),
        agent_key: format!("{agent}:x:"),
        acp_session: Some(format!("s-{id}")),
        can_resume: agent == "Claude",
        updated: updated.to_string(),
        messages,
    }
}

#[test]
fn a_conversation_is_kept_and_read_back_whole() {
    let s = store();
    let c = talk(
        "c1-a",
        "Claude",
        "2026-10-07 08:31",
        &["Add a Save button", "and a Cancel"],
    );
    s.save(&c).unwrap();
    assert_eq!(s.load("c1-a"), Some(c.clone()));
    // saving again replaces it, it does not add a second one
    let mut more = c;
    more.messages.push(ChatMessage::user("u9", "thanks"));
    s.save(&more).unwrap();
    assert_eq!(s.list().len(), 1);
    assert_eq!(s.load("c1-a").unwrap().turns(), 3);
    let _ = std::fs::remove_dir_all(s.dir());
}

#[test]
fn an_empty_conversation_is_not_kept_and_a_bad_id_is_refused() {
    let s = store();
    s.save(&talk("c2", "Claude", "2026-10-07 08:00", &[]))
        .unwrap();
    assert!(s.list().is_empty());
    assert!(
        s.save(&talk("../evil", "Claude", "2026-10-07 08:00", &["x"]))
            .is_err()
    );
    assert_eq!(s.load("../evil"), None);
    let _ = std::fs::remove_dir_all(s.dir());
}

#[test]
fn the_list_is_newest_first_grouped_and_searchable() {
    let s = store();
    s.save(&talk(
        "c1",
        "Codex",
        "2026-10-04 10:00",
        &["Port the settings dialog"],
    ))
    .unwrap();
    s.save(&talk(
        "c2",
        "Claude",
        "2026-10-07 08:31",
        &["Add a Save button\nwith an icon"],
    ))
    .unwrap();
    s.save(&talk(
        "c3",
        "Gemini",
        "2026-10-06 18:20",
        &["Rename AppState"],
    ))
    .unwrap();
    std::fs::write(s.dir().join("broken.json"), "{ not json").unwrap();
    let rows = s.summaries("2026-10-07", "");
    let ids: Vec<&str> = rows.iter().map(|r| r["id"].as_str().unwrap()).collect();
    assert_eq!(ids, ["c2", "c3", "c1"]);
    assert_eq!(rows[0]["title"], "Add a Save button");
    assert_eq!(
        (rows[0]["group"].as_str(), rows[0]["when"].as_str()),
        (Some("Today"), Some("08:31"))
    );
    assert_eq!(
        (rows[1]["group"].as_str(), rows[1]["when"].as_str()),
        (Some("Yesterday"), Some("18:20"))
    );
    assert_eq!(
        (rows[2]["group"].as_str(), rows[2]["when"].as_str()),
        (Some("Earlier"), Some("Oct 4"))
    );
    assert_eq!(
        (rows[0]["resumes"].as_bool(), rows[1]["resumes"].as_bool()),
        (Some(true), Some(false))
    );
    assert_eq!(rows[0]["turns"], 1);
    let found = s.summaries("2026-10-07", "gemini");
    assert_eq!(found.len(), 1);
    assert_eq!(s.summaries("2026-10-07", "SETTINGS")[0]["id"], "c1");
    let _ = std::fs::remove_dir_all(s.dir());
}

#[test]
fn yesterday_crosses_months_and_years() {
    assert_eq!(group_of("2025-12-31 23:59", "2026-01-01"), "Yesterday");
    assert_eq!(group_of("2026-02-28 10:00", "2026-03-01"), "Yesterday");
    assert_eq!(group_of("2026-02-27 10:00", "2026-03-01"), "Earlier");
    assert_eq!(when_of("2025-09-30 10:00", "2026-10-07"), "Sep 30");
}

#[test]
fn a_long_first_message_is_cut_and_ids_are_unique() {
    let long = "x".repeat(200);
    let c = talk("c9", "Claude", "2026-10-07 08:00", &[&long]);
    assert_eq!(c.title().chars().count(), 80);
    assert!(c.title().ends_with('…'));
    assert_ne!(Conversation::new_id(), Conversation::new_id());
}

#[test]
fn record_keeps_one_file_per_conversation_and_reopen_brings_its_turns_back() {
    let s = store();
    let live = crate::acp_live::LiveAgent::new();
    let mut open = None;
    let mut said = vec![ChatMessage::user("u0", "first question")];
    record(&s, &mut open, "Mock", &live, &said).unwrap();
    let id = open.clone().expect("an id on the first turn");
    said.push(ChatMessage::assistant("a0", "an answer"));
    said.push(ChatMessage::user("u1", "second question"));
    record(&s, &mut open, "Mock", &live, &said).unwrap();
    assert_eq!(
        open.as_deref(),
        Some(id.as_str()),
        "the same conversation keeps its id"
    );
    assert_eq!(s.list().len(), 1);

    let mut other = crate::acp_live::LiveAgent::new();
    assert_eq!(reopen(&s, &id, &mut other), Some(said));
    assert_eq!(reopen(&s, "c-missing", &mut other), None);
    let answer = mcp_json(&s, Some(&id), "");
    assert_eq!(answer["conversation"], id.as_str());
    assert_eq!(answer["history"][0]["title"], "first question");
    assert_eq!(answer["history"][0]["turns"], 2);
    let _ = std::fs::remove_dir_all(s.dir());
}

#[test]
fn the_list_rows_head_each_group_once_and_mark_the_open_one() {
    let s = store();
    s.save(&talk(
        "c1",
        "Codex",
        "2026-10-04 10:00",
        &["Port the settings dialog"],
    ))
    .unwrap();
    s.save(&talk(
        "c2",
        "Claude",
        "2026-10-07 08:31",
        &["Add a Save button", "and Cancel"],
    ))
    .unwrap();
    s.save(&talk(
        "c3",
        "Claude",
        "2026-10-07 07:52",
        &["Why does the build fail?"],
    ))
    .unwrap();
    let all = s.summaries("2026-10-07", "");
    let drawn = rows(&all, Some("c3"), 10);
    let groups: Vec<_> = drawn.iter().map(|r| r.group.as_deref()).collect();
    assert_eq!(groups, [Some("Today"), None, Some("Earlier")]);
    assert_eq!(drawn[0].meta, "Claude · 2 turns");
    assert_eq!(drawn[1].meta, "Claude · 1 turn");
    assert_eq!((drawn[0].resume, drawn[2].resume), ("resumes", "read only"));
    assert_eq!(
        drawn.iter().map(|r| r.open).collect::<Vec<_>>(),
        [false, true, false]
    );
    assert_eq!(
        rows(&all, None, 2).len(),
        2,
        "no more rows than the list has slots"
    );
    let _ = std::fs::remove_dir_all(s.dir());
}
