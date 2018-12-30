extern crate chrono;
extern crate clap;
extern crate git2;

mod git_utils;
mod string_utils;

use chrono::prelude::{DateTime, Utc};
use clap::{App, Arg};
use regex::Regex;

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

        let new_migration_files = match git_utils::find_new_files_at_path("db/migrate") {
            Ok(files) => files,
            Err(_e) => panic!("Unable to find migration files"),
        };

        let mut regexes: Vec<(String, &Regex, String)> = Vec::new();

        let yyyymmddhhmmss = Regex::new(
            r"(?x)
            (?P<y>(19|20)\d{2}) # yyyy
            (?P<m>(0\d)|(1[0-2])) # mm
            (?P<d>([0-2]\d)|(3[0-1])) # dd
            (?P<th>([0-1]\d)|(2[0-3])) # hh
            (?P<tm>[0-5]\d) # mm
            (?P<ts>[0-5]\d) # ss
            ",
        )
        .unwrap();
        regexes.push((
            String::from("yyyymmddhhmmss"),
            &yyyymmddhhmmss,
            String::from("%Y%m%d%H%M%S"),
        ));

        let ddmmyyyy = Regex::new(
            r"(?x)
            (?P<d>([0-2]\d)|(3[0-1])) # dd
            (?P<m>(0\d)|(1[0-2])) # mm
            (?P<y>(19|20)\d{2}) # yyyy
            ",
        )
        .unwrap();
        regexes.push((String::from("ddmmyyyy"), &ddmmyyyy, String::from("%d%m%Y")));

        let mmddyyyy = Regex::new(
            r"(?x)
            (?P<m>(0\d)|(1[0-2])) # mm
            (?P<d>([0-2]\d)|(3[0-1])) # dd
            (?P<y>(19|20)\d{2}) # yyyy
            ",
        )
        .unwrap();
        regexes.push((String::from("mmddyyyy"), &mmddyyyy, String::from("%m%d%Y")));

        let yyyymmdd = Regex::new(
            r"(?x)
            (?P<y>(19|20)\d{2}) # yyyy
            (?P<m>(0\d)|(1[0-2])) # mm
            (?P<d>([0-2]\d)|(3[0-1])) # dd
            ",
        )
        .unwrap();
        regexes.push((String::from("yyyymmdd"), &yyyymmdd, String::from("%Y%m%d")));

        for path_string in new_migration_files.iter() {
            for (re_name, re, replacement_pattern) in regexes.iter() {
                if re.is_match(path_string) {
                    let matches = re.find(path_string).unwrap();
                    let prefix = format!("Found {} at: ", re_name);
                    println!("{}{}", prefix, path_string);

                    let blank_prefix = std::iter::repeat(" ")
                        .take(prefix.len())
                        .collect::<String>();
                    let position_string = string_utils::range_position_string(
                        path_string,
                        matches.start(),
                        matches.end(),
                    )
                    .unwrap();
                    println!("{}{}", blank_prefix, position_string);
                    let now: DateTime<Utc> = Utc::now();
                    let mut after = String::from(path_string.as_str());
                    after.replace_range(
                        matches.start()..matches.end(),
                        now.format(replacement_pattern).to_string().as_str(),
                    );
                    println!("{}", after);
                    break;
                }
            }
        }
    } else {
        panic!("pending");
    }
}
