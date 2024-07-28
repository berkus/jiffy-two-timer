use jiffy_two_timer::GRAMMAR;

// for timing the cost savings of using a serialized matcher
fn main() {
    GRAMMAR.matcher().unwrap().parse("yesterday");
}