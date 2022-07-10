use walkdir::{WalkDir,DirEntry};
use std::process::Command;
use std::path::Path;

// Detect if given directory is a git repo or not
pub fn is_repo(tg_entry: &DirEntry) -> bool {
    let mut status: bool = false;
    for entry in WalkDir::new(tg_entry).into_iter().filter_map(|e| e.ok()) {
        if entry.path() == ".git" {
            status = true; 
        }
    }
    return status;
}

// Find git repos in cwd
pub fn find_repos() -> &'static Vec<DirEntry> {
    let mut paths_vec = Vec::new();
    for entry in WalkDir::new(".").min_depth(1).max_depth(3) {
        let is_git: bool = is_repo(entry);
        if is_git == true {
            paths_vec.push(entry);
        }
    }
    return &paths_vec;
}

// Run git pull from current repo
pub fn pull() {
    let _cmd = Command::new("git")
        .args(["pull"])
        .output()
        .expect("Could not perform git pull");
}

// Run git push from current repo
pub fn push() {
    let _cmd = Command::new("git")
        .args(["push"])
        .output()
        .expect("Could not perform git push");
}
