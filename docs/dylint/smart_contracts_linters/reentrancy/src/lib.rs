#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;
extern crate rustc_span;

use clippy_utils::diagnostics::span_lint_and_help;
use if_chain::if_chain;
use rustc_hir::intravisit::{Visitor, walk_stmt};
use rustc_hir::intravisit::{walk_expr, FnKind};
use rustc_hir::{Body, FnDecl, HirId, Stmt,};
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

        fn check_invoke_contract_call(r: &mut ReentrantStorage, expr: &Expr)  {
            if_chain! {
                if let ExprKind::MethodCall(func, _, args, _) = &expr.kind;
                if let function_name = func.ident.name.to_string();
                if function_name == "invoke_contract" ;
                then {
                        r.has_invoke_contract_call = true;
                        r.span = Some(expr.span);
                }
            }
        }
        fn check_allow_reentrancy(r: &mut ReentrantStorage, expr: &Expr) {
            if_chain! {
                if let ExprKind::MethodCall(func, _, args, _) = &expr.kind;
                if let function_name = func.ident.name.to_string();
                if function_name.contains("set_allow_reentry");
                then {
                        if_chain! {
                            if let ExprKind::Lit(lit) = &args[0].kind;
                            if &lit.node.to_string() == "true";
                            then {
                                r.allow_reentrancy_flag = true;
                            }
                        }
                    }
                }
        }
        fn check_state_change(r: &mut ReentrantStorage, s: &Stmt) {
            if_chain! {
                if let rustc_hir::StmtKind::Semi(expr) = &s.kind;
                if let rustc_hir::ExprKind::Assign(lhs, rhs, _) = &expr.kind;
                if let rustc_hir::ExprKind::Path(qpath) = &lhs.kind;
                if let rustc_hir::QPath::Resolved(_, path) = qpath;
                if let rustc_hir::def::Res::Def(rustc_hir::def::DefKind::Struct, _) = path.res;
                then {
                    dbg!({}, "Found a state change");
                    dbg!({}, s);
                    // self.span = Some(s.span);
                    r.state_change = true;
                }
            }
            if_chain! {
                // check access to balance.insert
                if let rustc_hir::StmtKind::Semi(expr) = &s.kind;
                if let rustc_hir::ExprKind::MethodCall(func, _, args, _) = &expr.kind;
                if let function_name = func.ident.name.to_string();
                if function_name == "insert";
                // Fix this: checking for "balance"
                // if let rustc_hir::ExprKind::Path(qpath) = &args[0].kind;
                // if let rustc_hir::QPath::Resolved(_, path) = qpath;
                // if let rustc_hir::def::Res::Def(rustc_hir::def::DefKind::Struct, _) = path.res ;
                then {
                    r.state_change = true;
                }
                else {
                    dbg!({}, s);
                }
            }

        }

        impl<'tcx> Visitor<'tcx> for ReentrantStorage {
            fn visit_stmt(&mut self, s: &'tcx Stmt<'tcx>) {
                // check for an statement that modifies the state
                // the state is modified if the statement is an assignment and modifies an struct
                // or if if invokes a function and the receiver is a env::balance
                if self.has_invoke_contract_call && self.allow_reentrancy_flag {
                    check_state_change(self, s);
                }
                else {
                    walk_stmt(self, s);
                }
            }

            

            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                if(self.allow_reentrancy_flag) {
                    check_invoke_contract_call(self, expr);
                }
                check_allow_reentrancy(self, expr);

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
            && reentrant_storage.state_change {
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
