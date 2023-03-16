#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;

use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::higher;
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
    pub DOS_UNBOUNDED_OPERATION,
    Warn,
    "description goes here"
}

impl<'tcx> LateLintPass<'tcx> for DosUnboundedOperation {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'_>) {
        if_chain! {
            if let Some(higher::ForLoop { pat: _, arg, body: _, .. }) = higher::ForLoop::hir(expr);
            // arg is the iterator expression
            if let ExprKind::Struct(_, field, _) = arg.kind;
            if field.len() == 2; //TODO: check if [exprfield] can be gt 2
            if let ExprKind::Field(base, _) = field[1].expr.kind; // self.field_name <- base: self, field_name: ident
            if let ExprKind::Path(path) = &base.kind;
            if let QPath::Resolved(None, ref path) = *path;
            if path.segments.iter().any(|base| base.ident.as_str().contains("self"));
            then {
                    span_lint_and_help(
                        cx,
                        DOS_UNBOUNDED_OPERATION,
                            expr.span,
                        "In order to prevent a single transaction from consuming all the gas in a block, unbounded operations must be avoided",
                        None,
                        "This loop seems to do not have a fixed number of iterations",
                    );
            }
	    }
    }
}
