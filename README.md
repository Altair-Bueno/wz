<!-- cargo-sync-readme start -->

# Wortzahl! Count words **fast**

`wz` is a faster alternative to GNU wc with UTF8 support and human readable
output, written in Rust

![wz](resources/wz.png)

# Features

## It's fast!

`wz` is heavily optimized for performance. Even if you are counting
multiple files, thanks to [rayon]. [Just look at the benchmarks](BENCH.md)

## Human readable and machine readable formats

Output the results on a nice table, or pipe them to another program
that reads JSON. Stop messing with `sed` and `awk`!

```sh
$ wz *(.) --output json | jq .total.lines
1470
```

## Multiple encoding support

Characters and line lengths are count using UTF8 or UTF16[^1] encoding,
meaning that files with non ASCII characters are count correctly

## Multiple line breaks support

Got a file from an old Macintosh? Change the line break to carriage
returns ('\r')

```sh
wz macintosh.txt -n cr
```

# Installation

## Cargo

```sh
cargo install wz
cargo install --git https://github.com/Altair-Bueno/wz.git
```

# Performance

See [BENCH.md](BENCH.md)

# Usage

Run `wz --help` to see the full list of options

[rayon]: https://crates.io/crates/rayon

[^1]: UTF16 support coming later

<!-- cargo-sync-readme end -->
