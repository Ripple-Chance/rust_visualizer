use super::*;
use crate::parser::events::{AnalysisEvent, EventKind};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BorrowInfo {
    pub kind: BorrowKind,
    pub scope_level: usize,
    pub is_active: bool,
    pub references: Vec<String>,
}

#[derive(Debug, Default)]
pub struct BorrowAnalyzer {
    borrows: HashMap<String, Vec<BorrowInfo>>,
    active_borrows: HashMap<String, BorrowKind>,
    results: Vec<AnalysisResult>,
}

impl BorrowAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn analyze(&mut self, events: &[AnalysisEvent]) -> &[AnalysisResult] {
        for event in events {
            self.process_event(event);
        }
        &self.results
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
            EventKind::BorrowCreated { name, kind, scope_level } => {
                self.track_borrow(name, kind.clone(), event.span, *scope_level);
            }
            _ => {}
        }
    }

    fn handle_var_defined(&mut self, var: &crate::parser::events::Variable) {
        self.borrows.insert(var.name.clone(), Vec::new());
    }

    fn handle_var_used(&mut self, name: &str, span: Span) {
        if let Some(borrow_list) = self.borrows.get_mut(name) {
            for borrow in borrow_list.iter_mut() {
                if borrow.is_active {
                    borrow.references.push(format!("{:?}", span));
                }
            }
        }
    }

    fn handle_scope_exit(&mut self, level: usize, _span: Span) {
        let names_to_check: Vec<String> = self.borrows.keys().cloned().collect();
        
        for name in names_to_check {
            if let Some(borrow_list) = self.borrows.get_mut(&name) {
                let mut active_kind: Option<BorrowKind> = None;
                
                for borrow in borrow_list.iter_mut() {
                    if borrow.scope_level == level && borrow.is_active {
                        borrow.is_active = false;
                        active_kind = Some(borrow.kind.clone());
                    }
                }
                
                if let Some(kind) = active_kind {
                    if let Some(active) = self.active_borrows.get(&name) {
                        if *active == kind {
                            let remaining = borrow_list.iter()
                                .filter(|b| b.is_active && b.kind == kind)
                                .count();
                            if remaining == 0 {
                                self.active_borrows.remove(&name);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn track_borrow(&mut self, name: &str, kind: BorrowKind, span: Span, scope_level: usize) {
        let borrow_info = BorrowInfo {
            kind: kind.clone(),
            scope_level,
            is_active: true,
            references: Vec::new(),
        };
        
        self.borrows.entry(name.to_string()).or_insert_with(Vec::new).push(borrow_info);
        self.active_borrows.insert(name.to_string(), kind.clone());
        
        self.results.push(AnalysisResult::BorrowCreated(BorrowEvent {
            kind,
            name: name.to_string(),
            span,
        }));
    }

    pub fn find_long_borrow_chains(&self, threshold: usize) -> Vec<(String, usize)> {
        let mut long_chains = Vec::new();
        
        for (name, borrow_list) in &self.borrows {
            for borrow in borrow_list {
                if borrow.references.len() >= threshold {
                    long_chains.push((name.clone(), borrow.references.len()));
                    break;
                }
            }
        }
        
        long_chains
    }
}
