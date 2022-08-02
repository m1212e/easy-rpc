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
            Ok(_) => {}
            Err(_) => {}
        };

        let result = generate_for_directory_recursively::<TypeScriptTranslator>(
            &test_files.join("input"),
            &test_files.join("output"),
            &"",
            "Server",
        )
        .unwrap();

        let h1 = hash_directory(&test_files.join("output")).unwrap();
        let h2 = hash_directory(&test_files.join("output_assert")).unwrap();
        assert_eq!(h1, h2);

        let mut v1: Vec<_> = result.get("Server").unwrap().to_owned();
        let mut v2: Vec<_> = vec!["api".to_string(), "auth".to_string()];
        v1.sort();
        v2.sort();
        assert_eq!(
            *v1,
            *v2
        );

        let mut v1: Vec<_> = result.get("Client").unwrap().to_owned();
        let mut v2: Vec<_> = vec!["api".to_string(), "auth".to_string()];
        v1.sort();
        v2.sort();
        assert_eq!(
            *v1,
            *v2
        );
    }

    /**
        Simple compare helper. Not perfect, but enough for this case because we got separate tests for the file content
    */
    fn hash_directory(dir: &std::path::Path) -> Result<u64, std::io::Error> {
        let mut paths: Vec<_> = std::fs::read_dir(dir)?.map(|r| r.unwrap()).collect();

        paths.sort_by_key(|a| a.path());
        let mut hasher = DefaultHasher::new();

        for entry in paths {
            if entry.file_type()?.is_dir() {
                hash_directory(&entry.path())?.hash(&mut hasher);
            } else {
                entry.file_name().to_str().unwrap().hash(&mut hasher);
            }
        }

        Ok(hasher.finish())
    }
}
