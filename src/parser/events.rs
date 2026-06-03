use proc_macro2::Span;
use std::fmt;

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
pub enum AnalysisEvent {
    VarDefined(Variable),
    VarUsed { name: String, span: Span, scope_level: usize },
    VarDropped { name: String, span: Span },
    FuncDefined(Function),
    ScopeEnter { level: usize, span: Span },
    ScopeExit { level: usize, span: Span },
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
        match self {
            AnalysisEvent::VarDefined(var) => write!(f, "DEFINE: {}", var),
            AnalysisEvent::VarUsed { name, .. } => write!(f, "USE: {}", name),
            AnalysisEvent::VarDropped { name, .. } => write!(f, "DROP: {}", name),
            AnalysisEvent::FuncDefined(func) => write!(f, "FUNCTION: {}", func.name),
            AnalysisEvent::ScopeEnter { level, .. } => write!(f, "ENTER_SCOPE: level {}", level),
            AnalysisEvent::ScopeExit { level, .. } => write!(f, "EXIT_SCOPE: level {}", level),
        }
    }
}
