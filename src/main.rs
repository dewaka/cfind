extern crate clap;
extern crate regex;
extern crate walkdir;

mod cfind;

use cfind::Searcher;
use clap::{App, Arg};

fn main() {
    let matches = App::new("cfind: close file find")
        .version("0.1")
        .author("Chathura C. <dcdewaka@gmail.com>")
        .about("Find a file or directory by walking up parent directories")
        .arg(
            Arg::with_name("search")
                .index(1)
                .required(true)
                .takes_value(true)
                .multiple(false)
                .help("Search term"),
        ).arg(
            Arg::with_name("path")
                .index(2)
                .required(false)
                .takes_value(true)
                .multiple(true)
                .help("Path(s) to start search from (default is current directory)"),
        ).arg(
            Arg::with_name("exact")
                .short("e")
                .required(false)
                .takes_value(false)
                .multiple(false)
                .help("Specify exact term search"),
        ).arg(
            Arg::with_name("all")
                .short("a")
                .required(false)
                .takes_value(false)
                .multiple(false)
                .help("Specify whether to find all occurrences"),
        ).get_matches();

    let search = matches.value_of("search").unwrap();

    let dirs: Vec<&str> = matches.values_of("path").unwrap_or_default().collect();

    // If we don't have directory arguments given, then by default search only
    // in the current directory
    let search_dirs = if dirs.is_empty() { vec!["."] } else { dirs };

    let is_exact = matches.occurrences_of("exact") > 0;
    let is_all = matches.occurrences_of("all") > 0;

    let searcher = Searcher::build().regex(!is_exact).find_all(is_all);

    let results = searcher.search(&search_dirs, search);

    if results.is_empty() {
        std::process::exit(1);
    }

    for res in results {
        println!("{}", res);
    }
}
