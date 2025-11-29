//! Example demonstrating dependency resolution with hierarchical tree visualization.
//!
//! This example simulates a dependency resolution process (similar to Maven/Gradle)
//! where dependencies are resolved in a tree structure. It demonstrates:
//! - Hierarchical progress bars using `IncrementalTree`
//! - Dynamic tree updates as dependencies are added
//! - Progress tracking for multiple concurrent downloads
//! - Real-time UI updates with formatted messages

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use console::style;
use indicatif::{HumanDuration, MultiProgress, MultiProgressAlignment, ProgressBar, ProgressStyle};
use once_cell::sync::Lazy;
use rand::rngs::ThreadRng;
use rand::{Rng, RngCore};
use treelog::build::IncrementalTree;

/// Configuration constants for the simulation.
struct Config;

impl Config {
    /// Percentage of terminal width to use for the main progress message.
    const TERMINAL_WIDTH_PERCENTAGE: f64 = 0.25;

    /// Minimum chunk size for progress increments (bytes).
    const MIN_CHUNK_SIZE: u64 = 1024;

    /// Maximum chunk size for progress increments (bytes).
    const MAX_CHUNK_SIZE: u64 = 102_400;

    /// Sleep duration between simulation steps (milliseconds).
    const SLEEP_DURATION_MS: u64 = 10;

    /// Threshold for random action selection (higher = more increments, less tree modifications).
    const ACTION_RANDOM_THRESHOLD: u64 = 16;

    /// Default terminal width if detection fails.
    const DEFAULT_TERMINAL_WIDTH: usize = 80;

    /// Width reserved for status labels (e.g., "Resolving", "Resolved").
    const LABEL_WIDTH: usize = 11;
}

/// Actions that can be taken during the simulation.
#[derive(Debug, Clone)]
enum Action {
    /// Add a new dependency to the tree (contains dependency index).
    ModifyTree(usize),
    /// Increment progress for a dependency (contains manager ID).
    IncProgressBar(usize),
    /// Stop the simulation (all dependencies resolved).
    Stop,
}

/// Status label for dependency resolution state.
#[derive(Clone, Debug)]
pub enum Label {
    /// Dependency is currently being resolved/downloaded.
    Resolving,
    /// Dependency has been fully resolved/downloaded.
    Resolved,
}

impl Label {
    fn format(&self) -> String {
        let s = match self {
            Label::Resolving => style("Resolving").blue().bold(),
            Label::Resolved => style("Resolved").green().bold(),
        };
        format!("{:>width$}", s, width = Config::LABEL_WIDTH)
    }

    fn completed(&self) -> Label {
        Label::Resolved
    }
}

/// Represents a single dependency in the resolution tree.
#[derive(Clone, Debug)]
struct Dependency {
    /// Progress bar tracking download/resolution progress.
    progress_bar: ProgressBar,
    /// Current status label.
    label: Label,
    /// Artifact identifier (e.g., "commons-lang3").
    artifact_id: String,
    /// Version string (e.g., "3.12.0").
    version: String,
    /// Index of parent dependency, or None for root dependencies.
    parent: Option<usize>,
}

/// Tracks the next dependency index to be added to the tree.
static NEXT_DEPENDENCY_INDEX: AtomicUsize = AtomicUsize::new(0);

/// Helper function to create a dependency with default label.
fn dependency(
    size_bytes: u64,
    artifact_id: &str,
    version: &str,
    parent: Option<usize>,
) -> Dependency {
    Dependency {
        progress_bar: ProgressBar::new(size_bytes),
        label: Label::Resolving,
        artifact_id: artifact_id.to_string(),
        version: version.to_string(),
        parent,
    }
}

