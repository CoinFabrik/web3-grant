#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;
extern crate rustc_span;

use clippy_utils::diagnostics::span_lint_and_help;
use if_chain::if_chain;
use rustc_hir::intravisit::Visitor;
use rustc_hir::intravisit::{walk_expr, FnKind};
use rustc_hir::{Body, FnDecl, HirId,};
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
    /// // example code where a warning is issued
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code that does not raise a warning
    /// ```
    pub REENTRANCY,
    Warn,
    "description goes here"
}

impl<'tcx> LateLintPass<'tcx> for Reentrancy {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'_>,
        body: &'tcx Body<'_>,
        _: Span,
        _: HirId,
    ) {
        struct ReentrantStorage {
            span: Option<Span>,
            has_invoke_contract_call: bool,
            allow_reentrancy_flag: bool,
        }

        impl<'tcx> Visitor<'tcx> for ReentrantStorage {
            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                if_chain! {
                    if let ExprKind::MethodCall(func, _, args, _) = &expr.kind;
                    if let function_name = func.ident.name.to_string();
                    then {
                        if function_name == "invoke_contract" {
                            self.has_invoke_contract_call = true;
                            self.span = Some(expr.span);
                            // TODO: Check if the state changes after this statement
                        } else if function_name.contains("set_allow_reentry") {
                            if_chain! {
                                if let ExprKind::Lit(lit) = &args[0].kind;
                                if &lit.node.to_string() == "true";
                                then {
                                    self.allow_reentrancy_flag = true;
                                }
                            }
                        }
                    }
                }

                walk_expr(self, expr);
            }
        }

        let mut reentrant_storage = ReentrantStorage {
            span: None,
            has_invoke_contract_call: false,
            allow_reentrancy_flag: false,
        };

        walk_expr(&mut reentrant_storage, &body.value);

        if reentrant_storage.has_invoke_contract_call && reentrant_storage.allow_reentrancy_flag {
            span_lint_and_help(
                cx,
                REENTRANCY,
                // body.value.span,
                reentrant_storage.span.unwrap(),
                "External calls could open the opportunity for a malicious contract to execute any arbitrary code",
                None,
                "This statement seems to call another contract after the flag set_allow_reentry was enabled [todo: check state changes after this statement]",
            );
        }
    }

}
