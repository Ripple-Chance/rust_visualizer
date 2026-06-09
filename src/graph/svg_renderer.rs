//! SVG renderer module for generating SVG output from DOT graphs
//!
//! This module provides functionality to render DOT graphs as SVG images.
//! It can either invoke external Graphviz tools or generate a basic SVG representation.

use std::process::Command;
use std::path::Path;
use std::io::Write;
use crate::graph::dot_export::{DotConfig, DotExporter};
use crate::graph::variable_graph::{VarGraph, VarNode};
use crate::analysis::{AnalysisResult, OwnershipStatus, BorrowKind};

/// SVG render configuration
#[derive(Debug, Clone)]
pub struct SvgConfig {
    /// Output width in pixels
    pub width: u32,
    /// Output height in pixels
    pub height: u32,
    /// Background color (transparent if None)
    pub background: Option<String>,
    /// Font family for labels
    pub font_family: String,
    /// Font size for node labels
    #[allow(dead_code)]
    pub font_size: u32,
    /// Graph title (uses filename if None)
    pub title: Option<String>,
}

impl Default for SvgConfig {
    fn default() -> Self {
        Self {
            width: 1200,
            height: 800,
            background: Some("#FFFFFF".to_string()),
            font_family: "Arial, sans-serif".to_string(),
            font_size: 12,
            title: None,
        }
    }
}

/// SVG render result
#[derive(Debug)]
pub struct SvgRender {
    /// Generated SVG content
    pub content: String,
    /// Whether external tool was used
    #[allow(dead_code)]
    pub external_tool: bool,
}

/// SVG renderer for DOT graphs
pub struct SvgRenderer {
    config: SvgConfig,
}

impl SvgRenderer {
    /// Create a new SVG renderer with default configuration
    pub fn new() -> Self {
        Self {
            config: SvgConfig::default(),
        }
    }

    /// Create a new SVG renderer with custom configuration
    pub fn with_config(config: SvgConfig) -> Self {
        Self { config }
    }

    /// Check if Graphviz dot command is available
    #[allow(dead_code)]
    pub fn is_dot_available() -> bool {
        Command::new("dot")
            .arg("-V")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Render a variable graph to SVG using Graphviz dot command
    #[allow(dead_code)]
    pub fn render_with_dot(&self, graph: &VarGraph, analysis_results: &[AnalysisResult]) -> Result<SvgRender, String> {
        let exporter = DotExporter::with_config(DotConfig::default());
        let dot_export = exporter.export(graph, analysis_results, None);

        // Try to use dot command
        let mut child = Command::new("dot")
            .arg("-Tsvg")
            .arg("-Gsize=12,8")
            .arg("-Gdpi=96")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn dot process: {}", e))?;

        if let Some(ref mut stdin) = child.stdin {
            stdin
                .write_all(dot_export.content.as_bytes())
                .map_err(|e| format!("Failed to write to dot: {}", e))?;
        }

        let output = child.wait_with_output()
            .map_err(|e| format!("Failed to wait for dot: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("dot command failed: {}", stderr));
        }

        let svg_content = String::from_utf8_lossy(&output.stdout).to_string();

        Ok(SvgRender {
            content: svg_content,
            external_tool: true,
        })
    }

    /// Render to DOT format (for manual conversion)
    #[allow(dead_code)]
    pub fn render_to_dot(&self, graph: &VarGraph, analysis_results: &[AnalysisResult]) -> String {
        let exporter = DotExporter::with_config(DotConfig::default());
        let dot_export = exporter.export(graph, analysis_results, None);
        dot_export.content
    }

    /// Generate a simple SVG representation without external tools
    /// This creates a basic visualization that doesn't require Graphviz
    /// If filename is provided, it will be used as the title (unless title is explicitly set)
    pub fn render_simple(&self, graph: &VarGraph, analysis_results: &[AnalysisResult], filename: Option<&str>) -> SvgRender {
        let exporter = DotExporter::with_config(DotConfig::default());
        let dot_export = exporter.export(graph, analysis_results, filename);

        let mut svg = String::new();

        // Determine title
        let title = self.config.title.clone().unwrap_or_else(|| {
            filename.map(|f| {
                // Extract just the filename without path
                std::path::Path::new(f).file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| f.to_string())
            }).unwrap_or_else(|| "Rust Ownership Graph".to_string())
        });

        // SVG header
        svg.push_str(&format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg"
     width="{}" height="{}" viewBox="0 0 {} {}">
"#,
            self.config.width,
            self.config.height,
            self.config.width,
            self.config.height
        ));

        // Background
        if let Some(bg) = &self.config.background {
            svg.push_str(&format!(
                r#"  <rect width="100%" height="100%" fill="{}"/>
"#,
                bg
            ));
        }

        // Title
        svg.push_str(&format!(
            r#"  <text x="50%" y="30" text-anchor="middle"
       font-family="{}" font-size="18" font-weight="bold">
    {}
  </text>
"#,
            self.config.font_family,
            title
        ));

        // Legend
        svg.push_str(r#"  <g id="legend" transform="translate(20, 50)">
    <text font-family="Arial" font-size="14" font-weight="bold">Legend</text>
"#);

        let legend_items = vec![
            ("Owned", "#4CAF50"),
            ("Moved", "#FF9800"),
            ("Borrowed", "#2196F3"),
            ("Dropped", "#9E9E9E"),
        ];

        for (i, (label, color)) in legend_items.iter().enumerate() {
            svg.push_str(&format!(
                r#"    <rect x="0" y="{}" width="20" height="20" fill="{}" stroke="{}"/>
    <text x="30" y="{}" font-family="Arial" font-size="12">{}</text>
"#,
                20 + i * 25,
                color,
                "#333",
                35 + i * 25,
                label,
            ));
        }

        svg.push_str("  </g>\n");

        // Statistics
        svg.push_str(&format!(
            r#"  <g id="stats" transform="translate({}, 50)">
    <text font-family="Arial" font-size="14" font-weight="bold">Statistics</text>
    <text x="0" y="20" font-family="Arial" font-size="12">Nodes: {}</text>
    <text x="0" y="40" font-family="Arial" font-size="12">Edges: {}</text>
  </g>
"#,
            self.config.width - 150,
            dot_export.node_count,
            dot_export.edge_count
        ));

        // Group nodes by scope level
        let scopes = self.group_by_scope(graph);
        
        // Calculate positions with scope grouping
        let _scope_positions = self.calculate_scope_positions(&scopes);
        
        let mut y_offset = 80; // Start below legend and stats
        
        // Draw scopes
        for (scope_level, nodes) in scopes {
            if nodes.is_empty() {
                continue;
            }
            
            let scope_height = 110;
            let node_spacing = 120;
            let node_width = 80;
            let padding = 20;
            
            // Calculate scope width based on nodes
            let scope_width = (nodes.len() as u32 - 1) * node_spacing + node_width + padding * 2;
            let scope_x = (self.config.width - scope_width) / 2;
            
            // Scope background
            svg.push_str(&format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"10\" fill=\"#E3F2FD\" stroke=\"#1976D2\" stroke-width=\"2\"/>\n",
                scope_x, y_offset, scope_width, scope_height
            ));
            
            // Scope label
            svg.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"Arial\" font-size=\"14\" font-weight=\"bold\" fill=\"#1976D2\">Scope {}</text>\n",
                scope_x + scope_width / 2,
                y_offset + 25,
                scope_level
            ));
            
