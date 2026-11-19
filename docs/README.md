# The CrawRAD manual

The user manual, written with [mdBook](https://rust-lang.github.io/mdBook/) and
organised by the [Diátaxis](https://diataxis.fr/) framework. Build it with:

    mdbook build docs      # output in docs/book/ (not versioned)
    mdbook serve docs      # live preview at http://localhost:3000

## Where a page goes

| Part | Folder | A page here… |
|---|---|---|
| Tutorials | `src/tutorials/` | teaches by building something, step by step |
| How-to guides | `src/how-to/` | gets one task done, for someone who knows the basics |
| Reference | `src/reference/` | describes facts, point by point (components and properties are generated) |
| Concepts | `src/concepts/` | explains why, without steps |

Rules:

- Write only what the IDE really does today. Every page is checked against the IDE.
- A planned chapter stays as a draft entry `- [Title]()` in `SUMMARY.md` until written.
- English only, like the IDE's interface.
