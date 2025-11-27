//! Indicatif integration for tree-based progress bars.
//!
//! This module provides integration with the `indicatif` crate to display
//! hierarchical progress bars with automatic tree positioning and a `{tree}`
//! template key for rendering tree prefixes.
//!
//! # Examples
//!
//! ## Simple Usage
//!
//! ```no_run
//! use indicatif::{ProgressBar, ProgressStyle};
//! use treelog::indicatif::TreeProgressManager;
//!
//! let manager = TreeProgressManager::new();
//! let mp = manager.get_multi_progress();
//!
//! // Add a root progress bar - messages are automatically updated when tree changes
//! let root_pb = ProgressBar::new(100);
//! root_pb.set_style(ProgressStyle::with_template("{wide_msg}").unwrap());
//! let root_id = manager.add_progress_bar(None, root_pb, mp);
//! manager.set_message(root_id, "Root task");
//!
//! // Add a child progress bar
//! let child_pb = ProgressBar::new(50);
//! child_pb.set_style(ProgressStyle::with_template("{wide_msg}").unwrap());
//! let child_id = manager.add_progress_bar(Some(root_id), child_pb, mp);
//! manager.set_message(child_id, "Child task");
//!
//! // When you add more items, all messages are automatically updated with correct prefixes!
//! ```

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use indicatif::{MultiProgress, ProgressBar, ProgressState};

use crate::level::LevelPath;
use crate::prefix::compute_prefix;
use crate::style::StyleConfig;

// Thread-local storage to track which progress bar is currently being rendered
thread_local! {
    static CURRENT_BAR_ID: std::cell::Cell<Option<usize>> = const { std::cell::Cell::new(None) };
}

/// Manages hierarchical progress bars with automatic tree positioning.
///
/// This manager wraps a `MultiProgress` and handles:
/// - Automatic positioning of progress bars based on parent-child relationships
/// - Tree prefix computation for the `{tree}` template key
/// - Maintaining tree structure state
pub struct TreeProgressManager {
    multi_progress: MultiProgress,
    state: Arc<Mutex<TreeState>>,
}

struct TreeState {
    items: Vec<ProgressBarItem>,
    index_to_position: HashMap<usize, usize>,
    parent_to_children: HashMap<usize, Vec<usize>>,
    tree_prefixes: HashMap<usize, String>,
    base_messages: HashMap<usize, String>,
    style: StyleConfig,
}

struct ProgressBarItem {
    id: usize,
    progress_bar: ProgressBar,
    parent: Option<usize>,
}

impl TreeProgressManager {
    /// Creates a new `TreeProgressManager` with default style configuration.
    pub fn new() -> Self {
        Self::with_style(StyleConfig::default())
    }

    /// Creates a new `TreeProgressManager` with a custom style configuration.
    pub fn with_style(style: StyleConfig) -> Self {
        Self {
            multi_progress: MultiProgress::new(),
            state: Arc::new(Mutex::new(TreeState {
                items: Vec::new(),
                index_to_position: HashMap::new(),
                parent_to_children: HashMap::new(),
                tree_prefixes: HashMap::new(),
                base_messages: HashMap::new(),
                style,
            })),
        }
    }

    /// Returns a reference to the underlying `MultiProgress`.
    pub fn get_multi_progress(&self) -> &MultiProgress {
        &self.multi_progress
    }

