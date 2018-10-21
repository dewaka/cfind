extern crate walkdir;

mod cfind;

use cfind::{Config, Searcher};

// TODO: Command line option parsing

fn main() {
    let config = Config {
        find_all: true,
        regex: false,
    };

    let searcher = Searcher::new(config);
    let res = searcher.search(".", "README.md");

    println!("Found results: {:?}", res);
}
