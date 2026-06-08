pub mod ownership;
pub mod borrow;
pub mod lifetime;

pub use ownership::OwnershipAnalyzer;
pub use borrow::BorrowAnalyzer;
pub use lifetime::LifetimeAnalyzer;

use proc_macro2::Span;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BorrowKind {
    Immutable,
    Mutable,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OwnershipStatus {
    Owned,
    Borrowed(BorrowKind),
    Moved,
    Dropped,
}

#[derive(Debug, Clone)]
pub struct BorrowEvent {
    pub kind: BorrowKind,
    pub name: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum AnalysisResult {
    OwnershipChange {
        name: String,
        new_status: OwnershipStatus,
        span: Span,
    },
    BorrowCreated(BorrowEvent),
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
            AnalysisResult::OwnershipChange { name, new_status, span } => {
                write!(f, "[OWNERSHIP] {} -> {} at {}", name, new_status, span_to_string(span))
            }
            AnalysisResult::BorrowCreated(event) => {
                write!(f, "[BORROW] {} {} at {}", event.kind, event.name, span_to_string(&event.span))
            }
        }
    }
}
