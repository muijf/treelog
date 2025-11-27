//! Example: Visualizing clap command structures with treelog.

use clap::{Arg, Command};
use treelog::Tree;

fn main() {
    println!("Clap Command Structure Visualization Example\n");

    // Create a command with subcommands and arguments
    let cmd = Command::new("myapp")
        .about("An example application")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Configuration file path"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output"),
        )
        .subcommand(
            Command::new("build").about("Build the project").arg(
                Arg::new("release")
                    .long("release")
                    .help("Build in release mode"),
            ),
        )
        .subcommand(
            Command::new("test").about("Run tests").arg(
                Arg::new("filter")
                    .long("filter")
                    .help("Filter tests by name"),
            ),
        )
        .subcommand(
            Command::new("run").about("Run the application").subcommand(
                Command::new("server")
                    .about("Run as server")
                    .arg(Arg::new("port").long("port").help("Server port")),
            ),
        );

    // Convert to tree
    let tree = Tree::from_clap_command(&cmd);
    println!("Command structure:");
    println!("{}", tree.render_to_string());

    println!("\n---\n");

    // Show a simpler example
    let simple_cmd = Command::new("simple")
        .arg(Arg::new("input").help("Input file"))
        .subcommand(Command::new("sub"));

    let simple_tree = Tree::from_clap_command(&simple_cmd);
    println!("Simple command structure:");
    println!("{}", simple_tree.render_to_string());
}
