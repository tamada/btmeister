use clap::{Parser, ValueEnum};
use std::io::{self, BufRead};
use std::path::PathBuf;

use btmeister::{IgnoreType, LogLevel, MeisterError, Result};

#[derive(Parser, Debug)]
#[clap(author, version, about, arg_required_else_help = true)]
pub(crate) struct Options {
    #[clap(flatten)]
    pub(crate) defopts: DefOpts,

    #[clap(flatten)]
    pub(crate) inputs: InputOpts,

    #[clap(flatten)]
    pub(crate) outputs: OutputOpts,

    #[arg(
        short,
        long,
        value_name = "LEVEL",
        default_value = "warn",
        help = "Specify the log level.",
        ignore_case = true
    )]
    pub(crate) level: LogLevel,

    #[cfg(debug_assertions)]
    #[clap(flatten)]
    pub(crate) compopts: CompletionOpts,
}

#[cfg(debug_assertions)]
#[derive(Parser, Debug)]
pub(crate) struct CompletionOpts {
    #[arg(
        long = "generate-completion-files",
        help = "Generate completion files",
        hide = true
    )]
    pub(crate) completion: bool,

    #[arg(
        long = "completion-out-dir",
        value_name = "DIR",
        default_value = "assets/completions",
        help = "Output directory of completion files",
        hide = true
    )]
    pub(crate) dest: PathBuf,
}

#[derive(Parser, Debug, Clone)]
pub(crate) struct InputOpts {
    #[arg(
        short = 'i',
        long = "ignore-type",
        default_value = "default",
        ignore_case = true,
        value_enum,
        value_name = "IGNORE_TYPE",
        help = "Specify the ignore type."
    )]
    pub(crate) ignore_types: Vec<IgnoreType>,

    #[arg(
        short,
        long,
        value_name = "EXCLUDEs",
        help = "Specify the filters of excluding files or directories."
    )]
    pub(crate) excludes: Vec<String>,

    #[arg(
        value_name = "PROJECTs",
        required = false,
        help = "The target project paths. If \"-\" was given, reads from stdin.
Also, the first character was \"@\", read from the file eliminating \"@\".
This parameters accept directories and archive files.
Supported archive files: tar, tar.bz2, tar.gz, tar.xz, tar.zstd, and zip."
    )]
    pub dirs: Vec<String>,
}

#[derive(Parser, Debug, Clone)]
pub(crate) struct OutputOpts {
    #[arg(
        short = 'L',
        long = "list-defs",
        help = "Print the build tools' definition list"
    )]
    pub(crate) list_defs: bool,

    #[arg(
        short,
        long,
        default_value_t = Format::Default,
        value_name = "FORMAT",
        value_enum,
        ignore_case = true,
        help = "Specify the output format"
    )]
    pub(crate) format: Format,
}

#[derive(Parser, Debug, Clone)]
pub(crate) struct DefOpts {
    #[arg(
        short = 'D',
        long,
        value_name = "DEFS_JSON",
        help = "Specify the definition of the build tools."
    )]
    pub(crate) definition: Option<PathBuf>,

    #[arg(
        long,
        value_name = "DEFS_JSON",
        help = "Specify the additional definitions of the build tools."
    )]
    pub(crate) append_defs: Option<PathBuf>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Format {
    Csv,
    Default,
    Json,
    Markdown,
    Xml,
    Yaml,
}

fn read_from_reader(r: Box<dyn BufRead>, parent: PathBuf) -> Result<Vec<String>> {
    let result = r
        .lines()
        .map_while(|r| r.ok())
        .map(|l| l.trim().to_string())
        .filter(|l| !l.starts_with("#") && !l.is_empty())
        .map(|name| parent.join(name).to_str().unwrap().to_string())
        .collect::<Vec<String>>();
    Ok(result)
}

fn read_from_stdin() -> Result<Vec<String>> {
    read_from_reader(Box::new(io::stdin().lock()), PathBuf::from("."))
}

fn read_from_file(filename: &str) -> Result<Vec<String>> {
    let parent = match PathBuf::from(filename).parent() {
        Some(p) => p.to_path_buf(),
        None => PathBuf::from("."),
    };
    match std::fs::File::open(filename) {
        Err(e) => Err(MeisterError::IO(e)),
        Ok(file) => read_from_reader(Box::new(std::io::BufReader::new(file)), parent),
    }
}

