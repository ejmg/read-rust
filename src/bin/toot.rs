extern crate failure;
extern crate getopts;
extern crate mammut;
extern crate read_rust;
extern crate serde;
extern crate serde_json;
extern crate uuid;

use failure::Error;
use getopts::Options;
use mammut::apps::{AppBuilder, Scopes};
use mammut::{Data, Mastodon, Registration, StatusBuilder};

use read_rust::categories::Categories;
use read_rust::feed::{Item, JsonFeed};
use read_rust::toot_list::{Toot, TootList};

use std::env;
use std::fs::File;
use std::io;
use std::path::Path;

const MASTODON_DATA_FILE: &str = ".mastodon-data.json";

fn connect_to_mastodon() -> Result<Mastodon, Error> {
    match File::open(MASTODON_DATA_FILE) {
        Ok(file) => {
            let data: Data = serde_json::from_reader(file)?;
            Ok(Mastodon::from_data(data))
        }
        Err(_) => register(),
    }
}

fn register() -> Result<Mastodon, Error> {
    let app = AppBuilder {
        client_name: "read-rust",
        redirect_uris: "urn:ietf:wg:oauth:2.0:oob",
        scopes: Scopes::Write,
        website: Some("https://readrust.net/"),
    };

    let mut registration = Registration::new("https://botsin.space");
    registration.register(app)?;
    let url = registration.authorise()?;

    println!("Click this link to authorize on Mastodon: {}", url);
    println!("Paste the returned authorization code: ");

    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input)?;

    let code = input.trim();
    let mastodon = registration.create_access_token(code.to_string())?;

    // Save app data for using on the next run.
    let file = File::create(MASTODON_DATA_FILE)?;
    let _ = serde_json::to_writer_pretty(file, &*mastodon)?;

    Ok(mastodon)
}

fn toot_text_from_item(item: &Item, categories: &Categories) -> String {
    let tags = item.tags
        .iter()
        .filter_map(|tag| {
            categories
                .hashtag_for_category(tag)
                .map(|hashtag| format!("#{}", hashtag))
        })
        .collect::<Vec<String>>()
        .join(" ");

    format!(
        "{title} by {author}: {url} #Rust {tags}",
        title = item.title,
        author = item.author.name,
        url = item.url,
        tags = tags
    )
}

fn run(
    tootlist_path: &str,
    json_feed_path: &str,
    categories_path: &str,
    dry_run: bool,
) -> Result<(), Error> {
    let tootlist_path = Path::new(tootlist_path);
    let mut tootlist = TootList::load(&tootlist_path)?;
    let feed = JsonFeed::load(Path::new(json_feed_path))?;
    let categories_path = Path::new(categories_path);
    let categories = Categories::load(&categories_path)?;

    let to_toot: Vec<Item> = feed.items
        .into_iter()
        .filter(|item| !tootlist.contains(&item.id))
        .collect();

    // if to_toot.is_empty() {
    //     println!("Nothing to toot!");
    //     return Ok(());
    // }

    let mastodon = connect_to_mastodon()?;
    for item in to_toot {
        let status_text = toot_text_from_item(&item, &categories);
        println!("• {}", status_text);
        if !dry_run {
            let _toot = mastodon.new_status(StatusBuilder::new(status_text))?;
        }
        tootlist.add_item(Toot { item_id: item.id });
    }

    if !dry_run {
        let _ = tootlist.save(&tootlist_path)?;
    }

    let test_toot = "https://rustedneuron.com/@jackwilliambell/101115962682339528".to_string();
    println!("Search results = {:?}", mastodon.search(test_toot, true));

    Ok(())
}

fn print_usage(program: &str, opts: &Options) {
    let usage = format!(
        "Usage: {} [options] tootlist.json jsonfeed.json categories.json",
        program
    );
    print!("{}", opts.usage(&usage));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("n", "dryrun", "don't toot, just show what would be tooted");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") || matches.free.is_empty() {
        print_usage(&program, &opts);
        return;
    }

    run(
        &matches.free[0],
        &matches.free[1],
        &matches.free[2],
        matches.opt_present("n"),
    ).expect("error");
}
