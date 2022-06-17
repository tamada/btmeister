---
title: ":runner: Usage"
---

## CLI

```sh
btmeister 0.3.20
Haruaki TAMADA
A tool for detecting build tools of the projects

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
docker run --rm -it -v $PWD:/home/btmeister ghcr.io/tamada/btmeister:latest
```

The working directory in the docker container is `/home/btmeister`.
The target project should be on the directory with `-v` flag of docker.

### Available versions

* `0.3.19`, `latest`