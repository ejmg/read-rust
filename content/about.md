title: About
layout: default.liquid
---

Read Rust collects interesting posts related to the [Rust programming
language][rust-lang]. It is run by [Wesley Moore][wezm] with contributions
from the Rust community. You can [follow me on Twitter][@wezm] or [Mastodon].

Read Rust is an open source project. The [source code][source] and [issue
tracker][issues] are hosted on GitHub.

<h2 id="feeds">Feeds</h2>

I am a proponent of the open web and as a result the content of Read Rust is
available in a number of machine readable formats. There are feeds available
for the whole site, as well as each of the categories:

<ul>
<li>Main feed (all posts): <a href="/all/feed.rss">RSS</a> or <a href="/all/feed.json">JSON</a></li>
{% for category in site.data.categories %}
<li>{{ category.name }}: <a href="{{ category.path }}feed.rss">RSS</a> or <a href="{{ category.path }}feed.json">JSON</a></li>
{% endfor %}
</ul>

In order to discover new posts I subscribe to a lot of Rust related RSS feeds.
The list is available in [OPML] (readily importable into [feed
readers](/faq.html#subscribe)) and JSON:

* Blog list: [OPML](/rust-blogs.opml) or [JSON]()

## Social Media

Read Rust also has an account on Twitter: [@read_rust], which tweets
each newly added post.

## Credits

* JSON Feed Icon: <https://jsonfeed.org/version/1>
* RSS Icon: <http://www.feedicons.com/>
* favicon: “[Book][favicon]” by Mike Rowe, from [the Noun Project]

[favicon]: https://thenounproject.com/term/book/17900
[rust-lang]: https://www.rust-lang.org/
[wezm]: http://www.wezm.net/about/
[source]: https://github.com/wezm/read-rust
[issues]: https://github.com/wezm/read-rust/issues
[@wezm]: https://twitter.com/wezm
[@read_rust]: https://twitter.com/read_rust
[OPML]: https://en.wikipedia.org/wiki/OPML
[Mastodon]: https://mastodon.social/@wezm