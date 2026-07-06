# Masterplan: Learn Rust By Building Something Useful

This repository should become a clone-and-learn Rust course for people who want to become comfortable writing normal, practical Rust programs. The learning path is chapter-based, assignment-driven, and centered on building one worthwhile application over time instead of solving disconnected toy exercises.

The intended final project is a desktop GUI application: **Focus Forge**, a personal workbench for learning, planning, and tracking real projects. It starts as a command-line notebook, grows into a persisted data model, then becomes a GUI app with search, task tracking, notes, tagging, import/export, and optional web requests for small useful features.

This is not an embedded Rust course. The priority is everyday programming: data modeling, ownership, errors, files, tests, modules, crates, async, HTTP, persistence, GUI architecture, and maintainable application design.

Mathematical maturity can be assumed to grow over time. The course should include a parallel math warm-up track that rebuilds university-level comfort gradually, without making the main Rust assignments depend on advanced math too early.

The expected learner already knows programming through classic object-oriented or object-capable languages such as C#, Java, or TypeScript. The course should use that background as a bridge: familiar ideas like classes, interfaces, exceptions, packages, collections, nullability, async, and application architecture should be compared directly with Rust's structs, enums, traits, modules, `Result`, `Option`, ownership, and crate ecosystem.

The learner may be rusty in programming in the very literal sense: a computer scientist or experienced developer who once had strong foundations, drifted away, got discouraged, or stopped building for a while, and now wants to return in full force. The curriculum should respect that background without assuming current fluency. It should rebuild momentum through quick visible wins, gentle repetition, and progressively richer challenges.

## North Star Decisions

These decisions should keep implementation from drifting once the repo starts growing.

- [ ] Build one coherent course, not a pile of unrelated Rust exercises.
- [ ] Use one Cargo workspace as the learner's home base.
- [ ] Build one worthwhile application, Focus Forge, across the whole course.
- [ ] Use chapter labs as safe sandboxes, not as replacements for the product.
- [ ] Teach Rust through repeated vertical slices, not a long front-loaded theory phase.
- [ ] Prefer a CLI before the GUI so the learner can practice logic, persistence, and testing with fast feedback.
- [ ] Use `egui` / `eframe` for the GUI unless implementation proves a better Rust-native choice.
- [ ] Keep web requests optional and late.
- [ ] Keep embedded Rust, unsafe Rust, and advanced systems programming out of the main path.
- [ ] Treat math as a parallel reactivation track that enriches the app.
- [ ] Treat OOP knowledge as a bridge, then gradually move the learner into idiomatic Rust design.
- [ ] Optimize the course for confidence, continuity, and independent rebuilding of skill.

## Repository Assumptions

- [ ] The learner can clone this repository.
- [ ] The learner uses VS Code.
- [ ] The learner has the Rust extension installed, preferably `rust-analyzer`.
- [ ] The learner has `cargo` available in the terminal.
- [ ] The learner is already comfortable with at least one language such as C#, Java, or TypeScript.
- [ ] The learner understands common object-oriented concepts like classes, methods, interfaces, inheritance, exceptions, collections, packages, and async calls.
- [ ] The learner may feel slow at first, even if they have strong underlying computer science ability.
- [ ] The course should treat forgotten knowledge as normal reactivation, not failure.
- [ ] The repository should work on Windows, macOS, and Linux whenever practical.
- [ ] Each chapter should be runnable with ordinary Cargo commands.
- [ ] Each assignment should be solvable without hidden services or paid APIs.
- [ ] Optional stretch tasks may use web requests, but the core course should work offline.

## Learning Experience Vision

- [ ] Make the repo feel like a workshop, not a textbook.
- [ ] Give every chapter a clear concept goal and a visible product improvement.
- [ ] Use increasingly difficult assignments:
  - [ ] Warm-up tasks to practice the immediate concept.
  - [ ] Main tasks that change the application.
  - [ ] Stretch tasks for learners who want more challenge.
  - [ ] Reflection prompts that make Rust's tradeoffs explicit.
- [ ] Keep the app useful at every stage.
- [ ] Prefer practical Rust idioms over clever tricks.
- [ ] Teach debugging, compiler errors, refactoring, and tests as part of the normal workflow.
- [ ] Include acceptance criteria for assignments so learners know when they are done.
- [ ] Include small automated tests where they help, but avoid turning the course into test-only puzzle solving.
- [ ] Use friendly examples, playful sample data, and visible progress.
- [ ] Run math refreshers in parallel with programming work, starting light and becoming more serious as the learner warms up.
- [ ] Use math to enrich the app where natural, especially progress scoring, search ranking, statistics, scheduling, and visualization.
- [ ] Regularly compare Rust concepts to C#, Java, and TypeScript equivalents.
- [ ] Teach which object-oriented instincts transfer well and which need to be replaced.
- [ ] Emphasize composition, explicit data flow, and sum types over inheritance-heavy design.
- [ ] Show how Rust handles common OOP-era problems: nulls, exceptions, shared mutable state, package boundaries, interfaces, and dependency injection.
- [ ] Keep the learning curve reasonable by alternating new Rust concepts with consolidation chapters.
- [ ] Design for confidence recovery: every chapter should contain at least one fast, satisfying result before the harder assignment.
- [ ] Make progress visible through the app, tests, screenshots, sample data, and chapter completion checklists.
- [ ] Normalize friction with the compiler as part of the learning loop.

## Target Final Application

**Focus Forge** should help a learner manage what they are learning or building.

By the end, the app should support:

- [ ] Projects with names, descriptions, status, and timestamps.
- [ ] Notes attached to projects.
- [ ] Tasks with completion status and priority.
- [ ] Tags for filtering and organization.
- [ ] Local persistence to disk.
- [ ] Import and export as JSON.
- [ ] A desktop GUI for browsing, editing, and searching data.
- [ ] A clean internal domain model separated from UI code.
- [ ] Basic validation and helpful error messages.
- [ ] Optional HTTP features, such as fetching release notes, docs links, or inspirational prompts from a simple public endpoint.
- [ ] A small but real settings system.
- [ ] Tests for important non-UI behavior.

Suggested GUI stack:

- [ ] Use `eframe` / `egui` for the GUI because it is approachable, cross-platform, Rust-native, and does not require a heavy web frontend.
- [ ] Keep GUI code separate from domain logic so the learner sees how real applications stay organized.

## OOP-To-Rust Bridge

- [ ] Assume the learner can already read ordinary application code in C#, Java, or TypeScript.
- [ ] Explain Rust by comparison when comparison helps, but do not pretend Rust is just OOP with different syntax.
- [ ] Translate familiar concepts deliberately:
  - [ ] Class with fields and methods -> struct plus `impl`.
  - [ ] Interface -> trait.
  - [ ] Enum with limited constants -> Rust enum, including data-carrying variants.
  - [ ] Null or undefined -> `Option<T>`.
  - [ ] Exception -> `Result<T, E>` for expected failures.
  - [ ] Package or namespace -> module and crate.
  - [ ] Generic class or method -> Rust generics and trait bounds.
  - [ ] Dependency injection -> passing concrete values, trait bounds, or trait objects at boundaries.
  - [ ] Inheritance -> usually composition, traits, enums, or delegation.
  - [ ] Garbage-collected references -> ownership, borrowing, `Rc`, `Arc`, `RefCell`, or `Mutex` depending on the problem.
- [ ] Call out false friends:
  - [ ] `mut` means the binding can be changed; it is not the same mental model as mutable object references in Java or C#.
  - [ ] A Rust `enum` is much more powerful than a Java or C# enum.
  - [ ] Traits are not class inheritance.
  - [ ] `String` and `&str` are not just two spellings of the same thing.
  - [ ] `clone()` should be a conscious choice, not the default way out of ownership trouble.
  - [ ] Panics are not Rust's normal replacement for exceptions.
