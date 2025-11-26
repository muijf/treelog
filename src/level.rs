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
