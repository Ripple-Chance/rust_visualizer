use super::variable_graph::VarGraph;
use super::svg_renderer::SvgConfig;
use crate::analysis::AnalysisResult;
use std::collections::HashMap;
use std::io::Write;

pub struct InteractiveSvgRenderer {
    config: SvgConfig,
}

impl InteractiveSvgRenderer {
    pub fn new(config: SvgConfig) -> Self {
        Self { config }
    }

    pub fn render_interactive(
        &self,
        graph: &VarGraph,
        analysis_results: &[AnalysisResult],
        title: &str,
    ) -> String {
        let mut svg = String::new();
        
        svg.push_str(&format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">
                <style>
                    .node {{ cursor: pointer; transition: all 0.3s ease; }}
                    .node:hover {{ filter: drop-shadow(0 0 8px rgba(33, 150, 243, 0.8)); transform: scale(1.05); }}
                    .scope-box {{ fill: #E3F2FD; stroke: #1976D2; stroke-width: 2; rx: 10; }}
                    .tooltip {{ visibility: hidden; position: absolute; background: #333; color: white; padding: 8px; border-radius: 4px; font-size: 12px; pointer-events: none; }}
                    .title {{ font-family: Arial, sans-serif; font-size: 20px; font-weight: bold; fill: #1976D2; }}
                    .legend-box {{ fill: #F5F5F5; stroke: #E0E0E0; stroke-width: 1; rx: 5; }}
                </style>
                <script type="text/javascript"><![CDATA[
                    function showTooltip(evt, text) {{
                        var tooltip = document.getElementById('tooltip');
                        tooltip.textContent = text;
                        tooltip.setAttribute('visibility', 'visible');
                        tooltip.setAttribute('x', (evt.clientX - window.scrollX + 10) + 'px');
                        tooltip.setAttribute('y', (evt.clientY - window.scrollY - 10) + 'px');
                    }}
                    function hideTooltip() {{
                        var tooltip = document.getElementById('tooltip');
                        tooltip.setAttribute('visibility', 'hidden');
                    }}
                    function highlightNode(nodeId) {{
                        document.querySelectorAll('.node').forEach(n => n.style.opacity = '0.3');
                        document.getElementById(nodeId).style.opacity = '1';
                    }}
                    function resetHighlight() {{
                        document.querySelectorAll('.node').forEach(n => n.style.opacity = '1');
                    }}
                ]]></script>
                <g transform="translate(50, 50)">
            "#,
            self.config.width,
            self.config.height,
            self.config.width,
            self.config.height
        ));

        svg.push_str(&format!("<text x=\"{}\" y=\"25\" class=\"title\" text-anchor=\"middle\">{}</text>\n", 
            self.config.width / 2 - 50, title));

        let mut y_offset = 60;
        let scopes = self.group_by_scope(graph);
        
        for (scope_id, nodes) in scopes {
            let scope_height = 110;
            let node_spacing = 120;
            let node_width = 80;
            let padding = 20;
            
            let scope_width = if nodes.is_empty() {
                200
            } else {
                (nodes.len() as u32 - 1) * node_spacing + node_width + padding * 2
            };
            let scope_x = (self.config.width - 100 - scope_width) / 2;

            svg.push_str(&format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" class=\"scope-box\"/>\n",
                scope_x, y_offset, scope_width, scope_height));
            svg.push_str(&format!("                    <text x=\"{}\" y=\"{}\" font-family=\"Arial\" font-size=\"14\" font-weight=\"bold\" fill=\"#1976D2\" text-anchor=\"middle\">Scope {}</text>\n",
                scope_x + scope_width / 2, y_offset + 25, scope_id));

            for (i, node) in nodes.iter().enumerate() {
                let node_x = scope_x + padding + (i as u32) * node_spacing;
                let node_y = y_offset + 55;
                let color = self.get_node_color(node, analysis_results);
                
                let font_color = if color == "#EEEEEE" || color == "#F44336" { "#333" } else { "#FFFFFF" };
                svg.push_str(&format!("<g id=\"node_{}\" class=\"node\" onmouseenter=\"showTooltip(evt, 'Name: {}&#10;Mutable: {}&#10;Used: {}&#10;Scope: {}')\" onmouseleave=\"hideTooltip()\" onclick=\"highlightNode('node_{}')\">\n",
                    node.name, node.name, node.is_mutable, node.used, scope_id, node.name));
                svg.push_str(&format!("                        <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"40\" rx=\"8\" fill=\"{}\" stroke=\"#333\" stroke-width=\"2\"/>\n",
                    node_x, node_y - 20, node_width, color));
                svg.push_str(&format!("                        <text x=\"{}\" y=\"{}\" font-family=\"Arial\" font-size=\"12\" font-weight=\"bold\" fill=\"{}\" text-anchor=\"middle\">{}</text>\n",
                    node_x + node_width / 2, node_y - 5, font_color, node.name));
                svg.push_str(&format!("                        <text x=\"{}\" y=\"{}\" font-family=\"Arial\" font-size=\"10\" fill=\"#666\" text-anchor=\"middle\">{}</text>\n",
                    node_x + node_width / 2, node_y + 12, self.get_status_label(node, analysis_results)));
                svg.push_str("                    </g>\n");
            }

            y_offset += scope_height + 30;
        }

        svg.push_str(&self.render_legend());
        svg.push_str("<text id='tooltip' class='tooltip' x='0' y='0'/>");
        svg.push_str("</g></svg>");
        
        svg
    }

    fn group_by_scope<'a>(&self, graph: &'a VarGraph) -> HashMap<usize, Vec<&'a crate::graph::variable_graph::VarNode>> {
        let mut scopes: HashMap<usize, Vec<&'a crate::graph::variable_graph::VarNode>> = HashMap::new();
        
        for node in graph.node_weights() {
            scopes.entry(node.scope_level).or_insert_with(Vec::new).push(node);
        }
        
        scopes
    }

    fn get_node_color(&self, node: &crate::graph::variable_graph::VarNode, results: &[AnalysisResult]) -> String {
        if !node.used {
            return "#EEEEEE".to_string();
        }
        
        let mut last_non_dropped_status: Option<&crate::analysis::OwnershipStatus> = None;
        for result in results {
            if let AnalysisResult::OwnershipChange { name, new_status, .. } = result {
                if &node.name == name {
                    match new_status {
                        crate::analysis::OwnershipStatus::Dropped => {},
                        _ => { last_non_dropped_status = Some(new_status); }
                    }
                }
            }
        }
        
        if let Some(status) = last_non_dropped_status {
            match status {
                crate::analysis::OwnershipStatus::Owned => "#4CAF50".to_string(),
                crate::analysis::OwnershipStatus::Moved => "#FF9800".to_string(),
                crate::analysis::OwnershipStatus::Borrowed(kind) => match kind {
                    crate::analysis::BorrowKind::Immutable => "#2196F3".to_string(),
                    crate::analysis::BorrowKind::Mutable => "#F44336".to_string(),
                },
                crate::analysis::OwnershipStatus::Dropped => "#9E9E9E".to_string(),
            }
        } else {
            "#4CAF50".to_string()
        }
    }

    fn get_status_label(&self, node: &crate::graph::variable_graph::VarNode, results: &[AnalysisResult]) -> String {
        if !node.used {
            return "UNUSED".to_string();
        }
        
        let mut last_non_dropped_status: Option<&crate::analysis::OwnershipStatus> = None;
        for result in results {
            if let AnalysisResult::OwnershipChange { name, new_status, .. } = result {
                if &node.name == name {
                    match new_status {
                        crate::analysis::OwnershipStatus::Dropped => {},
                        _ => { last_non_dropped_status = Some(new_status); }
                    }
                }
            }
        }
        
        if let Some(status) = last_non_dropped_status {
            match status {
                crate::analysis::OwnershipStatus::Owned => "OWNED",
                crate::analysis::OwnershipStatus::Moved => "MOVED",
                crate::analysis::OwnershipStatus::Borrowed(kind) => match kind {
                    crate::analysis::BorrowKind::Immutable => "BORROWED",
                    crate::analysis::BorrowKind::Mutable => "MUT BORROW",
                },
                crate::analysis::OwnershipStatus::Dropped => "DROPPED",
            }
        } else {
            "OWNED"
        }.to_string()
    }

    fn render_legend(&self) -> String {
        let mut legend = String::new();
        let legend_x = self.config.width - 180;
        let legend_y = 60;
        let colors = [
            ("#4CAF50", "Owned"),
            ("#FF9800", "Moved"),
            ("#2196F3", "Borrowed"),
            ("#F44336", "Mut Borrow"),
            ("#EEEEEE", "Unused"),
        ];
        
        legend.push_str(&format!(
            "<rect x=\"{}\" y=\"{}\" width=\"160\" height=\"140\" class=\"legend-box\"/>\n",
            legend_x, legend_y
        ));
        legend.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" font-family=\"Arial\" font-size=\"14\" font-weight=\"bold\" fill=\"#333\">Legend</text>\n",
            legend_x + 80, legend_y + 20
        ));
        
        for (i, (color, label)) in colors.iter().enumerate() {
            legend.push_str(&format!(
                "<rect x=\"{}\" y=\"{}\" width=\"20\" height=\"15\" rx=\"3\" fill=\"{}\"/>\n",
                legend_x + 15,
                legend_y + 40 + i as u32 * 22,
                color
            ));
            legend.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" font-family=\"Arial\" font-size=\"12\" fill=\"#333\">{}</text>\n",
                legend_x + 45,
                legend_y + 52 + i as u32 * 22,
                label
            ));
        }
        
        legend
    }

    pub fn write_to_file(&self, content: &str, path: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}