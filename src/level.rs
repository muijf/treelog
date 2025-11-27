//! Level path representation for tree rendering.

/// Represents a path through the tree, where each element indicates
/// whether that ancestor was the last child at its level.
///
/// This is used internally to track the tree structure for rendering
/// the appropriate prefix characters (branches, vertical lines, etc.).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LevelPath(Vec<bool>);

impl LevelPath {
    /// Creates a new empty level path (for the root node).
    #[inline]
    pub fn new() -> Self {
        LevelPath(Vec::new())
    }

    /// Creates a new level path from a vector of booleans.
    #[inline]
    pub fn from_vec(path: Vec<bool>) -> Self {
        LevelPath(path)
    }

    /// Returns the length of the path (depth in the tree).
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the path is empty (root level).
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns an iterator over the path elements.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = bool> + '_ {
        self.0.iter().copied()
    }

    /// Returns a slice of the path.
    #[inline]
    pub fn as_slice(&self) -> &[bool] {
        &self.0
    }

    /// Pushes a new element to the path.
    ///
    /// `is_last` indicates whether the current node is the last child at its level.
    #[inline]
    pub fn push(&mut self, is_last: bool) {
        self.0.push(is_last);
    }

    /// Returns a new path with an additional element appended.
    #[inline]
    pub fn with_child(&self, is_last: bool) -> Self {
        let mut new_path = self.0.clone();
        new_path.push(is_last);
        LevelPath(new_path)
    }
}

impl Default for LevelPath {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<bool>> for LevelPath {
    fn from(path: Vec<bool>) -> Self {
        LevelPath(path)
    }
}

impl From<LevelPath> for Vec<bool> {
    fn from(path: LevelPath) -> Self {
        path.0
    }
}

impl AsRef<[bool]> for LevelPath {
    fn as_ref(&self) -> &[bool] {
        &self.0
    }
}

impl LevelPath {
    /// Builds a LevelPath from a sequence of parent-child relationships.
    ///
    /// Given a function that can determine if a node at a given index is the last child
    /// of its parent, this constructs the appropriate LevelPath.
    ///
    /// # Arguments
    ///
    /// * `item_index` - The index of the item to build the path for
    /// * `get_parent` - Function that returns the parent index for a given item index, or None if root
    /// * `is_last_child` - Function that returns true if the given item is the last child of its parent
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::LevelPath;
    ///
    /// // Simple example: item 2 has parent 0, and is the last child
    /// let path = LevelPath::from_parent_chain(2, |idx| {
    ///     match idx {
    ///         1 => Some(0),
    ///         2 => Some(0),
    ///         _ => None,
    ///     }
    /// }, |idx| {
    ///     // Item 2 is the last child of parent 0
    ///     idx == 2
    /// });
    /// ```
    pub fn from_parent_chain<F, G>(item_index: usize, get_parent: F, is_last_child: G) -> Self
    where
        F: Fn(usize) -> Option<usize>,
        G: Fn(usize) -> bool,
    {
        let mut path = Vec::new();
        let mut current = Some(item_index);

        while let Some(idx) = current {
            if let Some(parent_idx) = get_parent(idx) {
                path.push(is_last_child(idx));
                current = Some(parent_idx);
            } else {
                break;
            }
        }

        path.reverse();
        LevelPath(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_parent_chain() {
        // Simple case: item 2 has parent 0, and is the last child
        let path = LevelPath::from_parent_chain(
            2,
            |idx| match idx {
                1 => Some(0),
                2 => Some(0),
                _ => None,
            },
            |idx| idx == 2, // Item 2 is the last child
        );

        // Path should be [false] (item 2 is not the last child of root... wait, that's wrong)
        // Actually, the path represents ancestors, so for item 2:
        // - Item 2's parent is 0
        // - Is 2 the last child of 0? Yes -> true
        // So path should be [true]
        assert_eq!(path.as_slice(), &[true]);

        // More complex: item 3 has parent 1, item 1 has parent 0
        // Item 3 is last child of 1, item 1 is not last child of 0
        let path = LevelPath::from_parent_chain(
            3,
            |idx| match idx {
                1 => Some(0),
                2 => Some(0),
                3 => Some(1),
                _ => None,
            },
            |idx| idx == 3 || idx == 2, // Items 2 and 3 are last children
        );

        // Path should be [false, true]:
        // - Item 1 (ancestor) is not last child of 0 -> false
        // - Item 3 is last child of 1 -> true
        assert_eq!(path.as_slice(), &[false, true]);
    }
}
