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

impl Meister {
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
    #[test]
    fn test_build_walker() {
    }
}