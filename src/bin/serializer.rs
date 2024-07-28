use jiffy_two_timer::GRAMMAR;
use serde_json::to_string;

// for constructing a serialized matcher
pub fn main() {
    println!("serde_json::from_str(r#\"{}\"#).unwrap();", to_string(&GRAMMAR.matcher().unwrap()).unwrap());
}