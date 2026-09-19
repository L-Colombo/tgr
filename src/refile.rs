use crate::{cli::RefileArgs, config::Userconfig, orgtree::Orgtree};
use grep::{
    regex::RegexMatcher,
    searcher::{Searcher, sinks::UTF8},
};
use std::{fs::File, process::exit};

/// This function returns the refiled files in form of a string.
/// The `write_refiled_file` in the `io` module will then take care of writing it to a file.
pub fn refile(args: &RefileArgs, mut cfg: Userconfig) -> String {
    let raw_pattern = match args.strict {
        false => format!(r":\w*{}\w*:", args.pattern),
        true => format!(r":{}:", args.pattern),
    };

    let files_to_search: Vec<String> =
        cfg.get_files_to_search(args.include.clone(), args.exclude.clone());

    let mut all_matches: Vec<(String, Vec<usize>)> = vec![];

    // PERF: consider to find a way to parallelize therse expensive io bound operations
    for file in files_to_search {
        let fname = match File::open(format!("{}{}", cfg.org_directory, &file)) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("{e}: Could not open file named {file}");
                exit(1)
            }
        };

        let matcher = match RegexMatcher::new(&raw_pattern) {
            Ok(matecher) => matecher,
            Err(_) => {
                eprintln!("Your regex pattern is possibly malformed");
                exit(1)
            }
        };

        let mut matches: Vec<usize> = vec![];

        let _ = Searcher::new().search_reader(
            &matcher,
            fname,
            UTF8(|linenr, _| {
                matches.push(linenr as usize);
                Ok(true)
            }),
        );

        if !matches.is_empty() {
            all_matches.push((file, matches));
        }
    }

    let mut org_trees: Vec<Orgtree> = vec![];

    for entry in all_matches {
        for line_nr in entry.1 {
            let mut ot = Orgtree {
                file_name: entry.0.clone(),
                line_nr,
                lines: vec![],
            };

            crate::orgtree::get_lines(&cfg, &mut ot);
            org_trees.push(ot);
        }
    }

    if org_trees.is_empty() {
        eprintln!("No matches found for pattern \"{}\"!", args.pattern);
        exit(1)
    }

    let mut str_buf = string_builder::Builder::default();
    let mut current_headline = org_trees[0].file_name.clone();

    str_buf.append("* ");
    str_buf.append(current_headline.clone());
    str_buf.append("\n");

    for org_tree in org_trees {
        if org_tree.file_name != current_headline {
            current_headline = org_tree.file_name.clone();
            str_buf.append("* ");
            str_buf.append(current_headline.clone());
            str_buf.append("\n");
        }

        for line in org_tree.lines {
            str_buf.append(line);
            str_buf.append("\n");
        }
    }

    match str_buf.string() {
        Ok(str_buf) => str_buf,
        Err(e) => {
            eprintln!("{e}: A problem occured getting the contents of your refile file");
            exit(1)
        }
    }
}
