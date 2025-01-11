use std::path::{Path, PathBuf};

use crate::cli::{IgnoreType, MeisterError, Result};
use crate::defs::{BuildToolDef, BuildToolDefs};

pub struct Meister {
    defs: BuildToolDefs,
    its: Vec<IgnoreType>,
}

pub struct BuildTools<'a> {
    pub base: PathBuf,
    pub tools: Vec<BuildTool<'a>>,
}

pub struct BuildTool<'a> {
    pub path: PathBuf,
    pub def: &'a BuildToolDef,
}

impl<'a> BuildTools<'a> {
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
            Ok(r) => Ok(Meister::new(r, vec![IgnoreType::Default])),
            Err(e) => Err(e),
        }
    }

    pub fn new(defs: BuildToolDefs, its: Vec<IgnoreType>) -> Self {
        Self { defs: defs, its: its }
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
                },
                Err(e) => 
                    errs.push(MeisterError::Warning(format!("walking: {}", e))),
            }
        }
        if errs.len() == 0 {
            Ok(BuildTools{ base: base, tools: result })
        } else {
            Err(MeisterError::Array(errs))
        }
    }
}

fn find_build_tool<'a>(meister: &'a Meister, path: &Path) -> Option<BuildTool<'a>> {
    for def in meister.defs.iter() {
        let pb = path.to_path_buf();
        if def.matches(&pb) {
            return Some(BuildTool{ path: pb, def: def});
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::Meister;

    #[test]

    fn test_build_walker() {
        if let Ok(meister) = Meister::new_as_default() {
            let r = meister.find(PathBuf::from("testdata/fibonacci"));
            assert!(r.is_ok());
            if let Ok(r) = r {
                assert_eq!(1, r.tools.len());
                assert_eq!("Gradle",  r.tools[0].def.name);
                if let Ok(p) = r.path_of(0) {
                    assert_eq!("build.gradle".to_string(), p);
                }
            }
        }
    }
}