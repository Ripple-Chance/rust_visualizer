use proc_macro2::Span;
use std::fmt;
use crate::analysis::BorrowKind;

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub span: Span,
    pub is_mutable: bool,
    pub scope_level: usize,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Variable>,
}

#[derive(Debug, Clone)]
pub struct AnalysisEvent {
    pub kind: EventKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum EventKind {
    VarDefined(Variable),
    VarUsed { name: String, scope_level: usize },
    FuncDefined(Function),
    ScopeEnter { level: usize },
    ScopeExit { level: usize },
    BorrowCreated { name: String, kind: BorrowKind, scope_level: usize },
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} (scope: {})",
            self.name,
            if self.is_mutable { "mut" } else { "immut" },
            self.scope_level
        )
    }
}

impl fmt::Display for AnalysisEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            EventKind::VarDefined(var) => write!(f, "DEFINE: {}", var),
            EventKind::VarUsed { name, .. } => write!(f, "USE: {}", name),
            EventKind::FuncDefined(func) => write!(f, "FUNCTION: {}", func.name),
            EventKind::ScopeEnter { level } => write!(f, "ENTER_SCOPE: level {}", level),
            EventKind::ScopeExit { level } => write!(f, "EXIT_SCOPE: level {}", level),
            EventKind::BorrowCreated { name, kind, .. } => write!(f, "BORROW: {} {}", kind, name),
        }
    }
}
