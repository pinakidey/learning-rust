pub mod greetings {
    pub mod english {
        pub fn hello()-> String {"Hello Rust!".to_string()}
    }
    pub mod french {
        pub fn hello()-> String {"Bonjour Rust!".to_string()}
    }
}
#[cfg(test)]
pub mod tests {
    #[test]
    pub fn it_works() {
        assert_eq!(greetings::english::hello() == "Hello Rust");
    }
}
