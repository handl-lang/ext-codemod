# ext-codemod

`hnl-ext-codemod` — `hnl codemod` as a dynamic `hnl` extension.

An `ext-*` crate: the declarative codemod pipeline packaged as a
`libhnl_ext_codemod.{so,dylib,dll}` shared library for the dumb `hnl` host. The
host discovers it via `--ext-dir`, `$HANDL_EXT_DIR`, `~/.handl/ext/`, or a dev
`target/debug/` next to the `hnl` binary.

PL-004 T-086. The spec model (`codemod.yaml`: targets, rules, the
plan/apply/revert lifecycle data) is `hnl-tool-codemod` in the sibling
`tool-codemod` repo; the registry lives in `handl-plugin-codemod` and the
command surface in `handl-plugin-ext-codemod`, both in the `host` repo.

Codemod **never destroys** — the lifecycle is `plan` → `apply` → `revert`;
`revert` is the only undo path.

## Commands

```text
hnl codemod [plan|apply|revert] <spec> [--json] [--check]
```

The canonical kit's language is injected into `codemod_command` exactly as the
built-in host did (its own `eprintln!` diagnostics reach the host's stderr
verbatim in-process).

## Development

```bash
cargo run --manifest-path xtask/Cargo.toml -- gate   # fmt → clippy → test → doc
```

`xtask/` provides the gate tasks (`fmt`, `clippy`, `test`, `doc`, `check`,
`gate`); `workflows/gate.yaml` is the per-push gate — the `handl-gates` bundle's rust campaign (fmt → clippy → test → doc), run by bupha/the fleet runner via `peregrin campaign run`. The crate
builds a `cdylib` + `rlib`; `cargo test` links the `rlib` statically.

Deps resolve from sibling checkouts via the `[patch]` table in `Cargo.toml`
(host@main does not yet carry `handl-plugin-codemod` /
`handl-plugin-ext-codemod`, and `tool-codemod` has no git remote) — keep the
`handl-lang/*` sibling layout.
