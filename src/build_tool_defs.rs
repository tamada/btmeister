use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;

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

    pub fn parse_from_pathstr(defs: String) -> Result<BuildToolDefs, Box<dyn std::error::Error>> {
        BuildToolDef::parse(PathBuf::from_str(&defs)?)
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
        BuildToolDef::parse_from_pathstr("defs/buildtools.json".to_string())
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
        let r = BuildToolDef::parse_from_pathstr("defs/buildtools.json".to_string());
        if let Ok(result) = r {
            assert_eq!(24, result.len())
        }
    }

}