- [ ] Include small "from OOP instincts to Rust design" notes in chapters where learners are likely to reach for inheritance, nulls, exceptions, or shared mutable objects.

## Motivation And Learning Psychology

- [ ] Design the course for a returning computer scientist who needs traction, not hand-holding.
- [ ] Treat confidence as a first-class curriculum outcome.
- [ ] Make every chapter pay off quickly:
  - [ ] First 10 minutes: run or change something visible.
  - [ ] First 30 minutes: complete a small assignment.
  - [ ] End of chapter: add a meaningful capability to Focus Forge.
- [ ] Use a rhythm of "recognize, translate, build, stretch":
  - [ ] Recognize a familiar idea from C#, Java, TypeScript, or CS fundamentals.
  - [ ] Translate it into Rust's model.
  - [ ] Build a small feature with it.
  - [ ] Stretch it into a richer, more open-ended assignment.
- [ ] Keep early assignments small enough to finish in one sitting.
- [ ] Let later assignments grow in design freedom only after the learner has rebuilt confidence.
- [ ] Repeat important concepts in different contexts instead of explaining them once and moving on.
- [ ] Prefer spiral learning:
  - [ ] Ownership appears first as simple move-vs-borrow.
  - [ ] Later it appears in collections.
  - [ ] Later it appears in persistence.
  - [ ] Later it appears in GUI state.
  - [ ] Later it appears in async boundaries.
- [ ] Create regular "competence checkpoints" where the learner can see how far they have come.
- [ ] Include "you probably remember this as..." notes for concepts that map to older CS or OOP knowledge.
- [ ] Include "Rust is asking for this because..." notes when the compiler feels fussy.
- [ ] Make compiler errors feel like coaching signals:
  - [ ] Predict what the compiler will complain about.
  - [ ] Run `cargo check`.
  - [ ] Read the first error carefully.
  - [ ] Fix the smallest thing.
  - [ ] Re-run quickly.
- [ ] Avoid long silent stretches where the learner writes a lot before seeing anything work.
- [ ] Prefer vertical slices over large foundations:
  - [ ] A tiny app that runs is better than a perfect architecture that does nothing.
  - [ ] Refactor after the learner feels the pain that the refactor solves.
- [ ] Add optional "returning to full force" challenges for learners who want to push harder after the main task.
- [ ] Avoid shame language, weed-out exercises, and clever puzzles that prove little about real programming ability.
- [ ] Make the app increasingly personal so progress feels owned, not merely completed.

## Chapter Rhythm

Each chapter should follow a consistent structure so the learner always knows how to approach the work.

- [ ] Orientation:
  - [ ] What this chapter adds to Focus Forge.
  - [ ] Which Rust concepts appear.
  - [ ] Which familiar OOP or CS ideas are being reactivated.
  - [ ] Which math warm-up runs in parallel.
- [ ] Quick win:
  - [ ] A tiny change that compiles and runs quickly.
  - [ ] A visible result in the CLI, GUI, test output, or saved data.
- [ ] Concept bridge:
  - [ ] Short explanation.
  - [ ] Comparison to C#, Java, or TypeScript.
  - [ ] One deliberately broken example and its compiler feedback.
- [ ] Guided assignment:
  - [ ] Clear steps.
  - [ ] Acceptance criteria.
  - [ ] Hints that do not give away the whole solution.
- [ ] Main build:
  - [ ] A meaningful feature for Focus Forge.
  - [ ] Tests or manual verification.
- [ ] Stretch path:
  - [ ] One deeper Rust challenge.
  - [ ] One product polish challenge.
  - [ ] One math or CS challenge.
- [ ] Reflection:
  - [ ] What became easier?
  - [ ] What still feels strange?
  - [ ] Which old programming instinct helped?
  - [ ] Which old programming instinct got in the way?

## Reward Design

- [ ] Show progress through a chapter checklist.
- [ ] Keep sample data fun enough that app improvements feel alive.
- [ ] Add visible app milestones:
  - [ ] "First project appears."
  - [ ] "Data survives restart."
  - [ ] "Search finds a forgotten note."
  - [ ] "The GUI edits real data."
  - [ ] "Import/export protects your work."
- [ ] Use before-and-after moments:
  - [ ] Before persistence: data disappears.
  - [ ] After persistence: data returns.
  - [ ] Before search: manual scanning.
  - [ ] After search: instant retrieval.
  - [ ] Before validation: confusing bad states.
  - [ ] After validation: useful feedback.
- [ ] Include small "ship it" moments where the learner builds a release binary or shares a demo workspace.
- [ ] Let learners personalize Focus Forge early with their own project names, tags, notes, and goals.
- [ ] Keep stretch tasks optional and clearly marked so ambition does not block completion.
- [ ] Provide recovery paths:
  - [ ] "If you are stuck, run this command."
  - [ ] "If this error appears, check these three things."
  - [ ] "If your code works but feels messy, continue; refactoring comes later."

## Learning Environment Architecture

The repository should be one coherent Rust workshop, not a drawer full of disconnected exercises. The best structure is an **agile spiral curriculum inside one Cargo workspace**.

The learner should experience the course as:

- [ ] One real product that grows chapter by chapter.
- [ ] Many small labs where risky concepts can be practiced safely.
- [ ] Reliable commands for checking, testing, running, debugging, and recovering.
- [ ] A VS Code setup that feels dependable from the first day.
- [ ] A path that repeatedly revisits Rust-specific ideas in richer contexts.

Pedagogical recommendation:

- [ ] Use an agile spiral, not waterfall.
- [ ] Avoid teaching ownership, errors, traits, lifetimes, async, testing, and GUI state as isolated one-time topics.
- [ ] Revisit important concepts repeatedly:
  - [ ] Ownership starts as move-vs-borrow.
  - [ ] Ownership returns with collections.
  - [ ] Ownership returns with persistence.
  - [ ] Ownership returns with GUI state.
  - [ ] Ownership returns with async boundaries.
  - [ ] Error handling starts with `Result`.
  - [ ] Error handling returns with persistence, CLI messages, HTTP, import/export, and GUI feedback.
  - [ ] Traits start as interface-like behavior.
  - [ ] Traits return as test seams, storage abstractions, parsing, display, and dependency boundaries.
- [ ] Build tiny vertical slices first.
- [ ] Refactor only after the learner has felt the pain the refactor solves.
- [ ] Keep the app running through the whole journey.

## Root Workspace Model

The root folder should be the learner's trusted home base.

- [ ] The root should contain the workspace manifest, documentation, VS Code configuration, chapter map, and sample data.
- [ ] The root should not itself become a large application crate.
- [ ] Real application code should live under `crates/`.
- [ ] Focused chapter experiments should live under `labs/`.
- [ ] Assignment instructions should live under `assignments/`.
- [ ] Longer explanations and recovery guides should live under `docs/`.
- [ ] Shared sample data should live under `sample_data/`.
- [ ] Testing support should be ordinary Cargo tests wherever possible.

Proposed structure:

