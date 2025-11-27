//! Example demonstrating dependency resolution with hierarchical tree visualization.

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use console::style;
use indicatif::{HumanDuration, MultiProgress, MultiProgressAlignment, ProgressBar, ProgressStyle};
use once_cell::sync::Lazy;
use rand::rngs::ThreadRng;
use rand::{Rng, RngCore};

const TERMINAL_WIDTH_PERCENTAGE: f64 = 0.25;
const MIN_CHUNK_SIZE: u64 = 1024;
const MAX_CHUNK_SIZE: u64 = 102_400;
const SLEEP_DURATION_MS: u64 = 3;
const ACTION_RANDOM_THRESHOLD: u64 = 16;
const DEFAULT_TERMINAL_WIDTH: usize = 80;
const TREE_VERTICAL: &str = "│  ";
const TREE_BRANCH: &str = "├─ ";
const TREE_LAST: &str = "└─ ";
const LABEL_WIDTH: usize = 11;

#[derive(Debug, Clone)]
enum Action {
    ModifyTree(usize),
    IncProgressBar(usize),
    Stop,
}

#[derive(Clone, Debug)]
pub enum Label {
    Resolving,
    Resolved,
}

impl Label {
    fn format(&self) -> String {
        let s = match self {
            Label::Resolving => style("Resolving").blue().bold(),
            Label::Resolved => style("Resolved").green().bold(),
        };
        format!("{:>width$}", s, width = LABEL_WIDTH)
    }

    fn completed(&self) -> Label {
        Label::Resolved
    }
}

#[derive(Clone, Debug)]
struct Dependency {
    index: usize,
    progress_bar: ProgressBar,
    label: Label,
    artifact_id: String,
    version: String,
    parent: Option<usize>,
}

static ELEM_IDX: AtomicUsize = AtomicUsize::new(0);

