use crate::cli;
use crate::defs;
use crate::fmt::Formatter as FormatterTrait;

pub(super) struct Formatter {}

impl FormatterTrait for Formatter {
    fn name(&self) -> &'static str {
        "yaml"
    }

    fn header_defs(&self) -> Option<String> {
        None
    }

    fn footer_defs(&self) -> Option<String> {
        None
    }

    fn format_def(&self, def: &defs::BuildToolDef, _: bool) -> cli::Result<String> {
        let files = &def
            .build_files
            .iter()
            .map(|s| format!("  - \"{}\"", s))
            .collect::<Vec<String>>()
            .join("\n");
        Ok(format!(
            r#"- name: "{}"
  build-files:
{}
  url: "{}""#,
            &def.name, files, &def.url
        ))
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
        let result = formatter.format_def(&def, false);
        assert!(result.is_ok());
        if let Ok(r) = result {
            assert_eq!(
                r#"- name: "Fake"
  build-files:
  - "Fakefile"
  url: "https://example.com""#
                    .to_string(),
                r
            );
        }
        assert_eq!(None, formatter.header_defs());
        assert_eq!(None, formatter.footer_defs());
    }
}
