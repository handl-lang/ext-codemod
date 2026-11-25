# ext-codemod

`hnl-ext-codemod` — `hnl codemod` as a dynamic `hnl` extension: the
declarative codemod pipeline packaged as a
`libhnl_ext_codemod.{so,dylib,dll}` cdylib the dumb `hnl` host discovers
at runtime.

## Layout

| path | role |
| --- | --- |
| `src/` | extension ABI exports + `codemod` command wiring |
| `xtask/` | vendored gate crate (`fmt`, `clippy`, `test`, `doc`, `gate`) |
| `workflows/gate.yaml` | per-push gate — `handl-gates` rust campaign via bupha |

## Contract

- PL-004 T-086. The spec model (`codemod.yaml`: targets, rules, the
  plan/apply/revert lifecycle data) is `hnl-tool-codemod` in the sibling
  `tool-codemod` repo; the registry lives in `handl-plugin-codemod` and
  the command surface in `handl-plugin-ext-codemod`, both in `host`.
- Commands: `hnl codemod [plan|apply|revert] <spec> [--json] [--check]`.
- Codemod **never destroys** — the lifecycle is `plan` → `apply` →
  `revert`; `revert` is the only undo path.
- Builds `cdylib` + `rlib`; `cargo test` links the `rlib` statically.

## Verification

```bash
cargo run --manifest-path xtask/Cargo.toml -- gate   # fmt → clippy → test → doc
```

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
