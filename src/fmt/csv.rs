use crate::cli;
use crate::defs;
use crate::fmt::Formatter as FormatterTrait;

pub(super) struct Formatter {}

impl FormatterTrait for Formatter {
    fn name(&self) -> &'static str {
        "csv"
    }
    fn format_def(&self, def: &defs::BuildToolDef, _: bool) -> cli::Result<String> {
        let name = &def.name;
        let url = &def.url;
        let result = def
            .build_files
            .iter()
            .map(|s| format!("{},{},{}", name, s, url))
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
            assert_eq!("Fake,Fakefile,https://example.com".to_string(), r);
        }
        assert_eq!(None, formatter.header_defs());
        assert_eq!(None, formatter.footer_defs());
    }
}
