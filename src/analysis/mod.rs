pub mod ownership;
pub mod borrow;
pub mod lifetime;

pub use ownership::OwnershipAnalyzer;
pub use borrow::BorrowAnalyzer;
pub use lifetime::LifetimeAnalyzer;

use proc_macro2::Span;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BorrowKind {
    Immutable,
    Mutable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OwnershipStatus {
    Owned,
    Borrowed(BorrowKind),
    Moved,
    Dropped,
}

#[derive(Debug, Clone)]
pub struct OwnershipRecord {
    pub name: String,
    pub span: Span,
    pub is_mutable: bool,
    pub scope_level: usize,
    pub status: OwnershipStatus,
    pub borrow_count: usize,
    pub mutable_borrow_count: usize,
    pub owner: Option<String>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct BorrowEvent {
    pub kind: BorrowKind,
    pub name: String,
    pub span: Span,
    pub scope_level: usize,
    pub is_return: bool,
}

#[derive(Debug, Clone)]
pub struct MoveEvent {
    pub name: String,
    pub target: Option<String>,
    pub span: Span,
    pub is_function_call: bool,
    pub is_assignment: bool,
}

#[derive(Debug, Clone)]
pub enum AnalysisResult {
    OwnershipChange {
        name: String,
        old_status: OwnershipStatus,
        new_status: OwnershipStatus,
        span: Span,
    },
    BorrowCreated(BorrowEvent),
    BorrowDropped {
        name: String,
        kind: BorrowKind,
        span: Span,
    },
    OwnershipMoved(MoveEvent),
    VariableCopied {
        name: String,
        target: String,
        span: Span,
    },
    BorrowConflict {
        name: String,
        existing_borrow: BorrowKind,
        new_borrow: BorrowKind,
        span: Span,
    },
    UseAfterMove {
        name: String,
        move_span: Span,
        use_span: Span,
    },
}

impl fmt::Display for BorrowKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BorrowKind::Immutable => write!(f, "&"),
            BorrowKind::Mutable => write!(f, "&mut"),
        }
    }
}

impl fmt::Display for OwnershipStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OwnershipStatus::Owned => write!(f, "owned"),
            OwnershipStatus::Borrowed(kind) => write!(f, "borrowed ({})", kind),
            OwnershipStatus::Moved => write!(f, "moved"),
            OwnershipStatus::Dropped => write!(f, "dropped"),
        }
    }
}

fn span_to_string(span: &Span) -> String {
    format!("{:?}", span)
}

impl fmt::Display for AnalysisResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnalysisResult::OwnershipChange { name, new_status, span, .. } => {
                write!(f, "[OWNERSHIP] {} -> {} at {}", name, new_status, span_to_string(span))
            }
            AnalysisResult::BorrowCreated(event) => {
                write!(f, "[BORROW] {} {} at {}", event.kind, event.name, span_to_string(&event.span))
            }
            AnalysisResult::BorrowDropped { name, kind, span } => {
                write!(f, "[DROP_BORROW] {} {} at {}", kind, name, span_to_string(span))
            }
            AnalysisResult::OwnershipMoved(event) => {
                let target = event.target.as_deref().unwrap_or("unknown");
                write!(f, "[MOVE] {} -> {} at {}", event.name, target, span_to_string(&event.span))
            }
            AnalysisResult::VariableCopied { name, target, span } => {
                write!(f, "[COPY] {} -> {} at {}", name, target, span_to_string(span))
            }
            AnalysisResult::BorrowConflict { name, existing_borrow, new_borrow, span } => {
                write!(f, "[CONFLICT] {}: existing {} borrow conflicts with new {} at {}", 
                    name, existing_borrow, new_borrow, span_to_string(span))      
            }
            AnalysisResult::UseAfterMove { name, use_span, .. } => {
                write!(f, "[ERROR] Use after move: {} at {}", name, span_to_string(use_span))
            }
        }
    }
}
