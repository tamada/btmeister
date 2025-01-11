use std::fs::OpenOptions;
use std::io::BufReader;
use std::path::PathBuf;

use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};

use crate::{verbose, Result, MeisterError};

#[derive(RustEmbed)]
#[folder = "assets"]
struct Asset;

#[derive(Serialize, Deserialize, Clone)]
pub struct BuildToolDef {
    pub name: String,
    #[serde(rename = "build-files")]
    pub build_files: Vec<String>,
    pub url: String,
    // #[serde(skip)]
    // matchers: Vec<Box<dyn Matcher>>,
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct BuildToolDefs {
    #[serde(flatten)]
    pub(crate) defs: Vec<BuildToolDef>,
}

impl BuildToolDefs {
    pub fn new(defs: Vec<BuildToolDef>) -> BuildToolDefs {
        BuildToolDefs { defs: defs }
    }

    pub fn parse(path: PathBuf) -> Result<BuildToolDefs> {
        match OpenOptions::new().read(true).open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                match serde_json::from_reader(reader) {
                    Ok(defs) => Ok(defs),
                    Err(e) => Err(MeisterError::Json(e)),
                }
            }
            Err(e) => Err(MeisterError::Io(e)),
        }
    }

    pub fn parse_from_asset() -> Result<BuildToolDefs> {
        if let Some(f) = Asset::get("buildtools.json") {
            match std::str::from_utf8(f.data.as_ref()) {
                Ok(string) => match serde_json::from_str(string) {
                    Ok(defs) => Ok(defs),
                    Err(e) => Err(MeisterError::Json(e)),
                },
                Err(e) => Err(MeisterError::Fatal(e.to_string())),
            }
        } else {
            Err(MeisterError::Fatal(
                "buildtools.json: no asset was included!!".to_string(),
            ))
        }
    }

    pub fn len(&self) -> usize {
        self.defs.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &BuildToolDef> + '_ {
        self.defs.iter()
    }

    pub fn extend(&mut self, second: BuildToolDefs) {
        self.defs.extend(second.defs);
    }

    pub fn append(&mut self, other: &mut BuildToolDefs) {
        self.defs.append(&mut other.defs);
    }
}

impl BuildToolDef {
    pub fn new(name: String, build_files: Vec<String>, url: String) -> Self {
        BuildToolDef {
            name,
            build_files,
            url,
        }
    }

    // pub fn matches(&self, p: &PathBuf) -> bool {
    //     for matcher in self.matchers.iter() {
    //         if matcher.matches(&p) {
    //             return true;
    //         }
    //     }
    //     false
    // }
}

pub fn construct(
    defs: Option<PathBuf>,
    append: Option<PathBuf>,
    v: &mut Box<dyn verbose::Verboser>,
) -> Result<BuildToolDefs> {
    let def = if let Some(path) = defs {
        v.log(format!("load definition from {:?}", path).as_str());
        BuildToolDefs::parse(path)
    } else {
        v.log("load definition from assets");
        BuildToolDefs::parse_from_asset()
    };
    match def {
        Err(e) => Err(e),
        Ok(mut def) => {
            let result = if let Some(append_path) = append {
                match BuildToolDefs::parse(append_path.clone()) {
                    Ok(mut additional_defs) => {
                        v.log(
                            format!("load additional definition from {:?}", append_path).as_str(),
                        );
                        def.append(&mut additional_defs);
                        Ok(def)
                    }
                    Err(e) => Err(e),
                }
            } else {
                Ok(def)
            };
            result
        }
    }
}

#[cfg(test)]
mod test {
    use core::panic;

    use super::*;

    #[test]
    fn test_parse() {
        let r = BuildToolDefs::parse_from_asset();
        match r {
            Ok(defs) => assert_eq!(45, defs.len()),
            Err(e) => panic!("fatal: {:?}", e),
        }
    }

    #[test]
    fn test_parse_other() {
        let r = BuildToolDefs::parse(PathBuf::from("testdata/append_def.json"));
        assert!(r.is_ok());
        if let Ok(result) = r {
            assert_eq!(2, result.len());
        }
    }

    #[test]
    fn test_construct1() {
        let r = construct(None, None, &mut verbose::none());
        assert!(r.is_ok());
        if let Ok(result) = r {
            assert_eq!(45, result.len());
        }
    }

    #[test]
    fn test_construct2() {
        let r = construct(
            Some(PathBuf::from("assets/buildtools.json")),
            None,
            &mut verbose::none(),
        );
        assert!(r.is_ok());
        if let Ok(result) = r {
            assert_eq!(45, result.len());
        }
    }

    #[test]
    fn test_construct3() {
        let r = construct(
            None,
            Some(PathBuf::from("testdata/append_def.json")),
            &mut verbose::none(),
        );
        assert!(r.is_ok());
        if let Ok(result) = r {
            assert_eq!(47, result.len());
        }
    }
}
