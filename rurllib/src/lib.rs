pub mod printer;
pub mod requester;

pub use self::printer::print::Print;
pub use self::printer::simple_printer::SimplePrinter;

pub use self::requester::request::Request;
pub use self::requester::reqwest_requester::ReqwestRequester;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
