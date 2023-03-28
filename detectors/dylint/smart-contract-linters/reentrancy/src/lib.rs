#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;
extern crate rustc_span;

use clippy_utils::diagnostics::span_lint_and_help;
use if_chain::if_chain;
use rustc_hir::intravisit::{walk_expr, FnKind};
use rustc_hir::intravisit::{walk_stmt, Visitor};
use rustc_hir::{Body, FnDecl, HirId, Stmt};
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
            state_change: bool,
        }

        fn check_invoke_contract_call(expr: &Expr) -> Option<Span> {
            if_chain! {
                if let ExprKind::MethodCall(func, _, _, _) = &expr.kind;
                if let function_name = func.ident.name.to_string();
                if function_name == "invoke_contract" ;
                then {
                        return Some(expr.span);
                }
            }
            return None;
        }
        fn check_allow_reentrancy(expr: &Expr) -> bool {
            if_chain! {
            if let ExprKind::MethodCall(func, _, args, _) = &expr.kind;
            if let function_name = func.ident.name.to_string();
            if function_name.contains("set_allow_reentry");
            then {
                    if_chain! {
                        if let ExprKind::Lit(lit) = &args[0].kind;
                        if &lit.node.to_string() == "true";
                        then {
                            return true;
                        }
                    }
                }
            }
            return false;
        }
        fn check_state_change(s: &Stmt) -> bool {
            if_chain! {
                if let rustc_hir::StmtKind::Semi(expr) = &s.kind;
                if let rustc_hir::ExprKind::Assign(lhs, ..) = &expr.kind;
                if let rustc_hir::ExprKind::Field(base, _) = lhs.kind; // self.field_name <- base: self, field_name: ident
                if let rustc_hir::ExprKind::Path(path) = &base.kind;
                if let rustc_hir::QPath::Resolved(None, ref path) = *path;
                if path.segments.iter().any(|base| base.ident.as_str().contains("self"));                then {
                    return true;
                }
            }
            if_chain! {
                // check access to balance.insert
                if let rustc_hir::StmtKind::Semi(expr) = &s.kind;
                if let rustc_hir::ExprKind::MethodCall(func, rec, ..) = &expr.kind;
                if let function_name = func.ident.name.to_string();
                if function_name == "insert";
                // Fix this: checking for "balances"
                if let rustc_hir::ExprKind::Field(base, _) = &rec.kind; // self.field_name <- base: self, field_name: ident
                if let rustc_hir::ExprKind::Path(path) = &base.kind;
                if let rustc_hir::QPath::Resolved(None, ref path) = *path;
                if path.segments.iter().any(|base| base.ident.as_str().contains("self"));
                then {
                    return true;
                }
            }
            return false;
        }

        impl<'tcx> Visitor<'tcx> for ReentrantStorage {
            fn visit_stmt(&mut self, stmt: &'tcx Stmt<'tcx>) {
                // check for an statement that modifies the state
                // the state is modified if the statement is an assignment and modifies an struct
                // or if if invokes a function and the receiver is a env::balance
                if self.has_invoke_contract_call && self.allow_reentrancy_flag {
                    if check_state_change(stmt) {
                        self.state_change = true;
                    }
                } else {
                    walk_stmt(self, stmt);
                }
            }

            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                if self.allow_reentrancy_flag {
                    let invoke_contract_span = check_invoke_contract_call(expr);
                    if invoke_contract_span.is_some() {
                        self.has_invoke_contract_call = true;
                        self.span = invoke_contract_span;
                    }
                }
                if check_allow_reentrancy(expr) {
                    self.allow_reentrancy_flag = true;
                }

                walk_expr(self, expr);
            }
        }

        let mut reentrant_storage = ReentrantStorage {
            span: None,
            has_invoke_contract_call: false,
            allow_reentrancy_flag: false,
            state_change: false,
        };

        walk_expr(&mut reentrant_storage, &body.value);

        if reentrant_storage.has_invoke_contract_call
            && reentrant_storage.allow_reentrancy_flag
            && reentrant_storage.state_change
        {
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
