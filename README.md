# btmeister

[![Version](https://img.shields.io/badge/Version-v1.0.0-green)](https://github.com/tamada/btmeister/releases/tag/v1.0.0)
[![License](https://img.shields.io/badge/License-MIT-green)](https://github.com/tamada/btmeister/blob/main/LICENSE)

Detecting the used build tools in the projects.

## :speaking_head: Description

This tool aims to detect the build tools in use for the project for surveying the share of the build tools.
The build tools build a project along with the rules defined in the build files.
The default names of the build files are fixed for each build tool.
This tool finds the build files from the specified directories, and identifies the build tools in use.

## How to use

### :runner: Usage

```shell
Usage: btmeister [-ahlV] [--append-defs] [--follow-symlink] [--no-ignore]
                 [-d=BUILD_FILE_DEFS] [-f=FORMAT] [--logging-config=FILE]
                 [--loglevel=LEVEL] [PROJECT......]
      [PROJECT......]    target projects. If "-" is given, reads from stdin,
                           and "@" in the first character, reads project list
                           from the file
  -a, --all              read hidden directory and files
      --append-defs      set to additional definitions by --definition option.
                           This option requires --definition option.
  -d, --definition=BUILD_FILE_DEFS
                         specify the definitions of build files.
  -f, --format=FORMAT    specify the output format
      --follow-symlink   follow symbolic links
  -h, --help             Show this help message and exit.
  -l, --list-defs        list definitions and exit.
      --logging-config=FILE
                         specify the configuration file for logging (java.util.
                           logging)
      --loglevel=LEVEL   specify the log level. default is warn. available off,
                           error, warn, info, debug, and all
      --no-ignore        do not respect ignore files (.gitignore).
  -V, --version          Print version information and exit.
```

Type `btmeister <project_dir>`, then the tool prints the build files and its tool names.

#### :whale: Docker image

* native image
  * ![Docker](https://img.shields.io/static/v1?label=Docker&message=ghcr.io/tamada/btmeister/native:1.0.0&color=green&logo=docker)
* Java (minimal JRE)
  * ![Docker](https://img.shields.io/static/v1?label=Docker&message=ghcr.io/tamada/btmeister/java:1.0.0&color=green&logo=docker)

```shell
docker run -it --rm -v $PWD:/app ghcr.io/tamada/btmeister/native:1.0.0 <ARGUMENTS_FOR_BTMEISTER>
```

### :anchor: Install

#### Package manager

* :beer: Homebrew
  * [![Homebrew](https://img.shields.io/badge/Homebrew-tamada/brew/btmeister-green?logo=homebrew)](https://github.com/tamada/homebrew-brew)
  * `brew install tamada/brew/btmeister` (before run the command, run `brew tap tamada/brew`)

#### Locally installing

* native image
  * `btmeister-native-1.0.0.tar.gz`
  * Execution
    * `${BTMEISTER_HOME}/bin/btmeister`
* Java
  * `btmeister-java-1.0.0.tar.gz`
  * Execution
    * `java -jar ${BTMEISTER_HOME}/libs/btmeister-1.0.0.jar`, or
    * `java --module-path ${BTMEISTER_HOME}/libs --module jp.cafebabe.btmeister/jp.cafebabe.btmeister.cli.Main`.

Download the file from above links, then extract it, and place into the certain directory.
The installed directory is called `BTMEISTER_HOME`.  

#### :muscle: Compiling yourself

```shell
git clone https://github.com/tamada/btmeister.git
cd btmeister
gradle build
```

## :fork_and_knife: Tool

### Findable build tools.

`btmeister` can find the following 33 build tools in the default.
However, `btmeister` cannot identify the build tools of Bazel, Blade, and Pants, from the build files.
Because, the names of their build files are all of `BUILD`.
The problem is rest to our future work.
To solve the problem, I just thinks it needs to understand the build files for distinguishing them.

* [Apache Ant](https://ant.apache.org/)
* [Apache Ivy](https://ant.apache.org/ivy/)
* [Apache Maven](https://maven.apache.org/)
* [autoconf](https://www.gnu.org/software/autoconf/)
* [automake](https://www.gnu.org/software/automake/)
* [Bazel](https://bazel.build/)
* [Blade](https://github.com/chen3feng/blade-build)
* [Buck](https://buck.build/)
* [Cake](https://cakebuild.net/)
* [Cargo](https://www.rust-lang.org)
* [Circle CI](https://circleci.com)
* [Docker](https://www.docker.com)
* [Earthly](https://earthly.dev/)
* [GitHub Actions](https://github.com/)
* [GitLab CI/CD](https://docs.gitlab.com/ee/ci/)
* [Gradle](https://gradle.org/)
* [Grunt](https://gruntjs.com/)
* [Gulp](https://gulpjs.com/)
* [Jenkins](https://www.jenkins.io)
* [just](https://github.com/casey/just)
* [latexmk](https://personal.psu.edu/jcc8/software/latexmk/)
* [llmk](https://github.com/wtsnjp/llmk)
* [Make](https://www.gnu.org/software/make/)
* [npm](https://www.npmjs.com/)
* [Pants](https://www.pantsbuild.org/)
* [please.build](https://please.build/)
* [PyBuilder](https://pybuilder.io/)
* [Rake](https://github.com/ruby/rake)
* [sbt](https://www.scala-sbt.org/index.html)
* [Travis](https://www.travis-ci.com)
* [distutils/setuptools/distribution](https://setuptools.pypa.io/en/latest/)
* [vagrant](https://www.vagrantup.com)
* [Webpack](https://webpack.js.org/)

## :smile: About

### License

[![License](https://img.shields.io/badge/License-MIT-green)](https://github.com/tamada/btmeister/blob/main/LICENSE)

### :man_office_worker: Developers :woman_office_worker:

* [Haruaki TAMADA](https://tamada.github.io/) ([tamada](https://github.com/tamada))

### :hammer_and_wrench: Related Tools

* [Licensee](https://github.com/licensee/licensee)
  * License detector for the projects.
* [linguist](https://github.com/github/linguist)
  * Programming languages detector for the projects.

