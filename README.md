# btmeister

[![build](https://github.com/tamada/btmeister/actions/workflows/build.yaml/badge.svg)](https://github.com/tamada/btmeister/actions/workflows/build.yaml)
[![Coverage Status](https://coveralls.io/repos/github/tamada/btmeister/badge.svg?branch=main)](https://coveralls.io/github/tamada/btmeister?branch=main)
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/tamada/btmeister)](https://rust-reportcard.xuri.me/report/github.com/tamada/btmeister)

[![Version](https://img.shields.io/badge/Version-v0.4.2-green)](https://github.com/tamada/btmeister/releases/tag/v0.4.2)
[![License](https://img.shields.io/badge/License-MIT-green)](https://github.com/tamada/btmeister/blob/main/LICENSE)
[![Docker](https://img.shields.io/badge/Docker-v0.4.2-green)](https://github.com/tamada/btmeister/pkgs/container/btmeister/)

Detecting the build tools in use.

![btmeister_logo](https://raw.githubusercontent.com/tamada/btmeister/main/site/static/images/logo.png)

## :speaking_head: Description

This tool aims to detect the build tools in use for the project for surveying the share of the build tools.
The build tools build a project along with the rules defined in the build files.
The default names of the build files are fixed for each build tool.
This tool finds the build files from the specified directories, and identifies the build tools in use.

## :runner: Usage

```sh
btmeister v0.4.2
Haruaki TAMADA
A tool for detecting build tools in use of the projects

USAGE:
    btmeister [OPTIONS] [PROJECTs]...

ARGS:
    <PROJECTs>...    The target project directories for btmeister.

OPTIONS:
    -@ <INPUT>                       Specify the file contains project path list. If INPUT is dash
                                     ('-'), read from STDIN.
        --append-defs <DEFS_JSON>    Specify the additional definitions of the build tools.
    -d, --definition <DEFS_JSON>     Specify the definition of the build tools.
    -f, --format <FORMAT>            Specify the output format [default: default] [possible values:
                                     default, json, yaml, xml]
    -h, --help                       Print help information
    -L, --list-defs                  Print the build tools' definition list
        --no-ignore                  Do not respect ignore files (.ignore, .gitignore, etc.)
    -V, --version                    Print version information
```

### Sample Output

```sh
$ btmeister . ~/go/src/github.com/tamada/rrh
cargo       ./Cargo.toml
make        /Users/tamada/go/src/github.com/tamada/rrh/Makefile
$ btmeister --format json . ~/go/src/github.com/tamada/rrh | jq .
[
  {
    "project":"btmeister",
    "path":"./",
    "build-tools":[
      {
        "file-name":"Cargo.toml",
        "tool-name":"cargo"
      }
    ]
  },
  {
    "project":"rrh",
    "path":"/Usrs/tamada/go/src/github.com/tamada/rrh",
    "build-tools":[
      {
        "file-name":"Makefile",
        "tool-name":"make"
      }
    ]
  }
]
```

## :whale: Docker

```sh
docker run --rm -it -v $PWD:/home/btmeister ghcr.io/tamada/btmeister:latest .
```

* Container OS
    * Working directory: `/home/btmeister`
    * entry point: `/opt/btmeister/btmeister`
    * user: `btmeister`


## :hammer_and_wrench: Related Tools

* [Licensee](https://github.com/licensee/licensee)
  * License detector for the projects.
* [linguist](https://github.com/github/linguist)
  * Programming languages detector for the projects.
