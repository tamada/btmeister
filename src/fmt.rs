mod csv;
mod default;
mod json;
mod xml;
mod yaml;

use std::path::PathBuf;

use btmeister::{BuildTool, Result};
use crate::cli::Format;
use crate::defs::BuildToolDef;
use crate::fmt::csv::Formatter as CsvFormatter;
use crate::fmt::default::Formatter as DefaultFormatter;
use crate::fmt::json::Formatter as JsonFormatter;
use crate::fmt::xml::Formatter as XmlFormatter;
use crate::fmt::yaml::Formatter as YamlFormatter;

pub trait Formatter {
    #[cfg(test)]
    fn name(&self) -> &'static str;

    fn header_defs(&self) -> Option<String>;
    fn footer_defs(&self) -> Option<String>;

    fn format_def(&self, def: &BuildToolDef, first: bool) -> Result<String>;

    fn header_files(&self) -> Option<String>;
    fn footer_files(&self) -> Option<String>;
    fn format_files(&self, path: &PathBuf, def: &Vec<BuildTool>, first: bool) -> Result<String>;
}

pub fn build_formatter(format: Format) -> Box<dyn Formatter> {
    match format {
        Format::Csv => Box::new(CsvFormatter {}),
        Format::Default => Box::new(DefaultFormatter {}),
        Format::Json => Box::new(JsonFormatter {}),
        Format::Xml => Box::new(XmlFormatter {}),
        Format::Yaml => Box::new(YamlFormatter {}),
    }
}

#[cfg(test)]
pub fn fake_build_def() -> BuildToolDef {
    BuildToolDef::new(
        "Fake".to_string(),
        vec!["Fakefile".to_string()],
        "https://example.com".to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_formatter() {
        assert_eq!("csv", build_formatter(Format::Csv).name());
        assert_eq!("default", build_formatter(Format::Default).name());
        assert_eq!("json", build_formatter(Format::Json).name());
        assert_eq!("xml", build_formatter(Format::Xml).name());
        assert_eq!("yaml", build_formatter(Format::Yaml).name());
    }
}
