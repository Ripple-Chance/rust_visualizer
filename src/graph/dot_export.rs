//! DOT export module for generating Graphviz DOT format output
//! 
//! This module provides functionality to export variable relationship graphs
//! to DOT format for visualization with Graphviz tools.

use crate::analysis::{AnalysisResult, BorrowKind, OwnershipStatus};
use crate::graph::variable_graph::{VarGraph, VarNode};
use std::collections::HashMap;

/// Configuration for DOT export
#[derive(Debug, Clone)]
pub struct DotConfig {
    /// Graph title
    pub title: String,
    /// Show variable ownership status
    pub show_ownership: bool,
    /// Show borrow relationships
    pub show_borrows: bool,
    /// Show scope levels
    pub show_scopes: bool,
    /// Use rankdir LR (left-to-right) instead of TB (top-to-bottom)
    pub horizontal: bool,
    /// Include unused variables
    pub show_unused: bool,
}

impl Default for DotConfig {
    fn default() -> Self {
        Self {
            title: "Rust Ownership Graph".to_string(),
            show_ownership: true,
            show_borrows: true,
            show_scopes: true,
            horizontal: false,
            show_unused: true,
        }
    }
}

/// Visual style configuration for DOT elements
#[derive(Debug, Clone)]
pub struct DotStyle {
    /// Node shape
    pub node_shape: String,
    /// Ownership status colors
    pub ownership_colors: HashMap<OwnershipStatus, String>,
    /// Borrow kind colors
    pub borrow_colors: HashMap<BorrowKind, String>,
    /// Unused variable style
    pub unused_style: String,
    /// Scope group style
    pub scope_style: String,
}

impl Default for DotStyle {
    fn default() -> Self {
        let mut ownership_colors = HashMap::new();
        ownership_colors.insert(OwnershipStatus::Owned, "#4CAF50".to_string()); // Green
        ownership_colors.insert(OwnershipStatus::Moved, "#FF9800".to_string()); // Orange
        ownership_colors.insert(OwnershipStatus::Borrowed(BorrowKind::Immutable), "#2196F3".to_string()); // Blue
        ownership_colors.insert(OwnershipStatus::Dropped, "#9E9E9E".to_string()); // Gray
        
        let mut borrow_colors = HashMap::new();
        borrow_colors.insert(BorrowKind::Immutable, "#2196F3".to_string()); // Blue
        borrow_colors.insert(BorrowKind::Mutable, "#F44336".to_string()); // Red
        
        Self {
            node_shape: "box".to_string(),
            ownership_colors,
            borrow_colors,
            unused_style: "style=filled,fillcolor=#EEEEEE,fontcolor=#9E9E9E".to_string(),
            scope_style: "style=filled,fillcolor=#E3F2FD,color=#1976D2".to_string(),
        }
    }
}

/// DOT export result
#[derive(Debug)]
pub struct DotExport {
    /// Generated DOT content
    pub content: String,
    /// Number of nodes
    pub node_count: usize,
    /// Number of edges
    pub edge_count: usize,
}

/// DOT exporter for variable relationship graphs
pub struct DotExporter {
    config: DotConfig,
    style: DotStyle,
}

impl DotExporter {
    /// Create a new DOT exporter with default configuration
    pub fn new() -> Self {
        Self {
            config: DotConfig::default(),
            style: DotStyle::default(),
        }
    }
    
    /// Create a new DOT exporter with custom configuration
    pub fn with_config(config: DotConfig) -> Self {
        Self {
            config,
            style: DotStyle::default(),
        }
    }
    
    /// Create a new DOT exporter with custom style
    pub fn with_style(config: DotConfig, style: DotStyle) -> Self {
        Self { config, style }
    }
    