    /// Adds a progress bar to the manager with an optional parent.
    ///
    /// # Arguments
    ///
    /// * `parent_id` - The ID of the parent progress bar, or `None` for a root bar
    /// * `progress_bar` - The progress bar to add
    /// * `multi_progress` - The MultiProgress instance (should be from `get_multi_progress()`)
    ///
    /// # Returns
    ///
    /// The unique ID assigned to this progress bar, which can be used as a parent for child bars.
    ///
    /// # Note
    ///
    /// After adding a progress bar, you should wrap it with `wrap_progress_bar` to enable
    /// the `{tree}` template key to work correctly.
    pub fn add_progress_bar(
        &self,
        parent_id: Option<usize>,
        progress_bar: ProgressBar,
        multi_progress: &MultiProgress,
    ) -> usize {
        let mut state = self.state.lock().unwrap();
        let id = state.items.len();

        let item = ProgressBarItem {
            id,
            progress_bar: progress_bar.clone(),
            parent: parent_id,
        };

        let insert_pos = state.find_insert_position(&item);
        multi_progress.insert(insert_pos, progress_bar.clone());

        state.items.insert(insert_pos, item);

        // Update position mappings for all items after the insertion point
        let items_to_update: Vec<(usize, usize)> = state
            .items
            .iter()
            .enumerate()
            .skip(insert_pos + 1)
            .map(|(pos, item)| (item.id, pos))
            .collect();
        for (item_id, pos) in items_to_update {
            state.index_to_position.insert(item_id, pos);
        }
        state.index_to_position.insert(id, insert_pos);

        // Update parent-child relationships
        if let Some(parent_id) = parent_id {
            state
                .parent_to_children
                .entry(parent_id)
                .or_default()
                .push(id);
        }

        // Update prefixes for affected subtrees
        let sibling_indices: Vec<usize> = if let Some(parent_id) = parent_id {
            state
                .parent_to_children
                .get(&parent_id)
                .cloned()
                .unwrap_or_default()
        } else {
            vec![id]
        };

        for sibling_idx in &sibling_indices {
            state.update_prefixes_for_subtree(*sibling_idx);
        }

        // Update all prefixes and messages after adding the new item
        drop(state);
        self.update_all_prefixes_and_messages();

        id
    }

    fn update_all_prefixes_and_messages(&self) {
        // Update all prefixes first
        self.update_prefixes();

        // Then update all messages
        let state = self.state.lock().unwrap();

        // Update all messages with their stored base messages
        for item in &state.items {
            if let Some(base_msg) = state.base_messages.get(&item.id) {
                let prefix = state
                    .tree_prefixes
                    .get(&item.id)
                    .cloned()
                    .unwrap_or_default();
                let full_message = if prefix.is_empty() {
                    base_msg.clone()
                } else {
                    format!("{}{}", prefix, base_msg)
                };
                item.progress_bar.set_message(full_message);
            }
        }
    }

    /// Updates the message of a progress bar to include its tree prefix (one-time update).
    ///
    /// This is a convenience method for one-time message updates that doesn't store the message.
    /// For automatic updates when the tree changes, use `set_message()` instead.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the progress bar
    /// * `progress_bar` - The progress bar to update
    /// * `message` - The message to display (prefix will be prepended)
    ///
    /// # See Also
    ///
    /// * `set_message()` - For automatic message updates when tree structure changes
    pub fn set_message_with_prefix(&self, id: usize, progress_bar: &ProgressBar, message: &str) {
        let prefix = self.get_prefix(id).unwrap_or_default();
        if prefix.is_empty() {
            progress_bar.set_message(message.to_string());
        } else {
            progress_bar.set_message(format!("{}{}", prefix, message));
        }
    }

