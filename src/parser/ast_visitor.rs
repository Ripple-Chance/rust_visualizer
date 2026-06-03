use proc_macro2::Span;
use syn::{visit::Visit, spanned::Spanned, ItemFn, Pat, PatType, Stmt};

use super::events::{AnalysisEvent, Function, Variable};

#[derive(Debug, Default)]
pub struct AstVisitor {
    pub events: Vec<AnalysisEvent>,
    scope_level: usize,
}

impl AstVisitor {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            scope_level: 0,
        }
    }

    fn extract_var_from_pat_with_scope(&self, pat: &Pat, scope_level: usize) -> Vec<Variable> {
        let mut vars = Vec::new();
        match pat {
            Pat::Ident(pat_ident) => {
                vars.push(Variable {
                    name: pat_ident.ident.to_string(),
                    span: pat.span(),
                    is_mutable: pat_ident.mutability.is_some(),
                    scope_level,
                });
            }
            Pat::Struct(pat_struct) => {
                for field in &pat_struct.fields {
                    vars.extend(self.extract_var_from_pat_with_scope(&field.pat, scope_level));
                }
            }
            Pat::Tuple(pat_tuple) => {
                for elem in &pat_tuple.elems {
                    vars.extend(self.extract_var_from_pat_with_scope(elem, scope_level));
                }
            }
            Pat::TupleStruct(pat_tuple_struct) => {
                for field in &pat_tuple_struct.elems {
                    vars.extend(self.extract_var_from_pat_with_scope(field, scope_level));
                }
            }
            Pat::Wild(_) => {}
            _ => {}
        }
        vars
    }

    fn extract_var_from_pat(&self, pat: &Pat) -> Vec<Variable> {
        self.extract_var_from_pat_with_scope(pat, self.scope_level)
    }

    fn record_var_use(&mut self, name: &str, span: Span) {
        self.events.push(AnalysisEvent::VarUsed {
            name: name.to_string(),
            span,
            scope_level: self.scope_level,
        });
    }
}

impl<'ast> Visit<'ast> for AstVisitor {
    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        let func_scope_level = self.scope_level + 1;
        
        let params: Vec<Variable> = i
            .sig
            .inputs
            .iter()
            .filter_map(|arg| {
                if let syn::FnArg::Typed(PatType { pat, .. }) = arg {
                    self.extract_var_from_pat_with_scope(pat, func_scope_level).into_iter().next()
                } else {
                    None
                }
            })
            .collect();

        let func = Function {
            name: i.sig.ident.to_string(),
            span: i.sig.ident.span(),
            parameters: params.clone(),
            scope_level: self.scope_level,
        };

        self.events.push(AnalysisEvent::FuncDefined(func));

        for param in params {
            self.events.push(AnalysisEvent::VarDefined(param));
        }

        self.scope_level += 1;
        self.events.push(AnalysisEvent::ScopeEnter {
            level: self.scope_level,
            span: i.block.stmts.first().map_or(i.sig.ident.span(), |s| s.span()),
        });

        syn::visit::visit_item_fn(self, i);

        self.events.push(AnalysisEvent::ScopeExit {
            level: self.scope_level,
            span: i.sig.ident.span(),
        });
        self.scope_level -= 1;
    }

    fn visit_stmt(&mut self, s: &'ast Stmt) {
        match s {
            Stmt::Local(local) => {
                for var in self.extract_var_from_pat(&local.pat) {
                    self.events.push(AnalysisEvent::VarDefined(var));
                }
            }
            Stmt::Expr(expr, _) => {
                self.visit_expr(expr);
            }
            Stmt::Item(_) => {}
            Stmt::Macro(mac) => {
                self.visit_macro(&mac.mac);
            }
        }
        syn::visit::visit_stmt(self, s);
    }

    fn visit_expr(&mut self, e: &'ast syn::Expr) {
        match e {
            syn::Expr::Path(path) => {
                if let Some(ident) = path.path.get_ident() {
                    self.record_var_use(&ident.to_string(), ident.span());
                }
            }
            syn::Expr::Call(call) => {
                if let syn::Expr::Path(path) = &*call.func {
                    if let Some(ident) = path.path.get_ident() {
                        self.record_var_use(&ident.to_string(), ident.span());
                    }
                }
                syn::visit::visit_expr_call(self, call);
            }
            syn::Expr::MethodCall(method) => {
                self.record_var_use(&method.method.to_string(), method.method.span());
                syn::visit::visit_expr_method_call(self, method);
            }
            _ => {
                syn::visit::visit_expr(self, e);
            }
        }
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        self.visit_token_stream(&mac.tokens);
    }

    fn visit_token_stream(&mut self, tokens: &'ast proc_macro2::TokenStream) {
        for tree in tokens.clone().into_iter() {
            match tree {
                proc_macro2::TokenTree::Ident(ident) => {
                    self.record_var_use(&ident.to_string(), ident.span());
                }
                proc_macro2::TokenTree::Group(group) => {
                    self.visit_token_stream(&group.stream());
                }
                _ => {}
            }
        }
    }

    fn visit_block(&mut self, b: &'ast syn::Block) {
        self.scope_level += 1;
        self.events.push(AnalysisEvent::ScopeEnter {
            level: self.scope_level,
            span: b.span(),
        });

        syn::visit::visit_block(self, b);

        self.events.push(AnalysisEvent::ScopeExit {
            level: self.scope_level,
            span: b.span(),
        });
        self.scope_level -= 1;
    }
}