/// Predefined dependency tree structure for the simulation.
/// Each tuple represents: (size_bytes, artifact_id, version, parent_index)
static DEPENDENCIES: Lazy<[Dependency; 24]> = Lazy::new(|| {
    [
        // Root dependencies (no parent)
        dependency(1_258_291, "commons-lang3", "3.12.0", None),
        dependency(2_936_012, "guava", "31.1-jre", Some(0)),
        dependency(419_430, "slf4j-api", "1.7.36", None),
        dependency(1_153_433, "logback-classic", "1.2.12", Some(2)),
        dependency(1_572_864, "jackson-databind", "2.15.2", Some(0)),
        dependency(1_887_436, "junit-jupiter", "5.9.3", Some(0)),
        dependency(4_404_019, "netty-all", "4.1.94.Final", None),
        dependency(943_718, "httpclient", "4.5.14", Some(6)),
        dependency(734_003, "okhttp", "4.11.0", None),
        dependency(524_288, "snakeyaml", "2.0", Some(8)),
        // Children of slf4j-api (index 2)
        dependency(314_572, "maven-model", "3.9.4", Some(2)),
        dependency(629_145, "aether-api", "1.1.0", Some(2)),
        dependency(943_718, "gson", "2.10.1", Some(2)),
        dependency(419_430, "commons-io", "2.11.0", Some(2)),
        // Other root dependencies
        dependency(1_363_148, "kotlin-stdlib", "1.9.0", None),
        dependency(1_782_579, "log4j-core", "2.20.0", Some(14)),
        dependency(209_715, "brigadier", "1.0.18", None),
        dependency(838_860, "configurate-yaml", "4.1.2", Some(6)),
        // Children of guava (index 1)
        dependency(314_572, "guava-testlib", "31.1-jre", Some(1)),
        dependency(104_857, "guava-annotations", "31.1-jre", Some(1)),
        // Children of jackson-databind (index 4)
        dependency(209_715, "jackson-annotations", "2.15.2", Some(4)),
        // Children of logback-classic (index 3)
        dependency(629_145, "logback-core", "1.2.12", Some(3)),
        // Children of httpclient (index 7)
        dependency(524_288, "httpcore", "4.4.16", Some(7)),
        dependency(314_572, "commons-codec", "1.15", Some(7)),
    ]
});

