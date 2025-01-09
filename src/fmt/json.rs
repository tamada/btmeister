use crate::cli;
use crate::defs;
use crate::fmt::Formatter as FormatterTrait;

pub(super) struct Formatter {}

impl FormatterTrait for crate::fmt::json::Formatter {
    fn name(&self) -> &'static str {
        "json"
    }

    fn header_defs(&self) -> Option<String> {
        Some("[".to_string())
    }

    fn footer_defs(&self) -> Option<String> {
        Some("]".to_string())
    }

    fn format_def(&self, def: &defs::BuildToolDef, first: bool) -> cli::Result<String> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::defs::fake_build_def;

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
}
