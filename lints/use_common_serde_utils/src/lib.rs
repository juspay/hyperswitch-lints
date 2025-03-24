#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

//extern crate rustc_arena;
//extern crate rustc_ast;
//extern crate rustc_ast_pretty;
//extern crate rustc_data_structures;
//extern crate rustc_errors;
extern crate rustc_hir;
//extern crate rustc_hir_pretty;
//extern crate rustc_index;
//extern crate rustc_infer;
//extern crate rustc_lexer;
//extern crate rustc_middle;
//extern crate rustc_mir_dataflow;
//extern crate rustc_parse;
//extern crate rustc_span;
//extern crate rustc_target;
//extern crate rustc_trait_selection;

use clippy_utils::{def_path_def_ids, diagnostics::span_lint_and_help};
use once_cell::sync::OnceCell;
use rustc_hir::{def_id::DefId, Expr, ExprKind, QPath};
use rustc_lint::{LateContext, LateLintPass};

use std::collections::HashSet;

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Known problems
    ///
    /// Remove if none.
    ///
    /// ### Example
    ///
    /// ```rust
    /// // example code where a warning is issued
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust
    /// // example code that does not raise a warning
    /// ```
    pub USE_COMMON_SERDE_UTILS,
    Warn,
    "description goes here"
}

const SERDE_JSON_FROM_STR: &[&str] = &["serde_json", "from_str"];
const SERDE_JSON_TO_STRING: &[&str] = &["serde_json", "to_string"];
const SERDE_JSON_FROM_VALUE: &[&str] = &["serde_json", "from_value"];
const SERDE_JSON_TO_VALUE: &[&str] = &["serde_json", "to_value"];

static DEF_IDS: OnceCell<HashSet<DefId>> = OnceCell::new();

impl LateLintPass<'_> for UseCommonSerdeUtils {
    // A list of things you might check can be found here:
    // https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html

    fn check_expr(&mut self, cx: &LateContext<'_>, expr: &'_ Expr<'_>) {
        if let ExprKind::Call(func, args) = expr.kind
            && args.len() == 1
            && let ExprKind::Path(ref qpath) = func.kind
            && let QPath::Resolved(_, path) = qpath
            && let Some(def_id) = path.res.opt_def_id()
        {
            let serde_def_ids = DEF_IDS.get_or_init(|| {
                def_path_def_ids(cx.tcx, SERDE_JSON_FROM_STR)
                    .chain(def_path_def_ids(cx.tcx, SERDE_JSON_TO_STRING))
                    .chain(def_path_def_ids(cx.tcx, SERDE_JSON_FROM_VALUE))
                    .chain(def_path_def_ids(cx.tcx, SERDE_JSON_TO_VALUE))
                    .collect()
            });

            if serde_def_ids.contains(&def_id) {
                span_lint_and_help(
                    cx,
                    USE_COMMON_SERDE_UTILS,
                    path.span,
                    "Direct usage of serde_json (de)serialization functions is discouraged",
                    None,
                    "Use the extension methods on the common_utils::ext_traits::Encode trait instead",
                );
            }
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test_example(env!("CARGO_PKG_NAME"), "tovalue");
}
