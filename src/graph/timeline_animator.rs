use super::variable_graph::VarGraph;
use crate::analysis::{AnalysisResult, OwnershipStatus};
use std::collections::HashMap;

pub struct TimelineAnimator {
    graph: VarGraph,
    results: Vec<AnalysisResult>,
    frame_count: usize,
}

impl TimelineAnimator {
    pub fn new(graph: VarGraph, results: Vec<AnalysisResult>) -> Self {
        let frame_count = results.len().max(1);
        Self { graph, results, frame_count }
    }

    pub fn get_frame_count(&self) -> usize {
        self.frame_count
    }

    pub fn generate_frame(&self, frame_index: usize) -> FrameData {
        let mut node_states = HashMap::new();
        let active_results = &self.results[..=frame_index.min(self.results.len() - 1)];
        
        for node in self.graph.node_weights() {
            let status = self.get_node_status_at_frame(node.name.as_str(), active_results);
            node_states.insert(node.name.clone(), status);
        }
        
        FrameData {
            frame_index,
            total_frames: self.frame_count,
            node_states,
            event_description: self.get_event_description(frame_index),
        }
    }

    fn get_node_status_at_frame(&self, node_name: &str, results: &[AnalysisResult]) -> OwnershipStatus {
        let mut status = OwnershipStatus::Owned;
        
        for result in results {
            if let AnalysisResult::OwnershipChange { name, new_status, .. } = result {
                if name == node_name {
                    status = new_status.clone();
                }
            }
        }
        
        status
    }

    fn get_event_description(&self, frame_index: usize) -> String {
        if frame_index >= self.results.len() {
            return "Analysis complete".to_string();
        }
        
        match &self.results[frame_index] {
            AnalysisResult::OwnershipChange { name, new_status, .. } => {
                format!("{}: {}", 
                    self.status_description(new_status),
                    name
                )
            }
            _ => "Unknown event".to_string(),
        }
    }

    fn status_description(&self, status: &OwnershipStatus) -> String {
        match status {
            OwnershipStatus::Owned => "Variable declared".to_string(),
            OwnershipStatus::Moved => "Ownership moved".to_string(),
            OwnershipStatus::Borrowed(kind) => match kind {
                crate::analysis::BorrowKind::Immutable => "Immutable borrow".to_string(),
                crate::analysis::BorrowKind::Mutable => "Mutable borrow".to_string(),
            },
            OwnershipStatus::Dropped => "Variable dropped".to_string(),
        }
    }

