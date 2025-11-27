//! Example: Visualizing Git repository structures with treelog.

use treelog::Tree;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Git Repository Structure Example\n");

    // Build tree from current Git repository
    let tree = Tree::from_git_repo(".")?;
    println!("Repository structure:");
    println!("{}", tree.render_to_string());

    println!("\n---\n");

    // Build tree from branches
    let repo = git2::Repository::open(".")?;
    let branches_tree = Tree::from_git_branches(&repo)?;
    println!("Branches:");
    println!("{}", branches_tree.render_to_string());

    println!("\n---\n");

    // Build tree from HEAD commit
    if let Ok(head) = repo.head()
        && let Ok(commit) = head.peel_to_commit()
        && let Ok(commit_tree) = Tree::from_git_commit_tree(&repo, &commit)
    {
        println!("HEAD commit tree:");
        println!("{}", commit_tree.render_to_string());
    }

    Ok(())
}
