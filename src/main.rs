use clap::{Parser, Subcommand};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;

use rust_visualizer::analysis::analyze;
use rust_visualizer::graph::dot_export::DotExporter;
use rust_visualizer::graph::interactive_svg::InteractiveSvgRenderer;
use rust_visualizer::graph::svg_renderer::{SvgConfig, SvgRenderer};
use rust_visualizer::graph::timeline_animator::TimelineAnimator;
use rust_visualizer::parser::parse_code;
use rust_visualizer::web::service::start_server;

#[derive(Parser)]
#[command(name = "rust_visualizer", version = "3.0.0", about = "Rust code ownership visualizer")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Analyze {
        input_file: String,
        
        #[arg(long)]
        dot: Option<String>,
        
        #[arg(long)]
        svg: Option<String>,
        
        #[arg(long)]
        interactive: Option<String>,
        
        #[arg(long)]
        animation: Option<String>,
        
        #[arg(long)]
        html: Option<String>,
        
        #[arg(long)]
        json: Option<String>,
    },
    
    Server {
        #[arg(long, default_value = "8080")]
        port: u16,
    },
    
    Batch {
        input_dir: String,
        
        #[arg(long)]
        output_dir: String,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Analyze { 
            input_file, 
            dot, 
            svg, 
            interactive, 
            animation,
            html,
            json 
        } => {
            analyze_file(&input_file, dot, svg, interactive, animation, html, json);
        }
        
        Commands::Server { port } => {
            println!("{} Starting web server on port {}...", 
                "[INFO]".blue(), port.to_string().green());
            let _ = tokio::runtime::Builder::new_current_thread().build().unwrap().block_on(start_server(port));
        }
        
        Commands::Batch { input_dir, output_dir } => {
            batch_analyze(&input_dir, &output_dir);
        }
    }
}

