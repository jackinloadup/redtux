extern crate clap;
extern crate rawr;

mod config;

use rawr::prelude::*;
use config::Config;

fn main() {

    // Creates a new client to access the reddit API. You need to set a user agent so Reddit knows
    // who is using this client.
    let client = RedditClient::new("your user agent here", AnonymousAuthenticator::new());
    // Access the subreddit /r/rust.
    let subreddit = client.subreddit("wallpapers");
    // Gets the hot listing of /r/rust. If the API request fails, we will panic with `expect`.
    let mut hot_listing = subreddit.hot(ListingOptions::default()).expect("Could not fetch post listing!");
    // Iterates through the top 50 posts of /r/rust. If you do not `take(n)`, this iterator will
    // continue forever!
    for post in hot_listing.take(1) {
        println!("{:?}", post.link_url());
    }
}

