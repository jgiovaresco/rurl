extern crate clap;

mod printer;
mod requester;

use clap::{App, Arg, ArgMatches};
use printer::print::Print;
use printer::simple_printer::SimplePrinter;
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

    let url = read_url(matches);

    let body = request(url).await;

    print_result(&body);

    Ok(())
}

fn read_url(matches: ArgMatches) -> Url {
    let url_input = matches.value_of("URL").unwrap();
    Url::parse(url_input).unwrap()
}

fn print_result(result: &Result<String, Box<dyn std::error::Error>>) {
    let mut std = std::io::stdout();
    let mut err = std::io::stderr();
    let mut printer = SimplePrinter::new(&mut std, &mut err);
    printer.print(result).expect("Fail to print the result");
}
