#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use crate::{
        transpiler::{run, ERPCError},
        util::assert_equal_directories::assert_equal_directories,
    };

    #[test]
    fn test_run() -> Result<(), ERPCError> {
        let mut test_files = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

        for dir in "transpiler/tests/run_test_files".split_terminator('/') {
            test_files = test_files.join(dir);
        }

        match fs::remove_dir_all(&test_files.join("output")) {
            Ok(_) => {}
            Err(_) => {}
        };

        let backend_path = test_files.join("target").join("backend");
        let frontend_path = test_files.join("target").join("frontend");

        run(&backend_path)?;
        run(&frontend_path)?;

        assert_equal_directories(&test_files.join("target"), &test_files.join("assert"));

        Ok(())
    }
}
