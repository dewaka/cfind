use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct Config {
    pub find_all: bool,
    pub regex: bool,
}

#[derive(Debug, Clone)]
pub struct Searcher {
    config: Config,
}

fn parent_folder(path: &Path) -> Option<&Path> {
    path.parent()
}

fn is_root(path: &Path) -> bool {
    match parent_folder(path) {
        None => true,
        Some(_) => false,
    }
}

impl Searcher {
    // TODO: Better error handling
    // TODO: Regex based matching

    pub fn new(config: Config) -> Searcher {
        Searcher { config }
    }

    // What we need to do is search for the given needle starting from the start
    // path till we reach the root
    pub fn search(&self, start: &str, needle: &str) -> Vec<String> {
        let mut results = vec![];

        let start_canonical = fs::canonicalize(start).unwrap();
        let mut current_path = Path::new(&start_canonical);

        loop {
            if self.find_in(&current_path, needle, &mut results) {
                // We have found the needle, and we only have to continue if we
                // need to find all possible matches
                if !self.config.find_all {
                    break;
                }
            }

            // If we are already in the root directory, then we are done searching
            if is_root(&current_path) {
                break;
            }

            // Otherwise set the current_path to the one directory up and keep
            // searching
            match parent_folder(&current_path) {
                Some(path) => current_path = path,
                None => {
                    // TODO: Better error handling
                    println!(
                        "Error - couldn't move up folder from: {:?}",
                        current_path.to_str()
                    );
                    break;
                }
            }
        }

        results
    }

    fn matches(&self, path: &Path, needle: &str) -> bool {
        match path.file_name() {
            Some(fname) => {
                let sname = fname.to_str().unwrap();
                sname == needle
            }
            None => false,
        }
    }

    fn find_in(&self, path: &Path, needle: &str, results: &mut Vec<String>) -> bool {
        let mut found = false;

        for entry in WalkDir::new(path).max_depth(1) {
            let current = entry.unwrap();
            let entry_path = current.path();

            if self.matches(&entry_path, needle) {
                found = true;
                results.push(entry_path.to_str().unwrap().to_string());

                if !self.config.find_all {
                    break;
                }
            }
        }

        found
    }
}
