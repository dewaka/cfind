use regex::Regex;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

struct Config {
    find_all: bool,
    regex: bool,
}

pub struct Searcher {
    config: Config,
}

struct Matcher {
    exact_term: Option<String>,
    regex_term: Option<Regex>,
}

impl Matcher {
    fn matches(&self, path: &Path) -> bool {
        match path.file_name() {
            Some(fname) => {
                let sname = fname
                    .to_str()
                    .expect("Could not convert path's file name to string!");

                match self.regex_term {
                    Some(ref re) => re.is_match(sname),
                    None => match self.exact_term {
                        Some(ref term) => sname == term,
                        None => false,
                    },
                }
            }
            None => false,
        }
    }

    fn new(term: &str, is_regex: bool) -> Self {
        if is_regex {
            Matcher {
                regex_term: Some(Regex::new(term).expect(&format!("Invalid regex: {}", term))),
                exact_term: None,
            }
        } else {
            Matcher {
                exact_term: Some(term.to_string()),
                regex_term: None,
            }
        }
    }
}

impl Searcher {
    // TODO: Better error handling

    pub fn build() -> Self {
        Searcher {
            config: Config {
                find_all: false,
                regex: true,
            },
        }
    }

    pub fn regex(mut self, reg: bool) -> Self {
        self.config.regex = reg;
        self
    }

    pub fn find_all(mut self, all: bool) -> Self {
        self.config.find_all = all;
        self
    }

    fn search_in_dir(&self, dir: &str, needle: &str, results: &mut Vec<String>) {
        let dir_canonical =
            fs::canonicalize(dir).expect(&format!("Path canonicalization failed for: {}", dir));
        let mut current_path = Path::new(&dir_canonical);

        let matcher = Matcher::new(needle, self.config.regex);

        loop {
            if self.find_in(&current_path, &matcher, results) {
                // We have found the needle, and we only have to continue if we
                // need to find all possible matches
                if !self.config.find_all {
                    break;
                }
            }

            // If we are already in the root directory, then we are done searching
            if self.is_root(&current_path) {
                break;
            }

            // Otherwise set the current_path to the one directory up and keep
            // searching
            match current_path.parent() {
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
    }

    // What we need to do is search for the given needle starting from the start
    // path till we reach the root
    pub fn search(&self, dirs: &[&str], needle: &str) -> Vec<String> {
        let mut results = vec![];

        for dir in dirs {
            self.search_in_dir(dir, needle, &mut results);
        }

        results
    }

    fn find_in(&self, path: &Path, matcher: &Matcher, results: &mut Vec<String>) -> bool {
        let mut found = false;

        for entry in WalkDir::new(path).max_depth(1) {
            let current = entry.expect(&format!("WalkDir entry failed for path: {:?}", path));
            let entry_path = current.path();

            if matcher.matches(&entry_path) {
                found = true;
                results.push(
                    entry_path
                        .to_str()
                        .expect(&format!(
                            "Path to string conversion failed for: {:?}",
                            entry_path
                        )).to_string(),
                );

                if !self.config.find_all {
                    break;
                }
            }
        }

        found
    }

    fn is_root(&self, path: &Path) -> bool {
        path.parent().is_none()
    }
}
