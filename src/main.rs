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
    use std::io::Write;
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
    let _ = std::io::stdout().flush();
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

#[cfg(debug_assertions)]
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
            return Err(btmeister::MeisterError::IO(e));
        }
        match File::create(destfile) {
            Ok(mut dest) => {
                clap_complete::generate(s, app, "btmeister", &mut dest);
                Ok(())
            }
            Err(e) => Err(btmeister::MeisterError::IO(e)),
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
    let (input_opts, output_opts, defopts) = (opts.inputs, opts.outputs, opts.defopts);
    #[cfg(debug_assertions)]
    let compopts = opts.compopts;
    let defs = match defs::construct(defopts.definition, defopts.append_defs, &mut verboser) {
        Err(e) => return Err(e),
        Ok(defs) => defs,
    };
    if cfg!(debug_assertions) {
        #[cfg(debug_assertions)]
        if compopts.completion {
            return gencomp::generate_completions(compopts.dest, &mut verboser);
        }
    }
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

fn errors_to_string(e: MeisterError) -> String {
    use MeisterError::*;
    match e {
        Array(errs) => errs
            .into_iter()
            .map(errors_to_string)
            .collect::<Vec<String>>()
            .join("\n"),
        Fatal(m) => format!("fatal: {}", m),
        IO(e) => format!("io error: {}", e),
        Json(e) => format!("parse error: {}", e),
        NotImplemented => "not implemented yet.".to_string(),
        NotProject(file) => format!("{}: not project", file),
        NoProjectSpecified() => "no project specified.".to_string(),
        ProjectNotFound(p) => format!("{}: project not found", p),
        UnsupportedArchiveFormat(f) => format!("{}: unsupported archive format", f),
        Warning(m) => format!("warning: {}", m),
    }
}

fn print_error(e: MeisterError) {
    println!("{}", errors_to_string(e));
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
    fn test_error_message() {
        use MeisterError::*;
        assert_eq!("fatal: test", errors_to_string(Fatal("test".to_string())));
        assert_eq!(
            "io error: test",
            errors_to_string(IO(std::io::Error::new(std::io::ErrorKind::Other, "test",)))
        );
        assert_eq!(
            "parse error: missing field `test`",
            errors_to_string(Json(serde::de::Error::missing_field("test")))
        );
        assert_eq!("not implemented yet.", errors_to_string(NotImplemented));
        assert_eq!(
            "no project specified.",
            errors_to_string(NoProjectSpecified())
        );
        assert_eq!(
            "test: project not found",
            errors_to_string(ProjectNotFound("test".to_string()))
        );
        assert_eq!(
            "test: unsupported archive format",
            errors_to_string(UnsupportedArchiveFormat("test".to_string())),
        );
        assert_eq!(
            "warning: test",
            errors_to_string(Warning("test".to_string()))
        );
        assert_eq!(
            "fatal: test\nfatal: test2",
            errors_to_string(Array(vec![
                Fatal("test".to_string()),
                Fatal("test2".to_string())
            ]))
        );
    }

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
            vec![
                "btmeister",
                "testdata/fibonacci",
                "--list-defs",
                "--format",
                "json",
            ]
            .iter()
            .map(|s| s.to_string())
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
