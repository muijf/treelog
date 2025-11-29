//! Command-line interface for treelog.

mod args;
mod handlers;
mod utils;

#[cfg(any(
    feature = "export-html",
    feature = "export-svg",
    feature = "export-dot"
))]
pub use args::ExportFormat;
#[cfg(feature = "transform")]
pub use args::TransformOp;
pub use args::{Cli, Commands, FromSource, OutputFormat, SortMethod};

use clap::Parser;
use handlers::*;

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::From { source } => handle_from(source, &cli),
        Commands::Render { input } => handle_render(input, &cli),
        Commands::Stats { input } => handle_stats(input),
        Commands::Search { pattern, input } => handle_search(pattern, input),
        #[cfg(feature = "transform")]
        Commands::Transform { operation, input } => handle_transform(operation, input, &cli),
        Commands::Sort {
            method,
            reverse,
            input,
        } => handle_sort(method, *reverse, input, &cli),
        #[cfg(feature = "compare")]
        Commands::Compare { first, second } => handle_compare(first, second),
        #[cfg(feature = "merge")]
        Commands::Merge {
            strategy,
            first,
            second,
        } => handle_merge(strategy, first, second, &cli),
        #[cfg(any(
            feature = "export-html",
            feature = "export-svg",
            feature = "export-dot"
        ))]
        Commands::Export { format, input } => handle_export(format, input),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
