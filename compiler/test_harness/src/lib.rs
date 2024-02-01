use expect_test::expect_file;
use std::ffi::OsStr;
use std::panic::RefUnwindSafe;
use std::path::Path;
use std::{env, fs};

pub fn run_test_dir(tests_dir: &str, test_fn: impl Fn(&Path, &str) -> String + RefUnwindSafe) {
    let tests_dir = {
        let current_dir = env::current_dir().unwrap();
        current_dir.join(format!("src/tests/{tests_dir}"))
    };

    let mut failed_tests = vec![];

    for entry in fs::read_dir(tests_dir).unwrap() {
        let test_path = entry.unwrap().path().canonicalize().unwrap();

        println!(
            "\n==== RUNNING TEST {:?} ====",
            test_path.file_stem().unwrap()
        );

        let file_name = test_path.file_name().unwrap().to_os_string();

        if test_path.extension() != Some(OsStr::new("test")) {
            continue;
        }

        let did_panic = std::panic::catch_unwind(|| {
            let test_content = fs::read_to_string(&test_path).unwrap();
            let (input, _expected) = test_content.split_once("\n===\n").unwrap();

            let result = test_fn(&test_path, input);

            let expected_test_content = format!("{input}\n===\n{result}\n");
            expect_file![test_path].assert_eq(&expected_test_content);
        })
        .is_err();

        if did_panic {
            failed_tests.push(file_name);
        }
    }

    assert!(
        failed_tests.is_empty(),
        "{} test(s) failed: {:?}",
        failed_tests.len(),
        failed_tests,
    );
}
