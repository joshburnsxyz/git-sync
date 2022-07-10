use walkdir::{WalkDir,DirEntry};
use std::process::Command;

// Detect if given directory is a git repo or not
pub fn is_repo(tg_entry: &DirEntry) -> bool {
    let mut status: bool = false;
    for entry in WalkDir::new(tg_entry).into_iter().filter_map(|e| e.ok()) {
        if entry == ".git" {
            status = true; 
        } else {
            status = false;
        }
        return status;
    }
}

// Find git repos in cwd
pub fn find_repos() -> &Vec<DirEntry> {
    let mut paths_vec = Vec::new();
    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        let is_git: bool = is_repo(entry);
        if is_git == true {
            paths_vec.push(entry.into_path());
        }
        return &paths_vec
    }
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
