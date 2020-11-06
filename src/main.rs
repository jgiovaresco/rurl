extern crate clap;

mod requester;

use clap::{App, Arg};
use requester::request;
use reqwest::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("rurl")
        .version("1.0")
        .author("Julien G. <dev@giovaresco.fr>")
        .arg(
            Arg::with_name("URL")
                .help("Sets the url to call")
                .required(true)
                .index(1),
        )
        .get_matches();

    let url_input = matches.value_of("URL").unwrap();
    let url = Url::parse(url_input).unwrap();

    let body = request(url).await?;
    println!("{}", body);

    Ok(())
}
