use std::path::{Path, PathBuf};
use path_matchers::{glob, PathMatcher};

use crate::cli::{IgnoreType, MeisterError, Result};
use crate::defs::{BuildToolDef, BuildToolDefs};

pub struct Meister {
    defs: Vec<BuildToolDef>,
    matchers: Vec<MultipleMatcher>,
    its: Vec<IgnoreType>,
}

#[derive(Clone)]
pub struct BuildTools {
    pub base: PathBuf,
    pub tools: Vec<BuildTool>,
}

#[derive(Clone)]
pub struct BuildTool {
    pub path: PathBuf,
    pub def: BuildToolDef,
}

trait Matcher {
    fn matches(&self, p: &PathBuf) -> bool;
}

impl BuildTools {
    pub fn path_of(&self, index: usize) -> Result<String> {
        if let Some(bt) = self.tools.get(index) {
            if let Ok(p) = bt.path.strip_prefix(self.base.clone()) {
                Ok(p.display().to_string())
            } else {
                Ok(bt.path.display().to_string())
            }
        } else {
            Err(MeisterError::Fatal(format!("index {} out of range", index)))
        }
    }
}

impl Meister {
    pub fn new_as_default() -> Result<Self> {
        match BuildToolDefs::parse_from_asset() {
            Ok(r) => Meister::new(r, vec![IgnoreType::Default]),
            Err(e) => Err(e),
        }
    }

    pub fn new(defs: BuildToolDefs, its: Vec<IgnoreType>) -> Result<Self>{
        match build_matchers(defs.defs.clone()) {
            Ok(m) => Ok(Self {
                defs: defs.defs.clone(),
                matchers: m,
                its: its,
            }),
            Err(e) => Err(e),
        }

    }

    pub fn find(&self, base: PathBuf) -> Result<BuildTools> {
        let mut result = vec![];
        let mut errs = vec![];

        let walker = build_walker(base.clone(), &self.its);
        for entry in walker {
            match entry {
                Ok(entry) => {
                    if let Some(bt) = find_build_tool(&self, entry.path()) {
                        result.push(bt);
                    }
                }
                Err(e) => errs.push(MeisterError::Warning(format!("walking: {}", e))),
            }
        }
        if errs.len() == 0 {
            Ok(BuildTools {
                base: base,
                tools: result,
            })
        } else {
            Err(MeisterError::Array(errs))
        }
    }
}

fn find_build_tool(meister: &Meister, path: &Path) -> Option<BuildTool> {
    for (def, matcher) in meister.defs.iter().zip(meister.matchers.iter()) {
        let pb = path.to_path_buf();
        if matcher.matches(&pb) {
            return Some(BuildTool { path: pb, def: def.clone() });
        }
    }
    None
}

fn build_walker(base: PathBuf, its: &Vec<IgnoreType>) -> ignore::Walk {
    ignore::WalkBuilder::new(base)
        .standard_filters(its.contains(&IgnoreType::Default))
        .hidden(its.contains(&IgnoreType::Hidden))
        .git_ignore(its.contains(&IgnoreType::GitIgnore))
        .git_global(its.contains(&IgnoreType::GitGlobal))
        .git_exclude(its.contains(&IgnoreType::GitExclude))
        .ignore(its.contains(&IgnoreType::Ignore))
        .build()
}

fn build_matcher_impl(filename: String) -> Result<Box<dyn Matcher>> {
    if filename.contains("/") || filename.contains("*") {
        match PathGlobMatcher::new(filename) {
            Ok(p) => Ok(Box::new(p) as Box<dyn Matcher>),
            Err(e) => Err(e),
        }
    } else {
        Ok(Box::new(FileNameMatcher::new(filename)) as Box<dyn Matcher>)
    }
}

