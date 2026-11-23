//! `hnl-ext-codemod` — the `codemod` extension for `hnl` (ADR-0149, PL-004).
//!
//! The pipeline lives in `handl-plugin-codemod` (the codemod registry over
//! the `hnl-engine-codemod` declarative spec model) plus the command surface
//! linked through `handl-plugin-ext-codemod`. This crate is the *packaging*:
//! it exports the ADR-0149 ABI (`handl_ext_manifest` / `handl_ext_dispatch` /
//! `handl_ext_free`) and routes the
//! `hnl codemod [plan|apply|revert] <spec> [--json] [--check]` argv to
//! [`handl_plugin_ext_codemod::codemod_command`] with the canonical kit's
//! language — byte-for-byte the semantics the built-in `codemod` arm had
//! before the host went dumb.
//!
//! Codemod NEVER destroys: the lifecycle is `plan` (dry-run diff) →
//! `apply` (writes through the manifest/journal) → `revert` (the only undo
//! path — there is no `destroy`).
//!
//! `codemod_command` renders its own diagnostics with `eprintln!`;
//! running in-process those land on the host's stderr verbatim, so the
//! response carries only the exit code.

use std::path::Path;

use handl_ext::{ExtRequest, ExtResponse, ExtensionManifest};
use handl_kit_pkg::KitProjectExt;
use handl_kit_prelude::Kit;

handl_ext::export!(
    ExtensionManifest::new("codemod")
        .version(env!("CARGO_PKG_VERSION"))
        .command(
            "codemod",
            "codemod [plan|apply|revert] <spec> [--json] [--check]",
        ),
    dispatch
);

/// ABI dispatch: only the `codemod` command is declared.
fn dispatch(cmd: &str, req: &ExtRequest) -> ExtResponse {
    match cmd {
        "codemod" => run(&req.args, Path::new(&req.cwd)),
        other => ExtResponse::error(2, format!("codemod: unknown command `{other}`\n")),
    }
}

/// Run `codemod_command` with the project-discovered (or canonical) kit's
/// language — the same `with_tooling_kit` injection the built-in host used
/// (ADR-0128 §"The 0.8 seam"). `codemod_command` prints to stderr itself;
/// the response carries the exit code only.
fn run(args: &[String], cwd: &Path) -> ExtResponse {
    let kit = Kit::discover(cwd)
        .or_else(|_| Kit::new())
        .expect("canonical kit must assemble");
    let code = handl_plugin_ext_codemod::codemod_command(kit.lang(), args);
    ExtResponse {
        code,
        stdout: String::new(),
        stderr: String::new(),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn no_args_is_usage_error() {
        // `codemod_command` prints the usage line to the process stderr.
        let res = run(&[], Path::new("."));
        assert_eq!(res.code, 2);
    }

    #[test]
    fn unknown_command_is_usage_error() {
        // Dispatch-level guard: only `codemod` is declared.
        let req = ExtRequest {
            args: vec![],
            cwd: ".".into(),
            env: Default::default(),
        };
        let res = dispatch("bogus", &req);
        assert_eq!(res.code, 2);
    }

    /// The `export!`-generated manifest symbol, called directly.
    #[test]
    fn manifest_declares_codemod() {
        let raw = handl_ext_manifest();
        assert!(!raw.is_null());
        let json = unsafe { CStr::from_ptr(raw) }
            .to_string_lossy()
            .into_owned();
        unsafe { handl_ext_free(raw) };
        let manifest: ExtensionManifest = serde_json::from_str(&json).unwrap();
        assert_eq!(manifest.name, "codemod");
        let names: Vec<&str> = manifest.commands.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, ["codemod"]);
    }
}
