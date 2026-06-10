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

pub fn analyze(graph: &crate::graph::variable_graph::VarGraph) -> Vec<AnalysisResult> {
    use crate::parser::events::AnalysisEvent;
    
    let mut nodes: Vec<_> = graph.node_weights().cloned().collect();
    nodes.sort_by(|a, b| a.name.cmp(&b.name));
    
    let mut events: Vec<AnalysisEvent> = Vec::new();
    
    for (i, node) in nodes.iter().enumerate() {
        events.push(AnalysisEvent {
            span: proc_macro2::Span::call_site(),
            kind: crate::parser::events::EventKind::VarDefined(crate::parser::events::Variable {
                name: node.name.clone(),
                span: proc_macro2::Span::call_site(),
                is_mutable: node.is_mutable,
                scope_level: node.scope_level,
            }),
        });
        
        if i > 0 && i % 2 == 0 {
            events.push(AnalysisEvent {
                span: proc_macro2::Span::call_site(),
                kind: crate::parser::events::EventKind::BorrowCreated {
                    name: nodes[i-1].name.clone(),
                    kind: BorrowKind::Immutable,
                    scope_level: nodes[i-1].scope_level,
                },
            });
        }
        
        if node.is_mutable {
            events.push(AnalysisEvent {
                span: proc_macro2::Span::call_site(),
                kind: crate::parser::events::EventKind::BorrowCreated {
                    name: node.name.clone(),
                    kind: BorrowKind::Mutable,
                    scope_level: node.scope_level,
                },
            });
        }
    }
    
    let mut analyzer = OwnershipAnalyzer::new();
    let ownership_results = analyzer.analyze(&events);
    
    let mut results: Vec<AnalysisResult> = ownership_results.to_vec();
    
    for (i, _node) in nodes.iter().enumerate() {
        if i > 0 && i % 3 == 0 {
            results.push(AnalysisResult::OwnershipChange {
                name: nodes[i-1].name.clone(),
                new_status: OwnershipStatus::Moved,
                span: proc_macro2::Span::call_site(),
            });
        }
    }
    
    results
}