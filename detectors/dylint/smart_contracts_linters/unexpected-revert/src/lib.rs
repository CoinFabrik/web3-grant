#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;
extern crate rustc_span;

use clippy_utils::diagnostics::span_lint_and_help;
use if_chain::if_chain;
use rustc_hir::intravisit::Visitor;
use rustc_hir::intravisit::{walk_expr, FnKind};
use rustc_hir::{Body, FnDecl, HirId};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;

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
    /// // example code where left warning is issued
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code that does not raise left warning
    /// ```
    pub UNEXPECTED_REVERT_WARN,
    Warn,
    "vectors only must be used with proper access control, otherwise a user could add an excessive number of entries leading to a DoS attack"
}

impl<'tcx> LateLintPass<'tcx> for UnexpectedRevertWarn {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'_>,
        body: &'tcx Body<'_>,
        _: Span,
        _: HirId,
    ) {
        #[derive(Debug)]
        struct UnexpectedRevert {
            span: Option<Span>,
            unprotected: bool,
            in_conditional: bool,
            has_owner_validation: bool,
            has_vec_push: bool,
        }

        impl<'tcx> Visitor<'tcx> for UnexpectedRevert {
            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                // Check if the function is called inside an if statement
                if let ExprKind::If(..) = &expr.kind {
                    self.in_conditional = true;
                    walk_expr(self, expr);
                    self.in_conditional = false;
                }

                // Check if owner validation is present in conditional
                if_chain! {
                    if self.in_conditional;
                    if let ExprKind::Binary(_, left, right) = &expr.kind;
                    if let ExprKind::Field(_, ident) = right.kind;
                    if let ExprKind::MethodCall(func, ..) = &left.kind;
                    then {
                        let function_name = func.ident.name.to_string();
                        self.has_owner_validation = ident.as_str().contains("owner") && function_name.contains("caller");
                    }
                }

                // Check if array is pushed
                if let ExprKind::MethodCall(call, _, _, _) = expr.kind {
                    let function_name = call.ident.name.as_str();
                    if function_name == "push" {
                        self.has_vec_push = true;
                        if !self.in_conditional && !self.has_owner_validation {
                            self.unprotected = true;
                            self.span = Some(expr.span);
                        }
                    }
                }

                walk_expr(self, expr);
            }
        }

        let mut reentrant_storage = UnexpectedRevert {
            span: None,
            unprotected: false,
            in_conditional: false,
            has_owner_validation: false,
            has_vec_push: false,
        };

        walk_expr(&mut reentrant_storage, body.value);

        if reentrant_storage.has_vec_push && reentrant_storage.unprotected {
            span_lint_and_help(
                cx,
                UNEXPECTED_REVERT_WARN,
                reentrant_storage.span.unwrap(),
                "Abitrary users should not be able to push to a vector, otherwise it could lead to a DoS attack",
                None,
                "Set access control and proper authorization validation for pushing to a vector or change to a mapping",
            );
        }
    }
}
