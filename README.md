# btmeister

Build tools detector for the projects.

![btmeister_logo](https://raw.githubusercontent.com/tamada/btmeister/main/docs/images/logo.png)

## :speaking_head: Description

This tool aims to detect the build tools in use for the project for surveying the share of the build tools.
The build tools build a project along with the rules defined in the build files.
The default names of the build files are fixed for each build tool.
This tool finds the build files from the specified directories, and identifies the build tools in use.

## :runner: Usage

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
$ btmeister btmeister ~/go/src/github.com/tamada/rrh
cargo       btmeister/Cargo.toml
make        /Users/tamada/go/src/github.com/tamada/rrh/Makefile
$ btmeister --format json btmeister rrh | jq .
[
  {
    "project":"btmeister",
    "path":"./btmeister",
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


## :hammer_and_wrench: Related Tools

* [Licensee](https://github.com/licensee/licensee)
  * License detector for the projects.
* [linguist](https://github.com/github/linguist)
  * Programming languages detector for the projects.