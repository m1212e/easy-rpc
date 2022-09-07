#[cfg(test)]
mod tests {
    use std::{fs, io::Write, path::Path, time::Duration};

    use crate::{
        transpiler::{run, ERPCError},
        util::assert_equal_directories::assert_equal_directories,
    };

    #[tokio::test]
    async fn test_run() -> Result<(), ERPCError> {
        let mut test_files = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

        for dir in "transpiler/tests/run_test_files".split_terminator('/') {
            test_files = test_files.join(dir);
        }

        let backend_path = test_files.join("target").join("backend");
        let frontend_path = test_files.join("target").join("frontend");

        run(&backend_path, false).await?;
        run(&frontend_path, false).await?;

        assert_equal_directories(&test_files.join("target"), &test_files.join("assert"));

        Ok(())
    }

    #[tokio::test]
    async fn test_run_watch() -> Result<(), ERPCError> {
        std::thread::spawn(|| {
            let mut path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
            path.extend("transpiler/tests/run_watch_test_files/frontend".split_terminator('/'));
            run(&path, true).await.unwrap();
        });

        let mut source_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        source_path
            .extend("transpiler/tests/run_watch_test_files/sources/api.erpc".split_terminator('/'));

        let mut assert_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        assert_path.extend(
            "transpiler/tests/run_watch_test_files/frontend/.erpc/generated/Frontend/api.ts"
                .split_terminator('/'),
        );
        let original_source_content = std::fs::read_to_string(&source_path).unwrap();

        for n in 7..10 {
            fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(&source_path)
                .unwrap()
                .write_all(format!("\nFrontend test{n}()\n").as_bytes())
                .unwrap();

            std::thread::sleep(Duration::from_secs(2));
            let assert_content = std::fs::read_to_string(&assert_path).unwrap();
            let assert_search = format!("this.test{n}");
            if !assert_content.contains(&assert_search) {
                panic!("\nCould not find \n\"{assert_search}\"\n in \n \"{assert_content}\"",)
            }
        }

        fs::write(source_path, original_source_content).unwrap();

        Ok(())
    }
}
