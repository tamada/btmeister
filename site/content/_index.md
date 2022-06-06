---
title: ":house: btmeister"
---

[![build](https://github.com/tamada/btmeister/actions/workflows/build.yaml/badge.svg)](https://github.com/tamada/btmeister/actions/workflows/build.yaml)
[![Coverage Status](https://coveralls.io/repos/github/tamada/btmeister/badge.svg?branch=main)](https://coveralls.io/github/tamada/btmeister?branch=main)
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/tamada/btmeister)](https://rust-reportcard.xuri.me/report/github.com/tamada/btmeister)

[![License](https://img.shields.io/badge/License-MIT-green)](https://github.com/tamada/btmeister/blob/main/LICENSE)

Detecting the build tools in use.

![btmeister_logo](https://raw.githubusercontent.com/tamada/btmeister/main/site/images/logo.png)

## :speaking_head: Description

This tool aims to detect the build tools in use for the project for surveying the share of the build tools.
The build tools build a project along with the rules defined in the build files.
The default names of the build files are fixed for each build tool.
This tool finds the build files from the specified directories, and identifies the build tools in use.

