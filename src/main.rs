use clap::{Parser, Subcommand};
use std::path::Path;
use syn::visit::Visit;

mod analysis;
mod error;
mod graph;
mod parser;

use error::{Result, VisualizerError};
use graph::variable_graph::GraphBuilder;
use graph::dot_export::{DotExporter, DotConfig};
use graph::svg_renderer::{SvgRenderer, SvgConfig};
use parser::{read_and_parse_file, AstVisitor};
use analysis::{OwnershipAnalyzer, BorrowAnalyzer, LifetimeAnalyzer};

#[derive(Parser)]
#[command(name = "rust_visualizer")]
#[command(about = "Rust code ownership and lifetime visualizer", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Analyze Rust source code")]
    Analyze {
        #[arg(help = "Path to the Rust source file")]
        file: String,
        
        #[arg(long, help = "List all variables")]
        list_vars: bool,
        
        #[arg(long, help = "Show unused variables")]
        unused: bool,
        
        #[arg(long, help = "Show all events")]
        events: bool,
        
        #[arg(long, help = "Show ownership analysis")]
        ownership: bool,
        
        #[arg(long, help = "Show borrow analysis")]
        borrow: bool,
        
        #[arg(long, help = "Show lifetime analysis")]
        lifetime: bool,
        
        #[arg(long, help = "Export to DOT format file")]
        dot: Option<String>,
        
        #[arg(long, help = "Export to SVG format file")]
        svg: Option<String>,
        
        #[arg(long, help = "Use horizontal layout")]
        horizontal: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { file, list_vars, unused, events, ownership, borrow, lifetime, dot, svg, horizontal } => {
            analyze_file(&file, list_vars, unused, events, ownership, borrow, lifetime, dot, svg, horizontal)
        }
    }
}

fn analyze_file(file: &str, list_vars: bool, unused: bool, show_events: bool, 
               ownership: bool, borrow: bool, lifetime: bool,
               dot_file: Option<String>, svg_file: Option<String>, horizontal: bool) -> Result<()> {
    let path = Path::new(file);
    
    if !path.exists() {
        return Err(VisualizerError::InvalidInput(format!(
            "File not found: {}",
            file
        )));
    }

    let ast = read_and_parse_file(file)?;
    
    let mut visitor = AstVisitor::new();
    visitor.visit_file(&ast);

    if show_events {
        println!("=== Analysis Events ===");
        for (i, event) in visitor.events.iter().enumerate() {
            println!("{:4}: {}", i, event);
        }
        println!();
    }

    let graph_builder = GraphBuilder::build_from_events(&visitor.events);

    if list_vars || !unused && !ownership && !borrow && !lifetime {
        graph_builder.print_summary();
    }

    if unused {
        let unused_vars = graph_builder.find_unused_variables();
        if unused_vars.is_empty() {
            println!("All variables are used!");
        } else {
            println!("Unused variables:");
            for var in unused_vars {
                println!("  - {} (scope: {})", var.name, var.scope_level);
            }
        }
    }

    if ownership {
        println!("=== Ownership Analysis ===");
        let mut ownership_analyzer = OwnershipAnalyzer::new();
        let ownership_results = ownership_analyzer.analyze(&visitor.events);
        for result in ownership_results {
            println!("{}", result);
        }
        println!();
    }

    if borrow {
        println!("=== Borrow Analysis ===");
        let mut borrow_analyzer = BorrowAnalyzer::new();
        let borrow_results = borrow_analyzer.analyze(&visitor.events);
        for result in borrow_results {
            println!("{}", result);
        }
        
        let long_chains = borrow_analyzer.find_long_borrow_chains(3);
        if !long_chains.is_empty() {
            println!("\nLong borrow chains (threshold: 3 references):");
            for (name, count) in long_chains {
                println!("  - {}: {} references", name, count);
            }
        }
        println!();
    }

    if lifetime {
        println!("=== Lifetime Analysis ===");
        let mut lifetime_analyzer = LifetimeAnalyzer::new();
        let lifetime_results = lifetime_analyzer.analyze(&visitor.events);
        for result in lifetime_results {
            println!("{}", result);
        }
        
        let summary = lifetime_analyzer.get_lifetime_summary();
        println!("\nLifetime Summary:");
        for (name, references) in summary {
            println!("  - {}: {} references", name, references);
        }
        println!();
    }

    // Export to DOT format
    if let Some(dot_path) = dot_file {
        let mut ownership_analyzer = OwnershipAnalyzer::new();
        let ownership_results = ownership_analyzer.analyze(&visitor.events);
        
        let config = DotConfig {
            title: "Rust Ownership Graph".to_string(),
            show_ownership: true,
            show_borrows: true,
            show_scopes: true,
            horizontal,
            show_unused: true,
        };
        
        let exporter = DotExporter::with_config(config);
        let graph = graph_builder.get_graph();
        let dot_export = exporter.export(&graph, ownership_results, Some(file));
        
        std::fs::write(&dot_path, &dot_export.content)
            .map_err(|e| VisualizerError::InvalidInput(format!("Failed to write DOT file: {}", e)))?;
        
        println!("✓ DOT file exported to: {}", dot_path);
        println!("  Nodes: {}, Edges: {}", dot_export.node_count, dot_export.edge_count);
        println!("  Use 'dot -Tpng {} -o output.png' to generate PNG", dot_path);
    }

    // Export to SVG format
    if let Some(svg_path) = svg_file {
        let mut ownership_analyzer = OwnershipAnalyzer::new();
        let ownership_results = ownership_analyzer.analyze(&visitor.events);
        
        let config = SvgConfig {
            width: 1200,
            height: 800,
            background: Some("#FFFFFF".to_string()),
            font_family: "Arial, sans-serif".to_string(),
            font_size: 12,
            title: None,
        };
        
        let renderer = SvgRenderer::with_config(config);
        let graph = graph_builder.get_graph();
        let svg = renderer.render_simple(&graph, ownership_results, Some(file));
        
        renderer.export_to_file(&svg, Path::new(&svg_path))
            .map_err(|e| VisualizerError::InvalidInput(e))?;
        
        println!("✓ SVG file exported to: {}", svg_path);
        println!("  Open this file in a web browser to view");
    }

    Ok(())
}
