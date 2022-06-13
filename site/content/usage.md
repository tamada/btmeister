---
title: ":runner: Usage"
---

## CLI

```sh
btmeister [OPTIONS] <PROJECTs...>
OPTIONS
        --append-defs       additional definitions of build files.
    -d, --definition <BUILD_FILE_DEFS>
                            specify the build file definitions.
    -f, --format <FORMAT>   specify the resultant format. 
                            Available: default, json, yaml, and xml.
    -@ <INPUT>              specify the input file contains project paths.
                            if INPUT is dash ('-'), read from STDIN.
    -h, --help              print this message.
PROJECT
    the target project of btmeister.
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
docker run --rm -v $PWD:/home/btmeister -it ghcr.io/tamada/btmeister:latest
```

The working directory in the docker container is `/home/btmeister`.
The target project should be on the directory with `-v` flag of docker.

### Available versions

* `0.1.3`, `latest`