use std::fs::File;
use tempfile::tempdir;

#[test]
#[ignore = "not actually a test"]
fn test_app() {
    todo!();
}

#[test]
fn test_create_filtered_argument_list() {
    let temp_dir = tempdir().unwrap();
    let dir_path = temp_dir.path();

    File::create(dir_path.join("file_1.txt")).unwrap();
    File::create(dir_path.join("file_2")).unwrap();
    File::create(dir_path.join("file_3.shit")).unwrap();

    let arg_list = crate::ArgumentList::new(vec![
        "/target/debug/wc".to_string(),
        "arg1".to_string(),
        "--bytes".to_string(),
        "arg2".to_string(),
        "arg3".to_string(),
        dir_path.join("file_1.txt").to_string_lossy().to_string(),
        dir_path.join("file_3.shit").to_string_lossy().to_string(),
        "arg4".to_string(),
        "--lines".to_string(),
        "--shit".to_string(),
        dir_path.join("file_2").to_string_lossy().to_string(),
        "--words".to_string(),
        "--------shitass".to_string(),
        "-&7808)(&)(@& )&% @#&%)('2'2'21'231!)".to_string(),
    ]);

    assert_eq!(
        arg_list,
        crate::ArgumentList {
            options: vec![
                "--bytes".to_string(),
                "--lines".to_string(),
                "--words".to_string(),
            ],
            paths: vec![
                dir_path.join("file_1.txt").to_string_lossy().to_string(),
                dir_path.join("file_3.shit").to_string_lossy().to_string(),
                dir_path.join("file_2").to_string_lossy().to_string(),
            ]
        }
    );
}

#[test]
fn test_check_options() {
    assert_eq!(
        crate::ArgumentList {
            options: vec![
                "--bytes".to_string(),
                "--lines".to_string(),
                "--words".to_string(),
            ],
            paths: vec![]
        }
        .check_options(),
        (true, true, true)
    );
    assert_eq!(
        crate::ArgumentList {
            options: vec!["--bytes".to_string(), "--lines".to_string(),],
            paths: vec![]
        }
        .check_options(),
        (true, false, true)
    );
    assert_eq!(
        crate::ArgumentList {
            options: vec!["--lines".to_string(),],
            paths: vec![]
        }
        .check_options(),
        (false, false, true)
    );
    assert_eq!(
        crate::ArgumentList {
            options: vec![],
            paths: vec![]
        }
        .check_options(),
        (false, false, false)
    );
}
