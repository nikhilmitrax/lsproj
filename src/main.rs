use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use colored::*;
use std::collections::HashMap;
use skim::prelude::*;
use std::io::Cursor;

// Configuration constants
const RECURSION_LIMIT: i32 = 3; // Same as Python version
const GENERIC_PROJECT: (&str, &str) = ("generic", "g");

#[derive(Debug)]
struct ProjectRoot {
    path: PathBuf,
    project_type: (String, String),
}

fn get_project_root_markers() -> HashMap<&'static str, (&'static str, &'static str)> {
    let mut markers = HashMap::new();
    markers.insert("package.json", ("JS", "/"));
    markers.insert("Pipfile", ("Python", ""));
    markers.insert("meson.build", ("C++", ""));
    markers.insert("CMakeLists.txt", ("C++", ""));
    markers.insert("pyproject.toml", ("Python", ""));
    markers.insert("requirements.txt", ("Python", ""));
    markers.insert("Cargo.toml", ("Rust", ""));
    markers
}

/// Recursively search for project directories
/// This matches the Python iterate_folder function
fn iterate_folder(root: &Path, n: i32) -> Vec<ProjectRoot> {
    // Stop recursion if we've gone too deep
    if n > RECURSION_LIMIT {
        return vec![];
    }

    let mut projects = Vec::new();
    
    // Try to read the directory
    if let Ok(entries) = fs::read_dir(root) {
        // Collect all valid entries
        let items: Vec<_> = entries.filter_map(Result::ok).collect();
        
        // Check if it's a project root
        let mut is_git = false;
        let markers = get_project_root_markers();
        
        // First check if any project markers exist in this directory
        for entry in &items {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
            
            if let Some(&project_type) = markers.get(file_name_str.as_ref()) {
                // This is a project root with a specific marker
                return vec![ProjectRoot {
                    path: root.to_path_buf(),
                    project_type: (project_type.0.to_string(), project_type.1.to_string()),
                }];
            }
            
            if file_name_str == ".git" {
                is_git = true;
            }
        }
        
        // If no specific markers but has .git, it's a generic project
        if is_git {
            return vec![ProjectRoot {
                path: root.to_path_buf(),
                project_type: (GENERIC_PROJECT.0.to_string(), GENERIC_PROJECT.1.to_string()),
            }];
        }
        
        // No project markers found, so recursively check subdirectories
        for entry in items {
            let path = entry.path();
            if path.is_dir() {
                projects.extend(iterate_folder(&path, n + 1));
            }
        }
    }
    
    projects
}

fn main() {
    // Enable error output to file for debugging
    // let home = env::var("HOME").unwrap_or_else(|_| String::from("/tmp"));
    // let log_path = format!("{}/lsproj_error.log", home);
    // let mut log_file = fs::OpenOptions::new()
    //     .create(true)
    //     .append(true)
    //     .open(&log_path)
    //     .ok();
    
    // Log startup info
    // if let Some(ref mut file) = log_file {
    //     let _ = writeln!(file, "\n--- New session started ---");
    //     let _ = writeln!(file, "lsproj started with args: {:?}", env::args().collect::<Vec<_>>());
    // }

    // Get search directories from command line arguments (skip the program name)
    let args: Vec<String> = env::args().skip(1).collect();
    let mut project_roots = Vec::new();

    // If no arguments provided, show usage
    if args.is_empty() {
        eprintln!("Usage: lsproj <directory1> [<directory2> ...]");
        std::process::exit(1);
    }

    // Process each search directory
    for search_folder in &args {
        let search_path = PathBuf::from(search_folder);
        
        // Try to canonicalize, but don't fail if we can't
        let search_path = fs::canonicalize(&search_path).unwrap_or_else(|_| search_path.clone());
        
        // Log search path after canonicalization
        // if let Some(ref mut file) = log_file {
        //     let _ = writeln!(file, "Searching in: {}", search_path.display());
        // }
        
        project_roots.extend(iterate_folder(&search_path, 1));
    }

    // If no projects found, exit
    if project_roots.is_empty() {
        eprintln!("No projects found in the specified directories.");
        std::process::exit(1);
    }

    // Create a list of formatted project entries for display
    let mut entries = Vec::new();
    for root in &project_roots {
        let project_name = root.path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        // Format similar to the Python version
        let entry = format!(".\t{}\t{}\t{}", 
            root.project_type.1.blue(),
            project_name.green(),
            root.path.display());
        entries.push(entry);
    }

    // Join entries with newlines for skim
    let input = entries.join("\n");

    // Create skim options
    let options = SkimOptionsBuilder::default()
        .height("100%".to_string())  // Smaller height to avoid fullscreen that requires terminal control
        .multi(false)
        .preview(Some("eza -lh --color=always --icons=always {4}".to_string())) // Preview command showing path
        .ansi(true)
        .nth(vec!["3..4".into()])
        .color(Some("bg+:24".to_string()))  // Better highlighting
        .build()
        .unwrap();

    // Create the skim input
    let item_reader_options = SkimItemReaderOption::default().ansi(true);
    let item_reader = SkimItemReader::new(item_reader_options);
    let items = item_reader.of_bufread(Cursor::new(input));

    // Run skim
    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    // Process selection
    if !selected_items.is_empty() {
        let selected = selected_items[0].output();
        // Extract the path from the selected line (it's the last field after tabs)
        let path = selected.split('\t').last().unwrap_or("").trim();

        let path_buf = PathBuf::from(path);
        // Log the selected path for debugging
        // if let Some(ref mut file) = log_file {
        //     let _ = writeln!(file, "Selected path: {}", path);
            
        //     // Verify the path exists
        //     let _ = writeln!(file, "Path exists: {}", path_buf.exists());
        // }
        
        // Print the full path, depending on whether we can canonicalize it
        if let Ok(canonical) = path_buf.canonicalize() {
            println!("{}", canonical.to_string_lossy());
        } else {
            // Fall back to the original path if canonicalization fails
            println!("{}", path);
        }
    }
}