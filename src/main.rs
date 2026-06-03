use clap::{Parser, Subcommand};
use std::path::Path;
use syn::visit::Visit;

mod error;
mod graph;
mod parser;

use error::{Result, VisualizerError};
use graph::variable_graph::GraphBuilder;
use parser::{read_and_parse_file, AstVisitor};

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
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { file, list_vars, unused, events } => {
            analyze_file(&file, list_vars, unused, events)
        }
    }
}

fn analyze_file(file: &str, list_vars: bool, unused: bool, show_events: bool) -> Result<()> {
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

    if list_vars || !unused {
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

    Ok(())
}
