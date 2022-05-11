use std::path::Path;

pub struct Ignore<'a> {
    files: Vec<gitignore::File<'a>>,
}

impl Ignore<'_> {
    pub fn is_ignore(&self, target: &Path) -> bool {
        for f in self.files {
            if f.is_excluded(target).unwrap() {
                true
            }
        }
        false
    }

    pub fn append(&self, dir: &Path) -> &Ignore {
        let ignore = dir.join(".gitignore");
        if dir.is_dir() && ignore.exists() {
            let file = gitignore::File::new(&ignore).unwrap();
            self.append_impl(&file)
        } else {
            self
        }
    }

    fn append_impl(&self, ignore_file: &gitignore::File) -> &Ignore {
        let mut vec = Vec::new();
        for f in self.files {
            vec.append(f);
        }
        vec.append(ignore_file);
        Ignore{ files: vec }
    }
}

