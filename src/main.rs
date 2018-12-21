extern crate clap;
extern crate git2;

use clap::{App, Arg};
use git2::Repository;

fn main() {
    let matches = App::new("Re-timestamp")
                    .version("1.0")
                    .author("sparkymat")
                    .about("Updates timestamps in file names")
                    .arg(Arg::with_name("rails-migration")
                         .short("R")
                         .long("rails-migration")
                         .help("Updates migration timestamps in db/migrate files which are not present in <master> branch"))
                    .get_matches();
    if matches.is_present("rails-migration") {
        println!("**Rails mode**");
        let repo = match Repository::open(".") {
            Ok(repo) => repo,
            Err(e) => panic!("Not a git repo"),
        };
    } else {
        panic!("pending");
    }
}