            // Draw nodes in this scope
            let node_y = y_offset + 55;
            for (i, node_idx) in nodes.iter().enumerate() {
                if let Some(node) = graph.node_weight(*node_idx) {
                    let node_x = scope_x + padding + node_width / 2 + (i as u32) * node_spacing;
                    let color = self.get_node_color(node, analysis_results);
                    let is_mutable = node.is_mutable;
                    
                    // Node rectangle
                    let stroke_color = "#333";
                    let font_color = if is_mutable { "#D32F2F" } else { "#333" };
                    let text_color = if !node.used { "#999" } else { font_color };
                    
                    svg.push_str(&format!(
                        r#"  <g transform="translate({}, {})">
    <rect x="-40" y="-20" width="80" height="40" rx="5" fill="{}" stroke="{}" stroke-width="2"/>
    <text x="0" y="5" text-anchor="middle" font-family="Arial" font-size="14" font-weight="bold" fill="{}">{}</text>
"#,
                        node_x, node_y,
                        color,
                        stroke_color,
                        text_color,
                        node.name
                    ));
                    
                    // Mutable indicator
                    if is_mutable {
                        let mut_color = "#D32F2F";
                        svg.push_str(&format!(
                            r#"    <text x="0" y="18" text-anchor="middle" font-family="Arial" font-size="10" fill="{}">mut</text>
"#,
                            mut_color
                        ));
                    }
                    
                    svg.push_str("  </g>\n");
                }
            }
            
