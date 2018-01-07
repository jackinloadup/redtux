extern crate clap;
extern crate rawr;
extern crate gio;
extern crate gtk;

mod config;

use rawr::prelude::*;
use config::Config;
use gio::prelude::*;
use gtk::prelude::*;
use std::env::args;


//fn main() {
//    let config = Config::new();
//    println!("{:?}", config);
//
//    // Creates a new client to access the reddit API. You need to set a user agent so Reddit knows
//    // who is using this client.
//    //let client = RedditClient::new("your user agent here", AnonymousAuthenticator::new());
//    //// Access the subreddit /r/rust.
//    //let subreddit = client.subreddit(&config.subreddit.to_owned()[..]);
//    //// Gets the hot listing of /r/rust. If the API request fails, we will panic with `expect`.
//    //let mut hot_listing = subreddit.hot(ListingOptions::default()).expect("Could not fetch post listing!");
//    //// Iterates through the top 50 posts of /r/rust. If you do not `take(n)`, this iterator will
//    //// continue forever!
//    //for post in hot_listing.take(1) {
//    //    println!("{:?}", post.link_url());
//    //}
//}




// make moving clones into closures more convenient
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    window.connect_delete_event(clone!(window => move |_, _| {
        window.destroy();
        Inhibit(false)
    }));

    let button = gtk::Button::new_with_label("Click me!");

    window.add(&button);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new("com.github.basic",
                                            gio::ApplicationFlags::empty())
                                       .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());

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

