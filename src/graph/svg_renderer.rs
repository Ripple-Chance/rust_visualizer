//! SVG renderer module for generating SVG output from DOT graphs
//! 
//! This module provides functionality to render DOT graphs as SVG images.
//! It can either invoke external Graphviz tools or generate a basic SVG representation.

use std::process::Command;
use std::path::Path;
use std::io::Write;
use crate::graph::dot_export::{DotConfig, DotExporter, DotStyle};
use crate::graph::variable_graph::VarGraph;
use crate::analysis::AnalysisResult;

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
    pub font_size: u32,
}

impl Default for SvgConfig {
    fn default() -> Self {
        Self {
            width: 1200,
            height: 800,
            background: Some("#FFFFFF".to_string()),
            font_family: "Arial, sans-serif".to_string(),
            font_size: 12,
        }
    }
}

/// SVG render result
#[derive(Debug)]
pub struct SvgRender {
    /// Generated SVG content
    pub content: String,
    /// Whether external tool was used
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
    pub fn is_dot_available() -> bool {
        Command::new("dot")
            .arg("-V")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
    
    /// Render a variable graph to SVG using Graphviz dot command
    pub fn render_with_dot(&self, graph: &VarGraph, analysis_results: &[AnalysisResult]) -> Result<SvgRender, String> {
        let exporter = DotExporter::with_config(DotConfig::default());
        let dot_export = exporter.export(graph, analysis_results);
        
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
    pub fn render_to_dot(&self, graph: &VarGraph, analysis_results: &[AnalysisResult]) -> String {
        let exporter = DotExporter::with_config(DotConfig::default());
        let dot_export = exporter.export(graph, analysis_results);
        dot_export.content
    }
    
    /// Generate a simple SVG representation without external tools
    /// This creates a basic visualization that doesn't require Graphviz
    pub fn render_simple(&self, graph: &VarGraph, analysis_results: &[AnalysisResult]) -> SvgRender {
        let exporter = DotExporter::with_config(DotConfig::default());
        let dot_export = exporter.export(graph, analysis_results);
        
        let mut svg = String::new();
        
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
    Rust Ownership Graph
  </text>
"#,
            self.config.font_family
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
                r#"    <rect x="0" y="{}" width="20" height="20" fill="{}" stroke="{stroke}"/>
    <text x="30" y="{}" font-family="Arial" font-size="12">{}</text>
"#,
                20 + i * 25,
                color,
                35 + i * 25,
                label,
                stroke = "#333"
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
        
        // Note about DOT export
        svg.push_str(&format!(
            r#"  <text x="50%" y="{}" text-anchor="middle" 
       font-family="Arial" font-size="12" fill="{color}">
    DOT format available for advanced visualization
  </text>
"#,
            self.config.height - 20,
            color = "#666"
        ));
        
        // Footer
        svg.push_str("</svg>\n");
        
        SvgRender {
            content: svg,
            external_tool: false,
        }
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
        
        let svg = renderer.render_simple(&graph, &results);
        
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