```text
LearnRust/
  Cargo.toml
  rust-toolchain.toml
  .gitignore
  README.md
  MASTERPLAN.md

  .github/
    workflows/
      ci.yml

  .vscode/
    extensions.json
    settings.json
    tasks.json
    launch.json

  chapters/
    00-setup.md
    01-basics.md
    02-ownership.md

  crates/
    focus_forge_core/
    focus_forge_cli/
    focus_forge_gui/

  labs/
    ch00_setup/
    ch01_basics/
    ch02_ownership/
    ch03_domain_modeling/
    ch04_collections/
    ch05_errors/
    ch06_modules_workspace/
    ch07_persistence/
    ch08_cli/
    ch09_traits_generics/
    ch10_testing_refactoring/
    ch11_gui_foundations/
    ch12_gui_state/
    ch13_search_tags/
    ch14_time_sorting/
    ch15_async_http/
    ch16_import_export/
    ch17_polish_packaging/
    ch18_capstone/

  assignments/
    ch00-setup.md
    ch01-basics.md
    ch02-ownership.md
    ch03-domain-modeling.md
    ch04-collections.md
    ch05-errors.md
    ch06-modules-workspace.md
    ch07-persistence.md
    ch08-cli.md
    ch09-traits-generics.md
    ch10-testing-refactoring.md
    ch11-gui-foundations.md
    ch12-gui-state.md
    ch13-search-tags.md
    ch14-time-sorting.md
    ch15-async-http.md
    ch16-import-export.md
    ch17-polish-packaging.md
    ch18-capstone.md

  docs/
    compiler-errors.md
    dependencies.md
    getting-unstuck.md
    glossary.md
    math-track.md
    oop-to-rust.md
    platform-notes.md
    testing-assignments.md
    vscode-workflow.md

  examples/
    ownership_flow.rs
    result_flow.rs

  sample_data/
    demo_workspace.json
```

## Product Plus Sandbox Model

Each chapter should produce two kinds of progress.

Skill artifact:

- [ ] A focused lab under `labs/chXX_*`.
- [ ] Small enough to break without fear.
- [ ] Designed to isolate the chapter's hardest concept.
- [ ] Verified with `cargo check`, `cargo test`, or visible output.

Product artifact:

- [ ] A meaningful improvement to Focus Forge.
- [ ] Added to `focus_forge_core`, `focus_forge_cli`, or `focus_forge_gui`.
- [ ] Verified by tests, a CLI command, a saved file, or a GUI behavior.
- [ ] Kept small enough that the app never disappears behind architecture work.

Recommended balance:

- [ ] Chapters 0-3: mostly labs, tiny product changes.
- [ ] Chapters 4-8: balanced labs and product work.
- [ ] Chapters 9-12: mostly product work, targeted labs for harder Rust concepts.
- [ ] Chapters 13-18: product-first, with labs only for experiments and advanced topics.

This avoids two bad extremes:

- [ ] Do not make every chapter a totally separate project, because that fragments the sense of growth.
- [ ] Do not make everything happen inside one giant app immediately, because that makes early mistakes too expensive.

## Workspace And Cargo TODO

- [ ] Add a root `Cargo.toml` workspace manifest.
- [ ] Use `resolver = "2"`.
- [ ] Add workspace members gradually.
- [ ] Use shared dependency versions where practical.
- [ ] Create `crates/focus_forge_core` for domain logic.
- [ ] Create `crates/focus_forge_cli` for early interaction.
- [ ] Create `crates/focus_forge_gui` for the later desktop app.
- [ ] Create early labs first:
  - [ ] `labs/ch00_setup`
  - [ ] `labs/ch01_basics`
  - [ ] `labs/ch02_ownership`
- [ ] Scaffold later labs only when useful, so the repo does not feel huge on day one.
- [ ] Make these root commands work:
  - [ ] `cargo check`
  - [ ] `cargo test`
  - [ ] `cargo fmt`
  - [ ] `cargo clippy --all-targets --all-features`
  - [ ] `cargo run -p focus_forge_cli`
  - [ ] `cargo run -p focus_forge_gui`

## VS Code Environment TODO

The learner should be able to rely on VS Code as a stable cockpit for the course.

- [ ] Add `.vscode/extensions.json`.
- [ ] Recommend:
  - [ ] `rust-lang.rust-analyzer`
  - [ ] `vadimcn.vscode-lldb`
  - [ ] `tamasfe.even-better-toml`
- [ ] Add `.vscode/settings.json`.
- [ ] Configure rust-analyzer for the workspace.
- [ ] Consider enabling format on save for Rust.
- [ ] Add `.vscode/tasks.json`.
- [ ] Add tasks for:
  - [ ] Check all.
  - [ ] Test all.
  - [ ] Format all.
  - [ ] Clippy all.
  - [ ] Run current chapter lab.
  - [ ] Test current chapter lab.
  - [ ] Run Focus Forge CLI.
  - [ ] Run Focus Forge GUI.
- [ ] Add `.vscode/launch.json`.
- [ ] Add debug configurations for:
  - [ ] Current lab.
  - [ ] CLI.
  - [ ] GUI.
- [ ] Document the VS Code workflow in `docs/vscode-workflow.md`.

## Assignment Verification TODO

Every assignment should provide tools to test whether the work is done.

- [ ] Add a "How to verify" section to every assignment.
- [ ] Prefer commands the learner can run from the root.
- [ ] Use fast checks whenever possible:
  - [ ] `cargo check -p ch01_basics`
  - [ ] `cargo test -p ch03_domain_modeling`
  - [ ] `cargo run -p focus_forge_cli -- project list`
- [ ] Use unit tests for domain behavior.
- [ ] Use integration tests for persistence and CLI behavior.
- [ ] Use manual verification checklists for GUI chapters.
- [ ] Add screenshot or visual check prompts for GUI milestones.
- [ ] Include at least one deliberate compiler-error exercise per chapter.
- [ ] Keep tests educational rather than puzzle-like.

Suggested testing crates:

- [ ] `assert_cmd` for CLI behavior.
- [ ] `predicates` for CLI output assertions.
- [ ] `tempfile` for persistence tests.
- [ ] `insta` later for stable snapshot-style output.
- [ ] `proptest` later for property-style math and invariants.
- [ ] `trybuild` later for compile-fail ownership and trait lessons.

## Assignment File Template

Each chapter assignment should use a predictable structure.

- [ ] Title.
- [ ] What you will build.
- [ ] Why it matters.
- [ ] Rust concepts.
- [ ] OOP-to-Rust bridge.
- [ ] Math warm-up.
- [ ] Quick win.
- [ ] Guided tasks.
- [ ] Main build task.
- [ ] Acceptance criteria.
- [ ] How to verify.
- [ ] Common errors.
- [ ] Recovery path.
- [ ] Stretch tasks.
- [ ] Reflection questions.

## Checkpoints And Recovery

- [ ] Use git tags for stable chapter checkpoints:
  - [ ] `chapter-00-start`
  - [ ] `chapter-00-complete`
  - [ ] `chapter-01-complete`
  - [ ] `chapter-02-complete`
- [ ] Avoid branch-heavy workflows for learners unless there is a clear reason.
- [ ] Document how to compare the learner's work against a checkpoint.
- [ ] Document how to reset a lab without destroying personal app work.
- [ ] Provide sample solutions only after the first full curriculum pass exists.
- [ ] Prefer recovery guides over immediate answer keys.
- [ ] Add `docs/getting-unstuck.md`.
- [ ] Add `docs/compiler-errors.md`.
- [ ] Add `docs/testing-assignments.md`.

## Implementation Phases

Build the learning environment in small releases. Each phase should leave the repo in a usable state.

Phase 0: Foundation

- [ ] Update `README.md` with the course promise and first-run instructions.
- [ ] Add `rust-toolchain.toml`.
- [ ] Add `.gitignore`.
- [ ] Add root Cargo workspace.
- [ ] Add `.vscode/extensions.json`, `.vscode/settings.json`, and basic tasks.
- [ ] Add `docs/getting-unstuck.md`.
- [ ] Add `docs/compiler-errors.md`.
- [ ] Add the first runnable lab.

Phase 1: First Feedback Loop

