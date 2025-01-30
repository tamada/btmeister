---
title: ":runner: Usage"
---

## CLI

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
  -i, --ignore-type <IGNORE_TYPE>  specify the ignore type. [default: default] 
                                   [possible values: default, hidden, ignore, git-ignore, git-global, git-exclude]
  -L, --list-defs                  Print the build tools' definition list
  -f, --format <FORMAT>            Specify the output format [default: default] 
                                   [possible values: csv, default, json, markdown, xml, yaml]
  -v, --verbose                    Show verbose output.
  -h, --help                       Print help (see more with '--help')
  -V, --version                    Print version
```

The default build tool definitions are available on the [GitHub repository](https://github.com/tamada/btmeister/blob/main/assets/buildtools.json) ([JSON schema](https://github.com/tamada/btmeister/blob/main/assets/buildtools.json.schema)).

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

[![Docker](https://img.shields.io/badge/Docker-ghcr.io/tamada/btmeister:0.7.3-blue?logo=docker)](https://github.com/tamada/btmeister/pkgs/container/btmeister/)

```sh
docker run --rm -it -v $PWD:/app ghcr.io/tamada/btmeister:latest .
```

- Container OS
  - Working directory: `/app`
  - entry point: `/opt/btmeister/btmeister`
  - user: `nonroot`

## Supported build tools

The btmeister supports the following build tools.

- [Apache Ant](https://ant.apache.org/)
- [Apache Ivy](https://ant.apache.org/ivy/)
- [Apache Maven](https://maven.apache.org/)
- [autoconf](https://www.gnu.org/software/autoconf/)
- [automake](https://www.gnu.org/software/automake/)
- [Bazel](https://bazel.build/)
- [Blade](https://github.com/chen3feng/blade-build)
- [Buck](https://buck.build/)
- [Cake](https://cakebuild.net/)
- [Cmake](https://cmake.org)
- [Cargo](https://www.rust-lang.org)
- [Cargo make](https://sagiegurari.github.io/cargo-make/)
- [Circle CI](https://circleci.com)
- [deno](https://deno.land/)
- [Docker](https://www.docker.com)
- [Docker Compose](https://docs.docker.com/compose/)
- [Earthly](https://earthly.dev/)
- [GitHub Actions](https://github.com/)
- [Go](https://golang.org/)
- [GitLab CI/CD](https://docs.gitlab.com/ee/ci/)
- [Gradle](https://gradle.org/)
- [Grunt](https://gruntjs.com/)
- [Gulp](https://gulpjs.com/)
- [Jenkins](https://www.jenkins.io)
- [just](https://github.com/casey/just)
- [latexmk](https://personal.psu.edu/jcc8/software/latexmk/)
- [llmk](https://github.com/wtsnjp/llmk)
- [Make](https://www.gnu.org/software/make/)
- [Mage](https://magefile.org/)
- [mise](https://mise.jdx.dev)
- [ninja](https://ninja-build.org)
- [npm](https://www.npmjs.com/)
- [Pants](https://www.pantsbuild.org/)
- [please.build](https://please.build/)
- [PyBuilder](https://pybuilder.io/)
- [Rake](https://github.com/ruby/rake)
- [rollup.js](https://rollupjs.org)
- [SCons](https://scons.org)
- [sbt](https://www.scala-sbt.org/index.html)
- [Task](https://taskfile.dev/)
- [Travis](https://www.travis-ci.com)
- [Terraform](https://www.terraform.io)
- [distutils/setuptools/distribution](https://setuptools.pypa.io/en/latest/)
- [vagrant](https://www.vagrantup.com)
- [Webpack](https://webpack.js.org/)
