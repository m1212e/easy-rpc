#[cfg(test)]
mod tests {
    use std::{
        collections::hash_map::DefaultHasher,
        fs,
        hash::{Hash, Hasher},
        path::Path,
    };

    use crate::transpiler::generator::{
        generate_for_directory_recursively, typescript::TypeScriptTranslator,
    };

    #[test]
    fn test_success() {
        let mut test_files = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

        for dir in
            "transpiler/generator/tests/recursive_class_generation_test_files".split_terminator('/')
        {
            test_files = test_files.join(dir);
        }

        match fs::remove_dir_all(&test_files.join("output")) {
            Ok(_) => {},
            Err(_) => {},
        };

        let result = generate_for_directory_recursively::<TypeScriptTranslator>(
            &test_files.join("input"),
            &test_files.join("output"),
            &"",
            "Server",
        )
        .unwrap();

        assert_eq!(
            hash_directory(&test_files.join("output")).unwrap(),
            hash_directory(&test_files.join("output_assert")).unwrap()
        );

        assert_eq!(
            result.get("Server").unwrap(),
            &vec!["api".to_string(), "auth".to_string()]
        );
        assert_eq!(
            result.get("Client").unwrap(),
            &vec!["api".to_string(), "auth".to_string()]
        );
    }

    /**
        Simple compare helper. Not perfect, but enough for this case
    */
    fn hash_directory(dir: &std::path::Path) -> Result<u64, std::io::Error> {
        let mut paths: Vec<_> = std::fs::read_dir(dir)?.map(|r| r.unwrap()).collect();

        paths.sort_by_key(|a| a.path());
        let mut hasher = DefaultHasher::new();

        for entry in paths {
            println!("{}", entry.path().to_str().unwrap());
            if entry.file_type()?.is_dir() {
                hash_directory(&entry.path())?.hash(&mut hasher);
            } else {
                let content = std::fs::read_to_string(&entry.path())?;
                entry.file_name().to_str().unwrap().hash(&mut hasher);
                content.hash(&mut hasher);
            }
        }

        Ok(hasher.finish())
    }
}
