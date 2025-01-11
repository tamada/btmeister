use clap::{Parser, ValueEnum};
use std::io::{self, BufRead};
use std::path::PathBuf;

use btmeister::{IgnoreType, MeisterError, Result};

#[derive(Parser, Debug)]
#[clap(author, version, about, arg_required_else_help = true)]
pub(crate) struct Options {
    #[clap(flatten)]
    pub(crate) defopts: DefOpts,

    #[clap(flatten)]
    pub(crate) inputs: InputOpts,

    #[clap(flatten)]
    pub(crate) outputs: OutputOpts,

    #[arg(short, long, help = "Show verbose output.")]
    pub(crate) verbose: bool,

    #[clap(flatten)]
    pub(crate) compopts: CompletionOpts,
}

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
        help = "specify the ignore type."
    )]
    pub(crate) ignore_types: Vec<IgnoreType>,

    #[arg(
        value_name = "PROJECTs",
        required = false,
        help = "The target project paths. If \"-\" was given, reads from stdin, and \"@\" was put on the first character, read from the file."
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
    Xml,
    Yaml,
}

fn read_from_reader(r: Box<dyn BufRead>) -> Result<Vec<String>> {
    let mut result = vec![];
    for line in r.lines() {
        if let Ok(l) = line {
            if l.starts_with("#") || l.trim().is_empty() {
                continue;
            }
            result.push(l);
        }
    }
    Ok(result)
}

fn read_from_stdin() -> Result<Vec<String>> {
    read_from_reader(Box::new(io::stdin().lock()))
}

fn read_from_file(filename: &str) -> Result<Vec<String>> {
    match std::fs::File::open(filename) {
        Err(e) => Err(MeisterError::Io(e)),
        Ok(file) => read_from_reader(Box::new(std::io::BufReader::new(file))),
    }
}

fn convert_and_push_item(item: &str, result: &mut Vec<PathBuf>, errs: &mut Vec<MeisterError>) {
    let path = PathBuf::from(item);
    if !path.exists() || !path.is_dir() {
        errs.push(MeisterError::ProjectNotFound(item.to_string()));
    } else {
        result.push(path);
    }
}

impl InputOpts {
    pub(crate) fn projects(&self) -> Result<Vec<PathBuf>> {
        let mut errs = vec![];
        let mut result = vec![];
        for item in self.dirs.clone() {
            if item == "-" {
                match read_from_stdin() {
                    Err(e) => errs.push(e),
                    Ok(results) => {
                        for item in results {
                            convert_and_push_item(&item, &mut result, &mut errs)
                        }
                    }
                }
            } else if item.starts_with("@") {
                match read_from_file(&item[1..]) {
                    Err(e) => errs.push(e),
                    Ok(results) => {
                        for item in results {
                            convert_and_push_item(&item, &mut result, &mut errs)
                        }
                    }
                }
            } else {
                convert_and_push_item(item.as_str(), &mut result, &mut errs);
            }
        }
        if errs.len() != 0 {
            Err(MeisterError::Array(errs))
        } else if result.len() == 0 {
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
        let opts = Options::parse_from(&["meister", "testdata/fibonacci", "testdata/hello"]);
        let projects = opts.inputs.projects();
        assert!(projects.is_ok());
        if let Ok(p) = projects {
            assert_eq!(2, p.len());
            assert_eq!(PathBuf::from("testdata/fibonacci"), p[0]);
            assert_eq!(PathBuf::from("testdata/hello"), p[1]);
        }
    }

    #[test]
    fn test_projects2() {
        let opts = Options::parse_from(&["meister", "@testdata/project_list.txt"]);
        let projects = opts.inputs.projects();
        assert!(projects.is_ok());
        if let Ok(p) = projects {
            assert_eq!(2, p.len());
            assert_eq!(PathBuf::from("testdata/hello"), p[0]);
            assert_eq!(PathBuf::from("testdata/fibonacci"), p[1]);
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
        let opts = Options::parse_from(&["meister", "@testdata/invalid_project_list.txt"]);
        let projects = opts.inputs.projects();
        assert!(projects.is_err());
        if let Err(MeisterError::Array(e)) = projects {
            assert_eq!(2, e.len());
            if let MeisterError::ProjectNotFound(p) = &e[0] {
                assert_eq!("not_exist_project", p);
            }
            if let MeisterError::ProjectNotFound(p) = &e[1] {
                assert_eq!("testdata/project_list.txt", p);
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
            if let MeisterError::Io(p) = &e[0] {
                assert_eq!(std::io::ErrorKind::NotFound, p.kind());
            }
        }
    }
}
