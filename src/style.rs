//! Tree style definitions and customization options.

/// Predefined tree styles with different character sets.
///
/// # Examples
///
/// ```
/// use treelog::{TreeStyle, StyleConfig};
///
/// let style = StyleConfig::from(TreeStyle::Unicode);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum TreeStyle {
    /// Unicode box drawing characters (default: ├─, └─, │)
    #[default]
    Unicode,
    /// Simple ASCII characters (+, `, |)
    Ascii,
    /// Box drawing characters (┌, └, │, ─)
    Box,
    /// Custom character set
    Custom {
        /// Character for branch (non-last child)
        branch: String,
        /// Character for last child
        last: String,
        /// Character for vertical line
        vertical: String,
        /// Character for empty space
        empty: String,
    },
}

impl TreeStyle {
    /// Returns the default Unicode style configuration.
    #[inline]
    pub fn unicode() -> StyleConfig {
        StyleConfig {
            branch: " ├─".to_string(),
            last: " └─".to_string(),
            vertical: " │ ".to_string(),
            empty: "   ".to_string(),
        }
    }

    /// Returns the ASCII style configuration.
    #[inline]
    pub fn ascii() -> StyleConfig {
        StyleConfig {
            branch: " +-".to_string(),
            last: " `-".to_string(),
            vertical: " | ".to_string(),
            empty: "   ".to_string(),
        }
    }

    /// Returns the box drawing style configuration.
    #[inline]
    pub fn box_drawing() -> StyleConfig {
        StyleConfig {
            branch: " ├─".to_string(),
            last: " └─".to_string(),
            vertical: " │ ".to_string(),
            empty: "   ".to_string(),
        }
    }
}

/// Configuration for tree rendering style.
///
/// This struct defines the characters used to draw the tree structure.
///
/// # Examples
///
/// ```
/// use treelog::StyleConfig;
///
/// let config = StyleConfig {
///     branch: " ├─".to_string(),
///     last: " └─".to_string(),
///     vertical: " │ ".to_string(),
///     empty: "   ".to_string(),
/// };
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StyleConfig {
    /// Character sequence for a branch (non-last child)
    pub branch: String,
    /// Character sequence for the last child
    pub last: String,
    /// Character sequence for vertical continuation
    pub vertical: String,
    /// Character sequence for empty space
    pub empty: String,
}

impl Default for StyleConfig {
    fn default() -> Self {
        TreeStyle::unicode()
    }
}

impl From<TreeStyle> for StyleConfig {
    fn from(style: TreeStyle) -> Self {
        match style {
            TreeStyle::Unicode => TreeStyle::unicode(),
            TreeStyle::Ascii => TreeStyle::ascii(),
            TreeStyle::Box => TreeStyle::box_drawing(),
            TreeStyle::Custom {
                branch,
                last,
                vertical,
                empty,
            } => StyleConfig {
                branch,
                last,
                vertical,
                empty,
            },
        }
    }
}

impl StyleConfig {
    /// Creates a new style configuration with custom characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::StyleConfig;
    ///
    /// let config = StyleConfig::custom("├─", "└─", "│", "   ");
    /// ```
    pub fn custom(
        branch: impl Into<String>,
        last: impl Into<String>,
        vertical: impl Into<String>,
        empty: impl Into<String>,
    ) -> Self {
        StyleConfig {
            branch: branch.into(),
            last: last.into(),
            vertical: vertical.into(),
            empty: empty.into(),
        }
    }

    /// Returns the character sequence for a branch at the given position.
    ///
    /// `is_last` indicates if this is the last child at this level.
    #[inline]
    pub fn get_branch(&self, is_last: bool) -> &str {
        if is_last { &self.last } else { &self.branch }
    }

    /// Returns the character sequence for vertical continuation.
    #[inline]
    pub fn get_vertical(&self) -> &str {
        &self.vertical
    }

    /// Returns the character sequence for empty space.
    #[inline]
    pub fn get_empty(&self) -> &str {
        &self.empty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_style() {
        let config = StyleConfig::default();
        assert_eq!(config.branch, " ├─");
        assert_eq!(config.last, " └─");
    }

    #[test]
    fn test_unicode_style() {
        let config = TreeStyle::unicode();
        assert_eq!(config.branch, " ├─");
        assert_eq!(config.last, " └─");
    }

    #[test]
    fn test_ascii_style() {
        let config = TreeStyle::ascii();
        assert_eq!(config.branch, " +-");
        assert_eq!(config.last, " `-");
    }

    #[test]
    fn test_custom_style() {
        let config = StyleConfig::custom(">", "<", "|", " ");
        assert_eq!(config.branch, ">");
        assert_eq!(config.last, "<");
    }

    #[test]
    fn test_get_branch() {
        let config = StyleConfig::default();
        assert_eq!(config.get_branch(false), " ├─");
        assert_eq!(config.get_branch(true), " └─");
    }
}
