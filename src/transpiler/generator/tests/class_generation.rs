#[cfg(test)]
mod tests {

    use std::{fs, path::Path};

    use crate::{
        transpiler::{
            config::Role,
            generator::{
                generate_for_directory, generate_for_directory_recursively,
                translator::typescript::TypeScriptTranslator,
            },
        },
        util::assert_equal_directories::assert_equal_directories,
    };

    #[test]
    fn test_recursive() {
        let mut test_files = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

        test_files.extend(
            "transpiler/generator/tests/recursive_class_generation_test_files"
                .split_terminator('/'),
        );

        match fs::remove_dir_all(&test_files.join("output")) {
            Ok(_) => {}
            Err(_) => {}
        };

        let result = generate_for_directory_recursively::<TypeScriptTranslator>(
            &test_files.join("input"),
            &test_files.join("output"),
            &"",
            "Server",
            &vec![
                Role {
                    documentation: None,
                    name: "Client".to_string(),
                    role_type: "browser".to_string(),
                },
                Role {
                    documentation: None,
                    name: "Server".to_string(),
                    role_type: "http-server".to_string(),
                },
            ],
            &vec![],
        );

        assert_eq!(result.1.len(), 0);
        let result = result.0;

        assert_equal_directories(
            &test_files.join("output_assert"),
            &test_files.join("output"),
        );

        let mut v1: Vec<_> = result.get("Server").unwrap().to_owned();
        let mut v2: Vec<_> = vec!["api".to_string(), "auth".to_string()];
        v1.sort();
        v2.sort();
        assert_eq!(*v1, *v2);

        let mut v1: Vec<_> = result.get("Client").unwrap().to_owned();
        let mut v2: Vec<_> = vec!["api".to_string(), "auth".to_string()];
        v1.sort();
        v2.sort();
        assert_eq!(*v1, *v2);
    }

    #[test]
    fn test_with_clients() {
        let mut test_files = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

        test_files.extend(
            "transpiler/generator/tests/client_class_generation_test_files".split_terminator('/'),
        );

        match fs::remove_dir_all(&test_files.join("output")) {
            Ok(_) => {}
            Err(_) => {}
        };

        assert_eq!(
            generate_for_directory::<TypeScriptTranslator>(
                &test_files.join("input"),
                &test_files.join("output"),
                "Server",
                &vec![
                    Role {
                        documentation: Some("This is some docs".to_string()),
                        name: "Client".to_string(),
                        role_type: "browser".to_string(),
                    },
                    Role {
                        documentation: Some("This is some docs".to_string()),
                        name: "Server".to_string(),
                        role_type: "http-server".to_string(),
                    },
                ],
                &vec![]
            )
            .len(),
            0
        );

        assert_equal_directories(
            &test_files.join("output_assert"),
            &test_files.join("output"),
        );
    }
}
