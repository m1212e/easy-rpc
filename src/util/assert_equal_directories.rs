// only used in tests
#[cfg(test)]
use colored::Colorize;
#[cfg(test)]
use text_diff::{diff, Difference};

/*
    Checks that every file existing in a also exists in b
*/
#[cfg(test)]
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

            let (_, changeset) = diff(
                &std::fs::read_to_string(&a.join(entry.file_name())).unwrap(),
                &std::fs::read_to_string(&b.join(entry.file_name())).unwrap(),
                "\n",
            );
            if changeset.len() > 0 {
                let mut diff_counter = 0;

                for diff in changeset {
                    match diff {
                        Difference::Same(ref x) => {
                            println!("{}\n", x);
                        }
                        Difference::Add(ref x) => {
                            print!("{}", format!("+{}\n", x).green());
                            diff_counter += 1;
                        }
                        Difference::Rem(ref x) => {
                            print!("{}", format!("-{}\n", x).red());
                            diff_counter += 1;
                        }
                    }
                }

                if diff_counter > 0 {
                    panic!(
                        "\nComparison between\n{}\nand\n{}\nfailed.",
                        a.join(entry.file_name()).to_str().unwrap(),
                        b.join(entry.file_name()).to_str().unwrap()
                    );
                }
            }
        }
    }
}
