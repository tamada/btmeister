mod csv;
mod default;
mod json;
mod xml;
mod yaml;

use crate::cli::{Format, Result};
use crate::defs::BuildToolDef;
use crate::fmt::csv::Formatter as CsvFormatter;
use crate::fmt::default::Formatter as DefaultFormatter;
use crate::fmt::json::Formatter as JsonFormatter;
use crate::fmt::xml::Formatter as XmlFormatter;
use crate::fmt::yaml::Formatter as YamlFormatter;

pub trait Formatter {
    fn name(&self) -> &'static str;

    fn header_defs(&self) -> Option<String>;
    fn footer_defs(&self) -> Option<String>;

    fn format_def(&self, def: &BuildToolDef, first: bool) -> Result<String>;

    // fn header_files() -> Option<String>;
    // fn footer_files() -> Option<String>;
    // fn format_file(&self, path: PathBuf, def: &BuildToolDef) -> Result<String>;
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
