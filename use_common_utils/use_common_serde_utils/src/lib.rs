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
use rustc_hir::{def_id::DefId, Expr, ExprKind, QPath};
use rustc_lint::{LateContext, LateLintPass};

use std::collections::HashSet;

dylint_linting::impl_late_lint! {
    /// ### What it does
    /// Checks and reports usage of raw serde (de)serialization functions.
    ///
    /// ### Why is this bad?
    /// (de)serialization should be done via ext traits implemented in common_utils to
    /// ensure consistency across Hyperswitch
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// serde_json::to_value(&my_data);
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// use common_utils::ext_traits::Encode;
    ///
    /// my_data.encode_to_value();
    /// ```
    pub USE_COMMON_SERDE_UTILS,
    Warn,
    "using raw serde (de)serialization functions over common_utils ext traits",
    UseCommonSerdeUtils::default()
}

const SERDE_JSON_FROM_STR: &[&str] = &["serde_json", "from_str"];
const SERDE_JSON_TO_STRING: &[&str] = &["serde_json", "to_string"];
const SERDE_JSON_FROM_VALUE: &[&str] = &["serde_json", "from_value"];
const SERDE_JSON_TO_VALUE: &[&str] = &["serde_json", "to_value"];

#[derive(Default)]
struct UseCommonSerdeUtils {
    serde_fn_def_ids: Option<HashSet<DefId>>,
}

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
            let serde_def_ids = self.serde_fn_def_ids.get_or_insert_with(|| {
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
