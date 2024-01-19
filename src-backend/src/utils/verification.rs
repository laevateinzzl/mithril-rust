use lazy_static::lazy_static;
use regex::Regex;

pub fn verify_email(email: &str) -> bool {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
    }
    RE.is_match(email)
}
