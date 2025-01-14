---
title: ":runner: Usage"
---

## CLI

```sh
Detecting build tools/task runners in use of the projects

Usage: btmeister [OPTIONS] [PROJECTs]...

Arguments:
  [PROJECTs]...  The target project paths. If "-" was given, reads from stdin,
                 and "@" was put on the first character, read from the file.

Options:
  -D, --definition <DEFS_JSON>     Specify the definition of the build tools.
      --append-defs <DEFS_JSON>    Specify the additional definitions of the build tools.
  -i, --ignore-type <IGNORE_TYPE>  specify the ignore type. [default: default] 
                                   [possible values: default, hidden, ignore, git-ignore, git-global, git-exclude]
  -L, --list-defs                  Print the build tools' definition list
  -f, --format <FORMAT>            Specify the output format [default: default]
                                   [possible values: csv, default, json, xml, yaml]
  -v, --verbose                    Show verbose output.
  -h, --help                       Print help (see more with '--help')
  -V, --version                    Print version
```

### Sample Output

```sh
$ btmeister ~/github.com/tamada/gibo-wrapper
/Users/tamada/github.com/tamada/gibo-wrapper
    Cargo.toml: Cargo
    Dockerfile: Docker
    build.rs: Cargo
$ btmeister --format json ~/github.com/tamada/gibo-wrapper | jq .
[
  {
    "base": "/Users/tamada/products/gibo-wrapper",
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
      }
    ]
  }
]
```

## :whale: Docker

[![Docker](https://img.shields.io/badge/Docker-ghcr.io/tamada/btmeister:0.6.5-blue?logo=docker)](https://github.com/tamada/btmeister/pkgs/container/btmeister/)

```sh
docker run --rm -it -v $PWD:/app ghcr.io/tamada/btmeister:latest .
```

* Container OS
  * Working directory: `/app`
  * entry point: `/opt/btmeister/btmeister`
  * user: `nonroot`

### Available versions

* latest, 0.6.0
* [0.5.0](https://github.com/tamada/btmeister/pkgs/container/btmeister/26088262?tag=0.5.0)