static DEPENDENCIES: Lazy<[Dependency; 24]> = Lazy::new(|| {
    [
        Dependency {
            index: 0,
            progress_bar: ProgressBar::new(1_258_291),
            label: Label::Resolving,
            artifact_id: "commons-lang3".to_string(),
            version: "3.12.0".to_string(),
            parent: None,
        },
        Dependency {
            index: 1,
            progress_bar: ProgressBar::new(2_936_012),
            label: Label::Resolving,
            artifact_id: "guava".to_string(),
            version: "31.1-jre".to_string(),
            parent: Some(0),
        },
        Dependency {
            index: 2,
            progress_bar: ProgressBar::new(419_430),
            label: Label::Resolving,
            artifact_id: "slf4j-api".to_string(),
            version: "1.7.36".to_string(),
            parent: None,
        },
        Dependency {
            index: 3,
            progress_bar: ProgressBar::new(1_153_433),
            label: Label::Resolving,
            artifact_id: "logback-classic".to_string(),
            version: "1.2.12".to_string(),
            parent: Some(2),
        },
        Dependency {
            index: 4,
            progress_bar: ProgressBar::new(1_572_864),
            label: Label::Resolving,
            artifact_id: "jackson-databind".to_string(),
            version: "2.15.2".to_string(),
            parent: Some(0),
        },
        Dependency {
            index: 5,
            progress_bar: ProgressBar::new(1_887_436),
            label: Label::Resolving,
            artifact_id: "junit-jupiter".to_string(),
            version: "5.9.3".to_string(),
            parent: Some(0),
        },
        Dependency {
            index: 6,
            progress_bar: ProgressBar::new(4_404_019),
            label: Label::Resolving,
            artifact_id: "netty-all".to_string(),
            version: "4.1.94.Final".to_string(),
            parent: None,
        },
        Dependency {
            index: 7,
            progress_bar: ProgressBar::new(943_718),
            label: Label::Resolving,
            artifact_id: "httpclient".to_string(),
            version: "4.5.14".to_string(),
            parent: Some(6),
        },
        Dependency {
            index: 8,
            progress_bar: ProgressBar::new(734_003),
            label: Label::Resolving,
            artifact_id: "okhttp".to_string(),
            version: "4.11.0".to_string(),
            parent: None,
        },
        Dependency {
            index: 9,
            progress_bar: ProgressBar::new(524_288),
            label: Label::Resolving,
            artifact_id: "snakeyaml".to_string(),
            version: "2.0".to_string(),
            parent: Some(8),
        },
        Dependency {
            index: 10,
            progress_bar: ProgressBar::new(314_572),
            label: Label::Resolving,
            artifact_id: "maven-model".to_string(),
            version: "3.9.4".to_string(),
            parent: Some(2),
        },
        Dependency {
            index: 11,
            progress_bar: ProgressBar::new(629_145),
            label: Label::Resolving,
            artifact_id: "aether-api".to_string(),
            version: "1.1.0".to_string(),
            parent: Some(2),
        },
        Dependency {
            index: 12,
            progress_bar: ProgressBar::new(943_718),
            label: Label::Resolving,
            artifact_id: "gson".to_string(),
            version: "2.10.1".to_string(),
            parent: Some(2),
        },
        Dependency {
            index: 13,
            progress_bar: ProgressBar::new(419_430),
            label: Label::Resolving,
            artifact_id: "commons-io".to_string(),
            version: "2.11.0".to_string(),
            parent: Some(2),
        },
        Dependency {
            index: 14,
            progress_bar: ProgressBar::new(1_363_148),
            label: Label::Resolving,
            artifact_id: "kotlin-stdlib".to_string(),
            version: "1.9.0".to_string(),
            parent: None,
        },
        Dependency {
            index: 15,
            progress_bar: ProgressBar::new(1_782_579),
            label: Label::Resolving,
            artifact_id: "log4j-core".to_string(),
            version: "2.20.0".to_string(),
            parent: Some(14),
        },
        Dependency {
            index: 16,
            progress_bar: ProgressBar::new(209_715),
            label: Label::Resolving,
            artifact_id: "brigadier".to_string(),
            version: "1.0.18".to_string(),
            parent: None,
        },
        Dependency {
            index: 17,
            progress_bar: ProgressBar::new(838_860),
            label: Label::Resolving,
            artifact_id: "configurate-yaml".to_string(),
            version: "4.1.2".to_string(),
            parent: Some(6),
        },
        Dependency {
            index: 18,
            progress_bar: ProgressBar::new(314_572),
            label: Label::Resolving,
            artifact_id: "guava-testlib".to_string(),
            version: "31.1-jre".to_string(),
            parent: Some(1),
        },
        Dependency {
            index: 19,
            progress_bar: ProgressBar::new(104_857),
            label: Label::Resolving,
            artifact_id: "guava-annotations".to_string(),
            version: "31.1-jre".to_string(),
            parent: Some(1),
        },
        Dependency {
            index: 20,
            progress_bar: ProgressBar::new(209_715),
            label: Label::Resolving,
            artifact_id: "jackson-annotations".to_string(),
            version: "2.15.2".to_string(),
            parent: Some(4),
        },
        Dependency {
            index: 21,
            progress_bar: ProgressBar::new(629_145),
            label: Label::Resolving,
            artifact_id: "logback-core".to_string(),
            version: "1.2.12".to_string(),
            parent: Some(3),
        },
        Dependency {
            index: 22,
            progress_bar: ProgressBar::new(524_288),
            label: Label::Resolving,
            artifact_id: "httpcore".to_string(),
            version: "4.4.16".to_string(),
            parent: Some(7),
        },
        Dependency {
            index: 23,
            progress_bar: ProgressBar::new(314_572),
            label: Label::Resolving,
            artifact_id: "commons-codec".to_string(),
            version: "1.15".to_string(),
            parent: Some(7),
        },
    ]
});

struct TreeState<'a> {
    items: Vec<&'a Dependency>,
    index_to_position: HashMap<usize, usize>,
    parent_to_children: HashMap<usize, Vec<usize>>,
    tree_prefixes: HashMap<usize, String>,
    active_indices: Vec<usize>,
}

