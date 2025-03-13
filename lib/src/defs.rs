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
use core::panic;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::path::PathBuf;

use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};

use crate::{MeisterError, Result};

#[derive(RustEmbed)]
#[folder = "../assets"]
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

impl Default for BuildToolDefs {
    /// The `default`` function parses the defitions of the build tools from the asset file included in the library,
    /// and build an object of BuildToolDefs.
    fn default() -> Self {
        if let Some(f) = Asset::get("buildtools.json") {
            match std::str::from_utf8(f.data.as_ref()) {
                Ok(string) => match serde_json::from_str(string) {
                    Ok(defs) => defs,
                    Err(e) => panic!("json error: {}", e),
                },
                Err(e) => panic!("fatal: {}", e),
            }
        } else {
            panic!("fatal: no asset was included!!");
        }
    }
}

impl BuildToolDefs {
    /// new creates a new BuildToolDefs object from the given definitions.
    pub fn new<I, T>(defs: I) -> BuildToolDefs
    where
        I: IntoIterator<Item = T>,
        T: Into<BuildToolDef>,
    {
        BuildToolDefs {
            defs: defs.into_iter().map(Into::into).collect(),
        }
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
pub fn construct(defs: Option<PathBuf>, append: Option<PathBuf>) -> Result<BuildToolDefs> {
    let def = if let Some(path) = defs {
        log::info!("load definition from {:?}", path.to_string_lossy());
        BuildToolDefs::parse(path)
    } else {
        log::info!("load definition from assets");
        Ok(BuildToolDefs::default())
    };
    match def {
        Err(e) => Err(e),
        Ok(mut def) => {
            if let Some(append_path) = append {
                match BuildToolDefs::parse(append_path.clone()) {
                    Ok(mut additional_defs) => {
                        log::info!(
                            "load additional definition from {:?}",
                            append_path.to_string_lossy()
                        );
                        def.append(&mut additional_defs);
                        Ok(def)
                    }
                    Err(e) => Err(e),
                }
            } else {
                Ok(def)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let defs = BuildToolDefs::default();
        assert_eq!(46, defs.len())
    }

    #[test]
    fn test_parse_other() {
        let r = BuildToolDefs::parse(PathBuf::from("../testdata/append_def.json"));
        assert!(r.is_ok());
        if let Ok(result) = r {
            assert_eq!(2, result.len());
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_construct1() {
        let r = construct(None, None);
        assert!(r.is_ok());
        if let Ok(result) = r {
            assert_eq!(46, result.len());
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_construct2() {
        let r = construct(Some(PathBuf::from("../assets/buildtools.json")), None);
        assert!(r.is_ok());
        if let Ok(result) = r {
            assert_eq!(46, result.len());
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_construct3() {
        let r = construct(None, Some(PathBuf::from("../testdata/append_def.json")));
        assert!(r.is_ok());
        if let Ok(result) = r {
            assert_eq!(48, result.len());
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_new_and_extend() {
        let mut defs1 = BuildToolDefs::new(Vec::<BuildToolDef>::new());
        assert_eq!(0, defs1.len());
        assert!(defs1.is_empty());

        let defs2 = BuildToolDefs::new(vec![BuildToolDef::new(
            "Fake".to_string(),
            vec!["Fakefile".to_string()],
            "https://example.com".to_string(),
        )]);
        assert_eq!(1, defs2.len());

        defs1.extend(defs2);

        assert_eq!(1, defs1.len());
        assert!(!defs1.is_empty());
    }
}
