#[cfg(test)] // This attribute tells Rust to compile and run the test code only when you run cargo test, not when you run cargo build.
mod tests {
    extern crate phrases;

    #[test]
    fn english_greeting_correct() {
        assert_eq!("Hello!", phrases::greetings::english::hello());
    }

    #[test]
    #[should_panic] // This test should fail
    fn english_greeting_not_correct() {
        assert_eq!("He", phrases::greetings::english::hello());
    }
}