- [ ] Create `chapters/00-setup.md`.
- [ ] Create `assignments/ch00-setup.md`.
- [ ] Create `labs/ch00_setup`.
- [ ] Ensure `cargo check`, `cargo test`, and the VS Code check task work.
- [ ] Add one intentionally broken compiler-error exercise.
- [ ] Add one passing test so the learner sees the green path early.

Phase 2: First Real Product Slice

- [ ] Create `crates/focus_forge_core`.
- [ ] Create `crates/focus_forge_cli`.
- [ ] Add the first project summary model.
- [ ] Add a CLI command that prints a hard-coded or sample project.
- [ ] Add unit tests for formatting and validation.
- [ ] Add Chapter 1 and Chapter 2 assignments.

Phase 3: Durable App Core

- [ ] Add domain models for projects, notes, tasks, status, priority, and tags.
- [ ] Add persistence with JSON.
- [ ] Add CLI commands for basic project and task workflows.
- [ ] Add sample data and temp-file tests.
- [ ] Add Chapters 3-8.

Phase 4: Abstraction, Testing, And Refactoring

- [ ] Add traits only where real boundaries exist.
- [ ] Add fake storage for tests.
- [ ] Add integration tests for CLI and persistence.
- [ ] Add refactoring assignments based on code the learner has already touched.
- [ ] Add Chapters 9-10.

Phase 5: GUI Payoff

- [ ] Create `crates/focus_forge_gui`.
- [ ] Open a basic `egui` window.
- [ ] Display real workspace data.
- [ ] Add editing, validation feedback, and saving.
- [ ] Add search and filters.
- [ ] Add Chapters 11-14.

Phase 6: Advanced Practical Rust

- [ ] Add optional async and HTTP enrichment.
- [ ] Add import, export, backups, settings, logging, and release builds.
- [ ] Add capstone extensions.
- [ ] Add Chapters 15-18.

Phase 7: Curriculum Hardening

- [ ] Run the course from a clean clone.
- [ ] Time each chapter.
- [ ] Fix unclear instructions.
- [ ] Tighten tests that are too brittle.
- [ ] Add CI.
- [ ] Tag stable chapter checkpoints.
- [ ] Decide whether to add solutions.

## Risk Register

- [ ] Risk: The course becomes too broad.
- [ ] Mitigation: Keep Focus Forge small and useful; move extra ideas to capstone extensions.
- [ ] Risk: Rust setup friction kills momentum.
- [ ] Mitigation: Make Chapter 0 tiny, testable, and heavily supported by VS Code tasks.
- [ ] Risk: Ownership feels like a wall.
- [ ] Mitigation: Teach ownership in repeated small contexts before large data structures and GUI state.
- [ ] Risk: The app architecture appears before the learner cares.
- [ ] Mitigation: Build vertical slices first, then refactor when the pain is visible.
- [ ] Risk: GUI work becomes too much too soon.
- [ ] Mitigation: Delay GUI until the CLI and core model have produced confidence.
- [ ] Risk: Math distracts from Rust.
- [ ] Mitigation: Keep math short, parallel, and tied to app behavior.
- [ ] Risk: Tests feel like arbitrary gates.
- [ ] Mitigation: Make tests explain user-visible behavior and include manual verification where appropriate.
- [ ] Risk: AI tools let learners skip understanding.
- [ ] Mitigation: Encourage hint-seeking, explanation, code review, and reflection prompts.
- [ ] Risk: The repo gets intimidating.
- [ ] Mitigation: Scaffold gradually and keep the first screen of README focused on what to do next.

## Naming And Convention TODO

- [ ] Use consistent chapter numbering: `ch00`, `ch01`, `ch02`.
- [ ] Use kebab-case for Markdown files, such as `ch02-ownership.md`.
- [ ] Use snake_case for crate and lab package names, such as `ch02_ownership`.
- [ ] Prefix lab packages clearly if Cargo package names become ambiguous, such as `lab_ch02_ownership`.
- [ ] Keep Focus Forge crates named:
  - [ ] `focus_forge_core`
  - [ ] `focus_forge_cli`
  - [ ] `focus_forge_gui`
- [ ] Keep public API names boring and obvious.
- [ ] Prefer domain names like `Project`, `Task`, `Note`, `Workspace`, `Storage`, and `WorkspaceError`.
- [ ] Avoid clever abbreviations in beginner-facing code.

## Chapter Dependency Contract

Each chapter should state what it depends on and what it promises to leave behind.

- [ ] Required starting checkpoint.
- [ ] Files the learner is expected to edit.
- [ ] Files the learner may read but should not need to edit.
- [ ] Commands that should work before starting.
- [ ] Commands that should work after finishing.
- [ ] Product capability added.
- [ ] Rust concept practiced.
- [ ] OOP bridge practiced.
- [ ] Math concept practiced.
- [ ] Recovery checkpoint if the learner gets lost.

## Operational Support TODO

These are the supporting systems that can make the difference between a good curriculum idea and a learning environment that actually holds up.

## Time Budget And Pacing TODO

- [ ] Give every chapter an estimated time range.
- [ ] Separate estimates for:
  - [ ] Quick win.
  - [ ] Guided assignment.
  - [ ] Main build.
  - [ ] Stretch tasks.
  - [ ] Math warm-up.
- [ ] Design early chapters to fit into 45-90 minute sessions.
- [ ] Let later chapters expand into 2-4 hour sessions when the learner has momentum.
- [ ] Add "stop points" where the learner can pause safely.
- [ ] Add "resume points" that explain what to run first when returning after days or weeks away.
- [ ] Include a short "minimum viable chapter" path for low-energy days.
- [ ] Include a "full force" path for days when the learner wants the harder version.

## Chapter Readiness Checklist

Before a chapter is considered ready, it should satisfy:

- [ ] The chapter starts from a known working state.
- [ ] The first command succeeds on a clean checkout.
- [ ] The quick win can be completed without solving the main assignment.
- [ ] The main assignment has clear acceptance criteria.
- [ ] The verification command is explicit.
- [ ] At least one common compiler error is anticipated and explained.
- [ ] The OOP-to-Rust bridge is present where relevant.
- [ ] The math warm-up is short and connected to the app.
- [ ] The learner can skip stretch tasks without losing the main path.
- [ ] The chapter ends with a visible app improvement or a clearly passed test.

## CI And Automation TODO

- [ ] Add GitHub Actions or equivalent CI once the workspace exists.
- [ ] Run on at least Windows and Linux.
- [ ] Consider macOS too if GUI build behavior becomes platform-sensitive.
- [ ] CI should run:
  - [ ] `cargo fmt --check`
  - [ ] `cargo check --workspace`
  - [ ] `cargo test --workspace`
  - [ ] `cargo clippy --workspace --all-targets --all-features`
- [ ] Keep CI fast enough that it reinforces confidence instead of becoming noise.
- [ ] Add a local script or documented command that mirrors CI.
- [ ] Avoid requiring learners to understand CI before they understand local Cargo workflows.
- [ ] Use CI as a maintainer safety net and a later teaching topic.

## Toolchain Pinning And Compatibility TODO

- [ ] Add `rust-toolchain.toml` once implementation begins.
- [ ] Pin to stable Rust, not nightly.
- [ ] Document how to update the toolchain with `rustup update`.
- [ ] Add `rustfmt.toml` only if the default style proves insufficient.
- [ ] Avoid exotic linker or platform setup in the main path.
- [ ] Prefer dependencies that work on Windows, macOS, and Linux.
- [ ] Document known platform quirks in `docs/platform-notes.md`.

## Dependency Policy TODO

