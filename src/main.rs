mod fmt;
mod cli;

use clap::Parser;
use btmeister::{BuildTools, Meister, MeisterError};
use btmeister::defs::{self, BuildToolDefs};
use btmeister::Result;
use btmeister::verbose::{self, Verboser};
use crate::fmt::Formatter;
use crate::cli::InputOpts;

fn list_defs(
    defs: BuildToolDefs,
    f: Box<dyn Formatter>,
    _: &mut Box<dyn Verboser>,
) -> Result<()> {
    if let Some(header) = f.header_defs() {
        println!("{}", header);
    }
    let mut errs = vec![];
    for (index, def) in defs.iter().enumerate() {
        match f.format_def(&def, index == 0) {
            Ok(s) => println!("{}", s),
            Err(e) => errs.push(e),
        }
    }
    if let Some(footer) = f.footer_defs() {
        println!("{}", footer);
    }
    if errs.len() == 0 {
        Ok(())
    } else {
        Err(MeisterError::Array(errs))
    }
}

fn print_results(r: Vec<BuildTools>, f: &Box<dyn Formatter>) -> Result<()> {
    let mut errs = vec![];
    if let Some(header) = f.header_files() {
        println!("{}", header);
    }
    for (i, bt) in r.iter().enumerate() {
        if let Err(e) = f.format_files(&bt.base, &bt.tools, i == 0) {
            errs.push(e);
        }
    }
    if let Some(footer) = f.footer_files() {
        println!("{}", footer);
    }
    if errs.is_empty() {
        Ok(())
    } else {
        Err(MeisterError::Array(errs))
    }
}

fn find_bt(defs: BuildToolDefs, opts: InputOpts) -> Result<Vec<BuildTools>> {
    let meister = match Meister::new(defs, opts.ignore_types.clone()) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };
    let mut errs = vec![];
    let mut result = vec![];
    match opts.projects() {
        Err(e) => return Err(e),
        Ok(projects) => {
            for project in projects {
                match meister.find(project) {
                    Ok(r) => result.push(r),
                    Err(e) => errs.push(e),
                }
            }
        }
    }
    if errs.is_empty() {
        Ok(result)
    } else {
        Err(MeisterError::Array(errs))
    }
}

fn perform(opts: cli::Options) -> Result<()> {
    let mut verboser = verbose::new(opts.verbose);
    let (input_opts, output_opts, defopts) = (opts.inputs, opts.outputs, opts.defopts);
    let defs = match defs::construct(defopts.definition, defopts.append_defs, &mut verboser) {
        Err(e) => return Err(e),
        Ok(defs) => defs,
    };
    let formatter = fmt::build_formatter(output_opts.format);
    if output_opts.list_defs {
        list_defs(defs, formatter, &mut verboser)
    } else {
        match find_bt(defs, input_opts) {
            Ok(r) => print_results(r, &formatter),
            Err(e) => Err(e),
        }
    }
}

fn print_error(e: MeisterError) {
    use MeisterError::*;
    match e {
        Array(errs) => {
            for e in errs {
                print_error(e);
            }
        }
        Fatal(m) => eprintln!("Fatal: {}", m),
        Io(e) => eprintln!("IO Error: {}", e),
        Json(e) => eprintln!("Parse Error: {}", e),
        NotImplemented => eprintln!("Not implemented yet."),
        NoProjectSpecified() => eprintln!("No project specified."),
        ProjectNotFound(p) => eprintln!("Project not found: {}", p),
        Warning(m) => eprintln!("Warning: {}", m),
    }
}

fn main() {
    let opts = cli::Options::parse();
    match perform(opts) {
        Ok(_) => {}
        Err(e) => {
            print_error(e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
