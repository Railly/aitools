use std::fs;
use std::path::{Path, PathBuf};
use tokio;

pub struct FileInfo {
    pub path: PathBuf,
    pub files: Vec<String>,
}

fn should_ignore(path: &Path, ignore_patterns: &[String]) -> bool {
    let path_str = path.to_string_lossy();
    ignore_patterns
        .iter()
        .any(|pattern| path_str.contains(pattern))
        || path_str.contains("node_modules")
        || path_str.contains("/.git/")
}

fn is_typescript_file(path: &Path) -> bool {
    path.extension()
        .map(|ext| ext == "ts" || ext == "tsx")
        .unwrap_or(false)
}

fn read_gitignore(root: &Path) -> Vec<String> {
    let gitignore_path = root.join(".gitignore");
    if gitignore_path.exists() {
        fs::read_to_string(gitignore_path)
            .map(|content| {
                content
                    .lines()
                    .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}

pub async fn get_file_structure(root: &str) -> Result<FileInfo, std::io::Error> {
    let path = PathBuf::from(root);
    let ignore_patterns = read_gitignore(&path);
    let mut files = Vec::new();

    for entry in walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let entry_path = entry.path();
        if !should_ignore(entry_path, &ignore_patterns) && is_typescript_file(entry_path) {
            if let Some(file_name) = entry_path.to_str() {
                files.push(file_name.to_string());
            }
        }
    }

    Ok(FileInfo { path, files })
}

pub async fn read_selected_files(
    files: &[String],
) -> Result<Vec<(String, String)>, std::io::Error> {
    let mut contents = Vec::new();
    for file in files {
        let content = tokio::fs::read_to_string(file).await?;
        contents.push((file.clone(), content));
    }
    Ok(contents)
}
