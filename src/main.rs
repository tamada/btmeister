mod btmeister;
mod cli;
mod defs;
mod fmt;
mod verbose;

use std::path::PathBuf;

use btmeister::{Meister, BuildTools};
use clap::Parser;
use cli::Result;
use fmt::Formatter;
use verbose::Verboser;

fn list_defs(defs: defs::BuildToolDefs, f: Box<dyn Formatter>, _: &mut Box<dyn Verboser>) -> Result<()> {
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

fn print_result(r: BuildTools) {
    let base = r.base.clone();
    println!("{}", r.base.display());
    for bt in r.tools {
        if let Ok(p) = bt.path.strip_prefix(base.clone()) {
            println!("    {}: {}", p.display(), bt.def.name);
        }
    }
}

fn find_each(meister: &btmeister::Meister, base: PathBuf) -> Result<()> {
    let mut errs = vec![];
    match meister.find(base.clone()) {
        Ok(r) => print_result(r),
        Err(e) => errs.push(e),
    }
    if errs.len() == 0 {
        Ok(())
    } else {
        Err(cli::MeisterError::Array(errs))
    }
}

fn find_bt(defs: defs::BuildToolDefs, opts: cli::Options) -> Result<()> {
    let mut errs = vec![];
    let meister = Meister::new(defs, opts.ignore_types.clone());
    match opts.projects() {
        Err(e) => return Err(e),
        Ok(projects) => {
            for project in projects {
                match find_each(&meister, project) {
                    Ok(_) => {}
                    Err(e) => errs.push(e),
                }
            }
        }
    }
    if errs.len() == 0 {
        Ok(())
    } else {
        Err(cli::MeisterError::Array(errs))
    }
}

fn perform(opts: cli::Options) -> cli::Result<()> {
    let mut verboser = verbose::new(opts.verbose);
    let defopts = opts.defopts.clone();
    let defs = match defs::construct(
        defopts.definition,
        defopts.append_defs,
        &mut verboser,
    ) {
        Err(e) => return Err(e),
        Ok(defs) => defs,
    };
    let formatter = fmt::build_formatter(opts.outputs.format);
    if opts.outputs.list_defs {
        list_defs(defs, formatter, &mut verboser)
    } else {
        find_bt(defs, opts)
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