use jiffy_two_timer::MATCHER;

// for timing the cost savings of using a serialized matcher
fn main() {
    MATCHER.parse("yesterday");
}