/// Tracks the current state of the dependency tree and progress bars.
struct TreeState {
    /// Maps dependency index to Dependency struct.
    dependencies: HashMap<usize, Dependency>,
    /// Maps dependency index to progress bar manager ID.
    dependency_index_to_manager_id: HashMap<usize, usize>,
    /// List of manager IDs for dependencies that are currently active (not completed).
    active_manager_ids: Vec<usize>,
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Gets the current terminal width, or returns a default if detection fails.
fn get_terminal_width() -> usize {
    term_size::dimensions()
        .map(|(w, _)| w)
        .unwrap_or(Config::DEFAULT_TERMINAL_WIDTH)
}

/// Creates the progress style for individual dependency items.
///
/// We don't use the {tree} template key because we manually format messages with prefixes.
/// The template key handler requires thread-local state which is unreliable here.
fn create_item_style() -> ProgressStyle {
    ProgressStyle::with_template("{wide_msg}").expect("Progress style template should be valid")
}

// ============================================================================
// UI Update Functions
// ============================================================================

/// Updates the main progress bar message with a list of currently active dependencies.
///
/// The message is truncated to fit within a percentage of the terminal width,
/// showing as many active items as possible with an ellipsis if truncated.
fn update_main_progress_message(
    main_progress_bar: &ProgressBar,
    tree_state: &TreeState,
    terminal_width: usize,
) {
    // Collect artifact IDs for all active dependencies
    let active_artifact_ids: Vec<&str> = tree_state
        .active_manager_ids
        .iter()
        .filter_map(|&manager_id| {
            // Find the dependency index that maps to this manager ID
            for (&dependency_index, &mapped_manager_id) in
                &tree_state.dependency_index_to_manager_id
            {
                if mapped_manager_id == manager_id
                    && let Some(dependency) = tree_state.dependencies.get(&dependency_index)
                {
                    return Some(dependency.artifact_id.as_str());
                }
            }
            None
        })
        .collect();

    if active_artifact_ids.is_empty() {
        main_progress_bar.set_message("");
        return;
    }

    const ELLIPSIS: &str = "...";
    let max_width = (terminal_width as f64 * Config::TERMINAL_WIDTH_PERCENTAGE) as usize;

    // Try to fit as many items as possible, starting from all items and working down
    for num_items in (1..=active_artifact_ids.len()).rev() {
        let message = if num_items < active_artifact_ids.len() {
            format!(
                "{}{}",
                active_artifact_ids[..num_items].join(", "),
                ELLIPSIS
            )
        } else {
            active_artifact_ids.join(", ")
        };

        if message.len() <= max_width {
            main_progress_bar.set_message(message);
            return;
        }
    }

    // If even ellipsis doesn't fit, show ellipsis or empty string
    main_progress_bar.set_message(if max_width >= ELLIPSIS.len() {
        ELLIPSIS.to_string()
    } else {
        String::new()
    });
}

// ============================================================================
// State Management Functions
// ============================================================================

/// Updates the list of active (incomplete) dependencies.
fn update_active_items(tree_state: &mut TreeState) {
    tree_state.active_manager_ids.clear();

    for (&dependency_index, dependency) in &tree_state.dependencies {
        if let Some(&manager_id) = tree_state
            .dependency_index_to_manager_id
            .get(&dependency_index)
        {
            let total_bytes = dependency.progress_bar.length().unwrap_or(0);
            let current_position = dependency.progress_bar.position();

            if current_position < total_bytes {
                tree_state.active_manager_ids.push(manager_id);
            }
        }
    }
}

// ============================================================================
// Action Selection and Handling
// ============================================================================

/// Determines the next action to take in the simulation.
///
/// Returns:
/// - `IncProgressBar` if we should increment progress on an active dependency
/// - `ModifyTree` if we should add a new dependency to the tree
/// - `Stop` if all dependencies are added and completed
fn get_action(rng: &mut dyn RngCore, tree_state: &TreeState) -> Action {
    let next_dependency_index = NEXT_DEPENDENCY_INDEX.load(Ordering::SeqCst);
    let active_manager_ids = &tree_state.active_manager_ids;

    // Decide whether to increment progress or modify tree
    // Prefer incrementing if there are active items, or if all dependencies are added
    let should_increment = rng.random_range(0..Config::ACTION_RANDOM_THRESHOLD) > 0
        || (next_dependency_index == DEPENDENCIES.len() && !active_manager_ids.is_empty());

    if should_increment && !active_manager_ids.is_empty() {
        // Randomly select an active dependency to increment
        let manager_id =
            active_manager_ids[rng.random_range(0..active_manager_ids.len() as u64) as usize];
        Action::IncProgressBar(manager_id)
    } else if next_dependency_index < DEPENDENCIES.len() {
        // Add the next dependency to the tree
        NEXT_DEPENDENCY_INDEX.fetch_add(1, Ordering::SeqCst);
        Action::ModifyTree(next_dependency_index)
    } else {
        // All dependencies added and (presumably) completed
        Action::Stop
    }
}

/// Increments progress for a specific dependency and updates the UI.
///
/// If the dependency completes, updates the main progress bar and refreshes active items.
fn handle_increment_progress(
    manager_id: usize,
    tree_state: &mut TreeState,
    tree: &Arc<Mutex<IncrementalTree>>,
    main_progress_bar: &ProgressBar,
    rng: &mut ThreadRng,
) {
    let dependency_index = tree_state.dependency_index_to_manager_id[&manager_id];
    let dependency = &tree_state.dependencies[&dependency_index];
    let progress_bar = &dependency.progress_bar;

    let total_bytes = progress_bar
        .length()
        .expect("Progress bar should have a length");
    let remaining_bytes = total_bytes.saturating_sub(progress_bar.position());

    // Increment by a random chunk size, but don't exceed remaining bytes
    let chunk_size = rng
        .random_range(Config::MIN_CHUNK_SIZE..=Config::MAX_CHUNK_SIZE)
        .min(remaining_bytes);

    progress_bar.inc(chunk_size);
    update_item_message(manager_id, dependency, tree, tree_state);

    // Check if this dependency is now complete
    if progress_bar.position() >= total_bytes {
        progress_bar.set_style(create_item_style());
        main_progress_bar.inc(1);
        update_active_items(tree_state);
    }
}

/// Updates the message displayed for a dependency's progress bar.
///
/// Formats the message with the current label (Resolving/Resolved), tree prefix,
/// artifact ID, and version.
fn update_item_message(
    manager_id: usize,
    dependency: &Dependency,
    tree: &Arc<Mutex<IncrementalTree>>,
    _tree_state: &TreeState,
) {
    let total_bytes = dependency.progress_bar.length().unwrap_or(0);
    let is_complete = total_bytes > 0 && dependency.progress_bar.position() >= total_bytes;

    let current_label = if is_complete {
        dependency.label.completed()
    } else {
        dependency.label.clone()
    };

    // Build the formatted prefix with tree structure if this is a child dependency
    let formatted_prefix = if dependency.parent.is_some() {
        tree.lock()
            .unwrap()
            .get_prefix(manager_id)
            .map(|tree_prefix| {
                format!(
                    "{}{} ",
                    style(&tree_prefix).black().bright(),
                    style(&dependency.artifact_id).white().bold()
                )
            })
            .unwrap_or_else(|| format!("{} ", style(&dependency.artifact_id).white().bold()))
    } else {
        format!("{} ", style(&dependency.artifact_id).white().bold())
    };

    let full_message = format!(
        "{} {}{}",
        current_label.format(),
        formatted_prefix,
        style(&dependency.version).dim()
    );

    dependency.progress_bar.set_message(full_message);
}

// ============================================================================
// Main Execution Functions
// ============================================================================

/// Initializes the progress display system.
///
/// Returns the initialized components needed for the simulation.
fn initialize_progress_display() -> (
    Arc<Mutex<IncrementalTree>>,
    MultiProgress,
    ProgressBar,
    TreeState,
    usize,
) {
    let tree = Arc::new(Mutex::new(IncrementalTree::new()));
    let multi_progress = MultiProgress::new();
    multi_progress.set_alignment(MultiProgressAlignment::Bottom);

    // Create main progress bar style
    let main_style = ProgressStyle::with_template(
        "\n{prefix:>11} [{wide_bar:.blue/black}] {pos}/{len}: {msg:.white} {elapsed:.dim} ",
    )
    .expect("Main progress style template should be valid")
    .progress_chars("━ ━");

    let total_items = DEPENDENCIES.len();
    let main_progress_bar = multi_progress.add(ProgressBar::new(total_items as u64));
    main_progress_bar.set_style(main_style);
    main_progress_bar.set_prefix(style("Building").blue().bold().to_string());

    // Initialize styles for all dependency progress bars
    for dependency in DEPENDENCIES.iter() {
        dependency.progress_bar.set_style(create_item_style());
    }

    let tree_state = TreeState {
        dependencies: HashMap::new(),
        dependency_index_to_manager_id: HashMap::new(),
        active_manager_ids: Vec::new(),
    };

    let terminal_width = get_terminal_width();

    main_progress_bar.tick();

    (
        tree,
        multi_progress,
        main_progress_bar,
        tree_state,
        terminal_width,
    )
}

/// Handles adding a new dependency to the tree.
fn handle_add_dependency(
    dependency_index: usize,
    tree: &Arc<Mutex<IncrementalTree>>,
    multi_progress: &MultiProgress,
    tree_state: &mut TreeState,
    main_progress_bar: &ProgressBar,
    terminal_width: usize,
) {
    if dependency_index >= DEPENDENCIES.len() {
        return;
    }

    let dependency = &DEPENDENCIES[dependency_index];

    // Find the parent's manager ID if this dependency has a parent
    let parent_manager_id = dependency.parent.and_then(|parent_dependency_index| {
        tree_state
            .dependencies
            .get(&parent_dependency_index)
            .and_then(|_| {
                tree_state
                    .dependency_index_to_manager_id
                    .get(&parent_dependency_index)
                    .copied()
            })
    });

    // Add the node to the tree
    let manager_id = {
        let mut tree_guard = tree.lock().unwrap();
        tree_guard.add_node("", parent_manager_id)
    };

    // Calculate insert position and insert into MultiProgress
    let insert_pos = {
        let tree_guard = tree.lock().unwrap();
        tree_guard.calculate_insert_position_for_existing(manager_id)
    };
    multi_progress.insert(insert_pos, dependency.progress_bar.clone());

    // Update state
    tree_state
        .dependencies
        .insert(dependency_index, dependency.clone());
    tree_state
        .dependency_index_to_manager_id
        .insert(dependency_index, manager_id);

    // We need to update messages with our custom formatting because
    // adding a new sibling can change which items are "last child" at various levels.
    for (&dep_idx, &mgr_id) in &tree_state.dependency_index_to_manager_id {
        if let Some(dep) = tree_state.dependencies.get(&dep_idx) {
            update_item_message(mgr_id, dep, tree, tree_state);
        }
    }

    update_active_items(tree_state);
    update_main_progress_message(main_progress_bar, tree_state, terminal_width);
}

/// Handles incrementing progress for a dependency.
fn handle_progress_increment(
    manager_id: usize,
    tree: &Arc<Mutex<IncrementalTree>>,
    tree_state: &mut TreeState,
    main_progress_bar: &ProgressBar,
    terminal_width: usize,
    rng: &mut ThreadRng,
) {
    // Verify this manager ID exists in our mapping
    if !tree_state
        .dependency_index_to_manager_id
        .values()
        .any(|&v| v == manager_id)
    {
        return;
    }

    handle_increment_progress(manager_id, tree_state, tree, main_progress_bar, rng);
    update_main_progress_message(main_progress_bar, tree_state, terminal_width);
}

/// Runs the main simulation loop until all dependencies are resolved.
fn run_simulation_loop(
    tree: Arc<Mutex<IncrementalTree>>,
    multi_progress: MultiProgress,
    main_progress_bar: ProgressBar,
    mut tree_state: TreeState,
    terminal_width: usize,
    start_time: Instant,
) {
    let mut rng = ThreadRng::default();

    loop {
        match get_action(&mut rng, &tree_state) {
            Action::Stop => {
                main_progress_bar.finish_and_clear();
                println!(
                    "\n\n{:>11} dev {} in {}",
                    style("Finished").green().bold(),
                    style("1.0.0").dim(),
                    style(HumanDuration(start_time.elapsed())).dim()
                );
                return;
            }
            Action::ModifyTree(dependency_index) => {
                handle_add_dependency(
                    dependency_index,
                    &tree,
                    &multi_progress,
                    &mut tree_state,
                    &main_progress_bar,
                    terminal_width,
                );
            }
            Action::IncProgressBar(manager_id) => {
                handle_progress_increment(
                    manager_id,
                    &tree,
                    &mut tree_state,
                    &main_progress_bar,
                    terminal_width,
                    &mut rng,
                );
            }
        }

        thread::sleep(Duration::from_millis(Config::SLEEP_DURATION_MS));
    }
}

pub fn main() {
    println!();
    let start_time = Instant::now();

    let (tree, multi_progress, main_progress_bar, tree_state, terminal_width) =
        initialize_progress_display();

    run_simulation_loop(
        tree,
        multi_progress,
        main_progress_bar,
        tree_state,
        terminal_width,
        start_time,
    );
}
