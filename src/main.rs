use clap::{Parser, Subcommand};
use std::path::Path;
use syn::visit::Visit;

mod analysis;
mod error;
mod graph;
mod parser;

use error::{Result, VisualizerError};
use graph::variable_graph::GraphBuilder;
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
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { file, list_vars, unused, events, ownership, borrow, lifetime } => {
            analyze_file(&file, list_vars, unused, events, ownership, borrow, lifetime)
        }
    }
}

fn analyze_file(file: &str, list_vars: bool, unused: bool, show_events: bool, 
               ownership: bool, borrow: bool, lifetime: bool) -> Result<()> {
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

    Ok(())
}
