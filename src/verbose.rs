use std::io::Write;

pub trait Verboser {
    fn log(&mut self, msg: &str);

    fn elog(&mut self, msg: &str);

    fn name(&self) -> String;
}

struct NoVerbose {}

struct VerboseImpl {}

impl Verboser for NoVerbose {
    fn log(&mut self, _: &str) {
        // do nothing
    }

    fn elog(&mut self, _: &str) {
        // do nothing
    }

    fn name(&self) -> String {
        "NoVerbose".to_string()
    }
}

impl Verboser for VerboseImpl {
    fn log(&mut self, msg: &str) {
        println!("{}", msg);
    }

    fn elog(&mut self, msg: &str) {
        eprintln!("{}", msg);
    }

    fn name(&self) -> String {
        "VerboseImpl".to_string()
    }
}

pub fn new(verbose: bool) -> Box<dyn Verboser> {
    if verbose {
        Box::new(VerboseImpl {})
    } else {
        none()
    }
}

pub fn none() -> Box<dyn Verboser> {
    Box::new(NoVerbose {})
}

mod tests {
    use super::*;

    #[test]
    fn test_verbose_out() {
        let verbose = new(true);
        assert_eq!(verbose.name(), "VerboseImpl".to_string())
    }

    #[test]
    fn test_verbose_none() {
        let verbose = new(false);
        assert_eq!(verbose.name(), "NoVerbose".to_string())
    }
}