impl<'a> TreeState<'a> {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            index_to_position: HashMap::new(),
            parent_to_children: HashMap::new(),
            tree_prefixes: HashMap::new(),
            active_indices: Vec::new(),
        }
    }

    fn compute_prefix(&self, item_idx: usize) -> String {
        let item = self.items[self.index_to_position[&item_idx]];
        let Some(parent_idx) = item.parent else {
            return String::new();
        };

        let mut prefix = String::new();
        let mut current = Some(parent_idx);
        let mut ancestors = Vec::new();

        while let Some(parent_idx) = current {
            if let Some(&parent_pos) = self.index_to_position.get(&parent_idx) {
                ancestors.push((parent_idx, parent_pos));
                current = self.items[parent_pos].parent;
            } else {
                break;
            }
        }

        ancestors.reverse();
        for (_ancestor_idx, ancestor_pos) in ancestors {
            if let Some(ancestor_parent) = self.items[ancestor_pos].parent
                && let Some(siblings) = self.parent_to_children.get(&ancestor_parent)
                && {
                    let ancestor_idx = self.items[ancestor_pos].index;
                    siblings.iter().any(|&sibling_idx| {
                        sibling_idx != ancestor_idx
                            && self
                                .index_to_position
                                .get(&sibling_idx)
                                .is_some_and(|&sibling_pos| sibling_pos > ancestor_pos)
                    })
                }
            {
                prefix.push_str(TREE_VERTICAL);
            }
        }

        if let Some(children) = self.parent_to_children.get(&parent_idx) {
            let mut existing_children: Vec<(usize, usize)> = children
                .iter()
                .filter_map(|&idx| self.index_to_position.get(&idx).map(|&pos| (idx, pos)))
                .collect();
            existing_children.sort_by_key(|(_, pos)| *pos);
            let is_last = existing_children.last().map(|(idx, _)| *idx) == Some(item_idx);
            prefix.push_str(if is_last { TREE_LAST } else { TREE_BRANCH });
        }

        prefix
    }

    fn find_insert_position(&self, item: &Dependency) -> usize {
        let Some(parent_idx) = item.parent else {
            return self.items.len();
        };

        let Some(&parent_pos) = self.index_to_position.get(&parent_idx) else {
            return self.items.len();
        };

        let children = match self.parent_to_children.get(&parent_idx) {
            Some(children) if !children.is_empty() => children,
            _ => return parent_pos + 1,
        };

        let last_child_idx = children.last().unwrap();
        let &last_child_pos = self.index_to_position.get(last_child_idx).unwrap();

        let mut last_descendant_pos = last_child_pos;
        let mut stack = vec![*last_child_idx];

        while let Some(current_idx) = stack.pop() {
            if let Some(descendants) = self.parent_to_children.get(&current_idx) {
                for &desc_idx in descendants {
                    if let Some(&desc_pos) = self.index_to_position.get(&desc_idx) {
                        last_descendant_pos = last_descendant_pos.max(desc_pos);
                        stack.push(desc_idx);
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
                    self.tree_prefixes.insert(idx, self.compute_prefix(idx));
                }
                if let Some(children) = self.parent_to_children.get(&idx) {
                    to_update.extend(children);
                }
            }
        }
    }

    fn add_item(&mut self, item: &'a Dependency, multi_progress: &MultiProgress) -> ProgressBar {
        let insert_pos = self.find_insert_position(item);
        let pb = multi_progress.insert(insert_pos, item.progress_bar.clone());

        self.items.insert(insert_pos, item);

        for (pos, item) in self.items.iter().enumerate().skip(insert_pos + 1) {
            self.index_to_position.insert(item.index, pos);
        }
        self.index_to_position.insert(item.index, insert_pos);

        if let Some(parent_idx) = item.parent {
            self.parent_to_children
                .entry(parent_idx)
                .or_default()
                .push(item.index);
        }

        let sibling_indices: Vec<usize> = if let Some(parent_idx) = item.parent {
            self.parent_to_children
                .get(&parent_idx)
                .cloned()
                .unwrap_or_default()
        } else {
            vec![item.index]
        };

        for sibling_idx in &sibling_indices {
            self.update_prefixes_for_subtree(*sibling_idx);
        }

        for sibling_idx in &sibling_indices {
            if let Some(&sibling_pos) = self.index_to_position.get(sibling_idx) {
                self.update_item_message(*sibling_idx, &self.items[sibling_pos].progress_bar);
            }
        }

        self.update_active_items();
        pb
    }

    fn update_item_message(&mut self, item_idx: usize, pb: &ProgressBar) {
        let &pos = self.index_to_position.get(&item_idx).unwrap();
        let item = self.items[pos];

        let len = item.progress_bar.length().unwrap_or(0);
        let is_complete = len > 0 && item.progress_bar.position() >= len;
        let current_label = if is_complete {
            item.label.completed()
        } else {
            item.label.clone()
        };

        let prefix = if item.parent.is_some() {
            let prefix = self.compute_prefix(item_idx);
            self.tree_prefixes.insert(item_idx, prefix.clone());
            format!(
                "{}{} ",
                style(&prefix).black().bright(),
                style(&item.artifact_id).white().bold()
            )
        } else {
            format!("{} ", style(&item.artifact_id).white().bold())
        };

        let tree_message = format!("{}{}", prefix, style(&item.version).dim());
        pb.set_message(format!("{} {}", current_label.format(), tree_message));
    }

    fn update_active_items(&mut self) {
        self.active_indices.clear();
        for (pos, item) in self.items.iter().enumerate() {
            if item.progress_bar.position() < item.progress_bar.length().unwrap_or(0) {
                self.active_indices.push(pos);
            }
        }
    }

    fn get_active_item_positions(&self) -> &[usize] {
        &self.active_indices
    }
}

fn get_terminal_width() -> usize {
    term_size::dimensions()
        .map(|(w, _)| w)
        .unwrap_or(DEFAULT_TERMINAL_WIDTH)
}

