/*!
 * This is a module for the build tool definitions.
 * Meister supports the following ways for building the definitions:
 *
 * - Load from the asset file included in the library.
 * - Load from the given file.
 * - Load from the asset file and append the definitions from the other file.
 * - Load from the given file and append the definitions from the other file.
 *
 * The definition file must be a JSON format file ([JSON schema](https://github.com/tamada/btmeister/blob/main/assets/buildtools.json.schema)).
 * The example of the definition file is as follows and the default definition file is [here](https://github.com/tamada/btmeister/blob/main/assets/buildtools.json).
 *
 * ```json
 * [
 *   {
 *     "name": "Fake",
 *     "build-files": ["Fakefile"],
 *     "url": "https://example.com"
 *   }
 * ]
 * ```
 */
use std::fs::OpenOptions;
use std::io::BufReader;
use std::path::PathBuf;

use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};

use crate::{verbose, MeisterError, Result};

#[derive(RustEmbed)]
#[folder = "assets"]
struct Asset;

/// BuildToolDef represents a build tool definition.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BuildToolDef {
    pub name: String,
    #[serde(rename = "build-files")]
    pub build_files: Vec<String>,
    pub url: String,
    // #[serde(skip)]
    // matchers: Vec<Box<dyn Matcher>>,
}

/// BuildToolDefs represents a collection of build tool definitions.
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct BuildToolDefs {
    #[serde(flatten)]
    pub(crate) defs: Vec<BuildToolDef>,
}

impl BuildToolDefs {
    /// new creates a new BuildToolDefs object from the given definitions.
    pub fn new(defs: Vec<BuildToolDef>) -> BuildToolDefs {
        BuildToolDefs { defs }
    }

    /// parse parses the defitions of the build tools from the given file and build an object of BuildToolDefs.
    pub fn parse(path: PathBuf) -> Result<BuildToolDefs> {
        match OpenOptions::new().read(true).open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                match serde_json::from_reader(reader) {
                    Ok(defs) => Ok(defs),
                    Err(e) => Err(MeisterError::Json(e)),
                }
            }
            Err(e) => Err(MeisterError::IO(e)),
        }
    }

    /// parse_from_asset parses the defitions of the build tools from the asset file included in the library,
    /// and build an object of BuildToolDefs.
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

    /// len returns the number of the build tool definitions.
    pub fn len(&self) -> usize {
        self.defs.len()
    }

    /// is_empty returns true if the build tool definitions are empty.
    pub fn is_empty(&self) -> bool {
        self.defs.is_empty()
    }

    /// iter returns an iterator of the build tool definitions.
    pub fn iter(&self) -> impl Iterator<Item = &BuildToolDef> + '_ {
        self.defs.iter()
    }

    /// extend appends the build tool definitions of the second object to the first object.
    pub fn extend(&mut self, second: BuildToolDefs) {
        self.defs.extend(second.defs);
    }

    /// append appends the build tool definitions of the second object to the first object.
    pub fn append(&mut self, other: &mut BuildToolDefs) {
        self.defs.append(&mut other.defs);
    }
}

impl BuildToolDef {
    /// new creates a new BuildToolDef object with the given name, build files, and URL.
    pub fn new(name: String, build_files: Vec<String>, url: String) -> Self {
        BuildToolDef {
            name,
            build_files,
            url,
        }
    }
}

/// The `construct` function creates a BuildToolDefs object from the given definition file and append file.
/// If defs is None, it reads the definition from the asset file.
/// This function supports four building definition ways.
///
/// - Load from the asset file included in the library.
///   - gives both `defs` and `append` is `None`.
/// - Load from the given file.
///   - gives `defs` is `Some` and `append` is `None`.
/// - Load from the asset file and append the definitions from the other file.
///   - gives `defs` is `None` and `append` is `Some`.
/// - Load from the given file and append the definitions from the other file.
///   - gives both `defs` and `append` is `Some`.
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
            assert_eq!(false, result.is_empty());
        }
    }

    #[test]
    fn test_construct1() {
        let r = construct(None, None, &mut verbose::none());
        assert!(r.is_ok());
        if let Ok(result) = r {
            assert_eq!(45, result.len());
            assert_eq!(false, result.is_empty());
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
            assert_eq!(false, result.is_empty());
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
            assert_eq!(false, result.is_empty());
        }
    }

    #[test]
    fn test_new_and_extend() {
        let mut defs1 = BuildToolDefs::new(vec![]);
        let defs2 = BuildToolDefs::new(vec![BuildToolDef::new(
            "Fake".to_string(),
            vec!["Fakefile".to_string()],
            "https://example.com".to_string(),
        )]);
        assert_eq!(0, defs1.len());
        assert_eq!(true, defs1.is_empty());
        assert_eq!(1, defs2.len());
        defs1.extend(defs2);
        assert_eq!(1, defs1.len());
        assert_eq!(false, defs1.is_empty());
    }
}
