//! Configuration options for tree rendering.

use crate::style::StyleConfig;

/// Type alias for node formatter functions.
#[cfg(feature = "formatters")]
type NodeFormatter = Box<dyn Fn(&str) -> String + Send + Sync>;

/// Type alias for leaf formatter functions.
#[cfg(feature = "formatters")]
type LeafFormatter = Box<dyn Fn(&str) -> String + Send + Sync>;

/// Configuration for rendering a tree.
///
/// This struct allows fine-grained control over how trees are rendered,
/// including style, colors, formatting, and output options.
///
/// # Examples
///
/// ```
/// use treelog::{RenderConfig, TreeStyle};
///
/// let config = RenderConfig::default()
///     .with_style(TreeStyle::Ascii)
///     .with_colors(false);
/// ```
pub struct RenderConfig {
    /// Style configuration for the tree
    pub style: StyleConfig,
    /// Whether to enable color output (requires `color` feature)
    pub colors: bool,
    /// Custom formatter function for node labels (requires `formatters` feature)
    #[cfg(feature = "formatters")]
    pub node_formatter: Option<NodeFormatter>,
    /// Custom formatter function for leaf lines (requires `formatters` feature)
    #[cfg(feature = "formatters")]
    pub leaf_formatter: Option<LeafFormatter>,
    /// Line ending character(s)
    pub line_ending: String,
}

impl Clone for RenderConfig {
    fn clone(&self) -> Self {
        RenderConfig {
            style: self.style.clone(),
            colors: self.colors,
            #[cfg(feature = "formatters")]
            node_formatter: None, // Cannot clone function pointers, reset to None
            #[cfg(feature = "formatters")]
            leaf_formatter: None, // Cannot clone function pointers, reset to None
            line_ending: self.line_ending.clone(),
        }
    }
}

impl std::fmt::Debug for RenderConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("RenderConfig");
        debug
            .field("style", &self.style)
            .field("colors", &self.colors);
        #[cfg(feature = "formatters")]
        {
            debug
                .field("node_formatter", &self.node_formatter.is_some())
                .field("leaf_formatter", &self.leaf_formatter.is_some());
        }
        debug.field("line_ending", &self.line_ending).finish()
    }
}

impl Default for RenderConfig {
    fn default() -> Self {
        RenderConfig {
            style: StyleConfig::default(),
            colors: false,
            #[cfg(feature = "formatters")]
            node_formatter: None,
            #[cfg(feature = "formatters")]
            leaf_formatter: None,
            line_ending: "\n".to_string(),
        }
    }
}

impl RenderConfig {
    /// Creates a new render configuration with default settings.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the style configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::{RenderConfig, TreeStyle};
    ///
    /// let config = RenderConfig::default().with_style(TreeStyle::Ascii);
    /// ```
    pub fn with_style(mut self, style: impl Into<StyleConfig>) -> Self {
        self.style = style.into();
        self
    }

    /// Sets whether colors should be enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::RenderConfig;
    ///
    /// let config = RenderConfig::default().with_colors(true);
    /// ```
    pub fn with_colors(mut self, colors: bool) -> Self {
        self.colors = colors;
        self
    }

    /// Sets a custom formatter for node labels.
    ///
    /// Requires the `formatters` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::RenderConfig;
    ///
    /// let config = RenderConfig::default()
    ///     .with_node_formatter(|label| format!("[{}]", label));
    /// ```
    #[cfg(any(feature = "formatters", doc))]
    pub fn with_node_formatter<F>(mut self, formatter: F) -> Self
    where
        F: Fn(&str) -> String + Send + Sync + 'static,
    {
        self.node_formatter = Some(Box::new(formatter));
        self
    }

    /// Sets a custom formatter for leaf lines.
    ///
    /// Requires the `formatters` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::RenderConfig;
    ///
    /// let config = RenderConfig::default()
    ///     .with_leaf_formatter(|line| format!("- {}", line));
    /// ```
    #[cfg(any(feature = "formatters", doc))]
    pub fn with_leaf_formatter<F>(mut self, formatter: F) -> Self
    where
        F: Fn(&str) -> String + Send + Sync + 'static,
    {
        self.leaf_formatter = Some(Box::new(formatter));
        self
    }

    /// Sets the line ending character(s).
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::RenderConfig;
    ///
    /// let config = RenderConfig::default().with_line_ending("\r\n");
    /// ```
    pub fn with_line_ending(mut self, ending: impl Into<String>) -> Self {
        self.line_ending = ending.into();
        self
    }

    /// Formats a node label using the configured formatter, if any.
    pub(crate) fn format_node(&self, label: &str) -> String {
        #[cfg(feature = "formatters")]
        {
            if let Some(ref formatter) = self.node_formatter {
                return formatter(label);
            }
        }
        label.to_string()
    }

    /// Formats a leaf line using the configured formatter, if any.
    pub(crate) fn format_leaf(&self, line: &str) -> String {
        #[cfg(feature = "formatters")]
        {
            if let Some(ref formatter) = self.leaf_formatter {
                return formatter(line);
            }
        }
        line.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::TreeStyle;

    #[test]
    fn test_default_config() {
        let config = RenderConfig::default();
        assert!(!config.colors);
        assert_eq!(config.line_ending, "\n");
    }

    #[test]
    fn test_with_style() {
        let config = RenderConfig::default().with_style(TreeStyle::Ascii);
        assert_eq!(config.style.branch, " +-");
    }

    #[test]
    fn test_with_colors() {
        let config = RenderConfig::default().with_colors(true);
        assert!(config.colors);
    }

    #[cfg(feature = "formatters")]
    #[test]
    fn test_with_node_formatter() {
        let config = RenderConfig::default().with_node_formatter(|label| format!("[{label}]"));
        assert_eq!(config.format_node("test"), "[test]");
    }

    #[cfg(feature = "formatters")]
    #[test]
    fn test_with_leaf_formatter() {
        let config = RenderConfig::default().with_leaf_formatter(|line| format!("- {line}"));
        assert_eq!(config.format_leaf("test"), "- test");
    }
}