- [ ] Introduce dependencies only when they teach something or remove unhelpful busywork.
- [ ] Prefer standard library first in early chapters.
- [ ] Add external crates gradually and explain why each one is worth using.
- [ ] Keep a dependency inventory in `docs/dependencies.md`.
- [ ] Suggested staged dependency introduction:
  - [ ] `serde` and `serde_json` for persistence.
  - [ ] `clap` for CLI command structure.
  - [ ] `thiserror` or hand-written errors depending on teaching value.
  - [ ] `anyhow` only at application boundaries, not in core domain logic.
  - [ ] `eframe` / `egui` for GUI.
  - [ ] `reqwest` and `tokio` for optional HTTP and async.
  - [ ] `directories` for platform-specific data paths.
  - [ ] `tracing` for logging once the app is large enough to need it.
- [ ] Explain when a crate is production-convenient but pedagogically too magical.
- [ ] Avoid hiding core Rust ideas behind helper crates too early.

## Accessibility And Ergonomics TODO

- [ ] Make docs readable in plain Markdown without special tooling.
- [ ] Keep line lengths and code snippets comfortable in VS Code.
- [ ] Use clear headings and consistent assignment structure.
- [ ] Avoid relying only on color in terminal or GUI output.
- [ ] Make CLI output readable in default terminals.
- [ ] Add keyboard-friendly GUI interactions where practical.
- [ ] Ensure GUI text is legible at common scaling settings.
- [ ] Explain any required fonts, icons, or visual assets.
- [ ] Keep screenshots optional, because learners may be on different platforms.

## Offline And Network Policy TODO

- [ ] The core course should work offline after dependencies are downloaded.
- [ ] Web requests should remain optional or have cached/sample-data alternatives.
- [ ] Network-dependent tests should be avoided by default.
- [ ] HTTP chapters should use mockable boundaries.
- [ ] Add sample JSON responses for offline async and HTTP exercises.
- [ ] Clearly label any task that requires internet access.

## Data Safety And Privacy TODO

- [ ] Treat learner-created Focus Forge data as valuable.
- [ ] Keep sample data separate from learner data.
- [ ] Avoid writing files outside the repo until the platform-data-path chapter.
- [ ] Before overwriting saved data, create backups in later chapters.
- [ ] Document where the app stores data.
- [ ] Never require committing personal learner data.
- [ ] Add `.gitignore` entries for local data, logs, target directories, and generated artifacts.

## Solutions And Hints Policy TODO

- [ ] Provide hints in layers:
  - [ ] Gentle nudge.
  - [ ] File or function to inspect.
  - [ ] Pseudocode.
  - [ ] Partial Rust snippet.
  - [ ] Full solution only when appropriate.
- [ ] Avoid full solutions in the main assignment text.
- [ ] Consider a `solutions/` area only after the curriculum has a stable first pass.
- [ ] Prefer "compare your approach" notes over copy-paste answer keys.
- [ ] Add "debugging questions" before direct fixes.
- [ ] Make the learner feel accompanied, not tested by a silent examiner.

## AI Assistance Policy TODO

- [ ] Assume learners may use AI tools while learning.
- [ ] Provide guidance on using AI without outsourcing understanding.
- [ ] Encourage questions like:
  - [ ] "Explain this compiler error in terms of ownership."
  - [ ] "Give me a hint, not the answer."
  - [ ] "Compare this Rust code to how I would write it in C#."
  - [ ] "Review my solution for idiomatic Rust."
- [ ] Discourage prompts that simply ask for complete assignment solutions.
- [ ] Include reflection prompts that force the learner to explain the final code in their own words.

## Assessment And Progress TODO

- [ ] Add a visible progress checklist for the whole course.
- [ ] Add per-chapter self-assessment:
  - [ ] I can run the code.
  - [ ] I can explain the new Rust concept.
  - [ ] I can fix one common error.
  - [ ] I can modify the feature without following exact steps.
  - [ ] I know what still feels unclear.
- [ ] Add periodic review chapters or review sections.
- [ ] Include "rebuild from memory" mini-exercises for confidence.
- [ ] Include "read unfamiliar Rust" exercises so the learner does not only write guided code.
- [ ] Include small code-review exercises where the learner identifies a bug, smell, or unnecessary clone.

## Code Quality Standards TODO

- [ ] Define what "good enough for this chapter" means.
- [ ] Avoid demanding production architecture before the learner is ready.
- [ ] Gradually introduce quality expectations:
  - [ ] Formatting.
  - [ ] Clear names.
  - [ ] Small functions.
  - [ ] Meaningful errors.
  - [ ] Tests for core behavior.
  - [ ] Separation of domain logic from UI.
  - [ ] Avoiding unnecessary clones.
  - [ ] Handling edge cases deliberately.
- [ ] Add code review checklists for later chapters.
- [ ] Make refactoring a recurring normal activity, not a punishment for earlier choices.

## Maintainer TODO

- [ ] Keep `MASTERPLAN.md` as the source of truth for curriculum direction.
- [ ] Keep `README.md` learner-facing and concise.
- [ ] Move detailed explanations into `docs/`.
- [ ] Keep chapter files focused on action.
- [ ] Update the masterplan when scope changes.
- [ ] Add a `CHANGELOG.md` once real implementation begins.
- [ ] Consider semantic versioning for curriculum releases.
- [ ] Tag stable milestones when chapters are complete.
- [ ] Periodically run through the course from a clean clone.

## Suggested Repository Structure

- [ ] `README.md` explains the course, prerequisites, and how to start.
- [ ] `MASTERPLAN.md` contains this long-term vision.
- [ ] `rust-toolchain.toml` pins the stable toolchain once implementation begins.
- [ ] `.gitignore` protects build output, local data, logs, and generated artifacts.
- [ ] `.github/workflows/ci.yml` verifies the workspace once code exists.
- [ ] `.vscode/` contains the recommended learner environment.
- [ ] `chapters/` contains chapter guides.
- [ ] `assignments/` contains assignment briefs and acceptance criteria.
- [ ] `crates/focus_forge_core/` contains domain models, validation, persistence, and tests.
- [ ] `crates/focus_forge_cli/` contains early command-line exercises.
- [ ] `crates/focus_forge_gui/` contains the final GUI application.
- [ ] `examples/` contains small focused Rust examples for concepts.
- [ ] `sample_data/` contains starter JSON files.
- [ ] `docs/` contains explanations that are too long for chapter files.

## Course Arc

### Chapter 0: Setup And First Run

- [ ] Goal: Make the learner productive in the repo.
- [ ] Psychological goal: prove immediately that the environment works and that the learner can still make code respond.
- [ ] Explain installing Rust through `rustup`.
- [ ] Explain checking `cargo --version` and `rustc --version`.
- [ ] Explain opening the repo in VS Code.
- [ ] Explain what `rust-analyzer` provides.
- [ ] Add a tiny `hello_rust` binary or example.
- [ ] Teach `cargo run`, `cargo check`, `cargo test`, and `cargo fmt`.

Assignments:

- [ ] Run the starter program.
- [ ] Change the greeting.
- [ ] Add one command-line argument.
- [ ] Break the code intentionally and read the compiler error.
- [ ] Fix formatting with `cargo fmt`.
- [ ] End with a tiny visible success message chosen by the learner.

Product improvement:

- [ ] The repo proves the toolchain works.

Math warm-up:

- [ ] Refresh arithmetic with percentages, ratios, averages, and units.
- [ ] Use small hand calculations to predict program output before running it.

### Chapter 1: Variables, Functions, And Basic Types

- [ ] Goal: Learn the basic shape of Rust programs.
- [ ] Psychological goal: make Rust feel like programming again before introducing the parts that feel alien.
- [ ] Cover `let`, mutability, functions, expressions, `String`, `&str`, integers, booleans, and simple formatting.
- [ ] Introduce the first idea of Focus Forge: a project name and a short description.
- [ ] Compare Rust functions and bindings with methods, local variables, and constants in C#, Java, and TypeScript.

