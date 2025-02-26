use std::io::Write;

use crate::defs;
use crate::fmt::Formatter as FormatterTrait;
use btmeister::{BuildTools, MeisterError, Result};

pub(super) struct Formatter {}

impl FormatterTrait for crate::fmt::json::Formatter {
    #[cfg(test)]
    fn name(&self) -> &'static str {
        "json"
    }

    fn header_defs(&self) -> Option<String> {
        Some("[".to_string())
    }

    fn footer_defs(&self) -> Option<String> {
        Some("]".to_string())
    }

    fn format_def(&self, def: &defs::BuildToolDef, first: bool) -> Result<String> {
        let files = &def
            .build_files
            .iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<String>>()
            .join(",");
        let separator = if first { "" } else { "," };
        let result = format!(
            r#"{}{{"name":"{}","build-files":[{}],"url":"{}"}}"#,
            separator, &def.name, files, &def.url
        );
        Ok(result)
    }

    fn format_files(&self, tools: &BuildTools, first: bool) -> Result<String> {
        let mut result = Vec::<u8>::new();
        let comma = if first { "" } else { "," };
        let _ = writeln!(
            result,
            r#"{}{{"base":"{}","build-tools":["#,
            comma,
            tools.base.display()
        );
        for (uindex, bt) in tools.tools.iter().enumerate() {
            let path = if let Ok(p) = bt.path.strip_prefix(tools.base.clone()) {
                p
            } else {
                bt.path.as_path()
            };
            let separator = if uindex == 0 { "" } else { "," };
            let _ = writeln!(
                result,
                r#"{}{{"path":"{}","tool-name":"{}"}}"#,
                separator,
                path.display(),
                bt.def.name
            );
        }
        let _ = writeln!(result, "]}}");
        String::from_utf8(result).map_err(|e| MeisterError::Fatal(format!("{}", e)))
    }

    fn header_files(&self) -> Option<String> {
        Some("[".to_string())
    }

    fn footer_files(&self) -> Option<String> {
        Some("]".to_string())
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
        let result = formatter.format_def(&def, true);
        assert!(result.is_ok());
        if let Ok(r) = result {
            assert_eq!(
                r#"{"name":"Fake","build-files":["Fakefile"],"url":"https://example.com"}"#
                    .to_string(),
                r
            );
        }
    }

    #[test]
    fn test_format_csv_second_or_later() {
        let formatter = Formatter {};
        let def = fake_build_def();
        let result = formatter.format_def(&def, false);
        assert!(result.is_ok());
        if let Ok(r) = result {
            assert_eq!(
                r#",{"name":"Fake","build-files":["Fakefile"],"url":"https://example.com"}"#
                    .to_string(),
                r
            );
        }

        assert_eq!(Some("[".to_string()), formatter.header_defs());
        assert_eq!(Some("]".to_string()), formatter.footer_defs());
    }

    #[test]
    fn test_format_buildtools() {
        let formatter = Formatter {};
        let tools = crate::fmt::fake_build_tools();
        let result = formatter.format_files(&tools, false);
        assert!(result.is_ok());
        if let Ok(r) = result {
            assert_eq!(
                r#",{"base":"fake/base/dir","build-tools":[
{"path":"Fakefile","tool-name":"Fake"}
,{"path":"Makefile","tool-name":"Make"}
]}
"#
                .to_string(),
                r
            );
        }
        assert_eq!(Some("[".to_string()), formatter.header_files());
        assert_eq!(Some("]".to_string()), formatter.footer_files());
    }
}
