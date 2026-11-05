# WS7: Error explainer

Agent: explain
Branch: `ws/explain`
Wave: 2
Depends on: WS0, WS2, WS4
Merge order: 8

## Goal

Turn the top compiler errors into two plain sentences and one link. Most beginners do not need a better compiler. They need a translation.

## Owns

- `rustlab/crates/rustlab-explain/**`
- `rustlab/content/errors.json`

## Must not touch

- `rustlab/crates/rustlab-core/**`
- `rustlab/crates/rustlab-check/**`
- `rustlab/crates/rustlab-cli/**`
- `rustlab/content/tracks/**`

The CLI integration is a small follow-up PR by the integrator. Do not edit the CLI crate in this workstream.

## Depends on

`rustlab-core::Diagnostic` and the diagnostic parser from WS2.

## Deliverables

`rustlab/content/errors.json`:

```json
{
  "schema_version": 1,
  "errors": [
    {
      "code": "E0382",
      "title": "Use of moved value",
      "body": "The value was moved into another function or variable, so the original binding no longer owns it. Borrow it with &, clone it, or pass it back.",
      "link": "https://doc.rust-lang.org/error_codes/E0382.html"
    }
  ]
}
```

`rustlab-explain`:

```rust
pub struct Explainer {
    pub code: String,
    pub title: String,
    pub body: String,
    pub link: String,
}

pub struct ExplainerIndex { /* ... */ }

impl ExplainerIndex {
    pub fn load(path: &Path) -> anyhow::Result<Self>;
    pub fn explain(&self, code: &str) -> Option<&Explainer>;
    pub fn explain_first(&self, diagnostics: &[Diagnostic]) -> Option<&Explainer>;
}
```

`explain_first` walks diagnostics in order, returns the first error with a known code, and checks children if the parent has no code.

## Error list

Cover these 25 codes:

E0382, E0502, E0499, E0597, E0716, E0515, E0106, E0277, E0308, E0433, E0425, E0412, E0061, E0599, E0507, E0603, E0616, E0624, E0658, E0310, E0521, E0596, E0503, E0505, E0508.

For each:

- Title: 2 to 5 words, plain language.
- Body: 2 sentences, 120 to 350 characters. Explain the cause and the fix. Do not repeat the compiler message.
- Link: the official Error Index page.

Examples of the tone:

- E0382: "The value was moved into another function or variable, so the original binding no longer owns it. Borrow it with &, clone it, or pass it back."
- E0502: "You asked for a mutable borrow while an immutable borrow was still alive. End the first borrow before you mutate, or restructure the function so the two uses do not overlap."
- E0597: "A reference outlived the value it points to. Move the value so it lives longer, or return an owned value instead of a reference."

## Tests

- The JSON parses and has schema version 1.
- All 25 codes are present and unique.
- Every body is 120 to 350 characters.
- Every link starts with `https://doc.rust-lang.org/error_codes/` and ends with the code.
- `explain` returns `None` for an unknown code.
- `explain_first` skips warnings and notes.
- `explain_first` finds a code in a child diagnostic.
- A fixture with a moved-value error maps to E0382.

Do not make network requests in tests. Check the link shape, not the HTTP status.

## Acceptance

From `rustlab/`:

```text
cargo test -p rustlab-explain
cargo clippy -p rustlab-explain --all-targets -- -D warnings
cargo fmt -p rustlab-explain -- --check
```

## Deliverable

A PR titled `explain: map the top 25 compiler errors to plain English`.

The PR description must include three sample explanations and the test output.
