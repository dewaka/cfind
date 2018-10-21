use regex::Regex;
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

struct Matcher {
    exact_term: Option<String>,
    regex_term: Option<Regex>,
}

impl Matcher {
    fn matches(&self, path: &Path) -> bool {
        match path.file_name() {
            Some(fname) => {
                let sname = fname.to_str().unwrap();

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

    fn new(term: &str, is_regex: bool) -> Matcher {
        if is_regex {
            Matcher {
                regex_term: Some(Regex::new(term).unwrap()),
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

        let matcher = Matcher::new(needle, self.config.regex);

        loop {
            if self.find_in(&current_path, &matcher, &mut results) {
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

        results
    }

    fn find_in(&self, path: &Path, matcher: &Matcher, results: &mut Vec<String>) -> bool {
        let mut found = false;

        for entry in WalkDir::new(path).max_depth(1) {
            let current = entry.unwrap();
            let entry_path = current.path();

            if matcher.matches(&entry_path) {
                found = true;
                results.push(entry_path.to_str().unwrap().to_string());

                if !self.config.find_all {
                    break;
                }
            }
        }

        found
    }

    fn is_root(&self, path: &Path) -> bool {
        match path.parent() {
            None => true,
            Some(_) => false,
        }
    }
}
