/*
    Checks that every file existing in a also exists in b
*/
pub fn assert_equal_directories(a: &std::path::Path, b: &std::path::Path) {
    for entry in std::fs::read_dir(a).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            assert_equal_directories(&a.join(entry.file_name()), &b.join(entry.file_name()));
        } else {
            if !std::path::Path::exists(&b.join(entry.file_name())) {
                panic!(
                    "Could not find file {}",
                    &b.join(entry.file_name()).to_str().unwrap()
                )
            }

            assert_eq!(
                std::fs::read_to_string(&a.join(entry.file_name())).unwrap(),
                std::fs::read_to_string(&b.join(entry.file_name())).unwrap()
            );
        }
    }
}
