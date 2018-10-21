extern crate clap;
extern crate regex;
extern crate walkdir;

mod cfind;

use cfind::{Config, Searcher};
use clap::{App, Arg};

fn main() {
    let matches = App::new("rename: bulk rename")
        .version("0.1")
        .author("Chathura C. <dcdewaka@gmail.com>")
        .about("Find a file or directory by walking up parent directories")
        .arg(
            Arg::with_name("directory")
                .short("d")
                .required(false)
                .takes_value(true)
                .multiple(false)
                .help("Directory to start search from. Default is the current directory."),
        ).arg(
            Arg::with_name("search")
                .index(1)
                .required(true)
                .takes_value(true)
                .help("Specify the directory to rename files in"),
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

    let dir = matches.value_of("directory").unwrap_or(".");
    let search = matches.value_of("search").unwrap();
    let is_exact = matches.occurrences_of("exact") > 0;
    let is_all = matches.occurrences_of("all") > 0;

    let config = Config {
        find_all: is_all,
        regex: !is_exact,
    };

    let searcher = Searcher::new(config);
    let results = searcher.search(dir, search);

    if results.is_empty() {
        std::process::exit(1);
    }

    for res in results {
        println!("{}", res);
    }
}
