mod btmeister;
mod cli;
mod defs;
mod fmt;
mod verbose;

use std::path::PathBuf;

use btmeister::{BuildTools, Meister};
use clap::Parser;
use cli::Result;
use fmt::Formatter;
use verbose::Verboser;

fn list_defs(
    defs: defs::BuildToolDefs,
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
        Err(cli::MeisterError::Array(errs))
    }
}

fn print_results(r: Vec<BuildTools>, f: &Box<dyn Formatter>) -> Result<()> {
    let mut errs = vec![];
    for bt in r {
        if let Err(e) = print_result(bt, f) {
            errs.push(e);
        }
    }
    if errs.is_empty() {
        Ok(())
    } else {
        Err(cli::MeisterError::Array(errs))
    }
}

fn print_result(r: BuildTools, f: &Box<dyn Formatter>) -> Result<()> {
    let base = r.base.clone();
    println!("{}", r.base.display());
    for bt in r.tools {
        if let Ok(p) = bt.path.strip_prefix(base.clone()) {
            println!("    {}: {}", p.display(), bt.def.name);
        }
    }
    Ok(())
}

fn find_each(meister: &btmeister::Meister, base: PathBuf) -> Result<BuildTools> {
    meister.find(base.clone())
}

fn find_bt(defs: defs::BuildToolDefs, opts: cli::InputOpts) -> Result<Vec<BuildTools>> {
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
                match find_each(&meister, project) {
                    Ok(r) => result.push(r),
                    Err(e) => errs.push(e),
                }
            }
        }
    }
    if errs.is_empty() {
        Ok(result)
    } else {
        Err(cli::MeisterError::Array(errs))
    }
}

fn perform(opts: cli::Options) -> cli::Result<()> {
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

fn print_error(e: cli::MeisterError) {
    use cli::MeisterError::*;
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
