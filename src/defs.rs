use std::fs::OpenOptions;
use std::io::BufReader;
use std::path::PathBuf;

use path_matchers::{glob, PathMatcher};
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};

use crate::cli::{MeisterError, Result};
use crate::verbose;

#[derive(RustEmbed)]
#[folder = "assets"]
struct Asset;

#[derive(Serialize, Deserialize)]
pub struct BuildToolDef {
    pub name: String,
    #[serde(rename = "build-files")]
    pub build_files: Vec<String>,
    pub url: String,
    #[serde(skip)]
    matchers: Vec<Box<dyn Matcher>>,
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct BuildToolDefs {
    #[serde(flatten)]
    defs: Vec<BuildToolDef>,
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
                    Ok(defs) => Ok(build_matchers(defs)),
                    Err(e) => Err(MeisterError::Json(e)),
                }
            }
            Err(e) => Err(MeisterError::Io(e)),
        }
    }

    fn parse_from_asset() -> Result<BuildToolDefs> {
        if let Some(f) = Asset::get("buildtools.json") {
            match std::str::from_utf8(f.data.as_ref()) {
                Ok(string) => match serde_json::from_str(string) {
                    Ok(defs) => Ok(build_matchers(defs)),
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

trait Matcher {
    fn matches(&self, p: &PathBuf) -> bool;
}

impl BuildToolDef {
    pub fn new(name: String, build_files: Vec<String>, url: String) -> Result<BuildToolDef> {
        let mut errs = vec![];
        let mut matchers = vec![];
        for filename in build_files.iter() {
            match build_matcher(filename.clone()) {
                Ok(m) => matchers.push(m),
                Err(e) => errs.push(e),
            }
        }
        let def = BuildToolDef {
            name,
            build_files,
            url,
            matchers,
        };
        if errs.len() > 0 {
            Err(MeisterError::Array(errs))
        } else {
            Ok(def)
        }
    }

    pub fn matches(&self, p: PathBuf) -> bool {
        for matcher in self.matchers.iter() {
            if matcher.matches(&p) {
                return true;
            }
        }
        false
    }
}

pub(crate) fn construct(
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

struct FileNameMatcher {
    name: String,
}
struct PathGlobMatcher {
    pattern: Box<dyn PathMatcher>,
}

impl Matcher for FileNameMatcher {
    fn matches(&self, p: &PathBuf) -> bool {
        if let Some(name) = p.file_name().unwrap().to_str() {
            return name == self.name;
        }
        false
    }
}

impl Matcher for PathGlobMatcher {
    fn matches(&self, p: &PathBuf) -> bool {
        self.pattern.matches(p)
    }
}

impl PathGlobMatcher {
    pub fn new(pattern: String) -> Result<PathGlobMatcher> {
        match glob(pattern.as_str()) {
            Ok(g) => Ok(PathGlobMatcher {
                pattern: Box::new(g),
            }),
            Err(e) => Err(MeisterError::Fatal(e.to_string())),
        }
    }
}

fn build_matcher(filename: String) -> Result<Box<dyn Matcher>> {
    if filename.contains("/") || filename.contains("*") {
        match PathGlobMatcher::new(filename) {
            Ok(p) => Ok(Box::new(p) as Box<dyn Matcher>),
            Err(e) => Err(e),
        }
    } else {
        Ok(Box::new(FileNameMatcher { name: filename }) as Box<dyn Matcher>)
    }
}

fn build_matchers(defs: BuildToolDefs) -> BuildToolDefs {
    let mut result = vec![];
    for mut def in defs.defs {
        let mut matchers = vec![];
        for file in def.build_files.iter() {
            match build_matcher(file.clone()) {
                Ok(m) => matchers.push(m),
                Err(e) => panic!("fatal: {:?}", e),
            }
        }
        def.matchers = matchers;
        result.push(def);
    }
    BuildToolDefs::new(result)
}

#[cfg(test)]
pub(crate) fn fake_build_def() -> BuildToolDef {
    BuildToolDef::new(
        "Fake".to_string(),
        vec!["Fakefile".to_string()],
        "https://example.com".to_string(),
    )
    .unwrap()
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

    #[test]
    fn test_matches1() {
        let def = BuildToolDef::new(
            "test".to_string(),
            vec!["*.rs".to_string()],
            "http://example.com".to_string(),
        );
        assert!(def.is_ok());
        if let Ok(d) = def {
            assert_eq!(true, d.matches(PathBuf::from("testdata/file1.rs")));
            assert_eq!(true, d.matches(PathBuf::from("file2.rs")));
        }
    }

    #[test]
    fn test_matches2() {
        let def = BuildToolDef::new(
            "test2".to_string(),
            vec!["some/dir/*.yaml".to_string()],
            "http://example.com".to_string(),
        );
        assert!(def.is_ok());
        if let Ok(d) = def {
            assert_eq!(false, d.matches(PathBuf::from("hoge.yaml")));
            assert_eq!(true, d.matches(PathBuf::from("some/dir/file2.yaml")));
            assert_eq!(false, d.matches(PathBuf::from("not/some/dir/file3.yaml")));
        }
    }

    #[test]
    fn test_matches3() {
        let def = BuildToolDef::new(
            "test2".to_string(),
            vec!["Somefile".to_string()],
            "http://example.com".to_string(),
        );
        assert!(def.is_ok());
        if let Ok(d) = def {
            assert_eq!(true, d.matches(PathBuf::from("Somefile")));
            assert_eq!(true, d.matches(PathBuf::from("some/dir/Somefile")));
        }
    }
}
