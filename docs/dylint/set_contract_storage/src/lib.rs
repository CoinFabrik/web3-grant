#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;

use clippy_utils::diagnostics::span_lint_and_help;
use if_chain::if_chain;
use rustc_hir::QPath;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Known problems
    /// Remove if none.
    ///
    /// ### Example
    /// ```rust
    /// // example code where a warning is issued
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code that does not raise a warning
    /// ```
    pub SET_STORAGE_WARN,
    Warn,
    "set_contract_storage only must be used with proper access control or input sanitation"
}

impl<'tcx> LateLintPass<'tcx> for SetStorageWarn {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &Expr<'_>) {
        if_chain! {
            if let ExprKind::Call(callee, _) = expr.kind;
            if let ExprKind::Path(method_path) = &callee.kind;
            if let QPath::Resolved(None, ref path) = *method_path;
            if path.segments.len() == 2 && path.segments[0].ident.name.as_str() == "env" && path.segments[1].ident.name.as_str() == "set_contract_storage";
            then {
                span_lint_and_help(
                    cx,
                    SET_STORAGE_WARN,
                    expr.span,
                    "Abitrary users should not have control over keys because it implies writing any value of a mapping, lazy variable, or the main struct of the contract located in position 0 of the storage",
                    None,
                    &format!("Set access control and proper authorization validation for the set_contract_storage() function"),
                );
            }
        }
    }
}