fn create_item_style() -> ProgressStyle {
    ProgressStyle::with_template("{wide_msg}").expect("Progress style template should be valid")
}

fn update_main_progress_message(
    pb_main: &ProgressBar,
    tree_state: &TreeState,
    terminal_width: usize,
) {
    let active_items: Vec<&str> = tree_state
        .get_active_item_positions()
        .iter()
        .map(|&pos| tree_state.items[pos].artifact_id.as_str())
        .collect();

    if active_items.is_empty() {
        pb_main.set_message("");
        return;
    }

    const ELLIPSIS: &str = "...";
    let max_width = (terminal_width as f64 * TERMINAL_WIDTH_PERCENTAGE) as usize;

    for num_items in (1..=active_items.len()).rev() {
        let message = if num_items < active_items.len() {
            format!("{}{}", active_items[..num_items].join(", "), ELLIPSIS)
        } else {
            active_items.join(", ")
        };
        if message.len() <= max_width {
            pb_main.set_message(message);
            return;
        }
    }

    pb_main.set_message(if max_width >= ELLIPSIS.len() {
        ELLIPSIS.to_string()
    } else {
        String::new()
    });
}

fn get_action(rng: &mut dyn RngCore, tree_state: &TreeState) -> Action {
    let elem_idx = ELEM_IDX.load(Ordering::SeqCst);
    let active_positions = tree_state.get_active_item_positions();
    let should_increment = rng.random_range(0..ACTION_RANDOM_THRESHOLD) > 0
        || (elem_idx == DEPENDENCIES.len() && !active_positions.is_empty());

    if should_increment && !active_positions.is_empty() {
        let pos = active_positions[rng.random_range(0..active_positions.len() as u64) as usize];
        Action::IncProgressBar(pos)
    } else if elem_idx < DEPENDENCIES.len() {
        ELEM_IDX.fetch_add(1, Ordering::SeqCst);
        Action::ModifyTree(elem_idx)
    } else {
        Action::Stop
    }
}

fn handle_increment_progress(
    pos: usize,
    tree_state: &mut TreeState,
    main_progress: &ProgressBar,
    rng: &mut ThreadRng,
) {
    let item = tree_state.items[pos];
    let len = item
        .progress_bar
        .length()
        .expect("Progress bar should have a length");
    let remaining = len.saturating_sub(item.progress_bar.position());
    let chunk_size = rng
        .random_range(MIN_CHUNK_SIZE..=MAX_CHUNK_SIZE)
        .min(remaining);

    item.progress_bar.inc(chunk_size);
    tree_state.update_item_message(item.index, &item.progress_bar);

    if item.progress_bar.position() >= len {
        item.progress_bar.set_style(create_item_style());
        main_progress.inc(1);
        tree_state.update_active_items();
    }
}

pub fn main() {
    println!();
    let start = Instant::now();
    let mp = Arc::new(MultiProgress::new());
    mp.set_alignment(MultiProgressAlignment::Bottom);
    let sty_main = ProgressStyle::with_template(
        "\n{prefix:>11} [{wide_bar:.blue/black}] {pos}/{len}: {msg:.white} {elapsed:.dim} ",
    )
    .expect("Main progress style template should be valid")
    .progress_chars("━ ━");
    let total_items = DEPENDENCIES.len();
    let pb_main = mp.add(ProgressBar::new(total_items as u64));
    pb_main.set_style(sty_main);
    pb_main.set_prefix(style("Building").blue().bold().to_string());
    for item in DEPENDENCIES.iter() {
        item.progress_bar.set_style(create_item_style());
    }
    let mut tree_state = TreeState::new();
    let terminal_width = get_terminal_width();
    let mut rng = ThreadRng::default();
    pb_main.tick();
    loop {
        match get_action(&mut rng, &tree_state) {
            Action::Stop => {
                pb_main.finish_and_clear();
                println!(
                    "\n\n{:>11} dev {} in {}",
                    style("Finished").green().bold(),
                    style("1.0.0").dim(),
                    style(HumanDuration(start.elapsed())).dim()
                );
                return;
            }
            Action::ModifyTree(elem_idx) => {
                if elem_idx >= DEPENDENCIES.len() {
                    continue;
                }
                let item = &DEPENDENCIES[elem_idx];
                tree_state.add_item(item, &mp);
                update_main_progress_message(&pb_main, &tree_state, terminal_width);
            }
            Action::IncProgressBar(pos) => {
                if pos >= tree_state.items.len() {
                    continue;
                }
                handle_increment_progress(pos, &mut tree_state, &pb_main, &mut rng);
                update_main_progress_message(&pb_main, &tree_state, terminal_width);
            }
        }
        thread::sleep(Duration::from_millis(SLEEP_DURATION_MS));
    }
}
