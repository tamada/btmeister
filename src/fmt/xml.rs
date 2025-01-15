use std::io::Write;

use crate::defs;
use crate::fmt::Formatter as FormatterTrait;
use btmeister::{BuildTools, MeisterError, Result};

pub(super) struct Formatter {}

impl FormatterTrait for Formatter {
    #[cfg(test)]
    fn name(&self) -> &'static str {
        "xml"
    }

    fn header_defs(&self) -> Option<String> {
        Some("<? xml version=\"1.0\" ?>\n<build-tool-defs>\n".to_string())
    }

    fn footer_defs(&self) -> Option<String> {
        Some("</build-tool-defs>".to_string())
    }

    fn format_def(&self, def: &defs::BuildToolDef, _: bool) -> Result<String> {
        let files = &def
            .build_files
            .iter()
            .map(|s| format!("            <build-file>{}</build-file>\n", s))
            .collect::<Vec<String>>()
            .concat();
        let result = format!(
            r#"    <build-tool-def>
        <name>{}</name>
        <build-files>
{}        </build-files>
        <url>{}</url>
    </build-tool-def>"#,
            &def.name, files, &def.url
        );
        Ok(result)
    }
    fn format_files(&self, tools: &BuildTools, _: bool) -> Result<String> {
        let mut result = Vec::<u8>::new();
        let _ = writeln!(
            result,
            r#"    <project>
        <base-path>{}</base-path>
        <build-files>"#,
            tools.base.display()
        );
        for bt in &tools.tools {
            let path_name = if let Ok(p) = bt.path.strip_prefix(tools.base.clone()) {
                p.display()
            } else {
                bt.path.display()
            };
            let _ = writeln!(
                result,
                r#"            <build-file tool-name="{}">{}</build-file>"#,
                bt.def.name, path_name
            );
        }
        let _ = writeln!(result, "        </build-files>\n    </project>");
        String::from_utf8(result).map_err(|e| MeisterError::Fatal(format!("{}", e)))
    }

    fn header_files(&self) -> Option<String> {
        Some("<? xml version=\"1.0\" ?>\n<build-tools>".to_string())
    }

    fn footer_files(&self) -> Option<String> {
        Some("</build-tools>".to_string())
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
                r#"    <build-tool-def>
        <name>Fake</name>
        <build-files>
            <build-file>Fakefile</build-file>
        </build-files>
        <url>https://example.com</url>
    </build-tool-def>"#,
                r
            );
        }
    }

    #[test]
    fn test_header_and_footer() {
        let formatter = Formatter {};
        assert_eq!(
            Some("<? xml version=\"1.0\" ?>\n<build-tool-defs>\n".to_string()),
            formatter.header_defs()
        );
        assert_eq!(
            Some("</build-tool-defs>".to_string()),
            formatter.footer_defs()
        );
    }

    #[test]
    fn test_format_buildtools() {
        let formatter = Formatter {};
        let tools = crate::fmt::fake_build_tools();
        let result = formatter.format_files(&tools, false);
        assert!(result.is_ok());
        if let Ok(r) = result {
            assert_eq!(
                r#"    <project>
        <base-path>fake/base/dir</base-path>
        <build-files>
            <build-file tool-name="Fake">Fakefile</build-file>
            <build-file tool-name="Make">Makefile</build-file>
        </build-files>
    </project>
"#
                .to_string(),
                r
            );
        }
    }

    #[test]
    fn test_header_and_footer2() {
        let formatter = Formatter {};
        assert_eq!(
            Some("<? xml version=\"1.0\" ?>\n<build-tools>".to_string()),
            formatter.header_files()
        );
        assert_eq!(Some("</build-tools>".to_string()), formatter.footer_files());
    }
}