fn analyze_file(
    input_file: &str, 
    dot: Option<String>, 
    svg: Option<String>, 
    interactive: Option<String>, 
    animation: Option<String>,
    html: Option<String>,
    json: Option<String>
) {
    let pb = ProgressBar::new(4);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{bar:40.cyan/blue}] {msg}")
        .unwrap()
        .progress_chars("#>-"));
    
    pb.set_message("Reading file...");
    let content = fs::read_to_string(input_file).expect("Failed to read input file");
    pb.inc(1);
    
    pb.set_message("Parsing code...");
    let graph = parse_code(&content).expect("Failed to parse code");
    pb.inc(1);
    
    pb.set_message("Analyzing ownership...");
    let results = analyze(&graph);
    pb.inc(1);
    
    let file_name = Path::new(input_file).file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "analysis".to_string());
    
    pb.set_message("Generating output...");
    
    if let Some(dot_path) = dot {
        let exporter = DotExporter::new();
        let dot_export = exporter.export(&graph, &results, Some(&file_name));
        fs::write(&dot_path, dot_export.content).expect("Failed to write DOT file");
        println!("{} DOT file exported to: {}", "[OK]".green(), dot_path.blue());
    }
    
    if let Some(svg_path) = svg {
        let renderer = SvgRenderer::new();
        let svg_render = renderer.render_simple(&graph, &results, Some(&file_name));
        fs::write(&svg_path, svg_render.content).expect("Failed to write SVG file");
        println!("{} SVG file exported to: {}", "[OK]".green(), svg_path.blue());
    }
    
    if let Some(interactive_path) = interactive {
        let config = SvgConfig::default();
        let renderer = InteractiveSvgRenderer::new(config);
        let svg_content = renderer.render_interactive(&graph, &results, &file_name);
        fs::write(&interactive_path, svg_content).expect("Failed to write interactive SVG file");
        println!("{} Interactive SVG file exported to: {}", "[OK]".green(), interactive_path.blue());
    }
    
    if let Some(animation_path) = animation {
        let animator = TimelineAnimator::new(graph.clone(), results.clone());
        let svg_content = animator.generate_animated_svg(&file_name);
        fs::write(&animation_path, svg_content).expect("Failed to write animation SVG file");
        println!("{} Animated SVG file exported to: {}", "[OK]".green(), animation_path.blue());
    }
    
    if let Some(html_path) = html {
        let animator = TimelineAnimator::new(graph.clone(), results.clone());
        let html_content = animator.generate_animated_html(&file_name);
        fs::write(&html_path, html_content).expect("Failed to write HTML animation file");
        println!("{} HTML animation file exported to: {}", "[OK]".green(), html_path.blue());
    }
    
    if let Some(json_path) = json {
        let used_count = graph.node_weights().filter(|n| n.used).count();
        let total_count = graph.node_count();
        let variables: Vec<_> = graph.node_weights().map(|n| {
            serde_json::json!({
                "name": n.name,
                "is_mutable": n.is_mutable,
                "used": n.used,
                "scope_level": n.scope_level
            })
        }).collect();
        let json_content = serde_json::json!({
            "file": input_file,
            "total_variables": total_count,
            "used_variables": used_count,
            "unused_variables": total_count - used_count,
            "variables": variables
        });
        fs::write(&json_path, serde_json::to_string_pretty(&json_content).unwrap())
            .expect("Failed to write JSON file");
        println!("{} JSON file exported to: {}", "[OK]".green(), json_path.blue());
    }
    
    pb.inc(1);
    pb.finish_with_message("Analysis complete!");
    
    println!("\n{}", "=== Variable Analysis Summary ===".yellow());
    let total_count = graph.node_count();
    let used_count = graph.node_weights().filter(|n| n.used).count();
    println!("Total variables: {}", total_count.to_string().bold());
    println!("Used variables: {}", used_count.to_string().green());
    println!("Unused variables: {}", (total_count - used_count).to_string().red());
    
    let unused_vars: Vec<_> = graph.node_weights()
        .filter(|n| !n.used)
        .map(|n| (n.name.clone(), n.scope_level))
        .collect();
    
    if !unused_vars.is_empty() {
        println!("\n{}", "Unused variables:".yellow());
        for (name, scope) in unused_vars {
            println!("  - {} (scope: {})", name, scope);
        }
    }
}

fn batch_analyze(input_dir: &str, output_dir: &str) {
    use walkdir::WalkDir;
    
    fs::create_dir_all(output_dir).expect("Failed to create output directory");
    
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {msg}")
        .unwrap());
    
    let mut file_count = 0;
    let mut success_count = 0;
    
    for entry in WalkDir::new(input_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension().map(|e| e == "rs").unwrap_or(false) {
            file_count += 1;
            pb.set_message(format!("Processing: {}", entry.path().display()));
            
            let file_name = entry.path().file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| format!("file{}", file_count));
            
            let content = match fs::read_to_string(entry.path()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{} Failed to read {}: {}", 
                        "[ERROR]".red(), entry.path().display(), e);
                    continue;
                }
            };
            
            let graph = match parse_code(&content) {
                Ok(g) => g,
                Err(e) => {
                    println!("{} Failed to parse {}: {}", 
                        "[ERROR]".red(), entry.path().display(), e);
                    continue;
                }
            };
            
            let results = analyze(&graph);
            
            let svg_path = Path::new(output_dir).join(format!("{}.svg", file_name));
            let renderer = SvgRenderer::new();
            let svg_render = renderer.render_simple(&graph, &results, Some(&file_name));
            
            if fs::write(&svg_path, svg_render.content).is_ok() {
                success_count += 1;
            } else {
                println!("{} Failed to write {}", 
                    "[ERROR]".red(), svg_path.display());
            }
        }
    }
    
    pb.finish_with_message("Batch processing complete!");
    
    println!("\n{}", "=== Batch Analysis Summary ===".yellow());
    println!("Total files processed: {}", file_count);
    println!("Success: {}", success_count.to_string().green());
    println!("Failed: {}", (file_count - success_count).to_string().red());
}