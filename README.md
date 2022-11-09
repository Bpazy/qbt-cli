# qbt-cli
[![Build And Test](https://github.com/Bpazy/qbt-cli/actions/workflows/build-and-test.yml/badge.svg)](https://github.com/Bpazy/qbt-cli/actions/workflows/build-and-test.yml)
![LICENSE](https://img.shields.io/github/license/Bpazy/qbt-cli)

A cli to manage qBittorrent

## Install
```
cargo install qbt --git https://github.com/Bpazy/qbt-cli 
```

## Usage
### 1. Usage: Overview
```ps1
PS C:\> qbt -h
Usage: qbt.exe [OPTIONS] <COMMAND>

Commands:
  add   Add new torrent
  list  Get torrent list
  help  Print this message or the help of the given subcommand(s)

Options:
      --verbose  Use verbose output
  -h, --help     Print help information
  -V, --version  Print version information
```
### 2. Usage: Add new torrent
```ps1
PS C:\> qbt add -h
Add new torrent

Usage: qbt.exe add [OPTIONS] <URI>

Arguments:
  <URI>  URLs separated with newlines

Options:
      --savepath <SAVEPATH>
          Download folder
      --cookie <COOKIE>
          Cookie sent to download the .torrent file
      --category <CATEGORY>
          Category for the torrent
      --tags <TAGS>
          Tags for the torrent, split by ','
      --skip-checking <SKIP_CHECKING>
          Skip hash checking. Possible values are true, false (default)
      --paused <PAUSED>
          Add torrents in the paused state. Possible values are true, false (default)
      --root-folder <ROOT_FOLDER>
          Create the root folder. Possible values are true, false, unset (default)
      --rename <RENAME>
          Rename torrent
      --up-limit <UP_LIMIT>
          Set torrent upload speed limit. Unit in bytes/second
      --dl-limit <DL_LIMIT>
          Set torrent download speed limit. Unit in bytes/second
      --ratio-limit <RATIO_LIMIT>
          Set torrent share ratio limit
      --seeding-time-limit <SEEDING_TIME_LIMIT>
          Set torrent seeding time limit. Unit in minutes
      --auto-tmm <AUTO_TMM>
          Whether Automatic Torrent Management should be used [possible values: true, false]
      --sequential-download <SEQUENTIAL_DOWNLOAD>
          Enable sequential download. Possible values are true, false (default)
      --first-last-piece-prio <FIRST_LAST_PIECE_PRIO>
          Prioritize download first last piece. Possible values are true, false (default)
  -h, --help
          Print help information
```
### Usage: Get torrent list
```ps1
PS C:\> qbt list -h
Get torrent list

Usage: qbt.exe list [OPTIONS]

Options:
  -f, --filter <FILTER>      
  -c, --category <CATEGORY>  
  -t, --tag <TAG>            
  -s, --sort <SORT>          
  -r, --reverse <REVERSE>    [possible values: true, false]
  -l, --limit <LIMIT>        
  -o, --offset <OFFSET>      
      --hashes <HASHES>      
  -h, --help                 Print help information
```

## Configure
`$HOME/.config/qbt/config.toml`:
```toml
qbittorrent_host="http://qbittorrent.example.host"
username="YOUR_USERNAME"
password="YOUR_PASSWORD"
```
