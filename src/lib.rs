pub mod analysis;
pub mod graph;
pub mod parser;
pub mod web;

pub use analysis::analyze;
pub use graph::dot_export::{DotConfig, DotExporter};
pub use graph::svg_renderer::{SvgConfig, SvgRenderer};
pub use graph::interactive_svg::InteractiveSvgRenderer;
pub use graph::timeline_animator::TimelineAnimator;
pub use parser::parse_code;