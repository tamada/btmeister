mod cli;
mod fmt;

use crate::cli::InputOpts;
use crate::fmt::Formatter;
use btmeister::defs::{self, BuildToolDefs};
use btmeister::verbose::{self, Verboser};
use btmeister::Result;
use btmeister::{BuildTools, Meister, MeisterError};
use clap::Parser;

fn list_defs(defs: BuildToolDefs, f: Box<dyn Formatter>, _: &mut Box<dyn Verboser>) -> Result<()> {
    if let Some(header) = f.header_defs() {
        println!("{}", header);
    }
    let mut errs = vec![];
    for (index, def) in defs.iter().enumerate() {
        match f.format_def(def, index == 0) {
            Ok(s) => println!("{}", s),
            Err(e) => errs.push(e),
        }
    }
    if let Some(footer) = f.footer_defs() {
        println!("{}", footer);
    }
    if errs.is_empty() {
        Ok(())
    } else {
        Err(MeisterError::Array(errs))
    }
}

fn print_results(r: Vec<BuildTools>, f: Box<dyn Formatter>) -> Result<()> {
    let mut errs = vec![];
    if let Some(header) = f.header_files() {
        println!("{}", header);
    }
    for (i, bt) in r.iter().enumerate() {
        match f.format_files(bt, i == 0) {
            Ok(s) => print!("{}", s),
            Err(e) => errs.push(e),
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

mod gencomp {
    use crate::cli::Options;
    use btmeister::verbose::Verboser;
    use btmeister::Result;
    use clap::{Command, CommandFactory};
    use clap_complete::Shell;
    use std::fs::File;
    use std::path::{Path, PathBuf};

    fn generate(
        s: Shell,
        app: &mut Command,
        outdir: &Path,
        file: &str,
        v: &mut Box<dyn Verboser>,
    ) -> Result<()> {
        let destfile = outdir.join(file);
        v.log(format!("generate completions for {}: {}", s, destfile.display()).as_str());
        if let Err(e) = std::fs::create_dir_all(destfile.parent().unwrap()) {
            return Err(btmeister::MeisterError::Io(e));
        }
        match File::create(destfile) {
            Ok(mut dest) => {
                clap_complete::generate(s, app, "btmeister", &mut dest);
                Ok(())
            }
            Err(e) => Err(btmeister::MeisterError::Io(e)),
        }
    }

    pub(crate) fn generate_completions(outdir: PathBuf, v: &mut Box<dyn Verboser>) -> Result<()> {
        let mut app = Options::command();
        app.set_bin_name("btmeister");
        let mut errs = vec![];
        let shells = vec![
            (Shell::Bash, "bash/btmeister"),
            (Shell::Elvish, "elvish/btmeister"),
            (Shell::Fish, "fish/btmeister"),
            (Shell::PowerShell, "powershell/btmeister"),
            (Shell::Zsh, "zsh/_btmeister"),
        ];
        for (shell, file) in shells {
            if let Err(e) = generate(shell, &mut app, &outdir, file, v) {
                errs.push(e);
            }
        }
        if errs.is_empty() {
            Ok(())
        } else {
            Err(btmeister::MeisterError::Array(errs))
        }
    }
}

fn perform(opts: cli::Options) -> Result<()> {
    let mut verboser = verbose::new(opts.verbose);
    let (input_opts, output_opts, defopts, compopts) =
        (opts.inputs, opts.outputs, opts.defopts, opts.compopts);
    let defs = match defs::construct(defopts.definition, defopts.append_defs, &mut verboser) {
        Err(e) => return Err(e),
        Ok(defs) => defs,
    };
    if compopts.completion {
        gencomp::generate_completions(compopts.dest, &mut verboser)
    } else {
        let formatter = fmt::build_formatter(output_opts.format);
        if output_opts.list_defs {
            list_defs(defs, formatter, &mut verboser)
        } else {
            match find_bt(defs, input_opts) {
                Ok(r) => print_results(r, formatter),
                Err(e) => Err(e),
            }
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

fn rust_main(args: Vec<String>) -> Result<()> {
    let opts = cli::Options::parse_from(args);
    perform(opts)
}

fn main() {
    match rust_main(std::env::args().collect()) {
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

    #[test]
    fn test_success() {
        let r = rust_main(
            vec!["btmeister", "testdata/fibonacci", "--format", "json"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        assert!(r.is_ok());
    }

    #[test]
    fn test_success_list_defs() {
        let r = rust_main(
            vec!["btmeister", "testdata/fibonacci", "--list-defs", "--format", "json" ]
            .iter().map(|s| s.to_string())
            .collect(),
        );
        assert!(r.is_ok());
    }

    #[test]
    fn test_project_not_found() {
        let r = rust_main(
            vec!["btmeister", "unknown/project"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        assert!(r.is_err());
    }

    #[test]
    fn test_gencomp() {
        use std::path::PathBuf;
        let r = rust_main(
            vec![
                "btmeister",
                "--generate-completion-files",
                "--completion-out-dir",
                "testgencomp",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        );
        assert!(r.is_ok());
        assert!(PathBuf::from("testgencomp/bash/btmeister").exists());
        assert!(PathBuf::from("testgencomp/elvish/btmeister").exists());
        assert!(PathBuf::from("testgencomp/fish/btmeister").exists());
        assert!(PathBuf::from("testgencomp/powershell/btmeister").exists());
        assert!(PathBuf::from("testgencomp/zsh/_btmeister").exists());

        std::fs::remove_dir_all("testgencomp").unwrap();
    }
}
