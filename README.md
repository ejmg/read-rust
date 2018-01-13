# Read Rust

Source code to [readrust.net][self]. Currently just a couple of feeds for
following [#Rust2018]. This repo contains the source to the website and tools
for updating the feeds.

## Contributing

See the [contributing guidelines][contributing].

## Building

### Website

The website is built with [Cobalt]. After [installing Cobalt][install-cobalt]
the site can be built with `cobalt build`.

### Tools

The tools are written in Rust, so `cargo build --release` will build them.
The tools themselves are:

* `add-url` add a new entry to `feed.json`
* `generate-rss` geneates `feed.rss` from `feed.json`

## The Feeds

Two feeds are published: `feed.json` and `feed.rss`. `feed.json` is a [JSON
Feed] and is the canonical feed. `feed.rss` is derived from `feed.json`. Don't
make manual edits to `feed.rss`.

[self]: http://readrust.net/
[contributing]: https://github.com/wezm/read-rust/blob/master/.github/contributing.md
[#Rust2018]: https://blog.rust-lang.org/2018/01/03/new-years-rust-a-call-for-community-blogposts.html
[Cobalt]: http://cobalt-org.github.io/
[install-cobalt]: http://cobalt-org.github.io/docs/install.html

