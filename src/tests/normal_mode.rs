#[cfg(test)]
mod tests {

    use std::path::Path;

    use crate::{
        run_main,
        util::{assert_equal_directories::assert_equal_directories, copy_dir::copy_dir},
    };

    #[tokio::test]
    async fn test_success() {
        let mut test_files = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        test_files = test_files.join("tests");
        let wdir = test_files.join("workdir_normal_mode");

        match std::fs::remove_dir_all(&wdir) {
            Ok(_) => {}
            Err(_) => {}
        };

        copy_dir(test_files.join("example_setup"), &wdir).unwrap();

        run_main(vec!["-p".to_string(), wdir.to_str().unwrap().to_string()]).await;

        assert_equal_directories(&wdir, &test_files.join("assert"));
    }
}