Assignments:

- [ ] Build a small CLI that prints a project summary.
- [ ] Add functions for formatting status lines.
- [ ] Add simple validation, such as rejecting an empty project name.
- [ ] Add tests for formatting functions.

Product improvement:

- [ ] Focus Forge can display a single hard-coded project summary.

Math warm-up:

- [ ] Refresh functions as input-output mappings.
- [ ] Compare mathematical functions with Rust functions.
- [ ] Add tiny calculations such as completion percentage or estimated remaining work.

### Chapter 2: Ownership, Borrowing, And References

- [ ] Goal: Make ownership feel useful instead of mysterious.
- [ ] Psychological goal: reduce fear of the borrow checker by using small, concrete examples with visible data flow.
- [ ] Cover moving, borrowing, references, cloning, and function signatures.
- [ ] Use project data as the recurring example.
- [ ] Contrast ownership with garbage-collected object references.
- [ ] Explain borrowing as temporary access, not as "Rust pointers" in the C sense or "object references" in the Java sense.

Assignments:

- [ ] Write functions that borrow project data without taking ownership.
- [ ] Write one function that intentionally consumes a value.
- [ ] Refactor duplicated string handling.
- [ ] Explain in comments why a borrow is better than a clone in one place.

Product improvement:

- [ ] Focus Forge can format and inspect project data without unnecessary copying.

Math warm-up:

- [ ] Refresh sets, subsets, and membership.
- [ ] Use projects, tags, and tasks as concrete examples of set relationships.
- [ ] Discuss why copying data has a cost, using simple size estimates.

### Chapter 3: Structs, Enums, And Domain Modeling

- [ ] Goal: Model real application concepts.
- [ ] Cover structs, methods, associated functions, enums, `Option`, and pattern matching.
- [ ] Compare structs plus `impl` to classes without inheritance.
- [ ] Compare Rust enums to sealed classes, discriminated unions, and TypeScript union types.
- [ ] Present `Option` as the normal replacement for nullable fields.
- [ ] Create core domain types:
  - [ ] `Project`
  - [ ] `Task`
  - [ ] `Note`
  - [ ] `ProjectStatus`
  - [ ] `Priority`

Assignments:

- [ ] Move domain types into a core crate or module.
- [ ] Add constructors for valid domain objects.
- [ ] Add methods like `complete_task`, `rename_project`, and `add_note`.
- [ ] Use enums for status and priority instead of raw strings.
- [ ] Add tests for status transitions.

Product improvement:

- [ ] Focus Forge has a real internal model.

Math warm-up:

- [ ] Refresh logic, truth tables, implication, conjunction, disjunction, and negation.
- [ ] Map enum states and validation rules to logical predicates.
- [ ] Express valid project states as simple invariants.

### Chapter 4: Collections And Iterators

- [ ] Goal: Work with lists of real data.
- [ ] Cover `Vec`, slices, iterator methods, `HashMap`, sorting, filtering, and collecting.
- [ ] Introduce multiple projects and tasks.

Assignments:

- [ ] Store several projects in memory.
- [ ] List all projects.
- [ ] Filter projects by status.
- [ ] Find overdue or high-priority tasks.
- [ ] Count tasks per project.
- [ ] Sort projects by name or updated timestamp.

Product improvement:

- [ ] Focus Forge can manage a small in-memory workspace.

Math warm-up:

- [ ] Refresh sequences, finite collections, counting, and basic combinatorics.
- [ ] Use filters and counts to reason about tasks by status, priority, and tag.
- [ ] Estimate the runtime difference between scanning one list and using a lookup map.

### Chapter 5: Error Handling That Feels Humane

- [ ] Goal: Learn Rust's recoverable error style.
- [ ] Psychological goal: replace exception-era habits with a sense that explicit failure paths make programs calmer.
- [ ] Cover `Result`, `Option`, `?`, custom errors, and user-facing messages.
- [ ] Avoid panics except in tests or truly impossible states.
- [ ] Compare `Result` with checked exceptions, unchecked exceptions, and TypeScript promise rejection patterns.
- [ ] Teach that explicit error return types make failure part of the API.

Assignments:

- [ ] Replace fragile code with `Result`.
- [ ] Add a custom error enum.
- [ ] Validate project names, task titles, and duplicate IDs.
- [ ] Write tests for invalid operations.
- [ ] Make CLI output helpful when something fails.

Product improvement:

- [ ] Focus Forge handles bad input gracefully.

Math warm-up:

- [ ] Refresh proof by cases.
- [ ] Treat error handling as exhaustive case analysis.
- [ ] Write small tables of possible inputs and expected outcomes.

### Chapter 6: Modules, Crates, And Project Organization

- [ ] Goal: Teach how Rust projects stay maintainable.
- [ ] Cover modules, visibility, crate boundaries, workspaces, and public APIs.
- [ ] Split application logic from interface code.
- [ ] Compare crates and modules with packages, namespaces, assemblies, npm packages, and Maven or NuGet dependencies.

Assignments:

- [ ] Create a Cargo workspace.
- [ ] Move domain logic into `focus_forge_core`.
- [ ] Keep the CLI as a separate binary crate.
- [ ] Expose a small public API from the core crate.
- [ ] Add integration tests.

Product improvement:

- [ ] Focus Forge now has a reusable core library.

Math warm-up:

- [ ] Refresh relations and mappings.
- [ ] Model module boundaries as mappings between public operations and hidden implementation details.
- [ ] Reason about which functions should be part of the public interface.

### Chapter 7: Files, Serialization, And Persistence

- [ ] Goal: Save and load real data.
- [ ] Cover filesystem basics, paths, JSON, `serde`, and version-tolerant data shapes.
- [ ] Introduce `Workspace` as the top-level saved state.

Assignments:

- [ ] Add `serde` support for domain models.
- [ ] Save workspace data to JSON.
- [ ] Load workspace data from JSON.
- [ ] Handle missing files by creating an empty workspace.
- [ ] Add sample data.
- [ ] Add round-trip tests.

Product improvement:

- [ ] Focus Forge remembers data between runs.

Math warm-up:

- [ ] Refresh trees and graph-shaped data at a conceptual level.
- [ ] Compare in-memory object graphs with serialized JSON trees.
- [ ] Discuss data schemas as structured mathematical objects.

### Chapter 8: Command-Line Interface As A Training Ground

- [ ] Goal: Build useful interaction before adding a GUI.
- [ ] Cover command parsing, user input, simple output tables, and command structure.
- [ ] Optional crate: `clap`.
- [ ] Compare CLI command handlers with controller or service methods from typical application architecture.

Assignments:

- [ ] Add commands like `project add`, `project list`, `task add`, `task done`, and `note add`.
- [ ] Read and write the workspace file.
- [ ] Print clear success and error messages.
- [ ] Add at least one command test or core-level test for each operation.

Product improvement:

- [ ] Focus Forge is a usable CLI project tracker.

Math warm-up:

- [ ] Refresh formal grammars informally through command syntax.
- [ ] Treat CLI commands as structured input languages.
- [ ] Sketch simple parse trees for commands like `task add`.

### Chapter 9: Traits, Generics, And Reusable Behavior

- [ ] Goal: Learn abstraction only after it solves a visible problem.
- [ ] Cover traits, trait bounds, generics, `Display`, `From`, `TryFrom`, and simple repository patterns.
- [ ] Compare traits with interfaces while showing the differences: static dispatch, trait bounds, blanket implementations, and trait objects.
- [ ] Explain when a trait object feels like interface-based polymorphism and when generics are a better fit.

Assignments:

- [ ] Implement `Display` for status and priority.
- [ ] Use `TryFrom<&str>` for parsing status or priority.
- [ ] Create a storage trait for loading and saving workspaces.
- [ ] Implement file-backed storage.
- [ ] Add an in-memory fake storage for tests.

