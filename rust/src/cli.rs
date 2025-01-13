use clap::{ArgEnum, Parser};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Options {
    #[clap(
        long,
        value_name = "DEFS_JSON",
        help = "Specify the additional definitions of the build tools."
    )]
    pub append_defs: Option<PathBuf>,

    #[clap(
        short,
        long,
        value_name = "DEFS_JSON",
        help = "Specify the definition of the build tools."
    )]
    pub definition: Option<PathBuf>,

    #[clap(
        short,
        long,
        default_value = "default",
        value_name = "FORMAT",
        arg_enum,
        help = "Specify the output format"
    )]
    pub format: Format,

    #[clap(
        short = 'L',
        long = "list-defs",
        help = "Print the build tools' definition list"
    )]
    pub list_defs: bool,

    #[clap(
        long = "no-ignore",
        help = "Do not respect ignore files (.ignore, .gitignore, etc.)"
    )]
    pub no_ignore: bool,

    #[clap(
        short = '@',
        value_name = "INPUT",
        help = "Specify the file contains project path list. If INPUT is dash ('-'), read from STDIN."
    )]
    pub project_list: Option<String>,

    #[clap(
        value_name = "PROJECTs",
        required = false,
        help = "The target project directories for btmeister."
    )]
    pub dirs: Vec<PathBuf>,
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

#[derive(Error, Debug)]
pub enum MeisterError {
    #[error("{0}: project directory not found")]
    ProjectNotFound(String),
    #[error("no projects are specified")]
    NoProjectSpecified(),
    #[error("both project list and directories are specified")]
    BothTargetSpecified(),
}