fn convert_and_push_item(item: &str, result: &mut Vec<PathBuf>, errs: &mut Vec<MeisterError>) {
    let path = PathBuf::from(item);
    if !path.exists() {
        errs.push(MeisterError::ProjectNotFound(item.to_string()));
    } else if path.is_file() {
        if btmeister::is_supported_archive_format(&path) {
            result.push(path);
        } else {
            errs.push(MeisterError::ProjectNotFound(item.to_string()));
        }
    } else if path.is_dir() {
        result.push(path);
    } else {
        errs.push(MeisterError::ProjectNotFound(item.to_string()));
    }
}

fn push_items_or_errs(
    r: Result<Vec<String>>,
    results: &mut Vec<PathBuf>,
    errs: &mut Vec<MeisterError>,
) {
    match r {
        Err(e) => errs.push(e),
        Ok(items) => {
            for item in items {
                convert_and_push_item(&item, results, errs)
            }
        }
    }
}

impl InputOpts {
    pub(crate) fn projects(&self) -> Result<Vec<PathBuf>> {
        let mut errs = vec![];
        let mut result = vec![];
        for item in self.dirs.iter() {
            if item == "-" {
                push_items_or_errs(read_from_stdin(), &mut result, &mut errs);
            } else if let Some(stripped) = item.strip_prefix('@') {
                push_items_or_errs(read_from_file(stripped), &mut result, &mut errs);
            } else {
                convert_and_push_item(item.as_str(), &mut result, &mut errs);
            }
        }
        if !errs.is_empty() {
            Err(MeisterError::Array(errs))
        } else if result.is_empty() {
            Err(MeisterError::NoProjectSpecified())
        } else {
            Ok(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_projects1() {
        let opts = Options::parse_from(&["meister", "../testdata/fibonacci", "../testdata/hello"]);
        let projects = opts.inputs.projects();
        assert!(projects.is_ok());
        if let Ok(p) = projects {
            assert_eq!(2, p.len());
            assert_eq!(PathBuf::from("../testdata/fibonacci"), p[0]);
            assert_eq!(PathBuf::from("../testdata/hello"), p[1]);
        }
    }

    #[test]
    fn test_projects2() {
        let opts = Options::parse_from(&["meister", "@../testdata/project_list.txt"]);
        let projects = opts.inputs.projects();
        if let Ok(p) = projects {
            assert_eq!(2, p.len());
            assert_eq!(PathBuf::from("../testdata/hello"), p[0]);
            assert_eq!(PathBuf::from("../testdata/fibonacci"), p[1]);
        } else {
            panic!("fatal: {:?}", projects);
        }
    }

    #[test]
    fn test_not_exist_project() {
        let opts = Options::parse_from(&["meister", "not_exist_project"]);
        let projects = opts.inputs.projects();
        assert!(projects.is_err());
        if let Err(MeisterError::Array(e)) = projects {
            assert_eq!(1, e.len());
            if let MeisterError::ProjectNotFound(p) = &e[0] {
                assert_eq!("not_exist_project", p);
            }
        }
    }

    #[test]
    fn test_invalid_project_list() {
        let opts = Options::parse_from(&["meister", "@../testdata/invalid_project_list.txt"]);
        let projects = opts.inputs.projects();
        assert!(projects.is_err());
        if let Err(MeisterError::Array(e)) = projects {
            assert_eq!(2, e.len());
            if let MeisterError::ProjectNotFound(p) = &e[0] {
                assert_eq!("../testdata/not_exist_project", p);
            }
            if let MeisterError::ProjectNotFound(p) = &e[1] {
                assert_eq!("../testdata/project_list.txt", p);
            }
        }
    }

    #[test]
    fn test_unknownfile() {
        let opts = Options::parse_from(&["meister", "@unknownfile"]);
        let projects = opts.inputs.projects();
        assert!(projects.is_err());
        if let Err(MeisterError::Array(e)) = projects {
            assert_eq!(1, e.len());
            if let MeisterError::IO(p) = &e[0] {
                assert_eq!(std::io::ErrorKind::NotFound, p.kind());
            }
        }
    }

    #[test]
    fn test_no_projects() {
        let opts = InputOpts {
            ignore_types: vec![],
            excludes: vec![],
            dirs: vec![],
        };
        let projects = opts.projects();
        assert!(projects.is_err());
        match projects {
            Err(MeisterError::NoProjectSpecified()) => assert!(true),
            _ => assert!(false),
        }
    }
}
