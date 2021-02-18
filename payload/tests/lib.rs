#[cfg(test)]
mod tests {
    extern crate payload;
    use payload::greetings;
    #[test]
    // #[should_panic]
    // #[ignore]
    pub fn test_english() {
        assert_eq!(greetings::english::hello(), "Hello Rust!");
    }

    #[test]
    pub fn test_french() {
        assert_eq!(greetings::french::hello(), "Bonjour Rust!");
    }
}