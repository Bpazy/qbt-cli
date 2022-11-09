# qbt-cli
[![Build And Test](https://github.com/Bpazy/qbt-cli/actions/workflows/build-and-test.yml/badge.svg)](https://github.com/Bpazy/qbt-cli/actions/workflows/build-and-test.yml)
![LICENSE](https://img.shields.io/github/license/Bpazy/qbt-cli)

A cli to manage qBittorrent

## Install
```
cargo install qbt --git https://github.com/Bpazy/qbt-cli 
```

## Usage
```ps1
PS C:\> qbt  -h
Usage: qbt.exe [OPTIONS] <COMMAND>

Commands:
  add
  list
  help  Print this message or the help of the given subcommand(s)

Options:
      --verbose
  -h, --help     Print help information
  -V, --version  Print version information
```

## Configure
`$HOME/.config/qbt/config.toml`:
```toml
qbittorrent_host="http://qbittorrent.example.host"
username="YOUR_USERNAME"
password="YOUR_PASSWORD"
```
