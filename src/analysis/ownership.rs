use super::*;
use crate::parser::events::{AnalysisEvent, EventKind};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct OwnershipRecordInternal {
    pub status: OwnershipStatus,
}

#[derive(Debug, Default)]
pub struct OwnershipAnalyzer {
    scopes: Vec<HashMap<String, OwnershipRecordInternal>>,
    results: Vec<AnalysisResult>,
}

impl OwnershipAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self::default();
        analyzer.scopes.push(HashMap::new());
        analyzer
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
                self.scopes.push(HashMap::new());
            }
            EventKind::ScopeExit { level, .. } => {
                self.handle_scope_exit(*level, event.span);
            }
            EventKind::FuncDefined(func) => {
                self.handle_func_defined(func);
            }
            EventKind::BorrowCreated { name, kind, .. } => {
                self.handle_borrow_created(name, kind, event.span);
            }
        }
    }

    fn handle_var_defined(&mut self, var: &crate::parser::events::Variable) {
        let record = OwnershipRecordInternal {
            status: OwnershipStatus::Owned,
        };
        
        while self.scopes.len() <= var.scope_level {
            self.scopes.push(HashMap::new());
        }
        
        self.scopes[var.scope_level].insert(var.name.clone(), record);
        
        self.results.push(AnalysisResult::OwnershipChange {
            name: var.name.clone(),
            new_status: OwnershipStatus::Owned,
            span: var.span,
        });
    }

    fn handle_var_used(&mut self, name: &str, _span: Span) {
        if let Some(record) = self.find_variable(name, self.scopes.len() - 1) {
            if let OwnershipStatus::Moved = record.status {
                // Handle use of moved value
            }
        }
    }

    fn handle_scope_exit(&mut self, level: usize, span: Span) {
        if level < self.scopes.len() {
            for (name, record) in &self.scopes[level] {
                if record.status != OwnershipStatus::Moved {
                    self.results.push(AnalysisResult::OwnershipChange {
                        name: name.clone(),
                        new_status: OwnershipStatus::Dropped,
                        span,
                    });
                }
            }
        }
        
        while self.scopes.len() > level + 1 {
            self.scopes.pop();
        }
    }

    fn handle_func_defined(&mut self, func: &crate::parser::events::Function) {
        for param in &func.parameters {
            let record = OwnershipRecordInternal {
                status: OwnershipStatus::Owned,
            };
            
            while self.scopes.len() <= param.scope_level {
                self.scopes.push(HashMap::new());
            }
            
            self.scopes[param.scope_level].insert(param.name.clone(), record);
        }
    }

    fn handle_borrow_created(&mut self, name: &str, kind: &BorrowKind, _span: Span) {
        let scopes_len = self.scopes.len();
        for level in (0..scopes_len).rev() {
            if let Some(scope) = self.scopes.get_mut(level) {
                if let Some(record) = scope.get_mut(name) {
                    match record.status {
                        OwnershipStatus::Owned => {
                            record.status = OwnershipStatus::Borrowed(kind.clone());
                        }
                        OwnershipStatus::Borrowed(ref existing_kind) => {
                            if *kind == BorrowKind::Mutable && *existing_kind != BorrowKind::Mutable {
                                // Cannot mutably borrow already borrowed value
                            }
                        }
                        _ => {}
                    }
                    return;
                }
            }
        }
    }

    fn find_variable(&self, name: &str, scope_level: usize) -> Option<&OwnershipRecordInternal> {
        for level in (0..=scope_level).rev() {
            if let Some(record) = self.scopes.get(level).and_then(|s| s.get(name)) {
                return Some(record);
            }
        }
        None
    }
}
