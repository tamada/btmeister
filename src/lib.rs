/*!
 * BtMeister aims to detect the build tools in the specified directory.
 * For this, the following steps should be performed:
 *
 * At first, read the build tool definitions from the specified file or
 * the default definitions, and build an instance of [BuildToolDefs].
 *
 * Next, build an object of [Meister] with the definitions and
 * directory traversing options (`its`: ignore types).
 * If the its is empty vector, the default value [IgnoreType::Default] will be used.
 *
 * Finally, detect the build tools in the specified directory and print the result.
 *
 * ```
 * // The first step
 * let defs_result = btmeister::defs::BuildToolDefs::parse_from_asset();
 * // or specifying the definition file.
 * // let defs_result = btmeister::defs::BuildToolDefs::parse(std::path::PathBuf::from("buildtools.json"));
 *
 * // The second step
 * let meister = btmeister::Meister::new(defs_result.unwrap(), vec![]);
 *
 * // The third step
 * let meister = btmeister::Meister::new_as_default().unwrap();
 * match meister.find(std::path::PathBuf::from("testdata/hello")) {
 *     Ok(r) => {
 *        println!("project: {}", r.base.display());
 *        for bt in r.tools {
 *            println!("  {}: {}", bt.def.name, bt.path.display());
 *         }
 *     },
 *     Err(e) => panic!("error: {:?}", e),
 * }
 *  ```
 */
pub mod defs;
mod extractors;
pub mod verbose;

use clap::ValueEnum;
use path_matchers::{glob, PathMatcher};
use serde_json::Error as JsonError;
use std::path::{Path, PathBuf};

use defs::{BuildToolDef, BuildToolDefs};

/// MeisterError represents an error of the project.
#[derive(Debug)]
pub enum MeisterError {
    /// arrays of [MeisterError].
    Array(Vec<MeisterError>),
    /// Fatal error.
    Fatal(String),
    /// IO error.
    IO(std::io::Error),
    /// JSON error.
    Json(JsonError),
    /// NotImplemented error.
    NotImplemented,
    /// specified directories or files is not a project.
    NotProject(String),
    /// if no project was specified.
    NoProjectSpecified(),
    /// The given project does not exist.
    ProjectNotFound(String),
    /// the given archive file was not supported.
    UnsupportedArchiveFormat(String),
    /// warning message.
    Warning(String),
}

/// IgnoreType represents the type of traversing options for [Meister].
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum IgnoreType {
    /// [IgnoreType::Hidden], [IgnoreType::Ignore], [IgnoreType::GitIgnore], [IgnoreType::GitGlobal], and [IgnoreType::GitExclude].
    /// All of the ignore types are enabled.
    Default,
    /// ignore hidden file.
    Hidden,
    /// ignore respecting `.ignore` file.
    Ignore,
    /// ignore respecting `.gitignore` file.
    GitIgnore,
    /// ignore respecting global git ignore file.
    GitGlobal,
    /// ignore respecting `.git/info/exclude` file.
    GitExclude,
}

pub fn is_supported_archive_format(arg: &PathBuf) -> bool {
    let name = arg.to_str().unwrap().to_lowercase();
    for (_, ext) in extractors::exts().iter() {
        if name.ends_with(ext) {
            return true;
        }
    }
    return false;
}

/// a result of the project.
pub type Result<T> = std::result::Result<T, MeisterError>;

/// Meister is a object for detecting the build tools in the specified directory.
/// This object contains the definitions of the build tools.
/// In use of user own build tool definitions, use `Meister::new` method for building the object.
pub struct Meister {
    defs: Vec<BuildToolDef>,
    matchers: Vec<MultipleMatcher>,
    its: Vec<IgnoreType>,
}

/// BuildTools represents a result of the `Meister::find` method.
/// This object contains the project directory and the detected files of build tools.
#[derive(Clone, Debug)]
pub struct BuildTools {
    /// The base directory of the project.
    pub base: PathBuf,
    /// The detected files of build tools.
    pub tools: Vec<BuildTool>,
}

