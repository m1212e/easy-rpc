#[cfg(test)]
mod tests {
    use crate::transpiler::{generator::typescript::client::generate_client, Role};

    #[test]
    fn test_success_foreign() {}

    #[test]
    fn test_success_callback() {
        let result = generate_client(
            false,
            &vec!["api".to_string(), "tracks".to_string()],
            Role {
                documentation: Some("Example docs".to_string()),
                name: "Server".to_string(),
                types: vec!["http-server".to_string()],
            },
            &vec!["Client".to_string()],
        );

        assert_eq!(result, "");
    }
}

//TODO write some tests whith variation (no docs, no return type etc.)