    /// Export a variable graph to DOT format
    pub fn export(&self, graph: &VarGraph, analysis_results: &[AnalysisResult]) -> DotExport {
        let mut content = String::new();
        let mut edge_count = 0;
        
        // Graph header
        content.push_str(&format!("digraph {} {{\n", self.sanitize_id(&self.config.title)));
        content.push_str("  // Graph attributes\n");
        content.push_str(&format!("  label=\"{}\";\n", self.config.title));
        content.push_str("  labelloc=t;\n");
        content.push_str("  fontsize=16;\n");
        content.push_str("  fontname=\"Arial\";\n");
        
        // Layout direction
        if self.config.horizontal {
            content.push_str("  rankdir=LR;\n");
        } else {
            content.push_str("  rankdir=TB;\n");
        }
        
        content.push_str("  compound=true;\n");
        content.push_str("  splines=ortho;\n");
        content.push_str("  nodesep=0.5;\n");
        content.push_str("  ranksep=0.8;\n\n");
        
        // Scope grouping
        if self.config.show_scopes {
            content.push_str("  // Scope subgraphs\n");
            let scopes = self.group_by_scope(graph);
            for (scope_level, nodes) in scopes {
                content.push_str(&format!("  subgraph cluster_scope_{} {{\n", scope_level));
                content.push_str(&format!("    label=\"Scope {}\";\n", scope_level));
                content.push_str(&format!("    {};\n", self.style.scope_style));
                
                for node_idx in nodes {
                    if let Some(node) = graph.node_weight(node_idx) {
                        // Skip unused variables if not configured to show them
                        if !self.config.show_unused && !node.used && node.defined {
                            continue;
                        }
                        content.push_str(&format!("    \"{}\";\n", self.node_id(node)));
                    }
                }
                
                content.push_str("  }\n");
            }
            content.push_str("\n");
        }
        
        // Generate nodes
        content.push_str("  // Variable nodes\n");
        for node_idx in graph.node_indices() {
            if let Some(node) = graph.node_weight(node_idx) {
                // Skip unused variables if not configured to show them
                if !self.config.show_unused && !node.used && node.defined {
                    continue;
                }
                
                content.push_str(&format!("  \"{}\" [", self.node_id(node)));
                
                // Node attributes
                let mut attrs = Vec::new();
                attrs.push(format!("label=\"{}\"", node.name));
                attrs.push(format!("shape={}", self.style.node_shape));
                
                if self.config.show_ownership {
                    // Determine ownership status from analysis results
                    let status = self.get_ownership_status(node, analysis_results);
                    if let Some(color) = self.style.ownership_colors.get(&status) {
                        attrs.push(format!("fillcolor=\"{}\"", color));
                        attrs.push("style=filled".to_string());
                    }
                }
                
                // Mark unused variables
                if node.defined && !node.used {
                    attrs.push(self.style.unused_style.clone());
                }
                
                // Mutable indicator
                if node.is_mutable {
                    attrs.push("fontcolor=#D32F2F".to_string());
                }
                
                content.push_str(&attrs.join(", "));
                content.push_str("];\n");
            }
        }
        content.push_str("\n");
        
        // Generate edges
        content.push_str("  // Variable relationships\n");
        for edge_idx in graph.edge_indices() {
            if let Some((source, target)) = graph.edge_endpoints(edge_idx) {
                edge_count += 1;
                content.push_str(&format!(
                    "  \"{}\" -> \"{}\" [",
                    self.node_id_from_index(graph, source),
                    self.node_id_from_index(graph, target)
                ));
                
                // Edge attributes
                content.push_str("arrowhead=vee, color=\"#757575\"];\n");
            }
        }
        content.push_str("\n");
        
        // Graph footer
        content.push_str("}\n");
        
        let node_count = graph.node_count();
        
        DotExport {
            content,
            node_count,
            edge_count,
        }
    }
    
    /// Get ownership status for a node based on analysis results
    fn get_ownership_status(&self, node: &VarNode, results: &[AnalysisResult]) -> OwnershipStatus {
        for result in results {
            if let AnalysisResult::OwnershipChange { name, new_status, .. } = result {
                if &node.name == name {
                    return new_status.clone();
                }
            }
        }
        OwnershipStatus::Owned
    }
    
    /// Group nodes by scope level
    fn group_by_scope(&self, graph: &VarGraph) -> HashMap<usize, Vec<petgraph::graph::NodeIndex>> {
        let mut scopes: HashMap<usize, Vec<petgraph::graph::NodeIndex>> = HashMap::new();
        
        for node_idx in graph.node_indices() {
            if let Some(node) = graph.node_weight(node_idx) {
                scopes
                    .entry(node.scope_level)
                    .or_insert_with(Vec::new)
                    .push(node_idx);
            }
        }
        
        scopes
    }
    
    /// Create a safe DOT node ID
    fn node_id(&self, node: &VarNode) -> String {
        format!("{}_{}", node.name, node.scope_level)
    }
    
    /// Create a safe DOT node ID from index
    fn node_id_from_index(&self, graph: &VarGraph, idx: petgraph::graph::NodeIndex) -> String {
        if let Some(node) = graph.node_weight(idx) {
            self.node_id(node)
        } else {
            format!("unknown_{:?}", idx)
        }
    }
    
    /// Sanitize string for DOT ID
    fn sanitize_id(&self, s: &str) -> String {
        s.chars()
            .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
            .collect()
    }
}

impl Default for DotExporter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::DiGraph;
    
    #[test]
    fn test_dot_export_basic() {
        let graph: VarGraph = DiGraph::new();
        let exporter = DotExporter::new();
        let results: Vec<AnalysisResult> = Vec::new();
        
        let export = exporter.export(&graph, &results);
        
        assert!(export.content.contains("digraph"));
        assert_eq!(export.node_count, 0);
        assert_eq!(export.edge_count, 0);
    }
    
    #[test]
    fn test_dot_config_defaults() {
        let config = DotConfig::default();
        
        assert_eq!(config.title, "Rust Ownership Graph");
        assert!(config.show_ownership);
        assert!(config.show_borrows);
        assert!(config.show_scopes);
        assert!(!config.horizontal);
    }
    
    #[test]
    fn test_dot_style_defaults() {
        let style = DotStyle::default();
        
        assert_eq!(style.node_shape, "box");
        assert_eq!(style.ownership_colors.len(), 4);
        assert_eq!(style.borrow_colors.len(), 2);
    }
    
    #[test]
    fn test_sanitize_id() {
        let exporter = DotExporter::new();
        
        assert_eq!(exporter.sanitize_id("test-graph"), "test_graph");
        assert_eq!(exporter.sanitize_id("test graph"), "test_graph");
        assert_eq!(exporter.sanitize_id("test-graph_123"), "test_graph_123");
    }
}