/// BuildTool represents a detected file for build tool.
#[derive(Clone, Debug)]
pub struct BuildTool {
    /// path of the detected file.
    pub path: PathBuf,
    /// the build tool definition corresponding to the detected file.
    pub def: BuildToolDef,
}

trait Matcher {
    fn matches(&self, p: &Path) -> bool;
}

impl BuildTools {
    /// path_of returns the relative path of the detected file from the project path.
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
    /// new_as_default creates a Meister object with the default build tool definitions.
    pub fn new_as_default() -> Result<Self> {
        match BuildToolDefs::parse_from_asset() {
            Ok(r) => Meister::new(r, vec![IgnoreType::Default]),
            Err(e) => Err(e),
        }
    }

    /// new creates a Meister object with the specified build tool definitions and ignore types.
    /// If its was the empty, the default value ([IgnoreType::Default]) will be used.
    pub fn new(defs: BuildToolDefs, its: Vec<IgnoreType>) -> Result<Self> {
        let its2 = if its.is_empty() {
            vec![IgnoreType::Default]
        } else {
            its
        };
        match build_matchers(defs.defs.clone()) {
            Ok(m) => Ok(Self {
                defs: defs.defs.clone(),
                matchers: m,
                its: its2,
            }),
            Err(e) => Err(e),
        }
    }

    /// find detects the build tools in the specified directory.
    pub fn find(&self, base: PathBuf) -> Result<BuildTools> {
        if base.is_file() {
            if is_supported_archive_format(&base) {
                self.find_archive(base)
            } else {
                Err(MeisterError::UnsupportedArchiveFormat(
                    base.display().to_string(),
                ))
            }
        } else {
            self.find_directory(base)
        }
    }

    fn find_archive(&self, base: PathBuf) -> Result<BuildTools> {
        let mut tools = vec![];
        match extractors::list_entries(base.clone()) {
            Err(e) => Err(e),
            Ok(entries) => {
                for entry in entries {
                    if let Some(bt) = find_build_tool(self, &PathBuf::from(entry)) {
                        tools.push(bt);
                    }
                }
                Ok(BuildTools { base, tools })
            }
        }
    }

    fn find_directory(&self, base: PathBuf) -> Result<BuildTools> {
        let mut result = vec![];
        let mut errs = vec![];
        let walker = build_walker(base.clone(), &self.its);
        for entry in walker {
            match entry {
                Ok(entry) => {
                    if let Some(bt) = find_build_tool(self, entry.path()) {
                        result.push(bt);
                    }
                }
                Err(e) => errs.push(MeisterError::Warning(format!("walking: {}", e))),
            }
        }
        if errs.is_empty() {
            Ok(BuildTools {
                base,
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
            return Some(BuildTool {
                path: pb,
                def: def.clone(),
            });
        }
    }
    None
}

fn build_walker(base: PathBuf, its: &[IgnoreType]) -> ignore::Walk {
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
        Ok(MultipleMatcher { matchers })
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
    fn matches(&self, p: &Path) -> bool {
        for matcher in self.matchers.iter() {
            if matcher.matches(p) {
                return true;
            }
        }
        false
    }
}

impl Matcher for FileNameMatcher {
    fn matches(&self, p: &Path) -> bool {
        if let Some(filename) = p.file_name() {
            if let Some(name) = filename.to_str() {
                return name == self.name;
            }
        }
        false
    }
}

impl Matcher for PathGlobMatcher {
    fn matches(&self, p: &Path) -> bool {
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
    use crate::defs::BuildToolDef;
    use std::path::PathBuf;

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
    fn test_archive_file() {
        if let Ok(meister) = Meister::new_as_default() {
            let r = meister.find(PathBuf::from("testdata/hello.tar"));
            assert!(r.is_ok());
            if let Ok(r) = r {
                assert_eq!(1, r.tools.len());
                assert_eq!("Cargo", r.tools[0].def.name);
                if let Ok(p) = r.path_of(0) {
                    assert_eq!("hello/Cargo.toml".to_string(), p);
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
