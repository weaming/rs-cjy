# rs-cjy

Convert between CSV, JSON, YAML.

## Install

After installation, you will get two executable binaries named `csv-json`, `json-yaml` and `csv-yaml`.

## Usage

They all accept from STDIN, print to STDOUT, and will try to dentect the input type then print as another type. If you want print pretty JSON, add `PRETTY=1` environment.

## TODO

* [x] keep order of headers
* [x] parse for different field types