    pub fn generate_animated_svg(&self, title: &str) -> String {
        let mut svg = String::new();
        let width = 900;
        let height = 250; // 减少高度，消除空白
        
        svg.push_str(&format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">
                <style>
                    .node {{ cursor: pointer; transition: fill 0.5s ease; }}
                    .scope-box {{ fill: #E3F2FD; stroke: #1976D2; stroke-width: 2; rx: 10; }}
                    .title {{ font-family: Arial, sans-serif; font-size: 20px; font-weight: bold; fill: #1976D2; }}
                </style>
            "#,
            width, height, width, height
        ));

        svg.push_str(&format!("<text x=\"{}\" y=\"30\" class=\"title\" text-anchor=\"middle\">{} - Timeline Animation</text>\n", 
            width / 2, title));

        for (frame_idx, _) in self.results.iter().enumerate() {
            svg.push_str(&format!("<g id=\"frame-{}\" class=\"frame-content\" style=\"display: {};\">\n", 
                frame_idx, if frame_idx == 0 { "block" } else { "none" }));
            svg.push_str(&self.render_frame_content(frame_idx));
            svg.push_str("</g>\n");
        }

        svg.push_str(&self.render_timeline_controls(width));
        svg.push_str("</svg>");
        
        svg
    }

    fn get_event_list_js(&self) -> String {
        let events: Vec<String> = self.results.iter()
            .map(|r| match r {
                AnalysisResult::OwnershipChange { name, new_status, .. } => {
                    format!("\"{}: {}\"",
                        self.status_description(new_status),
                        name
                    )
                }
                _ => "\"Unknown event\"".to_string(),
            })
            .collect();
        format!("[{}]", events.join(", "))
    }

    fn group_by_scope<'a>(&self, graph: &'a VarGraph) -> HashMap<usize, Vec<&'a crate::graph::variable_graph::VarNode>> {
        let mut scopes: HashMap<usize, Vec<&'a crate::graph::variable_graph::VarNode>> = HashMap::new();
        
        for node in graph.node_weights() {
            scopes.entry(node.scope_level).or_insert_with(Vec::new).push(node);
        }
        
        scopes
    }

    fn render_frame_content(&self, frame_idx: usize) -> String {
        let frame = self.generate_frame(frame_idx);
        let mut content = String::new();
        let mut y_offset = 60;
        let scopes = self.group_by_scope(&self.graph);
        
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
            let scope_x = (900 - scope_width) / 2;

            content.push_str(&format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" class=\"scope-box\"/>\n",
                scope_x, y_offset, scope_width, scope_height
            ));
            content.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" font-family=\"Arial\" font-size=\"14\" font-weight=\"bold\" fill=\"#1976D2\" text-anchor=\"middle\">Scope {}</text>\n",
                scope_x + scope_width / 2, y_offset + 25, scope_id
            ));

            for (i, node) in nodes.iter().enumerate() {
                let node_x = scope_x + padding + (i as u32) * node_spacing;
                let node_y = y_offset + 55;
                let status = frame.node_states.get(&node.name).unwrap();
                let color = self.status_to_color(status);
                
                content.push_str(&format!(
                    "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"40\" rx=\"8\" fill=\"{}\" stroke=\"#333\" stroke-width=\"2\" class=\"node\"/>\n",
                    node_x, node_y - 20, node_width, color
                ));
                content.push_str(&format!(
                    "<text x=\"{}\" y=\"{}\" font-family=\"Arial\" font-size=\"12\" font-weight=\"bold\" fill=\"{}\" text-anchor=\"middle\">{}</text>\n",
                    node_x + node_width / 2, node_y - 5,
                    if color == "#EEEEEE" || color == "#F44336" { "#333" } else { "#FFFFFF" },
                    node.name
                ));
            }

            y_offset += scope_height + 30;
        }
        
        content
    }

    fn status_to_color(&self, status: &OwnershipStatus) -> String {
        match status {
            OwnershipStatus::Owned => "#4CAF50".to_string(),
            OwnershipStatus::Moved => "#FF9800".to_string(),
            OwnershipStatus::Borrowed(kind) => match kind {
                crate::analysis::BorrowKind::Immutable => "#2196F3".to_string(),
                crate::analysis::BorrowKind::Mutable => "#F44336".to_string(),
            },
            OwnershipStatus::Dropped => "#9E9E9E".to_string(),
        }
    }

    fn render_timeline_controls(&self, _width: u32) -> String {
        // 不再在SVG中渲染控制条，控制按钮已移至HTML中
        String::new()
    }

    pub fn generate_animated_html(&self, title: &str) -> String {
        let svg_content = self.generate_animated_svg(title);
        let frame_count = self.frame_count;
        
        format!(
            r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Rust Ownership Timeline Animation - {}</title>
    <style>
        * {{
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }}
        body {{
            font-family: 'Segoe UI', Arial, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
            padding: 20px;
        }}
        .container {{
            background: white;
            border-radius: 16px;
            box-shadow: 0 20px 60px rgba(0,0,0,0.2);
            padding: 30px;
            max-width: 1000px;
            width: 100%;
        }}
        .header {{
            text-align: center;
            margin-bottom: 25px;
        }}
        .header h1 {{
            color: #1976D2;
            font-size: 28px;
            font-weight: 700;
            margin-bottom: 8px;
        }}
        .header p {{
            color: #666;
            font-size: 14px;
        }}
        .legend {{
            display: flex;
            justify-content: center;
            gap: 25px;
            margin-bottom: 25px;
            flex-wrap: wrap;
        }}
        .legend-item {{
            display: flex;
            align-items: center;
            gap: 8px;
            font-size: 14px;
            color: #333;
        }}
        .legend-color {{
            width: 24px;
            height: 24px;
            border-radius: 6px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }}
        .legend-color.owned {{ background: #4CAF50; }}
        .legend-color.immutable {{ background: #2196F3; }}
        .legend-color.mutable {{ background: #F55050; }}
        .legend-color.moved {{ background: #FF9800; }}
        .legend-color.unused {{ background: #E0E0E0; }}
        .svg-container {{
            background: #FAFAFA;
            border-radius: 12px;
            padding: 20px;
            margin-bottom: 20px;
        }}
        svg {{
            display: block;
            margin: 0 auto;
        }}
        .controls {{
            background: #F8F9FA;
            border-radius: 12px;
            padding: 20px;
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 20px;
        }}
        .btn {{
            width: 48px;
            height: 48px;
            border: none;
            border-radius: 50%;
            background: linear-gradient(135deg, #64B5F6, #42A5F5);
            color: white;
            font-size: 20px;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
            transition: all 0.3s ease;
            box-shadow: 0 4px 12px rgba(66, 165, 245, 0.4);
        }}
        .btn:hover {{
            transform: translateY(-2px);
            box-shadow: 0 6px 16px rgba(66, 165, 245, 0.5);
        }}
        .btn:active {{
            transform: translateY(0);
        }}
        .frame-info {{
            font-size: 16px;
            font-weight: 600;
            color: #333;
            min-width: 100px;
            text-align: center;
        }}
        .progress-container {{
            flex: 1;
            max-width: 300px;
        }}
        .progress-bar {{
            height: 8px;
            background: #E0E0E0;
            border-radius: 4px;
            overflow: hidden;
        }}
        .progress-fill {{
            height: 100%;
            background: linear-gradient(90deg, #4CAF50, #81C784);
            border-radius: 4px;
            transition: width 0.3s ease;
        }}
        .event-info {{
            margin-top: 15px;
            text-align: center;
            padding: 12px;
            background: #E3F2FD;
            border-radius: 8px;
            color: #1976D2;
            font-weight: 500;
            font-size: 14px;
        }}
        .node {{
            cursor: pointer;
            transition: all 0.3s ease;
        }}
        .node:hover {{
            transform: scale(1.05);
            filter: brightness(1.1);
        }}
        .scope-box {{
            fill: rgba(227, 242, 253, 0.8);
            stroke: #1976D2;
            stroke-width: 2;
            rx: 12;
        }}
        .title {{
            font-family: 'Segoe UI', Arial, sans-serif;
            font-size: 22px;
            font-weight: 700;
            fill: #1976D2;
        }}
        .scope-label {{
            font-family: 'Segoe UI', Arial, sans-serif;
            font-size: 16px;
            font-weight: 600;
            fill: #1976D2;
        }}
        .var-label {{
            font-family: 'Segoe UI', Arial, sans-serif;
            font-size: 14px;
            font-weight: 600;
            fill: white;
            text-shadow: 0 1px 2px rgba(0,0,0,0.2);
        }}
        .timeline-progress {{
            fill: #4CAF50;
            transition: width 0.3s ease;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>📊 Rust Ownership Timeline Animation</h1>
            <p>Visualizing Rust's ownership system - File: {}</p>
        </div>
        
        <div class="legend">
            <div class="legend-item">
                <div class="legend-color owned"></div>
                <span>Owned (拥有所有权)</span>
            </div>
            <div class="legend-item">
                <div class="legend-color immutable"></div>
                <span>Immutable Borrow (不可变借用)</span>
            </div>
            <div class="legend-item">
                <div class="legend-color mutable"></div>
                <span>Mutable Borrow (可变借用)</span>
            </div>
            <div class="legend-item">
                <div class="legend-color moved"></div>
                <span>Moved (所有权已移动)</span>
            </div>
            <div class="legend-item">
                <div class="legend-color unused"></div>
                <span>Unused (未使用)</span>
            </div>
        </div>
        
        <div class="svg-container">
            {}
        </div>
        
        <div class="controls">
            <button class="btn" onclick="prevFrame()" title="上一帧">
                ⏮️
            </button>
            <div class="frame-info">
                <span id="frame-info">Frame 1 / {}</span>
            </div>
            <div class="progress-container">
                <div class="progress-bar">
                    <div class="progress-fill" id="progress" style="width: 0%"></div>
                </div>
            </div>
            <button class="btn" onclick="nextFrame()" title="下一帧">
                ⏭️
            </button>
        </div>
        
        <div class="event-info" id="event-info">
            Initial state
        </div>
    </div>
    
    <script>
        var currentFrame = 0;
        var totalFrames = {};
        var events = {};
        
        function setFrame(frame) {{
            currentFrame = Math.max(0, Math.min(frame, totalFrames - 1));
            updateDisplay();
        }}
        
        function nextFrame() {{
            setFrame(currentFrame + 1);
        }}
        
        function prevFrame() {{
            setFrame(currentFrame - 1);
        }}
        
        function updateDisplay() {{
            document.querySelectorAll('.frame-content').forEach(el => el.style.display = 'none');
            document.getElementById('frame-' + currentFrame).style.display = 'block';
            document.getElementById('progress').style.width = ((currentFrame / (totalFrames - 1)) * 100) + '%';
            document.getElementById('frame-info').textContent = 'Frame ' + (currentFrame + 1) + ' / ' + totalFrames;
            document.getElementById('event-info').textContent = events[currentFrame];
        }}
    </script>
</body>
</html>"#,
            title, title, svg_content, frame_count, frame_count,
            self.get_event_list_js()
        )
    }
}

pub struct FrameData {
    pub frame_index: usize,
    pub total_frames: usize,
    pub node_states: HashMap<String, OwnershipStatus>,
    pub event_description: String,
}