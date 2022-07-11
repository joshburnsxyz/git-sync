use walkdir::{WalkDir,DirEntry};
use std::process::Command;
use std::path::Path;

// Detect if given directory is a git repo or not
pub fn is_repo(tg_entry: &Path) -> bool {
    let mut status: bool = false;
    for entry in WalkDir::new(tg_entry).into_iter().filter_map(|e| e.ok()) {
        println!("{:#?}",entry.path().to_str());
        if entry.path().to_str() == Some(".git") {
            status = true; 
        }
    }
    return status;
}

// Find git repos in cwd
pub fn find_repos() -> Vec<DirEntry> {
    let mut paths_vec = Vec::new();
    for entry in WalkDir::new(".").min_depth(1).max_depth(3) {
        if is_repo(entry.as_ref().unwrap().path()) == true {
            paths_vec.push(entry.unwrap());
        }
    }
    return paths_vec;
}

// Run git pull from current repo
pub fn pull() -> std::process::Output {
    let _cmd = Command::new("git")
        .args(["pull"])
        .output()
        .expect("Could not perform git pull");
        return _cmd;
}

// Run git push from current repo
pub fn push() -> std::process::Output {
    let _cmd = Command::new("git")
        .args(["push"])
        .output()
        .expect("Could not perform git push");
        return _cmd;
}
