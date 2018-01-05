use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    pub subreddit: String,
}

impl Config {
    pub fn new() -> Config {
        let matches = App::new("matcha")
            .version("0.1.0")
            .author("Bas Palmer <ik@baspalmer.nl>")
            .about("Reddit post viewer")
            .arg(
                Arg::with_name("subreddit")
                    .required(true)
                    .help("Sets the subreddit"),
            )
            .get_matches();

        let subreddit = matches.value_of("subreddit").unwrap().to_string();

        Config { subreddit }
    }
}
