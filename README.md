# btmeister

[![build](https://github.com/tamada/btmeister/actions/workflows/build.yaml/badge.svg)](https://github.com/tamada/btmeister/actions/workflows/build.yaml)
[![Coverage Status](https://coveralls.io/repos/github/tamada/btmeister/badge.svg?branch=main)](https://coveralls.io/github/tamada/btmeister?branch=main)
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/tamada/btmeister)](https://rust-reportcard.xuri.me/report/github.com/tamada/btmeister)

[![Version](https://img.shields.io/badge/Version-v0.7.4-green)](https://github.com/tamada/btmeister/releases/tag/v0.7.4)
[![License](https://img.shields.io/badge/License-MIT-green)](https://github.com/tamada/btmeister/blob/main/LICENSE)

[![Docker](https://img.shields.io/badge/Docker-ghcr.io/tamada/btmeister:0.7.4-blue?logo=docker)](https://github.com/tamada/btmeister/pkgs/container/btmeister/)
[![Homebrew](https://img.shields.io/badge/Homebrew-tamada/tap/btmeister-blue?logo=homebrew)](https://github.com/tamada/homebrew-tap)

Detecting the build tools in use.

![btmeister_logo](https://raw.githubusercontent.com/tamada/btmeister/main/site/static/images/logo.png)

## :speaking_head: Description

This tool aims to detect the build tools in use for the project for surveying the share of the build tools.
The build tools build a project along with the rules defined in the build files.
The default names of the build files are fixed for each build tool.
This tool finds the build files from the specified directories, and identifies the build tools in use.

## :runner: Usage

```sh
Detecting build tools/task runners in use of the projects

Usage: btmeister [OPTIONS] [PROJECTs]...

Arguments:
  [PROJECTs]...  The target project paths. If "-" was given, reads from stdin.
                 Also, the first character was "@", read from the file eliminating "@".
                 This parameters accept directories and archive files.
                 Supported archive files: tar, tar.bz2, tar.gz, tar.xz, tar.zstd, and zip.

Options:
  -D, --definition <DEFS_JSON>     Specify the definition of the build tools.
      --append-defs <DEFS_JSON>    Specify the additional definitions of the build tools.
  -i, --ignore-type <IGNORE_TYPE>  Specify the ignore type. [default: default] [possible values: default, hidden, ignore, git-ignore, git-global, git-exclude]
  -e, --excludes <EXCLUDEs>        Specify the filters of excluding files or directories.
  -L, --list-defs                  Print the build tools' definition list
  -f, --format <FORMAT>            Specify the output format [default: default] [possible values: csv, default, json, markdown, xml, yaml]
  -l, --level <LEVEL>              Specify the log level. [default: warn] [possible values: error, warn, info, debug, trace]
  -h, --help                       Print help (see more with '--help')
  -V, --version                    Print version
```

### Sample Output

```sh
$ btmeister ~/github.com/tamada/gibo-wrapper
/home/tamada/github.com/tamada/gibo-wrapper
    Cargo.toml: Cargo
    Dockerfile: Docker
    build.rs: Cargo
    .github/workflows/publish.yaml: GitHub Actions
    .github/workflows/docker.yaml: GitHub Actions
    .github/workflows/versionup.yaml: GitHub Actions
    .github/workflows/build.yaml: GitHub Actions
$ btmeister --format json ~/github.com/tamada/gibo-wrapper | jq .
[
  {
    "base": "/home/tamada/github.com/tamada/gibo-wrapper",
    "build-tools": [
      {
        "path": "Cargo.toml",
        "tool-name": "Cargo"
      },
      {
        "path": "Dockerfile",
        "tool-name": "Docker"
      },
      {
        "path": "build.rs",
        "tool-name": "Cargo"
      },
      {
        "path": ".github/workflows/publish.yaml",
        "tool-name": "GitHub Actions"
      },
      {
        "path": ".github/workflows/docker.yaml",
        "tool-name": "GitHub Actions"
      },
      {
        "path": ".github/workflows/versionup.yaml",
        "tool-name": "GitHub Actions"
      },
      {
        "path": ".github/workflows/build.yaml",
        "tool-name": "GitHub Actions"
      }
    ]
  }
]
```

## :whale: Docker

[![Docker](https://img.shields.io/badge/Docker-ghcr.io/tamada/btmeister:0.7.4-blue?logo=docker)](https://github.com/tamada/btmeister/pkgs/container/btmeister/)

```sh
docker run --rm -it -v $PWD:/app ghcr.io/tamada/btmeister:latest .
```

* Container OS
  * Working directory: `/app`
  * entry point: `/opt/btmeister/btmeister`
  * user: `nonroot`

## :hammer_and_wrench: Related Tools

* [Licensee](https://github.com/licensee/licensee)
  * License detector for the projects.
* [linguist](https://github.com/github/linguist)
  * Programming languages detector for the projects.
