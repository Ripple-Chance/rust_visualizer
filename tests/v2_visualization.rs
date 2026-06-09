//! Integration tests for v2.0 visualization features

use rust_visualizer::graph::dot_export::{DotConfig, DotExporter, DotStyle};
use rust_visualizer::graph::svg_renderer::{SvgRenderer, SvgConfig};
use rust_visualizer::graph::variable_graph::{VarNode, VarGraph};
use rust_visualizer::analysis::{AnalysisResult, BorrowKind, OwnershipStatus};
use petgraph::graph::DiGraph;
use proc_macro2::Span;
use std::collections::HashMap;

#[test]
fn test_dot_export_with_variables() {
    // Create a simple graph with variables
    let mut graph: VarGraph = DiGraph::new();
    let node1 = graph.add_node(VarNode {
        name: "x".to_string(),
        is_mutable: true,
        scope_level: 0,
        defined: true,
        used: true,
    });
    let node2 = graph.add_node(VarNode {
        name: "y".to_string(),
        is_mutable: false,
        scope_level: 0,
        defined: true,
        used: false,
    });
    graph.add_edge(node1, node2, ());
    
    let exporter = DotExporter::new();
    let results: Vec<AnalysisResult> = Vec::new();
    let export = exporter.export(&graph, &results, None);
    
    // Verify DOT content
    assert!(export.content.contains("digraph"));
    assert!(export.content.contains("x_0"));
    assert!(export.content.contains("y_0"));
    assert_eq!(export.node_count, 2);
    assert_eq!(export.edge_count, 1);
}

#[test]
fn test_dot_export_config() {
    let config = DotConfig {
        title: "Test Graph".to_string(),
        show_ownership: true,
        show_borrows: false,
        show_scopes: false,
        horizontal: true,
        show_unused: true,
    };
    
    let exporter = DotExporter::with_config(config);
    let graph: VarGraph = DiGraph::new();
    let results: Vec<AnalysisResult> = Vec::new();
    let export = exporter.export(&graph, &results, None);
    
    assert!(export.content.contains("Test_Graph"));
    assert!(export.content.contains("rankdir=LR"));
}

#[test]
fn test_dot_style_customization() {
    let mut ownership_colors = HashMap::new();
    ownership_colors.insert(OwnershipStatus::Owned, "#FF0000".to_string());
    
    let style = DotStyle {
        node_shape: "circle".to_string(),
        ownership_colors,
        borrow_colors: HashMap::new(),
        unused_style: "style=filled".to_string(),
        scope_style: "style=dashed".to_string(),
    };
    
    let config = DotConfig::default();
    let exporter = DotExporter::with_style(config, style);
    
    let mut graph: VarGraph = DiGraph::new();
    graph.add_node(VarNode {
        name: "test".to_string(),
        is_mutable: false,
        scope_level: 0,
        defined: true,
        used: true,
    });
    
    let results: Vec<AnalysisResult> = Vec::new();
    let export = exporter.export(&graph, &results, None);
    
    assert!(export.content.contains("shape=circle"));
}

#[test]
fn test_svg_renderer_simple() {
    let renderer = SvgRenderer::new();
    let graph: VarGraph = DiGraph::new();
    let results: Vec<AnalysisResult> = Vec::new();
    
    let svg = renderer.render_simple(&graph, &results, None);
    
    assert!(svg.content.contains("<svg"));
    assert!(svg.content.contains("</svg>"));
    assert!(!svg.external_tool);
}

#[test]
fn test_svg_config_defaults() {
    let config = SvgConfig::default();
    
    assert_eq!(config.width, 1200);
    assert_eq!(config.height, 800);
    assert_eq!(config.font_size, 12);
    assert_eq!(config.font_family, "Arial, sans-serif");
}

#[test]
fn test_scope_grouping() {
    let config = DotConfig {
        title: "Scope Test".to_string(),
        show_scopes: true,
        ..Default::default()
    };
    
    let mut graph: VarGraph = DiGraph::new();
    graph.add_node(VarNode {
        name: "outer".to_string(),
        is_mutable: false,
        scope_level: 0,
        defined: true,
        used: true,
    });
    graph.add_node(VarNode {
        name: "inner".to_string(),
        is_mutable: true,
        scope_level: 1,
        defined: true,
        used: true,
    });
    
    let exporter = DotExporter::with_config(config);
    let results: Vec<AnalysisResult> = Vec::new();
    let export = exporter.export(&graph, &results, None);
    
    assert!(export.content.contains("cluster_scope_0"));
    assert!(export.content.contains("cluster_scope_1"));
}

#[test]
fn test_unused_variable_filtering() {
    let config = DotConfig {
        show_unused: false,
        ..Default::default()
    };
    
    let mut graph: VarGraph = DiGraph::new();
    graph.add_node(VarNode {
        name: "used".to_string(),
        is_mutable: false,
        scope_level: 0,
        defined: true,
        used: true,
    });
    graph.add_node(VarNode {
        name: "unused".to_string(),
        is_mutable: false,
        scope_level: 0,
        defined: true,
        used: false,
    });
    
    let exporter = DotExporter::with_config(config);
    let results: Vec<AnalysisResult> = Vec::new();
    let export = exporter.export(&graph, &results, None);
    
    // Check that the node is not in the DOT output
    assert!(!export.content.contains("unused_0"));
    // The used variable should still be present
    assert!(export.content.contains("used_0"));
}

#[test]
fn test_ownership_status_in_export() {
    let mut graph: VarGraph = DiGraph::new();
    graph.add_node(VarNode {
        name: "x".to_string(),
        is_mutable: true,
        scope_level: 0,
        defined: true,
        used: true,
    });
    
    let results = vec![
        AnalysisResult::OwnershipChange {
            name: "x".to_string(),
            new_status: OwnershipStatus::Borrowed(BorrowKind::Mutable),
            span: Span::call_site(),
        },
    ];
    
    let exporter = DotExporter::new();
    let export = exporter.export(&graph, &results, None);
    
    // Graph should be generated with ownership info
    assert!(export.content.contains("x_0"));
}

#[test]
fn test_horizontal_layout() {
    let config = DotConfig {
        horizontal: true,
        ..Default::default()
    };
    
    let exporter = DotExporter::with_config(config);
    let graph: VarGraph = DiGraph::new();
    let results: Vec<AnalysisResult> = Vec::new();
    let export = exporter.export(&graph, &results, None);
    
    assert!(export.content.contains("rankdir=LR"));
}
