use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select};
use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub path: PathBuf,
}

pub fn parse_args() -> Args {
    Args::parse()
}

struct FileNode {
    name: String,
    path: PathBuf,
    is_dir: bool,
    children: BTreeMap<String, FileNode>,
}

impl FileNode {
    fn new(path: PathBuf) -> Self {
        let is_dir = path.is_dir();
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        FileNode {
            name,
            path,
            is_dir,
            children: BTreeMap::new(),
        }
    }

    fn add_child(&mut self, child: FileNode) {
        self.children.insert(child.name.clone(), child);
    }
}

fn build_file_tree(root: &Path) -> FileNode {
    let mut root_node = FileNode::new(root.to_path_buf());
    if root_node.is_dir {
        for entry in std::fs::read_dir(root).expect("Failed to read directory") {
            if let Ok(entry) = entry {
                let path = entry.path();
                let file_name = path.file_name().unwrap().to_string_lossy();

                if path.is_dir() && (file_name.starts_with('.') || file_name == "node_modules") {
                    continue;
                }

                if path.is_dir()
                    || path
                        .extension()
                        .map_or(false, |ext| ext == "ts" || ext == "tsx")
                {
                    let child = build_file_tree(&path);
                    root_node.add_child(child);
                }
            }
        }
    }
    root_node
}

pub fn interactive_file_selection(
    root_path: &Path,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file_tree = build_file_tree(root_path);
    let mut current_node = &file_tree;
    let mut selected_files = HashSet::new();
    let theme = ColorfulTheme::default();
    let mut last_index = 0;

    loop {
        let mut items: Vec<(String, bool)> = current_node
            .children
            .iter()
            .map(|(name, node)| (name.clone(), node.is_dir))
            .collect();

        items.sort_by(|a, b| match (a.1, b.1) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.0.cmp(&b.0),
        });

        let items: Vec<String> = items
            .iter()
            .enumerate()
            .map(|(index, (name, is_dir))| {
                let prefix = if index == items.len() - 1 {
                    "└──"
                } else {
                    "├──"
                };
                if *is_dir {
                    format!("{} ▸ {}", prefix, name)
                } else {
                    let marker = if selected_files.contains(
                        &current_node.children[name]
                            .path
                            .to_string_lossy()
                            .into_owned(),
                    ) {
                        "✓"
                    } else {
                        "◦"
                    };
                    format!("{} {} {}", prefix, marker, name)
                }
            })
            .collect();

        let mut display_items = items.clone();
        if current_node.path != root_path {
            display_items.insert(0, "┌── ▴ ..".to_string());
        }
        display_items.push("─── ◉ Finish selection".to_string());

        let selection = Select::with_theme(&theme)
            .default(last_index.min(display_items.len() - 1))
            .items(&display_items)
            .interact()?;

        last_index = selection;

        if selection == display_items.len() - 1 {
            break;
        } else if selection == 0 && current_node.path != root_path {
            /* Go up one level */
            let parent_path = current_node.path.parent().unwrap();
            current_node = &file_tree;
            for component in parent_path
                .strip_prefix(root_path)
                .unwrap_or(parent_path)
                .components()
            {
                if let std::path::Component::Normal(name) = component {
                    current_node = &current_node.children[&name.to_string_lossy().into_owned()];
                }
            }
            last_index = 0; // Reset index when going up
        } else {
            let selected_index = if current_node.path != root_path {
                selection - 1
            } else {
                selection
            };
            let selected_item = &items[selected_index];
            let parts: Vec<&str> = selected_item.split_whitespace().collect();
            let selected_name = parts[parts.len() - 1];

            if let Some(selected_node) = current_node.children.get(selected_name) {
                if selected_node.is_dir {
                    current_node = selected_node;
                    last_index = 0; // Reset index when entering a directory
                } else {
                    let file_path = selected_node.path.to_string_lossy().into_owned();
                    if selected_files.contains(&file_path) {
                        selected_files.remove(&file_path);
                    } else {
                        selected_files.insert(file_path);
                    }
                    // Don't change last_index to keep the cursor position
                }
            } else {
                println!("Error: Selected item not found in current directory");
            }
        }
    }

    if selected_files.is_empty() {
        println!("No files selected. Please select at least one file.");
        return interactive_file_selection(root_path);
    }

    Ok(selected_files.into_iter().collect())
}
