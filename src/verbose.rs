/*!
 * This module provides the simple logging mechanism.
 * `new` function creates a new `Verboser` object.
 * `none` function creates a new `Verboser` object that does nothing.
 */

/// The `Verboser` trait provides the logging mechanism.
pub trait Verboser {
    /// print the given message as a log message.
    fn log(&mut self, msg: &str);

    /// print the given message as an error message.
    fn elog(&mut self, msg: &str);

    #[cfg(test)]
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

    #[cfg(test)]
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

    #[cfg(test)]
    fn name(&self) -> String {
        "VerboseImpl".to_string()
    }
}

/// If gives `true`, it returns a new `Verboser` object that prints the log message.
/// If not, it returns an `Verboser` object that does nothing.
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_verbose_out() {
        let verbose = super::new(true);
        assert_eq!(verbose.name(), "VerboseImpl".to_string())
    }

    #[test]
    fn test_verbose_none() {
        let verbose = super::new(false);
        assert_eq!(verbose.name(), "NoVerbose".to_string())
    }
}
