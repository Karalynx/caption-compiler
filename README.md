# caption-compiler
A CLI tool that allows you to compile and describe Valve's closed captions.

[![Build Status]][Build Link] [![Crates Status]][Crates Link]

[Build Status]: https://github.com/Karalynx/caption-compiler/actions/workflows/build.yml/badge.svg
[Build Link]: https://github.com/Karalynx/caption-compiler/actions
[Crates Status]: https://img.shields.io/crates/v/caption-compiler.svg
[Crates Link]: https://crates.io/crates/caption-compiler

## Install
This tool can be installed via cargo:

```bash
$ cargo install caption-compiler
```

## Usage
```bash
Usage: caption-compiler --input <INPUT> <COMMAND>

Commands:
  compile   Compiles to .DAT file
  describe  Describes .DAT file
  help      Print this message or the help of the given subcommand(s)

Options:
  -i, --input <INPUT>  Input filepath
  -h, --help           Print help
```

## Examples
```bash
$ caption-compiler -i closecaption_english.dat describe

Caption: "<sfx><norepeat:4><clr:255,176,0>[Heavy gun firing]\0"
Hash: 2399413701
Block: 0
Offset: 0
Length: 102

Caption: "<sfx><norepeat:4>[Mudskipper Engine Start]\0"
Hash: 3280962098
Block: 0
Offset: 102
Length: 86

Caption: "<sfx><norepeat:4>[Mudskipper Engine Stop]\0"
Hash: 4243304205
Block: 0
Offset: 188
Length: 84

...
```
