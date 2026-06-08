use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

use super::super::parser::events::{AnalysisEvent, Variable};

#[derive(Debug, Clone, PartialEq)]
pub struct VarNode {
    pub name: String,
    pub is_mutable: bool,
    pub scope_level: usize,
    pub defined: bool,
    pub used: bool,
}

pub type VarGraph = DiGraph<VarNode, ()>;

pub struct GraphBuilder {
    graph: VarGraph,
    node_map: HashMap<(String, usize), NodeIndex>,
}

impl GraphBuilder {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
        }
    }

    pub fn build_from_events(events: &[AnalysisEvent]) -> Self {
        let mut builder = Self::new();
        
        for event in events {
            match &event.kind {
                super::super::parser::events::EventKind::VarDefined(var) => {
                    builder.add_variable(var);
                }
                super::super::parser::events::EventKind::VarUsed { name, scope_level } => {
                    builder.record_use(name, *scope_level);
                }
                _ => {}
            }
        }
        
        builder
    }

    fn add_variable(&mut self, var: &Variable) {
        let key = (var.name.clone(), var.scope_level);
        
        if self.node_map.contains_key(&key) {
            return;
        }
        
        let node = VarNode {
            name: var.name.clone(),
            is_mutable: var.is_mutable,
            scope_level: var.scope_level,
            defined: true,
            used: false,
        };
        
        let node_idx = self.graph.add_node(node);
        self.node_map.insert(key, node_idx);
    }

    fn record_use(&mut self, name: &str, scope_level: usize) {
        let key = (name.to_string(), scope_level);
        if let Some(&node_idx) = self.node_map.get(&key) {
            if let Some(node) = self.graph.node_weight_mut(node_idx) {
                node.used = true;
            }
        } else {
            let parent_scope = scope_level - 1;
            let parent_key = (name.to_string(), parent_scope);
            if let Some(&node_idx) = self.node_map.get(&parent_key) {
                if let Some(node) = self.graph.node_weight_mut(node_idx) {
                    node.used = true;
                }
            }
        }
    }

    pub fn find_unused_variables(&self) -> Vec<&VarNode> {
        self.graph
            .node_weights()
            .filter(|node| node.defined && !node.used)
            .collect()
    }

    pub fn print_summary(&self) {
        let total_vars = self.graph.node_count();
        let used_vars = self.graph.node_weights().filter(|n| n.used).count();
        let unused_vars = self.find_unused_variables().len();

        println!("=== Variable Analysis Summary ===");
        println!("Total variables: {}", total_vars);
        println!("Used variables: {}", used_vars);
        println!("Unused variables: {}", unused_vars);
        println!();

        if unused_vars > 0 {
            println!("Unused variables:");
            for var in self.find_unused_variables() {
                println!("  - {} (scope: {})", var.name, var.scope_level);
            }
        }
    }
}
