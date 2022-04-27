use clap::{ArgEnum, Parser};
use std::error::Error;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::{Path, PathBuf};
use thiserror::Error;

mod build_tool_defs;

#[derive(Parser)]
#[clap(
    name = "btmeister",
    author = "Haruaki TAMADA",
    version = "1.0.0",
    about = "Identifying the build tools of the projects in use."
)]
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
    fn validate(&self) -> Option<MeisterError> {
        if self.project_list.is_some() && !self.dirs.is_empty() {
            Some(MeisterError::BothTargetSpecified())
        } else if self.project_list.is_none() && self.dirs.is_empty() {
            Some(MeisterError::NoProjectSpecified())
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Format {
    Default,
    Json,
    Yaml,
    Xml,
}

struct BuildTool {
    path: PathBuf,
    def: build_tool_defs::BuildToolDef,
}

impl BuildTool {
    fn parse(project: String) -> Vec<BuildTool> {
        Vec::new()
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

fn perform_each(
    target: &Path,
    _defs: &build_tool_defs::BuildToolDefs,
) -> Result<i32, Box<dyn Error>> {
    if !target.exists() {
        Err(Box::new(MeisterError::ProjectNotFound(target.display().to_string())))
    } else {

        Ok(0)
    }
}

fn perform(opts: Options) -> Result<i32, Box<dyn Error>> {
    let defs = build_tool_defs::construct(opts.definition, opts.append_defs)?;
    let targets = parse_targets(opts.project_list, opts.dirs)?;
    for target in targets {
        if let Err(e) = perform_each(target.as_path(), &defs) {
            println!("{}", e);
        }
    }
    Ok(0)
}

fn main() {
    let opts = Options::parse();
    if let Some(err) = opts.validate() {
        println!("{}", err)
    }
    std::process::exit(match perform(opts) {
        Err(err) => {
            println!("{}", err);
            1
        }
        Ok(code) => code,
    })
}