    /// Returns a closure that can be used with `ProgressStyle::with_key("tree", ...)`.
    ///
    /// This closure will look up the tree prefix for the progress bar being rendered
    /// and write it to the template output.
    ///
    /// # Note
    ///
    /// Due to limitations in indicatif's API, this handler uses a thread-local to track
    /// which bar is being rendered. For best results, use `WrappedProgressBar` methods
    /// or call `set_message_with_prefix` to manually set messages with prefixes.
    pub fn get_tree_key_handler(
        &self,
    ) -> impl Fn(&ProgressState, &mut dyn std::fmt::Write) + Send + Sync + Clone + 'static {
        let state = self.state.clone();
        move |_progress_state: &ProgressState, w: &mut dyn std::fmt::Write| {
            CURRENT_BAR_ID.with(|cell| {
                if let Some(id) = cell.get() {
                    let state = state.lock().unwrap();
                    if let Some(prefix) = state.tree_prefixes.get(&id) {
                        let _ = write!(w, "{}", prefix);
                    }
                }
            })
        }
    }

    /// Wraps a progress bar to enable the `{tree}` template key.
    ///
    /// This creates a wrapper that sets the thread-local bar ID before rendering,
    /// allowing the template key handler to identify which bar is being rendered.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the progress bar (returned from `add_progress_bar`)
    /// * `progress_bar` - The progress bar to wrap
    ///
    /// # Returns
    ///
    /// A wrapped progress bar that can be used with the `{tree}` template key.
    pub fn wrap_progress_bar(&self, id: usize, progress_bar: ProgressBar) -> WrappedProgressBar {
        WrappedProgressBar {
            id,
            progress_bar,
            state: self.state.clone(),
        }
    }

    /// Sets the base message for a progress bar and automatically formats it with the tree prefix.
    ///
    /// This method stores the base message and automatically updates the progress bar's message
    /// with the current tree prefix. The message will be automatically updated when the tree
    /// structure changes.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the progress bar
    /// * `base_message` - The base message to display (tree prefix will be prepended automatically)
    pub fn set_message(&self, id: usize, base_message: impl Into<String>) {
        let base_msg = base_message.into();
        let mut state = self.state.lock().unwrap();
        state.base_messages.insert(id, base_msg.clone());

        let prefix = state.tree_prefixes.get(&id).cloned().unwrap_or_default();
        let full_message = if prefix.is_empty() {
            base_msg
        } else {
            format!("{}{}", prefix, base_msg)
        };

        if let Some(item) = state.items.iter().find(|item| item.id == id) {
            item.progress_bar.set_message(full_message);
        }
    }

    fn update_prefixes(&self) {
        let mut state = self.state.lock().unwrap();
        // Update all prefixes
        let items_to_update: Vec<usize> = state
            .items
            .iter()
            .filter(|item| item.parent.is_some())
            .map(|item| item.id)
            .collect();
        for id in items_to_update {
            let prefix = state.compute_prefix(id);
            state.tree_prefixes.insert(id, prefix);
        }
    }

    /// Gets the tree prefix for a specific progress bar ID.
    ///
    /// This can be used to manually set the prefix in the progress bar's message
    /// if the `{tree}` template key approach doesn't work for your use case.
    pub fn get_prefix(&self, id: usize) -> Option<String> {
        let state = self.state.lock().unwrap();
        state.tree_prefixes.get(&id).cloned()
    }
}

/// A wrapped progress bar that enables the `{tree}` template key.
///
/// This wrapper sets the thread-local bar ID before any rendering operations,
/// allowing the template key handler to identify which bar is being rendered.
pub struct WrappedProgressBar {
    id: usize,
    progress_bar: ProgressBar,
    #[allow(dead_code)]
    state: Arc<Mutex<TreeState>>,
}

impl WrappedProgressBar {
    /// Returns a reference to the underlying progress bar.
    pub fn inner(&self) -> &ProgressBar {
        &self.progress_bar
    }

    /// Increments the progress bar and sets the thread-local ID.
    pub fn inc(&self, delta: u64) {
        CURRENT_BAR_ID.set(Some(self.id));
        self.progress_bar.inc(delta);
    }

    /// Sets the position and sets the thread-local ID.
    pub fn set_position(&self, pos: u64) {
        CURRENT_BAR_ID.set(Some(self.id));
        self.progress_bar.set_position(pos);
    }

    /// Finishes the progress bar.
    pub fn finish(&self) {
        self.progress_bar.finish();
    }

    /// Finishes the progress bar with a message.
    pub fn finish_with_message(&self, msg: &str) {
        self.progress_bar.finish_with_message(msg.to_string());
    }

