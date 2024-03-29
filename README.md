mit
===

[![crates.io](https://img.shields.io/crates/v/mit.svg)](https://crates.io/crates/mit)
[![Documentation](https://docs.rs/mit/badge.svg)](https://docs.rs/mit)
[![Build Status](https://travis-ci.org/btwiuse/mit.svg?branch=master)](https://travis-ci.org/btwiuse/mit)

mit is an MIT{,-0} license generator

```
$ mit --help
generate MIT{,-0} license

Usage: mit [OPTIONS] --author <AUTHOR>

Options:
  -0, --zero             Use MIT-0 variant
  -y, --year <YEAR>      Set year [optional]
  -a, --author <AUTHOR>  Set author name
  -h, --help             Print help
  -V, --version          Print version
```

## Install

```
$ cargo install mit
```

## Usage

Print license content

```
$ mit --author btwiuse
MIT License

Copyright (c) btwiuse

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

Use [MIT-0](https://github.com/aws/mit-0) variant

```
$ mit -0 --author btwiuse
MIT No Attribution

Copyright (c) btwiuse

Permission is hereby granted, free of charge, to any person obtaining a copy of this
software and associated documentation files (the "Software"), to deal in the Software
without restriction, including without limitation the rights to use, copy, modify,
merge, publish, distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
```

Save to LICENSE file

```
$ mit --author btwiuse > LICENSE
```
