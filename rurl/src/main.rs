extern crate clap;

use clap::{App, Arg, ArgMatches};
use rurllib::{Print, SimplePrinter};
use rurllib::{Request, ReqwestRequester};

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

    let requester = ReqwestRequester::default();
    let url = read_url(matches);
    let body = requester.request(url).await;

    print_result(&body);

    Ok(())
}

fn read_url(matches: ArgMatches) -> String {
    String::from(matches.value_of("URL").unwrap())
}

fn print_result(result: &Result<String, Box<dyn std::error::Error>>) {
    let mut std = std::io::stdout();
    let mut err = std::io::stderr();
    let mut printer = SimplePrinter::new(&mut std, &mut err);
    printer.print(result).expect("Fail to print the result");
}
