#[cfg(test)]
mod tests {

    use std::{fs, path::Path};

    use crate::transpiler::{
        generator::{
            generate_for_directory, generate_for_directory_recursively,
            typescript::TypeScriptTranslator,
        },
        Role,
    };

    #[test]
    fn test_recursive() {
        let mut test_files = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

        for dir in
            "transpiler/generator/tests/recursive_class_generation_test_files".split_terminator('/')
        {
            test_files = test_files.join(dir);
        }

        match fs::remove_dir_all(&test_files.join("output")) {
            Ok(_) => {}
            Err(_) => {}
        };

        let result = generate_for_directory_recursively::<TypeScriptTranslator>(
            &test_files.join("input"),
            &test_files.join("output"),
            &"",
            "server",
        )
        .unwrap();

        assert_equal_directories(
            &test_files.join("output"),
            &test_files.join("output_assert"),
        );

        let mut v1: Vec<_> = result.get("server").unwrap().to_owned();
        let mut v2: Vec<_> = vec!["api".to_string(), "auth".to_string()];
        v1.sort();
        v2.sort();
        assert_eq!(*v1, *v2);

        let mut v1: Vec<_> = result.get("client").unwrap().to_owned();
        let mut v2: Vec<_> = vec!["api".to_string(), "auth".to_string()];
        v1.sort();
        v2.sort();
        assert_eq!(*v1, *v2);
    }

    #[test]
    fn test_with_clients() {
        let mut test_files = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

        for dir in
            "transpiler/generator/tests/client_class_generation_test_files".split_terminator('/')
        {
            test_files = test_files.join(dir);
        }

        match fs::remove_dir_all(&test_files.join("output")) {
            Ok(_) => {}
            Err(_) => {}
        };

        generate_for_directory::<TypeScriptTranslator>(
            &test_files.join("input"),
            &test_files.join("output"),
            "Server",
            &vec![
                Role {
                    documentation: Some("This is some docs".to_string()),
                    name: "Server".to_string(),
                    types: vec!["http-server".to_string()],
                },
                Role {
                    documentation: Some("This is some docs".to_string()),
                    name: "Client".to_string(),
                    types: vec!["browser".to_string()],
                },
            ],
            &vec!["Client".to_string()],
        )
        .unwrap();

        assert_equal_directories(
            &test_files.join("output"),
            &test_files.join("output_assert"),
        );
    }

    fn assert_equal_directories(a: &std::path::Path, b: &std::path::Path) {
        for entry in std::fs::read_dir(a).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                assert_equal_directories(&a.join(entry.file_name()), &b.join(entry.file_name()));
            } else {
                assert!(std::path::Path::exists(&b.join(entry.file_name())));

                assert_eq!(
                    std::fs::read_to_string(&a.join(entry.file_name())).unwrap(),
                    std::fs::read_to_string(&b.join(entry.file_name())).unwrap()
                );
            }
        }
    }
}
