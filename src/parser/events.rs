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
    pub span: Span,
    pub parameters: Vec<Variable>,
    pub scope_level: usize,
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
    VarDropped { name: String },
    FuncDefined(Function),
    ScopeEnter { level: usize },
    ScopeExit { level: usize },
    BorrowCreated { name: String, kind: BorrowKind, scope_level: usize },
    BorrowDropped { name: String, kind: BorrowKind },
    OwnershipMoved { name: String, target: Option<String>, is_function_call: bool },
    VariableCopied { name: String, target: String },
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
            EventKind::VarDropped { name } => write!(f, "DROP: {}", name),
            EventKind::FuncDefined(func) => write!(f, "FUNCTION: {}", func.name),
            EventKind::ScopeEnter { level } => write!(f, "ENTER_SCOPE: level {}", level),
            EventKind::ScopeExit { level } => write!(f, "EXIT_SCOPE: level {}", level),
            EventKind::BorrowCreated { name, kind, .. } => write!(f, "BORROW: {} {}", kind, name),
            EventKind::BorrowDropped { name, kind } => write!(f, "DROP_BORROW: {} {}", kind, name),
            EventKind::OwnershipMoved { name, target, is_function_call } => {
                let target_str = target.as_deref().unwrap_or("unknown");
                if *is_function_call {
                    write!(f, "MOVE_CALL: {} -> {}", name, target_str)
                } else {
                    write!(f, "MOVE: {} -> {}", name, target_str)
                }
            }
            EventKind::VariableCopied { name, target } => write!(f, "COPY: {} -> {}", name, target),
        }
    }
}
