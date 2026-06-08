use super::*;
use crate::parser::events::{AnalysisEvent, EventKind};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Lifetime {
    pub end_span: Span,
    pub scope_level: usize,
    pub references: Vec<String>,
}

#[derive(Debug, Default)]
pub struct LifetimeAnalyzer {
    lifetimes: HashMap<String, Lifetime>,
    active_lifetimes: HashSet<String>,
    results: Vec<AnalysisResult>,
}

impl LifetimeAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn analyze(&mut self, events: &[AnalysisEvent]) -> Vec<AnalysisResult> {
        for event in events {
            self.process_event(event);
        }
        self.results.clone()
    }

    fn process_event(&mut self, event: &AnalysisEvent) {
        match &event.kind {
            EventKind::VarDefined(var) => {
                self.handle_var_defined(var);
            }
            EventKind::VarUsed { name, .. } => {
                self.handle_var_used(name, event.span);
            }
            EventKind::ScopeEnter { .. } => {
            }
            EventKind::ScopeExit { level, .. } => {
                self.handle_scope_exit(*level, event.span);
            }
            EventKind::FuncDefined(func) => {
                self.handle_func_defined(func);
            }
            _ => {}
        }
    }

    fn handle_var_defined(&mut self, var: &crate::parser::events::Variable) {
        let lifetime = Lifetime {
            end_span: var.span,
            scope_level: var.scope_level,
            references: Vec::new(),
        };
        
        self.lifetimes.insert(var.name.clone(), lifetime);
        self.active_lifetimes.insert(var.name.clone());
    }

    fn handle_var_used(&mut self, name: &str, span: Span) {
        if let Some(lifetime) = self.lifetimes.get_mut(name) {
            lifetime.references.push(format!("{:?}", span));
            lifetime.end_span = span;
        }
    }

    fn handle_scope_exit(&mut self, level: usize, span: Span) {
        let to_remove: Vec<String> = self.lifetimes
            .iter()
            .filter(|(_, l)| l.scope_level == level)
            .map(|(name, _)| name.clone())
            .collect();
        
        for name in to_remove {
            if let Some(lifetime) = self.lifetimes.get_mut(&name) {
                lifetime.end_span = span;
                self.active_lifetimes.remove(&name);
                
                self.results.push(AnalysisResult::OwnershipChange {
                    name: name.clone(),
                    new_status: OwnershipStatus::Dropped,
                    span,
                });
            }
        }
    }

    fn handle_func_defined(&mut self, func: &crate::parser::events::Function) {
        for param in &func.parameters {
            let lifetime = Lifetime {
                end_span: param.span,
                scope_level: param.scope_level,
                references: Vec::new(),
            };
            
            self.lifetimes.insert(param.name.clone(), lifetime);
            self.active_lifetimes.insert(param.name.clone());
        }
    }

    pub fn get_lifetime_summary(&self) -> Vec<(String, usize)> {
        self.lifetimes
            .iter()
            .map(|(name, l)| {
                (name.clone(), l.references.len())
            })
            .collect()
    }
}
