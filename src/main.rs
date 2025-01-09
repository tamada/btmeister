mod btmeister;
mod cli;
mod defs;
mod fmt;
mod verbose;

use std::path::PathBuf;

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

fn find_each(defs: &defs::BuildToolDefs, project: &PathBuf, opts: &cli::Options) -> Result<()> {
    let mut errs = vec![];

    if errs.len() == 0 {
        Ok(())
    } else {
        Err(cli::MeisterError::Array(errs))
    }
}

fn find_bt(defs: defs::BuildToolDefs, opts: cli::Options) -> Result<()> {
    let mut errs = vec![];
    match opts.projects() {
        Err(e) => return Err(e),
        Ok(projects) => {
            for project in projects {
                match find_each(&defs, &project, &opts) {
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
    let defs = match defs::construct(
        opts.defopts.definition,
        opts.defopts.append_defs,
        &mut verboser,
    ) {
        Err(e) => return Err(e),
        Ok(defs) => defs,
    };
    let formatter = fmt::build_formatter(opts.outputs.format);
    if opts.outputs.list_defs {
        list_defs(defs, formatter, &mut verboser)
    } else {
        Err(cli::MeisterError::NotImplemented)
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