            y_offset += scope_height + 30; // Add spacing between scopes
        }

        // Footer
        svg.push_str(&format!(
            r#"  <text x="50%" y="{}" text-anchor="middle"
       font-family="Arial" font-size="12" fill="{}">
    Generated by RustVisualizer v2.0
  </text>
"#,
            self.config.height - 20,
            "#666"
        ));

        svg.push_str("</svg>\n");

        SvgRender {
            content: svg,
            external_tool: false,
        }
    }

    /// Calculate positions for nodes using a simple grid layout
    #[allow(dead_code)]
    fn calculate_node_positions(&self, graph: &VarGraph) -> std::collections::HashMap<petgraph::graph::NodeIndex, (u32, u32)> {
        let mut positions = std::collections::HashMap::new();
        let node_count = graph.node_count() as u32;

        if node_count == 0 {
            return positions;
        }

        // Use a simple grid layout
        let cols = ((node_count as f32).sqrt().ceil() as u32).max(1);
        let rows = ((node_count + cols - 1) / cols).max(1);

        let margin_left = 200;  // Leave space for legend
        let margin_top = 150;   // Leave space for title
        let margin_right = 50;
        let margin_bottom = 100;

        let available_width = self.config.width - margin_left - margin_right;
        let available_height = self.config.height - margin_top - margin_bottom;

        let cell_width = available_width / cols;
        let cell_height = available_height / rows;

        for (i, node_idx) in graph.node_indices().enumerate() {
            let row = i as u32 / cols;
            let col = i as u32 % cols;

            let x = margin_left + col * cell_width + cell_width / 2;
            let y = margin_top + row * cell_height + cell_height / 2;

            positions.insert(node_idx, (x, y));
        }

        positions
    }

    /// Get color for a node based on its ownership status and usage
    fn get_node_color(&self, node: &VarNode, analysis_results: &[AnalysisResult]) -> String {
        // Unused variables get a light gray color
        if !node.used {
            return "#EEEEEE".to_string();
        }

        // Default color for owned variables
        let mut color = "#4CAF50".to_string();

        // Check analysis results for ownership changes - use the last non-Dropped state
        // (Dropped is the final state of all variables when scope exits, so we ignore it)
        let mut last_non_dropped_status: Option<&OwnershipStatus> = None;
        for result in analysis_results {
            if let AnalysisResult::OwnershipChange { name, new_status, .. } = result {
                if &node.name == name {
                    match new_status {
                        OwnershipStatus::Dropped => {
                            // Don't update - keep the previous non-Dropped status
                        }
                        _ => {
                            last_non_dropped_status = Some(new_status);
                        }
                    }
                }
            }
        }

        if let Some(status) = last_non_dropped_status {
            match status {
                OwnershipStatus::Owned => color = "#4CAF50".to_string(),
                OwnershipStatus::Moved => color = "#FF9800".to_string(),
                OwnershipStatus::Borrowed(kind) => match kind {
                    BorrowKind::Immutable => color = "#2196F3".to_string(),
                    BorrowKind::Mutable => color = "#F44336".to_string(),
                },
                OwnershipStatus::Dropped => color = "#9E9E9E".to_string(),
            }
        }

        color
    }
    
    /// Group nodes by scope level
    fn group_by_scope(&self, graph: &VarGraph) -> Vec<(usize, Vec<petgraph::graph::NodeIndex>)> {
        let mut scopes: std::collections::HashMap<usize, Vec<petgraph::graph::NodeIndex>> = std::collections::HashMap::new();
        
        for node_idx in graph.node_indices() {
            if let Some(node) = graph.node_weight(node_idx) {
                scopes
                    .entry(node.scope_level)
                    .or_insert_with(Vec::new)
                    .push(node_idx);
            }
        }
        
        // Convert to sorted vector
        let mut scopes_vec: Vec<_> = scopes.into_iter().collect();
        scopes_vec.sort_by(|a, b| a.0.cmp(&b.0));
        scopes_vec
    }
    
    /// Calculate positions for scopes
    fn calculate_scope_positions(&self, scopes: &[(usize, Vec<petgraph::graph::NodeIndex>)]) -> std::collections::HashMap<usize, (u32, u32)> {
        let mut positions = std::collections::HashMap::new();
        let mut y = 80; // Start below title and legend
        
        for (scope_level, nodes) in scopes {
            let scope_width = (nodes.len() as u32) * 120 + 40;
            let x = (self.config.width - scope_width) / 2;
            positions.insert(*scope_level, (x, y));
            y += 130; // 100 height + 30 spacing
        }
        
        positions
    }

    /// Export SVG to a file
    pub fn export_to_file(&self, svg: &SvgRender, path: &Path) -> Result<(), String> {
        std::fs::write(path, &svg.content)
            .map_err(|e| format!("Failed to write SVG file: {}", e))
    }
}

impl Default for SvgRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::DiGraph;

    #[test]
    fn test_svg_config_defaults() {
        let config = SvgConfig::default();

        assert_eq!(config.width, 1200);
        assert_eq!(config.height, 800);
        assert_eq!(config.font_size, 12);
    }

    #[test]
    fn test_render_simple() {
        let graph: VarGraph = DiGraph::new();
        let renderer = SvgRenderer::new();
        let results: Vec<AnalysisResult> = Vec::new();

        let svg = renderer.render_simple(&graph, &results, None);

        assert!(svg.content.contains("<svg"));
        assert!(svg.content.contains("</svg>"));
        assert!(!svg.external_tool);
    }

    #[test]
    fn test_render_to_dot() {
        let graph: VarGraph = DiGraph::new();
        let renderer = SvgRenderer::new();
        let results: Vec<AnalysisResult> = Vec::new();

        let dot = renderer.render_to_dot(&graph, &results);

        assert!(dot.contains("digraph"));
        assert!(dot.contains("}"));
    }
}
