# perbin

a paste bin.

A fork of w4/bin that's still pretty minimalist. Adds simple persistence and customizes the theme to my own preference.

[bin](https://bin.gy/) is written in Rust in around 300 lines of code. It's fast, it's simple, there's code highlighting and you can ⌘+A without going to the 'plain' page. It's revolutionary in the paste bin industry, disrupting markets and pushing boundaries never seen before.

##### so how do you get perbin?

Compile it from source using Cargo:

```bash
# nix-shell provides an environment with rust/cargo installed
$ nix-shell

[nix-shell:~/Code/bin]$ cargo build --release
   Compiling bin v1.0.0 (/Users/jordanjd/Code/bin)
    Finished release [optimized] target(s) in 3.61s

[nix-shell:~/Code/bin]$ ./target/release/bin
    ...
```

##### how do you run it?

```bash
$ ./bin
```

##### funny, what settings are there?

```
$ ./bin

Usage: bin [<bind_addr>] [--buffer-size <buffer-size>] [--max-paste-size <max-paste-size>]

a pastebin.

Positional Arguments:
  bind_addr         socket address to bind to (default: 127.0.0.1:8820)

Options:
  --buffer-size     maximum amount of pastes to store before rotating (default:
                    1000)
  --max-paste-size  maximum paste size in bytes (default. 32kB)
  --help            display usage information
```

##### is there curl support?

```bash
$ curl -X PUT --data 'hello world' https://bin.gy
https://bin.gy/cateettary
$ curl https://bin.gy/cateettary
hello world
```

##### how does syntax highlighting work?

To get syntax highlighting you need to add the file extension at the end of your paste URL.
