//! **THE TOOL DESCRIPTION IS THE CONTRACT, so it cannot be allowed to go stale.**
//!
//! The agent that drives this IDE over MCP is often a raw terminal CLI with no
//! harness: the tool's own `description` is the ONLY channel that reaches it. A
//! description that names two frameworks when the catalogue offers three does
//! not fail anything — it simply means nobody ever asks for the third.
//!
//! That happened. `project.create` said *"'makepad' (default) or 'egui'"* for
//! the whole time Slint was a selectable, generating target: the handler
//! resolved `"slint"` perfectly well, and no agent reading the description had
//! any reason to try it.
//!
//! The fix is not a corrected string, which rots again at the fourth framework.
//! It is this test. `crawrad-mcp` does not depend on `crawrad-project`, so it
//! cannot read the catalogue; this crate sees both, and checks here that they
//! agree.

use crawrad_project::frameworks::CATALOGUE;

/// The `framework` argument's description, as the live tool list carries it.
fn framework_description() -> String {
    let tools = crawrad_mcp::tools::build_tools_list();
    let create = tools
        .iter()
        .find(|t| t.name == "project.create")
        .expect("project.create is a tool");
    create.input_schema["properties"]["framework"]["description"]
        .as_str()
        .expect("the framework argument documents itself")
        .to_ascii_lowercase()
}

#[test]
fn project_create_offers_every_framework_the_catalogue_can_generate() {
    let said = framework_description();
    for f in CATALOGUE {
        if f.target.is_none() {
            // Planned, with no generator: `resolve_framework` refuses it, so
            // the description must NOT invite it.
            assert!(
                !said.contains(&f.name.to_ascii_lowercase()),
                "`project.create` offers `{}`, which has no generator and is refused",
                f.name
            );
            continue;
        }
        assert!(
            said.contains(&f.name.to_ascii_lowercase()),
            "`project.create` can create a `{}` project and never says so. An agent \
             reading this description will never ask for it:\n  {said}",
            f.name
        );
    }
}
