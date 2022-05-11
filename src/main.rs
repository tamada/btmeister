use clap::{ArgEnum, Parser};
use std::error::Error;
use std::fs::{read_dir, File};
use std::io::{stdin, BufRead, BufReader};
use std::path::{Path, PathBuf};
use thiserror::Error;
use build_tool_defs::{BuildToolDef,BuildToolDefs,construct};
use walkdir::WalkDir;

mod build_tool_defs;
mod git_ignore;

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

fn find_ignore_file(ignore_file: &Path) -> Option<gitignore::File> {
    if !ignore_file.exists() || !ignore_file.is_file() {
        return None
    }
    match gitignore::File::new(ignore_file) {
        Ok(r) => Some(r),
        Err(e) => None,
    }
}

fn find_build_tools(target: &Path, defs: &BuildToolDefs, ignore: &mut git_ignore::Ignore) -> Result<Vec<BuildTool>, Box<dyn Error>>{
    let mut build_tools = Vec::new();
    let this_ignore = ignore.append(target);
    for entry in read_dir(target)? {
        if let Ok(e) = entry {
            let path = e.path();
            if path.is_dir() && !this_ignore.is_ignore(target) {
                find_build_tools(path.as_path(), defs, ignore)
            }
        }
    }

    Ok(build_tools)
}

fn find_build_tools2(target: &Path, defs: &BuildToolDefs) -> Result<Vec<BuildTool>, Box<dyn Error>> {
    let mut build_tools = Vec::new();
    for entry in WalkDir::new(target) {
        let entry = &entry?;
        if let Some(def) = find_build_tools_impl(entry.path(), defs) {
            build_tools.push(BuildTool {
                path: entry.path().to_path_buf(),
                def,
            });
        }
    }
    Ok(build_tools)
}

fn print_result(results: Vec<BuildTool>) -> Result<i32, Box<dyn Error>> {
    for result in results {
        println!("{}: {}", result.path.display(), result.def.name);
    }
    Ok(0)
}

fn perform_each(
    target: &Path,
    defs: &build_tool_defs::BuildToolDefs,
) -> Result<i32, Box<dyn Error>> {
    if !target.exists() {
        Err(Box::new(MeisterError::ProjectNotFound(target.display().to_string())))
    } else {
        match find_build_tools(target, defs, &mut git_ignore::Ignore{files: vec!()}) {
            Ok(results) => print_result(results),
            Err(e) => Err(e),
        }
    }
}

fn perform(opts: Options) -> Result<i32, Box<dyn Error>> {
    let defs = construct(opts.definition, opts.append_defs)?;
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
