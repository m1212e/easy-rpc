#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::transpiler::generator::{generate_for_directory_recursively, typescript::TypeScriptTranslator};

    #[test]
    fn test_success() {
        let mut test_files = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

        for dir in
            "transpiler/generator/tests/recursive_class_generation_test_files".split_terminator('/')
        {
            test_files = test_files.join(dir);
        }

        let result = generate_for_directory_recursively::<TypeScriptTranslator>(
            &test_files.join("input"),
            &test_files.join("output"),
            &"",
            "Server"
        ).unwrap();

        //TODO: check results in dir and variable

    }
}
