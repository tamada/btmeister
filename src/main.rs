use clap::{ArgEnum, Parser};
use std::error::Error;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;
use btmeister::{BuildToolDef,BuildToolDefs,construct};
use ignore::WalkBuilder;

mod btmeister;
mod formatter;

#[derive(Parser)]
#[clap(author, version, about)]
struct Options {
    #[clap(
        long,
        value_name = "JSON",
        help = "Specify the additional definitions of the build tools."
    )]
    append_defs: Option<PathBuf>,

    #[clap(
        short,
        long,
        value_name = "JSON",
        help = "Specify the definition of the build tools."
    )]
    definition: Option<PathBuf>,

    #[clap(
        short,
        long,
        default_value = "default",
        value_name = "FORMAT",
        arg_enum,
        help = "Specify the output format"
    )]
    format: Format,

    #[clap(short = 'L', long = "list-defs", help = "print the build tools' definition list")]
    list_defs: bool,

    #[clap(long = "no-ignore", help = "Do not respect ignore files (.ignore, .gitignore, etc.)")]
    no_ignore: bool,

    #[clap(
        short = '@',
        value_name = "INPUT",
        help = "Specify the file contains project path list. If INPUT is dash ('-'), read from STDIN."
    )]
    project_list: Option<String>,

    #[clap(
        value_name = "PROJECTs",
        required = false,
        help = "The target project directories for btmeister."
    )]
    dirs: Vec<PathBuf>,
}

impl Options {
    pub fn validate(&self) -> Option<MeisterError> {
        if self.project_list.is_some() && !self.dirs.is_empty() {
            Some(MeisterError::BothTargetSpecified())
        } else if !self.list_defs && self.project_list.is_none() && self.dirs.is_empty() {
            Some(MeisterError::NoProjectSpecified())
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Format {
    Default,
    Json,
    Yaml,
    Xml,
}

pub struct BuildTool {
    path: PathBuf,
    def: BuildToolDef,
}

impl BuildTool {
    pub fn new(path: PathBuf, def: BuildToolDef) -> BuildTool {
        BuildTool { path, def }
    }
}

#[derive(Error, Debug)]
enum MeisterError {
    #[error("{0}: project directory not found")]
    ProjectNotFound(String),
    #[error("no projects are specified")]
    NoProjectSpecified(),
    #[error("both project list and directories are specified")]
    BothTargetSpecified(),
}

fn open_impl(file: String) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match &*file {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file)?))),
    }
}

fn parse_project_list(list_file: String) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let f = open_impl(list_file)?;
    let mut lines = Vec::new();
    for line in f.lines() {
        lines.push(PathBuf::from(line.unwrap()));
    }
    Ok(lines)
}

fn parse_targets(
    project_list: Option<String>,
    dirs: Vec<PathBuf>,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    if let Some(x) = project_list {
        parse_project_list(x)
    } else {
        Ok(dirs)
    }
}

fn extract_file_name(target: &Path) -> Option<&str> {
    if let Some(name) = target.file_name() {
        name.to_str()
    } else {
        None
    }
}

fn find_build_tools_impl(target: &Path, defs: &BuildToolDefs) -> Option<BuildToolDef> {
    if let Some(file_name) = extract_file_name(target) {
       for def in defs {
            for build_file in &def.build_files {
                if file_name == build_file {
                    return Some(def.clone());
                }
            }
        }
    }
    None
}

fn find_build_tools(target: &PathBuf, defs: &BuildToolDefs, no_ignore: bool) -> Result<Vec<BuildTool>, Box<dyn Error>> {
    let mut build_tools = Vec::new();
    for result in WalkBuilder::new(target)
            .ignore(!no_ignore).git_ignore(!no_ignore).build() {
        match result {
            Ok(entry) => if let Some(def) = find_build_tools_impl(entry.path(), defs) {
                build_tools.push(BuildTool::new(entry.path().to_path_buf(), def));
            },
            Err(err) => eprintln!("ERROR: {}", err),
        }
    }
    Ok(build_tools)
}

fn perform_each(target: &PathBuf, defs: &BuildToolDefs, no_ignore: bool,
        formatter: &Box<dyn formatter::Formatter>,
        dest: &mut Box<dyn Write>) -> Result<i32, Box<dyn Error>> {
    if !target.exists() {
        Err(Box::new(MeisterError::ProjectNotFound(target.display().to_string())))
    } else {
        match find_build_tools(target, defs, no_ignore) {
            Ok(results) => Ok(formatter.print(dest, target, results)),
            Err(e) => Err(e),
        }
    }
}

fn perform(opts: Options, dest: &mut Box<dyn Write>) -> Result<i32, Box<dyn Error>> {
    let defs = construct(opts.definition, opts.append_defs)?;
    let formatter = <dyn formatter::Formatter>::build(opts.format);
    if opts.list_defs {
        formatter.print_defs(dest, &defs);
    } else {
        let targets = parse_targets(opts.project_list, opts.dirs)?;
        for target in targets {
            if let Err(e) = perform_each(&target, &defs, opts.no_ignore, &formatter, dest) {
                println!("{}", e);
            }
        }
    }
    dest.flush().unwrap();
    Ok(0)
}

fn main() {
    let opts = Options::parse();
    if let Some(err) = opts.validate() {
        println!("{}", err)
    }

    let mut dest: Box<dyn Write> = Box::new(BufWriter::new(stdout()));
    std::process::exit(match perform(opts, &mut dest) {
        Err(err) => {
            println!("{}", err);
            1
        }
        Ok(code) => code,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let opts = Options::parse_from("testdata/fibonacci testdata/hello".split(" "));
        let defs = btmeister::construct(opts.definition, opts.append_defs).unwrap();

        let r1 = find_build_tools(&PathBuf::from("testdata/fibonacci"), &defs, false).unwrap();
        assert_eq!(1, r1.len());
        let item1 = r1.get(0).unwrap();

        assert_eq!(PathBuf::from("testdata/fibonacci/build.gradle"), item1.path);
        assert_eq!("Gradle", item1.def.name);
    }

    #[test]
    fn test_validate() {
        let opts = Options::parse_from(["btmeister", "testdata/fibonacci"]);
        let r = opts.validate();
        assert!(r.is_none());
    }

    #[test]
    fn test_validate_both_project_list_dirs() {
        let opts = Options::parse_from("btmeister -@ testdata/project_list.txt testdata/fibonacci".split(" "));
        let r = opts.validate();
        assert!(r.is_some());
        println!("{}", r.unwrap());
    }

    #[test]
    fn test_validate_no_project_given() {
        let opts = Options::parse_from(["btmeister"]);
        let r = opts.validate();
        assert!(r.is_some());
    }

    #[test]
    fn test_validate_ok() {
        let opts = Options::parse_from(["btmeister", "testdata/fibonacci"]);
        let r = opts.validate();
        assert!(r.is_none());
    }
}