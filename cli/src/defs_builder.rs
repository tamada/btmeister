use std::fs::File;
use std::io::{BufRead, BufReader};

use btmeister::defs::{self, BuildToolDefs};
use btmeister::{Filter, Result};

use crate::cli;

pub fn construct(opts: cli::DefOpts) -> Result<BuildToolDefs> {
    let (d, a, f) = (opts.definition, opts.append_defs, opts.filter);
    match defs::construct(d, a) {
        Ok(r) => {
            let filter = f.build_filter();
            if filter == Filter::None {
                Ok(r)
            } else {
                Ok(r.filter(filter))
            }
        },
        Err(e) => Err(e),
    }
}

impl cli::FilterOpts {
    fn build_filter(self) -> Filter {
        match (self.includes, self.excludes, self.include_files, self.exclude_files) {
            (Some(v), _, _, _)  => Filter::Includes(parse_item(v)),
            (_, Some(v), _, _)  => Filter::Excludes(parse_item(v)),
            (_, _, Some(v), _)  => Filter::IncludeFiles(parse_item(v)),
            (_, _, _, Some(v))  => Filter::ExcludeFiles(parse_item(v)),
            (None, None, None, None) => Filter::None,
        }
    }
}

fn parse_item(item: String) -> Vec<String> {
    if let Some(filename) = item.strip_prefix('@') {
        match File::open(filename) {
            Ok(f) => read_file_content(f),
            Err(_) => vec![],
        }
    } else {
        item.split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>()
    }
}

fn read_file_content(f: File) -> Vec<String> {
    let r = BufReader::new(f).lines()
        .filter_map(|s| Some(s.unwrap()))
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();
    r
}