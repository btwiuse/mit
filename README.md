# mit

mit is an MIT license generator

```
$ mit -h
mit 0.1.3
btwiuse <btwiuse@gmail.com>
generate MIT license

USAGE:
    mit [OPTIONS] --author <AUTHOR>

OPTIONS:
    -a, --author <AUTHOR>    Author name
    -h, --help               Print help information
    -V, --version            Print version information
    -y, --year <YEAR>        Year
```

## Install

```
$ cargo install mit
```

## Usage

print license content

```
$ mit --author btwiuse
MIT License

Copyright (c) 2022 btwiuse

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

save to [LICENSE](./LICENSE) file

```
$ mit --author btwiuse > LICENSE
```