fn build_matcher(def: BuildToolDef) -> Result<MultipleMatcher> {
    let mut matchers = vec![];
    let mut errs = vec![];
    for file in def.build_files.iter() {
        match build_matcher_impl(file.clone()) {
            Ok(m) => matchers.push(m),
            Err(e) => errs.push(e),
        }
    }
    if errs.is_empty() {
        Ok(MultipleMatcher{matchers: matchers})
    } else {
        Err(MeisterError::Array(errs))
    }
}

fn build_matchers(defs: Vec<BuildToolDef>) -> Result<Vec<MultipleMatcher>> {
    let mut result = vec![];
    let mut errs = vec![];
    for def in defs {
        match build_matcher(def) {
            Ok(m) => result.push(m),
            Err(e) => errs.push(e),
        }
    }
    if errs.is_empty() {
        Ok(result)
    } else {
        Err(MeisterError::Array(errs))
    }
}

struct MultipleMatcher {
    matchers: Vec<Box<dyn Matcher>>,
}

struct FileNameMatcher {
    name: String,
}
struct PathGlobMatcher {
    pattern: Box<dyn PathMatcher>,
}

impl Matcher for MultipleMatcher {
    fn matches(&self, p: &PathBuf) -> bool {
        for matcher in self.matchers.iter() {
            if matcher.matches(p) {
                return true;
            }
        }
        false
    }
}

impl Matcher for FileNameMatcher {
    fn matches(&self, p: &PathBuf) -> bool {
        if let Some(filename) = p.file_name() {
            if let Some(name) = filename.to_str() {
                return name == self.name;
            }
        }
        false
    }
}

impl Matcher for PathGlobMatcher {
    fn matches(&self, p: &PathBuf) -> bool {
        self.pattern.matches(p)
    }
}

impl FileNameMatcher {
    pub fn new(name: String) -> FileNameMatcher {
        FileNameMatcher { name }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use crate::defs::BuildToolDef;

    #[test]

    fn test_build_walker() {
        if let Ok(meister) = Meister::new_as_default() {
            let r = meister.find(PathBuf::from("testdata/fibonacci"));
            assert!(r.is_ok());
            if let Ok(r) = r {
                assert_eq!(1, r.tools.len());
                assert_eq!("Gradle", r.tools[0].def.name);
                if let Ok(p) = r.path_of(0) {
                    assert_eq!("build.gradle".to_string(), p);
                }
            }
        }
    }
    #[test]
    fn test_matches1() {
        let def = BuildToolDef::new(
            "test".to_string(),
            vec!["*.rs".to_string()],
            "http://example.com".to_string(),
        );
        let matcher = build_matcher(def);
        assert!(matcher.is_ok());
        if let Ok(d) = matcher {
            assert_eq!(true, d.matches(&PathBuf::from("testdata/file1.rs")));
            assert_eq!(true, d.matches(&PathBuf::from("file2.rs")));
        }
    }

    #[test]
    fn test_matches2() {
        let def = BuildToolDef::new(
            "test2".to_string(),
            vec!["some/dir/*.yaml".to_string()],
            "http://example.com".to_string(),
        );
        let matcher = build_matcher(def);
        assert!(matcher.is_ok());
        if let Ok(d) = matcher {
            assert_eq!(false, d.matches(&PathBuf::from("hoge.yaml")));
            assert_eq!(true, d.matches(&PathBuf::from("some/dir/file2.yaml")));
            assert_eq!(false, d.matches(&PathBuf::from("not/some/dir/file3.yaml")));
        }
    }

    #[test]
    fn test_matches3() {
        let def = BuildToolDef::new(
            "test2".to_string(),
            vec!["Somefile".to_string()],
            "http://example.com".to_string(),
        );
        let matcher = build_matcher(def);
        assert!(matcher.is_ok());
        if let Ok(d) = matcher {
            assert_eq!(true, d.matches(&PathBuf::from("Somefile")));
            assert_eq!(true, d.matches(&PathBuf::from("some/dir/Somefile")));
        }
    }
}
