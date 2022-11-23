# qbt-cli
[![Build And Test](https://github.com/Bpazy/qbt-cli/actions/workflows/build-and-test.yml/badge.svg)](https://github.com/Bpazy/qbt-cli/actions/workflows/build-and-test.yml)
![LICENSE](https://img.shields.io/github/license/Bpazy/qbt-cli)

A cli to manage qBittorrent.

## Install
```
cargo install qbt --git https://github.com/Bpazy/qbt-cli 
```
Or download latest stable release version from [release page](https://github.com/Bpazy/qbt-cli/releases). And put it under the `$PATH`.

Linux example:
```
wget -O /usr/local/bin/qbt https://github.com/Bpazy/qbt-cli/releases/latest/download/qbt-${REPLACE_ME_WITH_VERSION}-linux-amd64
chmod +x /usr/local/bin/qbt
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
### 3. Usage: Get torrent list
```ps1
PS C:\> qbt list -h
Get torrent list

Usage: qbt.exe list [OPTIONS]

Options:
  -f, --filter <FILTER>      Filter torrent list by state. Allowed state filters: all, downloading, seeding, completed, paused, active, inactive, resumed, stalled, stalled_uploading, stalled_downloading, errored
  -c, --category <CATEGORY>  Get torrents with the given category (empty string means "without category"; no "category" parameter means "any category". Remember to URL-encode the category name. For example, My category becomes My%20category
  -t, --tag <TAG>            Get torrents with the given tag (empty string means "without tag"; no "tag" parameter means "any tag". Remember to URL-encode the category name. For example, My tag becomes My%20tag
  -s, --sort <SORT>          Sort torrents by given key. They can be sorted using any field of the response's JSON array (which are documented below) as the sort key
  -r, --reverse <REVERSE>    Enable reverse sorting. Defaults to false [possible values: true, false]
  -l, --limit <LIMIT>        Limit the number of torrents returned
  -o, --offset <OFFSET>      Set offset (if less than 0, offset from end)
      --hashes <HASHES>      Filter by hashes. Can contain multiple hashes separated by |
  -h, --help                 Print help information
```

## Configure
`$HOME/.config/qbt/config.toml`:
```toml
qbittorrent_host="http://qbittorrent.example.host"
username="YOUR_USERNAME"
password="YOUR_PASSWORD"
```