Product improvement:

- [ ] Focus Forge can swap storage implementations in tests.

Math warm-up:

- [ ] Refresh abstraction, equivalence, and algebraic laws.
- [ ] Define expected laws for storage, such as save-then-load returning equivalent data.
- [ ] Compare trait contracts with mathematical interfaces.

### Chapter 10: Testing, Debugging, And Refactoring

- [ ] Goal: Make change less scary.
- [ ] Psychological goal: teach the learner how to regain control when code starts feeling tangled.
- [ ] Cover unit tests, integration tests, test fixtures, `dbg!`, logging basics, and refactoring in small steps.

Assignments:

- [ ] Add tests for project lifecycle behavior.
- [ ] Add tests for persistence behavior.
- [ ] Refactor one messy function into smaller functions.
- [ ] Add a bug intentionally, write a failing test, then fix it.
- [ ] Add `cargo clippy` to the recommended workflow.

Product improvement:

- [ ] Focus Forge has enough tests to support larger changes.

Math warm-up:

- [ ] Refresh quantifiers with examples like "for all tasks" and "there exists a matching project".
- [ ] Turn informal requirements into testable propositions.
- [ ] Introduce simple property-style thinking without requiring a property testing crate yet.

### Chapter 11: GUI Foundations With egui

- [ ] Goal: Create the first desktop window.
- [ ] Psychological goal: deliver a strong visual reward after the foundational Rust chapters.
- [ ] Cover event loops at a high level, immediate-mode GUI concepts, app state, panels, buttons, labels, text input, and simple lists.
- [ ] Keep the UI modest and functional.
- [ ] Contrast immediate-mode GUI with component-based UI patterns from JavaFX, WinForms, WPF, React, or TypeScript frontend frameworks.

Assignments:

- [ ] Create `focus_forge_gui`.
- [ ] Open a window with the app name.
- [ ] Show the list of projects from sample data.
- [ ] Add a text input for a new project.
- [ ] Add a button that creates a project in memory.
- [ ] Keep domain changes inside the core crate.

Product improvement:

- [ ] Focus Forge has a basic desktop GUI.

Math warm-up:

- [ ] Refresh coordinate systems, rectangles, alignment, and proportional sizing.
- [ ] Connect layout decisions to geometry.
- [ ] Reason about UI state transitions as a small state machine.

### Chapter 12: GUI State, Editing, And Feedback

- [ ] Goal: Build an application that feels coherent.
- [ ] Cover selected items, forms, validation feedback, temporary UI state, and saving changes.

Assignments:

- [ ] Add a project detail view.
- [ ] Edit project name and description.
- [ ] Add tasks from the GUI.
- [ ] Complete tasks from the GUI.
- [ ] Show validation errors without crashing.
- [ ] Save automatically or with a clear save action.

Product improvement:

- [ ] Focus Forge is useful for managing real project notes and tasks.

Math warm-up:

- [ ] Refresh finite state machines.
- [ ] Model selected project, edit mode, validation state, and save state as transitions.
- [ ] Identify impossible UI states and prevent them in code.

### Chapter 13: Search, Filters, And Tags

- [ ] Goal: Add the kind of feature that makes the app actually pleasant.
- [ ] Cover text search, case handling, simple indexing decisions, and UI filtering.

Assignments:

- [ ] Add tags to projects and tasks.
- [ ] Filter by status, priority, and tag.
- [ ] Search across project names, descriptions, notes, and tasks.
- [ ] Highlight or clearly show matching results.
- [ ] Add tests for search behavior.

Product improvement:

- [ ] Focus Forge can handle enough data to need organization.

Math warm-up:

- [ ] Refresh vector spaces at an intuitive level through search scoring.
- [ ] Introduce term frequency, simple ranking, and weighted matches without heavy theory.
- [ ] Compare exact matching, partial matching, and scored matching.

### Chapter 14: Time, Sorting, And Small Product Decisions

- [ ] Goal: Handle real-world fields and product tradeoffs.
- [ ] Cover timestamps, date formatting, sorting, and deterministic tests.
- [ ] Suggested crate: `chrono` or `time`.

Assignments:

- [ ] Add created and updated timestamps.
- [ ] Sort by recently updated.
- [ ] Add due dates for tasks.
- [ ] Show overdue tasks.
- [ ] Make tests deterministic by injecting time or isolating time-dependent logic.

Product improvement:

- [ ] Focus Forge becomes a practical planner, not just a notebook.

Math warm-up:

- [ ] Refresh discrete time, intervals, ordering, and calendar arithmetic.
- [ ] Discuss edge cases around time zones and date boundaries.
- [ ] Use deterministic clocks in tests to make time-dependent behavior predictable.

### Chapter 15: Async And Web Requests, Used Sparingly

- [ ] Goal: Learn async Rust in a practical, bounded way.
- [ ] Cover `async`, `.await`, `tokio`, `reqwest`, timeouts, and error handling.
- [ ] Keep web features optional.

Assignments:

- [ ] Add a command or GUI action that fetches Rust release information or documentation links.
- [ ] Parse a small JSON response.
- [ ] Cache the result locally.
- [ ] Show network errors clearly.
- [ ] Add a mockable boundary so tests do not depend on the live internet.

Product improvement:

- [ ] Focus Forge can enrich the local workspace with a small external data source.

Math warm-up:

- [ ] Refresh probability and uncertainty through unreliable networks.
- [ ] Discuss retry limits, timeouts, and expected failure rates.
- [ ] Model cache freshness with simple time intervals.

### Chapter 16: Import, Export, And Backups

- [ ] Goal: Treat user data with respect.
- [ ] Cover file dialogs if practical, backup files, schema versions, and migration basics.

Assignments:

- [ ] Export a workspace to JSON.
- [ ] Import a workspace from JSON.
- [ ] Create a backup before overwriting data.
- [ ] Add a simple data version field.
- [ ] Write tests for importing old or partial data.

Product improvement:

- [ ] Focus Forge data is portable and safer to modify.

Math warm-up:

- [ ] Refresh versioning as ordered states.
- [ ] Think of migrations as functions from old data shapes to new data shapes.
- [ ] Discuss reversibility, partial functions, and data loss.

### Chapter 17: Polish, Packaging, And The Real App Feel

- [ ] Goal: Turn a learning project into something shareable.
- [ ] Cover release builds, app icons, configuration directories, logging, and basic packaging notes.

Assignments:

- [ ] Add `cargo build --release` instructions.
- [ ] Store data in a sensible platform-specific location.
- [ ] Add a settings panel.
- [ ] Add light/dark theme support if the GUI stack makes it easy.
- [ ] Add empty states and helpful messages.
- [ ] Create a small demo workspace.

Product improvement:

- [ ] Focus Forge feels like a small real application.

Math warm-up:

- [ ] Refresh descriptive statistics.
- [ ] Add optional progress summaries such as completed task ratios, streaks, and recent activity counts.
- [ ] Use charts sparingly where they clarify real data.

### Chapter 18: Capstone Extensions

- [ ] Goal: Let learners choose a direction without leaving the course unfinished.
- [ ] Psychological goal: turn the learner from course-follower back into independent builder.
- [ ] Offer several extension tracks:
  - [ ] Markdown notes.
  - [ ] Recurring tasks.
  - [ ] Kanban board view.
  - [ ] Pomodoro timer.
  - [ ] Local full-text search.
  - [ ] SQLite persistence.
  - [ ] Calendar export.
  - [ ] Simple plugin-like command system.
  - [ ] Sync-ready architecture without implementing sync.

Assignments:

