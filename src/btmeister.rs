use std::collections::HashMap;
use std::path::PathBuf;

use crate::cli::Result;
use crate::defs::{BuildToolDef, BuildToolDefs};

pub struct Meister {
    defs: BuildToolDefs,
}

pub struct MeisterOpts {
    pub respect_ignore: bool,
}

impl Meister {
    pub fn new(defs: BuildToolDefs) -> Self {
        Self { defs }
    }

    pub fn traverse(base: PathBuf) -> Result<HashMap<PathBuf, BuildToolDef>> {
        let mut result = HashMap::new();

        Ok(result)
    }
}
