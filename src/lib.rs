pub fn hello_dev() -> &'static str {
    "Hello, Dev!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_dev() {
        assert_eq!(hello_dev(), "Hello, Dev!");
    }
}
