extern crate chrono;
extern crate clap;
extern crate git2;

use chrono::prelude::{DateTime, Utc};
use clap::{App, Arg};
use git2::{BranchType, Repository};
use regex::Regex;

fn range_position_string(value: &String, start_pos: usize, end_pos: usize) -> Option<String> {
    if start_pos > value.len() || end_pos > value.len() || start_pos >= end_pos {
        return None;
    }
    let mut position_string = String::with_capacity(value.len());
    for (i, _ch) in value.chars().enumerate() {
        if i == start_pos {
            position_string.push('└');
        } else if i == end_pos - 1 {
            position_string.push('┘');
        } else if (i > start_pos) && (i < end_pos - 1) {
            position_string.push('─');
        } else {
            position_string.push(' ');
        }
    }

    return Some(position_string);
}

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
        let repo = match Repository::discover(".") {
            Ok(repo) => repo,
            Err(_e) => panic!("Not a git repo"),
        };
        let head = match repo.head() {
            Ok(reference) => reference,
            Err(_e) => panic!("Unable to get ref to HEAD"),
        };
        if !head.is_branch() {
            panic!("Not in a git branch now")
        }
        let branch_name = head
            .shorthand()
            .unwrap_or_else(|| panic!("Unable to get name of branch"));
        if branch_name == "master" {
            panic!("You can't run this from master branch");
        }
        let head_commit = match head.peel_to_commit() {
            Ok(commit) => commit,
            Err(_e) => panic!("Unable to find commit at HEAD"),
        };
        let head_commit_tree = match head_commit.tree() {
            Ok(tree) => tree,
            Err(_e) => panic!("Unable to find tree for HEAD commit"),
        };

        let master_branch = match repo.find_branch("master", BranchType::Local) {
            Ok(branch) => branch,
            Err(_e) => panic!("Unable to find master"),
        };
        let master_branch_ref = master_branch.into_reference();
        let master_branch_tree = match master_branch_ref.peel_to_tree() {
            Ok(tree) => tree,
            Err(_e) => panic!("Unable to get master tree"),
        };

        let diff = match repo.diff_tree_to_tree(
            Some(&master_branch_tree),
            Some(&head_commit_tree),
            None,
        ) {
            Ok(diff) => diff,
            Err(_e) => panic!("Unable to get diff"),
        };
        let mut new_migration_files: Vec<String> = Vec::new();
        for delta in diff.deltas() {
            let path = delta.new_file().path().unwrap();
            if delta.old_file().id().is_zero()
                && !delta.new_file().id().is_zero()
                && path.starts_with("db/migrate/")
            {
                new_migration_files.push(String::from(path.to_str().unwrap()));
            }
        }

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
                    let position_string =
                        range_position_string(path_string, matches.start(), matches.end()).unwrap();
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
