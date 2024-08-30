use crate::file_system::FileInfo;
use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use tempfile::NamedTempFile;

/// Formats the file structure into a grouped, hierarchical string
fn format_grouped_file_structure(info: &FileInfo, selected_files: &[String]) -> String {
    let mut grouped = BTreeMap::new();
    for file in selected_files {
        let relative_path = Path::new(file)
            .strip_prefix(&info.path)
            .unwrap_or(Path::new(file));
        add_to_group(&mut grouped, relative_path);
    }
    format_group(&grouped, 0)
}

/// Recursively adds a path to the grouped structure
fn add_to_group(group: &mut BTreeMap<String, BTreeMap<String, ()>>, path: &Path) {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            add_to_group(group, parent);
        }
    }
    let component = path.file_name().unwrap().to_string_lossy().into_owned();
    group.entry(component).or_insert_with(BTreeMap::new);
}

/// Formats the grouped structure into a string with proper indentation
fn format_group(group: &BTreeMap<String, BTreeMap<String, ()>>, depth: usize) -> String {
    let mut output = String::new();
    for (key, value) in group {
        let indent = "  ".repeat(depth);
        if value.is_empty() {
            // This is a file
            output.push_str(&format!("{}{}\n", indent, key));
        } else {
            // This is a directory
            output.push_str(&format!("{}{}:\n", indent, key));
            output.push_str(&format_group_inner(value, depth + 1));
        }
    }
    output
}

/// Formats the innermost level of the grouped structure (files)
fn format_group_inner(group: &BTreeMap<String, ()>, depth: usize) -> String {
    let mut output = String::new();
    for key in group.keys() {
        let indent = "  ".repeat(depth);
        // Add each file with proper indentation
        output.push_str(&format!("{}{}\n", indent, key));
    }
    output
}

pub fn format_output_for_neovim(
    file_structure: &FileInfo,
    selected_files: &[String],
    signatures: &[(String, Vec<String>)],
    contents: &[(String, String)],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut output = String::new();

    // File Structure
    output.push_str("# File Structure\n");
    output.push_str(&format_grouped_file_structure(
        file_structure,
        selected_files,
    ));
    output.push_str("\n");

    // Files, Signatures, and Contents
    for (file_path, file_contents) in contents {
        let relative_path = Path::new(file_path)
            .strip_prefix(&file_structure.path)
            .unwrap_or(Path::new(file_path))
            .to_string_lossy();

        output.push_str(&format!("# {}\n", relative_path));

        // Signatures
        if let Some((_, sigs)) = signatures.iter().find(|(path, _)| path == file_path) {
            output.push_str("## Signatures\n");
            for sig in sigs {
                output.push_str(&format!("- {}\n", sig));
            }
            output.push_str("\n");
        }

        // File contents
        output.push_str("## File contents\n");
        output.push_str("```\n");
        output.push_str(file_contents);
        output.push_str("\n```\n\n");
    }

    // Create a temporary file
    let mut temp_file = NamedTempFile::new()?;
    {
        let file = temp_file.as_file_mut();
        file.write_all(output.as_bytes())?;
    }

    // Open the temporary file in Neovim
    Command::new("nvim").arg(temp_file.path()).status()?;

    Ok(())
}