    /// Sets the message.
    pub fn set_message(&self, msg: impl Into<std::borrow::Cow<'static, str>>) {
        CURRENT_BAR_ID.set(Some(self.id));
        self.progress_bar.set_message(msg);
    }

    /// Sets the prefix.
    pub fn set_prefix(&self, prefix: impl Into<std::borrow::Cow<'static, str>>) {
        CURRENT_BAR_ID.set(Some(self.id));
        self.progress_bar.set_prefix(prefix);
    }

    /// Sets the style.
    pub fn set_style(&self, style: indicatif::ProgressStyle) {
        CURRENT_BAR_ID.set(Some(self.id));
        self.progress_bar.set_style(style);
    }

    /// Ticks the progress bar.
    pub fn tick(&self) {
        CURRENT_BAR_ID.set(Some(self.id));
        self.progress_bar.tick();
    }
}

impl std::ops::Deref for WrappedProgressBar {
    type Target = ProgressBar;

    fn deref(&self) -> &Self::Target {
        &self.progress_bar
    }
}

impl Default for TreeProgressManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TreeState {
    fn compute_prefix(&self, item_idx: usize) -> String {
        let item = &self.items[self.index_to_position[&item_idx]];
        let Some(_parent_idx) = item.parent else {
            return String::new();
        };

        // Build LevelPath by walking up the ancestor chain
        let level_path = LevelPath::from_parent_chain(
            item_idx,
            |idx| {
                if let Some(&pos) = self.index_to_position.get(&idx) {
                    self.items[pos].parent
                } else {
                    None
                }
            },
            |idx| {
                // Determine if this item is the last child of its parent
                if let Some(&pos) = self.index_to_position.get(&idx)
                    && let Some(parent_idx) = self.items[pos].parent
                    && let Some(children) = self.parent_to_children.get(&parent_idx)
                {
                    // Get all children that are already in the tree, sorted by position
                    let mut existing_children: Vec<(usize, usize)> = children
                        .iter()
                        .filter_map(|&child_idx| {
                            self.index_to_position
                                .get(&child_idx)
                                .map(|&pos| (child_idx, pos))
                        })
                        .collect();
                    existing_children.sort_by_key(|(_, pos)| *pos);
                    // Check if this item is the last child
                    if let Some((last_idx, _)) = existing_children.last() {
                        return *last_idx == idx;
                    }
                }
                false
            },
        );

        compute_prefix(&level_path, &self.style)
    }

    fn find_insert_position(&self, item: &ProgressBarItem) -> usize {
        let Some(parent_id) = item.parent else {
            return self.items.len();
        };

        let Some(&parent_pos) = self.index_to_position.get(&parent_id) else {
            return self.items.len();
        };

        let children = match self.parent_to_children.get(&parent_id) {
            Some(children) if !children.is_empty() => children,
            _ => return parent_pos + 1,
        };

        let last_child_id = children.last().unwrap();
        let &last_child_pos = self.index_to_position.get(last_child_id).unwrap();

        let mut last_descendant_pos = last_child_pos;
        let mut stack = vec![*last_child_id];

        while let Some(current_id) = stack.pop() {
            if let Some(descendants) = self.parent_to_children.get(&current_id) {
                for &desc_id in descendants {
                    if let Some(&desc_pos) = self.index_to_position.get(&desc_id) {
                        last_descendant_pos = last_descendant_pos.max(desc_pos);
                        stack.push(desc_id);
                    }
                }
            }
        }

        last_descendant_pos + 1
    }

    fn update_prefixes_for_subtree(&mut self, root_idx: usize) {
        let mut to_update = vec![root_idx];
        while let Some(idx) = to_update.pop() {
            if let Some(&pos) = self.index_to_position.get(&idx) {
                if self.items[pos].parent.is_some() {
                    let prefix = self.compute_prefix(idx);
                    self.tree_prefixes.insert(idx, prefix);
                }
                if let Some(children) = self.parent_to_children.get(&idx) {
                    to_update.extend(children);
                }
            }
        }
    }
}
