#[cfg(test)]
mod tests {
    use serde_json::Error;

    use crate::transpiler::config::{parse_config, parse_roles};

    #[test]
    fn test_roles() -> Result<(), Error> {
        let result = parse_roles(
            "[
    {
        \"name\": \"Server\",
        \"type\": \"http-server\"
    },
    {
        \"name\": \"Client\",
        \"type\": \"browser\",
        \"documentation\": \"This is the browser client\"
    }
]"
            .as_bytes(),
        )?;

        assert_eq!(result[0].name, "Server".to_string());
        assert_eq!(
            result[0].role_type,
            "http-server".to_string()
        );
        assert_eq!(result[0].documentation, None);

        assert_eq!(result[1].name, "Client".to_string());
        assert_eq!(result[1].role_type, "browser".to_string());
        assert_eq!(
            result[1].documentation.as_ref().unwrap(),
            "This is the browser client"
        );

        Ok(())
    }

    #[test]
    fn test_config() -> Result<(), Error> {
        let result = parse_config(
            "{
                \"sources\": [\"../../erpc-sources\", \"../../erpc-sources2\"],
                \"role\": \"frontend\"
              }"
            .as_bytes(),
        )?;

        assert_eq!(result.role, "frontend");
        assert_eq!(
            result.sources,
            vec!["../../erpc-sources", "../../erpc-sources2"]
        );

        Ok(())
    }
}
