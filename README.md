# Sagasu

[![Build status](https://ci.appveyor.com/api/projects/status/tm9rhcfsx0qv4y70/branch/master?svg=true)](https://ci.appveyor.com/project/DoumanAsh/sagasu/branch/master)
[![Build Status](https://travis-ci.org/DoumanAsh/sagasu.svg?branch=master)](https://travis-ci.org/DoumanAsh/sagasu)
[![Crates.io](https://img.shields.io/crates/v/sagasu.svg)](https://crates.io/crates/sagasu)
[![Dependency status](https://deps.rs/crate/sagasu/0.3.1/status.svg)](https://deps.rs/crate/sagasu)

Simple utility to find files and/or directories

## Download links

* Windows [32bit](https://github.com/DoumanAsh/sagasu/releases/download/0.3.1/sagasu-0.3.1-i686-pc-windows-msvc.zip)
* Windows [64bit](https://github.com/DoumanAsh/sagasu/releases/download/0.3.1/sagasu-0.3.1-x86_64-pc-windows-msvc.zip)
* Linux [64bit](https://github.com/DoumanAsh/sagasu/releases/download/0.3.1/sagasu-0.3.1-x86_64-unknown-linux-gnu.zip)
* OSX [64bit](https://github.com/DoumanAsh/sagasu/releases/download/0.3.1/sagasu-0.3.1-x86_64-apple-darwin.zip)

## Usage

```
USAGE:
    sagasu.exe [FLAGS] [OPTIONS] <pattern> [path]...

FLAGS:
    -d, --dir        Flag whether to print directories or not. By default is true, if file is not specified
    -f, --file       Flag whether to print executables or not. By default is true, if dir is not specified
    -h, --help       Prints help information
    -m, --machine    Specifies that usage comes from another application. Disables colors.
    -q, --quiet      Ignore errors during search.
    -s, --symlink    Follow symbolic links. By default they are not followed.
    -V, --version    Prints version information

OPTIONS:
        --hop <max_hop>       Specifies depth of recursion.
        --minhop <min_hop>    Minimum number of hops before starting to look. [default: 0]
        --sep <sep>           Specifies separator character between each entry. By default newline

ARGS:
    <pattern>    Pattern to filter by. Allowed types: Regex
    <path>...    Specifies directory where to look. [default: .]
```
