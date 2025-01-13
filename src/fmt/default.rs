use std::io::Write;

use crate::defs;
use crate::fmt::Formatter as FormatterTrait;
use btmeister::{BuildTools, MeisterError, Result};

pub(super) struct Formatter {}

impl FormatterTrait for Formatter {
    #[cfg(test)]
    fn name(&self) -> &'static str {
        "default"
    }

    fn format_def(&self, def: &defs::BuildToolDef, _: bool) -> Result<String> {
        let name = &def.name;
        let result = def
            .build_files
            .iter()
            .map(|s| format!("{}: {}", name, s))
            .collect::<Vec<String>>()
            .join("\n");
        Ok(result)
    }

    fn header_defs(&self) -> Option<String> {
        None
    }

    fn footer_defs(&self) -> Option<String> {
        None
    }

    fn format_files(&self, tools: &BuildTools, _: bool) -> Result<String> {
        let mut result = Vec::<u8>::new();
        let _ = writeln!(result, "{}", tools.base.display());
        for bt in &tools.tools {
            if let Ok(p) = bt.path.strip_prefix(tools.base.clone()) {
                let _ = writeln!(result, "    {}: {}", p.display(), bt.def.name);
            }
        }
        String::from_utf8(result).map_err(|e| MeisterError::Fatal(format!("{}", e)))
    }

    fn header_files(&self) -> Option<String> {
        None
    }

    fn footer_files(&self) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fmt::fake_build_def;

    #[test]
    fn test_format_csv() {
        let formatter = Formatter {};
        let def = fake_build_def();
        let result = formatter.format_def(&def, false);
        assert!(result.is_ok());
        if let Ok(r) = result {
            assert_eq!("Fake: Fakefile".to_string(), r);
        }
        assert_eq!(None, formatter.header_defs());
        assert_eq!(None, formatter.footer_defs());
    }

    #[test]
    fn test_format_buildtools() {
        let formatter = Formatter {};
        let tools = crate::fmt::fake_build_tools();
        let result = formatter.format_files(&tools, false);
        assert!(result.is_ok());
        if let Ok(r) = result {
            assert_eq!(
                r#"fake/base/dir
    Fakefile: Fake
    Makefile: Make
"#
                .to_string(),
                r
            );
        }
        assert_eq!(None, formatter.header_files());
        assert_eq!(None, formatter.footer_files());
    }
}
