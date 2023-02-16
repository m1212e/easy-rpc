#[cfg(test)]
mod tests {

    use std::path::Path;

    use tokio::time::{sleep, Duration};

    use crate::{
        run_main,
        util::{assert_equal_directories::assert_equal_directories, copy_dir::copy_dir},
    };

    #[tokio::test]
    async fn test_success() {
        let mut test_files = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        test_files = test_files.join("tests");
        let wdir = test_files.join("workdir_watch_mode");

        match std::fs::remove_dir_all(&wdir) {
            Ok(_) => {}
            Err(_) => {}
        };

        copy_dir(test_files.join("example_setup"), &wdir).unwrap();

        let api_file = (&wdir.join("sources")).join("api.erpc");
        std::fs::remove_file(&api_file).unwrap();

        let w_dir_clone = wdir.clone();
        tokio::spawn(async move {
            run_main(vec![
                "-w".to_string(),
                "-p".to_string(),
                (&w_dir_clone).to_str().unwrap().to_string(),
            ])
            .await;
        });

        sleep(Duration::from_millis(1000)).await;
        std::fs::copy(
            ((test_files.join("example_setup")).join("sources")).join("api.erpc"),
            api_file,
        )
        .unwrap();
        sleep(Duration::from_millis(1000)).await;
        assert_equal_directories(&wdir, &test_files.join("assert"));
    }
}
