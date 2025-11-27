//! Git repository integration for visualizing Git structures using git2.

use crate::tree::Tree;

impl Tree {
    /// Builds a tree from a Git repository structure.
    ///
    /// Requires the `git2` feature.
    ///
    /// This creates a tree showing the repository structure with branches and commits.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use treelog::Tree;
    ///
    /// let tree = Tree::from_git_repo(".").unwrap();
    /// ```
    #[cfg(feature = "git2")]
    pub fn from_git_repo<P: AsRef<std::path::Path>>(path: P) -> Result<Self, git2::Error> {
        let repo = git2::Repository::open(path)?;
        let mut tree = Tree::new_node("repository".to_string());

        // Add branches
        if let Ok(branches_tree) = Self::from_git_branches(&repo) {
            tree.add_child(branches_tree);
        }

        // Add HEAD commit tree if available
        if let Ok(head) = repo.head()
            && let Ok(commit) = head.peel_to_commit()
            && let Ok(commit_tree) = Self::from_git_commit_tree(&repo, &commit)
        {
            tree.add_child(commit_tree);
        }

        Ok(tree)
    }

    /// Builds a tree from a specific Git commit's tree.
    ///
    /// Requires the `git2` feature.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use treelog::Tree;
    /// use git2::Repository;
    ///
    /// let repo = Repository::open(".").unwrap();
    /// let commit = repo.head().unwrap().peel_to_commit().unwrap();
    /// let tree = Tree::from_git_commit_tree(&repo, &commit).unwrap();
    /// ```
    #[cfg(feature = "git2")]
    pub fn from_git_commit_tree(
        repo: &git2::Repository,
        commit: &git2::Commit,
    ) -> Result<Self, git2::Error> {
        let tree_obj = commit.tree()?;
        let label = format!(
            "commit {} ({})",
            commit.id().to_string().chars().take(7).collect::<String>(),
            commit.summary().unwrap_or("no message")
        );
        Self::from_git_tree(repo, &tree_obj, &label)
    }

    /// Builds a tree from Git branches.
    ///
    /// Requires the `git2` feature.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use treelog::Tree;
    /// use git2::Repository;
    ///
    /// let repo = Repository::open(".").unwrap();
    /// let tree = Tree::from_git_branches(&repo).unwrap();
    /// ```
    #[cfg(feature = "git2")]
    pub fn from_git_branches(repo: &git2::Repository) -> Result<Self, git2::Error> {
        let branches = repo.branches(Some(git2::BranchType::Local))?;
        let mut tree = Tree::new_node("branches".to_string());

        for branch_result in branches {
            let (branch, _) = branch_result?;
            let name = branch.name()?.unwrap_or("unknown").to_string();

            let mut branch_tree = Tree::new_node(name.clone());

            if let Ok(commit) = branch.get().peel_to_commit() {
                let commit_summary = commit.summary().unwrap_or("no message").to_string();
                branch_tree.add_child(Tree::new_leaf(format!("commit: {}", commit_summary)));
            }

            tree.add_child(branch_tree);
        }

        Ok(tree)
    }

    #[cfg(feature = "git2")]
    fn from_git_tree(
        repo: &git2::Repository,
        tree_obj: &git2::Tree,
        label: &str,
    ) -> Result<Self, git2::Error> {
        let mut children = Vec::new();

        for entry in tree_obj.iter() {
            let name = entry.name().unwrap_or("unknown").to_string();
            match entry.kind() {
                Some(git2::ObjectType::Tree) => {
                    let obj = entry.to_object(repo)?;
                    let sub_tree = obj.as_tree().unwrap();
                    let child = Self::from_git_tree(repo, sub_tree, &name)?;
                    children.push(child);
                }
                Some(git2::ObjectType::Blob) => {
                    let obj = entry.to_object(repo)?;
                    let blob = obj.as_blob().unwrap();
                    let size = blob.content().len();
                    children.push(Tree::new_leaf(format!("{} ({} bytes)", name, size)));
                }
                _ => {
                    children.push(Tree::new_leaf(format!("{} (unknown type)", name)));
                }
            }
        }

        if children.is_empty() {
            Ok(Tree::new_leaf(label.to_string()))
        } else {
            Ok(Tree::Node(label.to_string(), children))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "git2")]
    #[test]
    fn test_git_repo_parsing() {
        // This test requires a valid Git repository
        // We'll just check that the function exists and can be called
        let result = Tree::from_git_repo(".");
        // This might fail if not run from a git repo, which is fine for a test
        if let Ok(tree) = result {
            assert!(tree.is_node());
        }
    }
}
