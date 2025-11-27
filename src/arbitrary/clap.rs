//! Clap command-line argument tree visualization.

use crate::tree::Tree;

impl Tree {
    /// Builds a tree from a clap::Command structure.
    ///
    /// Requires the `clap` feature.
    ///
    /// Visualizes the command hierarchy including subcommands and arguments.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    /// use clap::Command;
    ///
    /// let cmd = Command::new("myapp")
    ///     .subcommand(Command::new("subcommand"));
    /// let tree = Tree::from_clap_command(&cmd);
    /// ```
    #[cfg(feature = "arbitrary-clap")]
    pub fn from_clap_command(cmd: &clap::Command) -> Self {
        let name = cmd.get_name().to_string();
        let mut children = Vec::new();

        // Add subcommands
        for subcmd in cmd.get_subcommands() {
            children.push(Self::from_clap_command(subcmd));
        }

        // Add arguments/flags
        for arg in cmd.get_arguments() {
            let mut arg_parts = Vec::new();
            if let Some(short) = arg.get_short() {
                arg_parts.push(format!("-{}", short));
            }
            if let Some(long) = arg.get_long() {
                if !arg_parts.is_empty() {
                    arg_parts.push(", ".to_string());
                }
                arg_parts.push(format!("--{}", long));
            }
            if arg_parts.is_empty() {
                arg_parts.push(arg.get_id().as_str().to_string());
            }
            let mut arg_label = format!("arg: {}", arg_parts.join(""));
            if let Some(help) = arg.get_help() {
                arg_label.push_str(&format!(" ({})", help));
            }
            children.push(Tree::new_leaf(arg_label));
        }

        // Note: Global arguments are handled by clap automatically
        // They don't need to be explicitly added to the tree

        if children.is_empty() {
            Tree::new_leaf(name)
        } else {
            Tree::Node(name, children)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "arbitrary-clap")]
    #[test]
    fn test_from_clap_command() {
        use clap::Command;

        let cmd = Command::new("test").subcommand(Command::new("sub"));
        let tree = Tree::from_clap_command(&cmd);
        assert!(tree.is_node());
        assert_eq!(tree.label(), Some("test"));
    }

    #[cfg(feature = "arbitrary-clap")]
    #[test]
    fn test_from_clap_command_with_args() {
        use clap::{Arg, Command};

        let cmd = Command::new("test").arg(Arg::new("input").short('i').long("input"));
        let tree = Tree::from_clap_command(&cmd);
        assert!(tree.is_node());
    }
}
