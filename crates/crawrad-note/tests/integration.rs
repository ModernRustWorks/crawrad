use crawrad_note::note;

#[note("test-free-fn")]
fn compute_sum(a: i32, b: i32) -> i32 {
    a + b
}

#[note("test-struct")]
#[derive(Debug, PartialEq, Eq)]
struct ConfigItem {
    name: &'static str,
    priority: u32,
}

#[note("test-enum")]
#[derive(Debug, PartialEq, Eq)]
enum StatusState {
    Idle,
    Running(u32),
}

struct Engine {
    state: StatusState,
}

impl Engine {
    #[note("test-impl-fn")]
    const LIMIT: u32 = 7;

    #[note("test-impl-fn")]
    fn current_state(&self) -> &StatusState {
        &self.state
    }
}

#[note("test-trait")]
trait Worker {
    #[note("test-trait")]
    fn work(&self) -> &'static str;

    #[note("test-trait")]
    fn tagged(&self) -> &'static str {
        "tagged"
    }
}

impl Worker for Engine {
    fn work(&self) -> &'static str {
        "engine-running"
    }
}

#[test]
fn free_fn_executes_correctly() {
    assert_eq!(compute_sum(10, 32), 42);
}

#[test]
fn struct_instantiates_and_holds_values() {
    let item = ConfigItem {
        name: "test",
        priority: 1,
    };
    assert_eq!(item.name, "test");
    assert_eq!(item.priority, 1);
}

#[test]
fn enum_variants_match_properly() {
    let status = StatusState::Running(100);
    match status {
        StatusState::Idle => panic!("expected running"),
        StatusState::Running(progress) => assert_eq!(progress, 100),
    }
}

#[test]
fn impl_method_callable_with_in_body_tracking() {
    let engine = Engine {
        state: StatusState::Idle,
    };
    assert_eq!(engine.current_state(), &StatusState::Idle);
}

#[test]
fn trait_implementable_and_dispatchable() {
    let engine = Engine {
        state: StatusState::Idle,
    };
    assert_eq!(engine.work(), "engine-running");
}

#[note("test-free-fn")]
static GREETING: &str = "hi";

#[test]
fn consts_and_statics_inside_and_outside_impls_keep_their_value() {
    assert_eq!(Engine::LIMIT, 7);
    assert_eq!(GREETING, "hi");
    let engine = Engine {
        state: StatusState::Idle,
    };
    assert_eq!(engine.tagged(), "tagged");
}
