use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::BufReader;
use std::ops::Deref;
use std::path::PathBuf;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "defs"]
struct Asset;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BuildToolDef {
    pub name: String,
    #[serde(rename = "build-files")]
    pub build_files: Vec<String>,
    pub url: String,
}

pub type BuildToolDefs = std::vec::Vec<BuildToolDef>;

impl BuildToolDef {
    pub fn parse(path: PathBuf) -> Result<BuildToolDefs, Box<dyn std::error::Error>> {
        let file = OpenOptions::new().read(true).open(path)?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }

    fn parse_from_asset() -> Result<BuildToolDefs, Box<dyn std::error::Error>> {
        if let Some(f) = Asset::get("buildtools.json") {
            let string = std::str::from_utf8(f.data.as_ref())?;
            Ok(serde_json::from_str(string)?)
        } else {
            panic!("buildtools.json: file not found");
        }
    }
}

pub fn merge_build_tools(first: BuildToolDefs, second: BuildToolDefs) -> BuildToolDefs {
    let mut result = BuildToolDefs::new();
    for item in first {
        result.push(item);
    }
    for item in second {
        result.push(item);
    }
    result
}

pub fn construct(
    defs: Option<PathBuf>,
    append: Option<PathBuf>,
) -> Result<BuildToolDefs, Box<dyn std::error::Error>> {
    let def = if let Some(path) = defs {
        BuildToolDef::parse(path)
    } else {
        BuildToolDef::parse_from_asset()
    }?;
    let result = if let Some(append_path) = append {
        let additional_defs = BuildToolDef::parse(append_path)?;
        merge_build_tools(def, additional_defs)
    } else {
        def
    };
    Ok(result)
}

#[cfg(test)] 
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let r = BuildToolDef::parse_from_asset();
        assert!(r.is_ok());
        if let Ok(result) = r {
            assert_eq!(24, result.len());
        }
    }

    #[test]
    fn test_parse_other() {
        let r = BuildToolDef::parse(PathBuf::from("testdata/append_def.json"));
        assert!(r.is_ok());
        if let Ok(result) = r {
            assert_eq!(1, result.len());
        } 
    }

    #[test]
    fn test_construct1() {
        let r = construct(None, None);
        assert!(r.is_ok());
        if let Ok(result) = r {
            assert_eq!(24, result.len());
        }
    }

    #[test]
    fn test_construct2() {
        let r = construct(Some(PathBuf::from("defs/buildtools.json")), None);
        assert!(r.is_ok());
        if let Ok(result) = r {
            assert_eq!(24, result.len());
        }
    }

    #[test]
    fn test_construct3() {
        let r = construct(None, Some(PathBuf::from("testdata/append_def.json")));
        assert!(r.is_ok());
        if let Ok(result) = r {
            assert_eq!(25, result.len());
        }
    }
}

