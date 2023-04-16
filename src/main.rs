fn get_greeting() -> String {
    "Hello, World!".to_owned()
}

fn main() {
    println!("{}", get_greeting());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_greeting() {
        assert_eq!(get_greeting(), "Hello, World!".to_owned());
    }
}
