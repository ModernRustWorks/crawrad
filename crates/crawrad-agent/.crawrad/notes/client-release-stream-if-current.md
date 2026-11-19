---
id: client-release-stream-if-current
kind: doc
target_file: src/client.rs
line_hint: 1501
author: 
created: 
title: "release_stream_if_current"
tags: []
signature: "pub (crate) fn release_stream_if_current (sink : & Mutex < Option < Sender < AcpStreamEvent > > > , active_turn : & AtomicU64 , turn : u64 ,)"
---

Unhooks the streaming sink of a prompt turn if still owned.

Unhooks the streaming sink of a prompt turn — **only if that turn still owns it**.

The waiter of turn N sends `Done` and then unhooks the sink. Whoever gets
that `Done` may start turn N+1 at once, and the waiter of turn N can reach
this line AFTER turn N+1 installed its own sink. Unhooking unconditionally
then dropped every `TextDelta` of turn N+1: the second answer arrived as a
bare `Done`, with no text. Seen as the intermittent failure of
`test_acp_client_streaming_and_wire_continuity` (30/09/2026), and in the
chat as a second prompt whose answer comes back empty.