- [ ] Pick one extension.
- [ ] Write a short design note.
- [ ] Implement it behind a clean boundary.
- [ ] Add tests for the core behavior.
- [ ] Update the user guide.

Product improvement:

- [ ] Focus Forge becomes personally useful to the learner.

Math warm-up:

- [ ] Let learners choose a math-flavored extension if they want one.
- [ ] Possible extensions include scheduling heuristics, search scoring, habit statistics, graph views, or optimization of daily task selection.
- [ ] Require a short explanation of the mathematical idea behind the chosen extension.

## Parallel Math Track

- [ ] Treat math as a companion thread, not a gate.
- [ ] Assume university-level math ability can return with practice.
- [ ] Start with concrete arithmetic, logic, sets, and functions.
- [ ] Gradually reintroduce discrete math, state machines, probability, statistics, and light linear algebra.
- [ ] Tie each math topic to something visible in Focus Forge.
- [ ] Keep math exercises short enough to fit beside programming assignments.
- [ ] Include optional deeper problems for learners who want the university-level challenge sooner.
- [ ] Avoid making early Rust progress depend on advanced math.
- [ ] Use math to sharpen thinking about invariants, correctness, tests, performance, and product behavior.

Suggested math progression:

- [ ] Chapters 0-2: arithmetic, percentages, functions, sets, and data size estimates.
- [ ] Chapters 3-5: logic, predicates, invariants, proof by cases, and error tables.
- [ ] Chapters 6-8: relations, mappings, trees, structured data, and informal grammars.
- [ ] Chapters 9-10: abstraction, algebraic laws, quantifiers, and property-style reasoning.
- [ ] Chapters 11-12: geometry, layout, state machines, and transition diagrams.
- [ ] Chapters 13-14: search scoring, discrete time, ordering, and deterministic testing.
- [ ] Chapters 15-16: probability, uncertainty, cache freshness, version ordering, and migrations.
- [ ] Chapters 17-18: descriptive statistics, visualization, heuristics, optimization, and learner-chosen mathematical extensions.

Math assignment format:

- [ ] One short refresher explanation.
- [ ] One hand-solved example.
- [ ] One Rust-backed exercise that computes or validates the idea.
- [ ] One app integration idea.
- [ ] One optional university-level stretch problem.

## Assignment Design Rules

- [ ] Every assignment should include:
  - [ ] Learning objective.
  - [ ] Starting point.
  - [ ] Tasks.
  - [ ] Acceptance criteria.
  - [ ] Hints.
  - [ ] Stretch goal.
  - [ ] Reflection question.
- [ ] Assignments should increase in difficulty inside each chapter.
- [ ] Earlier assignments should be specific.
- [ ] Later assignments should leave more design choices to the learner.
- [ ] Avoid requiring learners to guess hidden implementation details.
- [ ] Include compiler-error reading exercises.
- [ ] Include refactoring exercises after enough code exists.
- [ ] Include occasional "explain this signature" prompts.

## Rust Concepts To Cover Deliberately

- [ ] Toolchain basics: `cargo`, `rustc`, `rustup`, `rustfmt`, `clippy`.
- [ ] Variables and mutability.
- [ ] Functions and expressions.
- [ ] Ownership, borrowing, references, and lifetimes at an introductory level.
- [ ] Structs, enums, pattern matching, and methods.
- [ ] `Option` and `Result`.
- [ ] Collections and iterators.
- [ ] Modules, crates, workspaces, and visibility.
- [ ] Serialization and deserialization with `serde`.
- [ ] File IO and paths.
- [ ] Error types and user-facing errors.
- [ ] Traits and generics.
- [ ] Testing and test organization.
- [ ] CLI development.
- [ ] GUI development with app state.
- [ ] Async and HTTP as an optional advanced feature.
- [ ] Configuration and data directories.
- [ ] Release builds and practical packaging.

## Things To Avoid

- [ ] Do not make the course primarily about algorithms.
- [ ] Do not make the final app a throwaway todo list with no personality.
- [ ] Do not start with lifetimes as an abstract topic.
- [ ] Do not overuse macros early.
- [ ] Do not require nightly Rust.
- [ ] Do not introduce embedded Rust, unsafe Rust, or low-level systems programming in the main path.
- [ ] Do not make web requests required for core progress.
- [ ] Do not hide too much behind libraries before the learner understands the shape of the problem.
- [ ] Do not turn GUI work into a design showcase; keep it useful, clear, and maintainable.

## Fun And Worthwhile Details

- [ ] Use sample project names that feel alive:
  - [ ] "Build a Tiny Synth"
  - [ ] "Learn Rust Ownership"
  - [ ] "Plan a Weekend Hike"
  - [ ] "Ship My First Desktop App"
- [ ] Add small celebrations for completed tasks in the GUI.
- [ ] Include a progress dashboard.
- [ ] Let learners tag projects with moods or contexts, such as `deep-work`, `quick-win`, `blocked`, or `weekend`.
- [ ] Add a "Today" view near the end of the course.
- [ ] Add a "What changed?" panel that shows recently edited projects.
- [ ] Make import/export good enough that learners can keep using their own data after the course.

## Documentation TODO

- [ ] Expand `README.md` with:
  - [ ] Course pitch.
  - [ ] Prerequisites.
  - [ ] Setup instructions.
  - [ ] Chapter index.
  - [ ] Common troubleshooting.
- [ ] Create `chapters/00-setup.md`.
- [ ] Create `chapters/01-basics.md`.
- [ ] Create assignment templates.
- [ ] Add a glossary for common Rust terms.
- [ ] Add a "when the compiler is angry" guide.
- [ ] Add a "how to ask for help" guide.
- [ ] Add `docs/vscode-workflow.md`.
- [ ] Add `docs/testing-assignments.md`.
- [ ] Add `docs/dependencies.md`.
- [ ] Add `docs/platform-notes.md`.
- [ ] Add `docs/oop-to-rust.md`.
- [ ] Add `docs/math-track.md`.

## Build TODO

- [ ] Initialize a Cargo workspace.
- [ ] Add `rust-toolchain.toml`.
- [ ] Add `.gitignore`.
- [ ] Add `.vscode/` recommendations and tasks.
- [ ] Add a tiny starter binary.
- [ ] Add `focus_forge_core`.
- [ ] Add `focus_forge_cli`.
- [ ] Add `focus_forge_gui`.
- [ ] Add formatting and linting instructions.
- [ ] Add sample data.
- [ ] Add a basic test suite.
- [ ] Add CI after local commands are stable.
- [ ] Add chapter tags or branches only if they make the learning flow easier.

## Definition Of Done For The Learning Environment

- [ ] A learner can clone the repo and run the first exercise in under ten minutes.
- [ ] A learner can open the repo in VS Code and receive useful extension recommendations.
- [ ] A learner can run `cargo check`, `cargo test`, and `cargo fmt` from the root.
- [ ] Each chapter produces a visible improvement.
- [ ] The assignments are ordered from guided to open-ended.
- [ ] Every assignment has a clear verification command or manual verification checklist.
- [ ] Every chapter has a quick win, a main build task, and optional stretch tasks.
- [ ] Every chapter has a recovery path for common mistakes.
- [ ] Time estimates and safe stop points are documented.
- [ ] The final GUI app is useful enough that the learner might keep using it.
- [ ] The codebase demonstrates normal, idiomatic Rust without becoming intimidating.
- [ ] The course teaches confidence with compiler feedback.
- [ ] The core course works without live web services.
- [ ] Learner-created data is not accidentally committed or overwritten.
- [ ] CI verifies formatting, checking, linting, and tests once code exists.
- [ ] The dependency list is intentional and explained.
- [ ] The OOP-to-Rust bridge and parallel math track are present throughout the course.
- [ ] The repo remains approachable from a clean checkout.
