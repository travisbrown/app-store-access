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

## Combined interface

There is also a simplified interface that allows you to run searches against both stores with a
single command.

This interface will provide a unified CSV view, with an initial column indicating the platform.

```bash
$ target/release/app-store-access-cli -vvv api suggest --query clothes
apple,clothes,en,us,clothes shopping
apple,clothes,en,us,clothesline mobile
apple,clothes,en,us,clothes designer
apple,clothes,en,us,clothes remover
apple,clothes,en,us,clothes
apple,clothes,en,us,clothes selling apps
apple,clothes,en,us,ai clothes changer
apple,clothes,en,us,clothes mentor
apple,clothes,en,us,clothes changer
apple,clothes,en,us,clothes finder
google,clothes,en,us,clothes shopping apps
google,clothes,en,us,clothes skin for roblox
google,clothes,en,us,clothes remover scanner photo
google,clothes,en,us,clothes app
google,clothes,en,us,clothes designer app
```

Similarly for search, where the term is also included:

```bash
$ target/release/app-store-access-cli -vvv api search --query clothing
apple,clothing,518684914,com.garageitaly.garage,929687794,Depop - Buy & Sell Clothes,Depop Ltd
apple,clothing,383915209,com.abercrombie.hollister,339041770,Hollister Co.,Abercrombie & Fitch
apple,clothing,1367363988,com.tapcart.npQt0DXZpj,1363591855,Fashion Nova: Trendy Shopping,Fashion Nova Inc.
apple,clothing,437373394,com.pacsun.pacsun,437373397,Pacsun,Pacific Sunwear
apple,clothing,358821736,com.uo.StoreLocator,358821739,Urban Outfitters,Urban Outfitters
apple,clothing,834465911,com.hm.goe,380487412,H&M,H&M
apple,clothing,342792281,com.gap.shopon,296279554,Old Navy: Shop for New Clothes,Gap Inc.
apple,clothing,547951480,com.inditex.zara.iphone,341323285,ZARA,"Industria de Diseno Textil, S.A."
apple,clothing,1588252721,com.tapcart.Gftde68uJs,1588252723,edikted,Edikted
apple,clothing,326347260,com.gap.gapdenim,296279554,"Gap: Apparel, denim and more",Gap Inc.
...
google,clothing,,com.louisphilippe.abfrl,ABFRL+E-commerce,Louis Philippe Shopping App,ABFRL E-commerce
google,clothing,,com.house.jjs,JJ's+House,JJ's House:Wedding&Formal Wear,JJ's House
google,clothing,,co.tapcart.app.id_T6ETUv5G9w,Ally+Fashion,Ally Fashion,Ally Fashion
google,clothing,,ua.mad.intertop,INTERTOP+LLC,INTERTOP: Fashion Store,INTERTOP LLC
google,clothing,,com.ua.shop,Under+Armour,Under Armour,Under Armour
google,clothing,,com.levistrauss.customer,Levi's+Mobile+App,Levi's - Shop Denim & More,Levi's Mobile App
google,clothing,,com.cocoplay.black.friday,Coco+Play+By+TabTale,Black Friday Fashion Mall Game,Coco Play By TabTale
google,clothing,,com.belk.android.belk,Belk,Belk – Shopping App,Belk
google,clothing,,com.levi.levis247,Levi's+Mobile+App,Levi's® - Shop Denim & More,Levi's Mobile App
google,clothing,,com.truereligion.app.truereligion,True+Religion+Sales+LLC,True Religion | Since 2002,True Religion Sales LLC
```

Note that the second search results column will always be empty for Google, since Google does not provide numeric IDs.

This combined version of the search command also provides a `--full` flag that will cause the full
app information to be downloaded for each search result, and a `search-all` command that takes a `--query-file` parameter
pointing to a text file where every line will be used as a query string.

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

And for the simplified combined interface:

```
Make API requests and save responses

Usage: app-store-access-cli api [OPTIONS] <COMMAND>

Commands:
  search      Perform a search for a given query string
  search-all  Perform searches for a list of queries provided as lines in the indicated text file
  suggest     Look up autocomplete suggestions for a given query string
  help        Print this message or the help of the given subcommand(s)

Options:
      --apple-archive <APPLE_ARCHIVE>    [default: data/apple/]
  -v, --verbose...                       Level of verbosity
      --google-archive <GOOGLE_ARCHIVE>  [default: data/google/]
  -h, --help                             Print help
```

## License

This software is licensed under the [GNU General Public License v3.0][gpl-v3] (GPL-3.0).

[gpl-v3]: https://www.gnu.org/licenses/gpl-3.0.en.html
[rust]: https://rust-lang.org/
[rust-installation]: https://doc.rust-lang.org/cargo/getting-started/installation.html
