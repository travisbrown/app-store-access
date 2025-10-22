# app-store-access

[![Rust build status](https://img.shields.io/github/actions/workflow/status/travisbrown/app-store-access/ci.yaml?branch=main)](https://github.com/travisbrown/app-store-access/actions)
[![Coverage status](https://img.shields.io/codecov/c/github/travisbrown/app-store-access/main.svg)](https://codecov.io/github/travisbrown/app-store-access)

A set of [Rust][rust] libraries for working with the Google and Apple app stores.

## Overview

This project includes four pieces for each of the Google and Apple app store APIs:

1. An API client.
2. A data model representing API requests and responses as Rust definitions.
3. A data store for collecting requests and responses.
4. A command-line tool for making requests and saving API interactions to the data store.

It does not include any kind of index for the data.

At some point these four pieces will probably be split out into three separate packages, but while
the project is under active development, there are only three packages in total in this repository
(one for shared code, and one for each app store).

## Limitations

This project is under active development, and deserialization is designed to fail fast. If for
example a Google Play API response includes an unknown review criterion field, the response will be
saved to the data store, and the operation will then fail with an error. This approach is intended
to promote quick iteration, so that we know immediately when the API is returning data that we
haven't modeled, and can update our models.

## Quick start

You will need to have [installed][rust-installation] Rust and Cargo on your system for
anything in this repository to work.

First you need to build the project:

```bash
$ cargo build --release
```

Then you can run commands:

```bash
$ target/release/apple-scraper -vvv api search --query chess
07:41:52 [INFO] Downloading 213 search results
329218549,com.chess.iphone,329218552,Chess - Play & Learn Online,Chess.com
311395856,R6WLZBVJLW.ChessFree,289278460,Chess ∙,Optime Software LLC
1351939098,com.eivaagames.RealChess3DFree,705287622,Real Chess 3D,Sailendu Behera
319305999,com.byterun.chess,1453865533,Chess,Vintolo Ltd
1591188795,com.gamovation.chessclubpilot,1263417065,Chess - Offline Board Game,GamoVation
...

$ target/release/google-scraper -vv api search --query chess
com.chess,5068259210636929122,Chess - Play and Learn Online,Chess.com
com.gamovation.chessclubpilot,6969432818927594171,Chess - Offline Board Game,GamoVation
com.splendapps.checkmate,8546574316005690870,Chess Online & Offline,Splend Apps
net.kusik.wristchess,Lukas+Kusik,Wrist Chess,Lukas Kusik
com.jetstartgames.chess,Chess+Prince,Chess,Chess Prince
...

$ target/release/apple-scraper -vvv api suggest --query chess --country de
chess,en,de,chess.com
chess,en,de,chess
chess,en,de,chess clock
chess,en,de,chess online
chess,en,de,chess universe
chess,en,de,chessable
chess,en,de,chess clash
chess,en,de,chess club
chess,en,de,chess timer
chess,en,de,chess offline

$ target/release/google-scraper -vvv api suggest --query chess
chess,en,us,chess game
chess,en,us,chess timer
chess,en,us,chess puzzles
chess,en,us,chess free
chess,en,us,chess offline
```

Most commands will save a record of the request and response to `data/apple` or `data/google`.
The autocomplete suggestion commands are an exception, since in both cases they are treated
differently by the APIs (for example the Apple API returns a p-list instead of JSON).

Most commands will print a summary of the response to standard out as comma-separated rows.
This output is intended primarily for spot-checking and quick reference. In most cases you should
be working with the data by ingesting the contents of the `data` directories into an index (not
provided in this repository).

Note that the options and CSV output for the two platforms are not identical. For example
every Google request requires both a country and a language (possibly provided as an implicit
default), but this is not the case for Apple. In the output above, you can see that the Apple
app rows include both a numeric ID and a "bundle ID" (a reverse domain name), while Google apps
only have a single ID (formatted as a reverse domain name).

## Command-line reference

For Apple:

```
$ target/release/apple-scraper -h
Usage: apple-scraper [OPTIONS] <COMMAND>

Commands:
  api    Make API requests and save responses
  store  Read data from the local store
  help   Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Level of verbosity
  -h, --help        Print help
  -V, --version     Print version
```

And for the `api` sub-command for Apple:

```
$ target/release/apple-scraper api -h
Make API requests and save responses

Usage: apple-scraper api [OPTIONS] <COMMAND>

Commands:
  app                Request details for an app by ID
  search             Perform a search for a given query string
  lookup-ids         Look up apps or developers by ID (option can be provided multiple times)
  lookup-bundle-ids  Look up apps by bundle ID (option can be provided multiple times)
  reviews            Look up reviews for an app by ID
  ratings            Look up ratings by app ID (warning: raw HTML)
  suggest            Look up autocomplete suggestions for a given query string
  help               Print this message or the help of the given subcommand(s)

Options:
      --archive <ARCHIVE>  [default: data/apple/]
  -v, --verbose...         Level of verbosity
  -h, --help               Print help
```

For Google:

```
$ target/release/google-scraper -h
Usage: google-scraper [OPTIONS] <COMMAND>

Commands:
  api    Make API requests and save responses
  store  Read data from the local store
  help   Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Level of verbosity
  -h, --help        Print help
  -V, --version     Print version
```

And for the `api` sub-command for Google:

```
$ target/release/google-scraper api -h
Make API requests and save responses

Usage: google-scraper api [OPTIONS] <COMMAND>

Commands:
  app        Request details for an app by ID
  search     Perform a search for a given query string
  developer  Request a list of apps for a developer by ID (may be an integer or a string)
  reviews    Look up reviews for an app by ID
  suggest    Look up autocomplete suggestions for a given query string
  help       Print this message or the help of the given subcommand(s)

Options:
      --country <COUNTRY>  [default: us]
  -v, --verbose...         Level of verbosity
      --lang <LANG>        [default: en]
      --archive <ARCHIVE>  [default: data/google/]
  -h, --help               Print help
```

## License

This software is licensed under the [GNU General Public License v3.0][gpl-v3] (GPL-3.0).

[gpl-v3]: https://www.gnu.org/licenses/gpl-3.0.en.html
[rust]: https://rust-lang.org/
[rust-installation]: https://doc.rust-lang.org/cargo/getting-started/installation.html
