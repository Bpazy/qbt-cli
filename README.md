# aqu
[![Build And Test](https://github.com/Bpazy/aqu/actions/workflows/build-and-test.yml/badge.svg)](https://github.com/Bpazy/aqu/actions/workflows/build-and-test.yml)

Add qbittorrent url quickly.

## Install
```
cargo install aqu --git https://github.com/Bpazy/aqu 
```

## Usage
```ps1
PS C:\workspace\github.com\bpazy\aqu> aqu --help
Usage: aqu.exe [OPTIONS] <URI>

Arguments:
  <URI>

Options:
  -v, --verbose
  -r, --rename <RENAME>      [default: ]
  -c, --category <CATEGORY>  [default: ]
  -h, --help                 Print help information
  -V, --version              Print version information

```

## Configure
`$HOME/.config/aqu/config.toml`:
```toml
qbittorrent_host="http://qbittorrent.example.host"
username="YOUR_USERNAME"
password="YOUR_PASSWORD"
```
