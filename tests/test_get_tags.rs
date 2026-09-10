use pretty_assertions::assert_eq;
use relative_path::RelativePath;
use std::env::current_dir;
use tgr::{
    config::Userconfig,
    io::{get_all_tags, get_tags_from_file},
};

#[test]
fn get_tags_from_all_files() {
    let project_root_directory = current_dir().unwrap();
    let org_dir_path = RelativePath::new("/tests/org_files/")
        .to_path(&project_root_directory)
        .to_str()
        .unwrap()
        .to_string();

    let mut cfg: Userconfig = Userconfig {
        org_directory: org_dir_path,
        exclude_files: None,
        exclude_pattern: None,
        exclude_patterns: None,
    };

    assert_eq!(
        get_all_tags(&mut cfg),
        Some(vec![
            String::from("bar_another_file"),
            String::from("bar_file_1"),
            String::from("bar_file_2"),
            String::from("baz_another_file"),
            String::from("baz_file_1"),
            String::from("baz_file_2"),
            String::from("foo_another_file"),
            String::from("foo_file_1"),
            String::from("foo_file_2"),
            String::from("non_delimiters_another_file"),
            String::from("non_delimiters_file_1"),
            String::from("non_delimiters_file_2"),
            String::from("trailing_baz_another_file"),
            String::from("trailing_baz_file_1"),
            String::from("trailing_baz_file_2"),
            String::from("trailing_foo_another_file"),
            String::from("trailing_foo_file_1"),
            String::from("trailing_foo_file_2"),
        ])
    )
}

#[test]
fn get_tags_from_all_files_with_exclude_files() {
    let project_root_directory = current_dir().unwrap();
    let org_dir_path = RelativePath::new("/tests/org_files/")
        .to_path(&project_root_directory)
        .to_str()
        .unwrap()
        .to_string();

    let mut cfg: Userconfig = Userconfig {
        org_directory: org_dir_path,
        exclude_files: Some(vec![String::from("org_file2.org")]),
        exclude_pattern: None,
        exclude_patterns: None,
    };

    assert_eq!(
        get_all_tags(&mut cfg),
        Some(vec![
            String::from("bar_another_file"),
            String::from("bar_file_1"),
            String::from("baz_another_file"),
            String::from("baz_file_1"),
            String::from("foo_another_file"),
            String::from("foo_file_1"),
            String::from("non_delimiters_another_file"),
            String::from("non_delimiters_file_1"),
            String::from("trailing_baz_another_file"),
            String::from("trailing_baz_file_1"),
            String::from("trailing_foo_another_file"),
            String::from("trailing_foo_file_1"),
        ])
    )
}

#[test]
fn get_tags_from_all_files_with_exclude_pattern() {
    let project_root_directory = current_dir().unwrap();
    let org_dir_path = RelativePath::new("/tests/org_files/")
        .to_path(&project_root_directory)
        .to_str()
        .unwrap()
        .to_string();

    let mut cfg: Userconfig = Userconfig {
        org_directory: org_dir_path,
        exclude_files: None,
        exclude_pattern: Some(r"another\w*".to_string()),
        exclude_patterns: None,
    };

    assert_eq!(
        get_all_tags(&mut cfg),
        Some(vec![
            String::from("bar_file_1"),
            String::from("bar_file_2"),
            String::from("baz_file_1"),
            String::from("baz_file_2"),
            String::from("foo_file_1"),
            String::from("foo_file_2"),
            String::from("non_delimiters_file_1"),
            String::from("non_delimiters_file_2"),
            String::from("trailing_baz_file_1"),
            String::from("trailing_baz_file_2"),
            String::from("trailing_foo_file_1"),
            String::from("trailing_foo_file_2"),
        ])
    )
}

#[test]
fn get_tags_from_all_files_with_exclude_pattern_and_exclude_files() {
    let project_root_directory = current_dir().unwrap();
    let org_dir_path = RelativePath::new("/tests/org_files/")
        .to_path(&project_root_directory)
        .to_str()
        .unwrap()
        .to_string();

    let mut cfg: Userconfig = Userconfig {
        org_directory: org_dir_path,
        exclude_files: Some(vec![String::from("org_file2.org")]),
        exclude_pattern: Some(String::from("another\\w*")),
        exclude_patterns: None,
    };

    assert_eq!(
        get_all_tags(&mut cfg),
        Some(vec![
            String::from("bar_file_1"),
            String::from("baz_file_1"),
            String::from("foo_file_1"),
            String::from("non_delimiters_file_1"),
            String::from("trailing_baz_file_1"),
            String::from("trailing_foo_file_1"),
        ])
    )
}

#[test]
fn get_tags_from_all_files_with_exclude_patterns() {
    let project_root_directory = current_dir().unwrap();
    let org_dir_path = RelativePath::new("/tests/org_files/")
        .to_path(&project_root_directory)
        .to_str()
        .unwrap()
        .to_string();

    let mut cfg: Userconfig = Userconfig {
        org_directory: org_dir_path,
        exclude_files: None,
        exclude_pattern: None,
        exclude_patterns: Some(vec![String::from("\\d")]),
    };

    assert_eq!(
        get_all_tags(&mut cfg),
        Some(vec![
            String::from("bar_another_file"),
            String::from("baz_another_file"),
            String::from("foo_another_file"),
            String::from("non_delimiters_another_file"),
            String::from("trailing_baz_another_file"),
            String::from("trailing_foo_another_file"),
        ])
    )
}

#[test]
fn get_tags_from_one_file_that_has_tags() {
    let project_root_directory = current_dir().unwrap();
    let org_dir_path = RelativePath::new("/tests/org_files/")
        .to_path(&project_root_directory)
        .to_str()
        .unwrap()
        .to_string();

    let cfg: Userconfig = Userconfig {
        org_directory: org_dir_path,
        exclude_files: None,
        exclude_pattern: None,
        exclude_patterns: None,
    };

    let file: String = String::from("org_file1.org");

    assert_eq!(
        get_tags_from_file(&cfg, file),
        Some(vec![
            String::from("bar_file_1"),
            String::from("baz_file_1"),
            String::from("foo_file_1"),
            String::from("non_delimiters_file_1"),
            String::from("trailing_baz_file_1"),
            String::from("trailing_foo_file_1"),
        ])
    )
}

#[test]
fn get_tags_from_one_file_that_has_no_tags() {
    let project_root_directory = current_dir().unwrap();
    let org_dir_path = RelativePath::new("/tests/org_files/")
        .to_path(&project_root_directory)
        .to_str()
        .unwrap()
        .to_string();

    let cfg: Userconfig = Userconfig {
        org_directory: org_dir_path,
        exclude_files: None,
        exclude_pattern: None,
        exclude_patterns: None,
    };

    let file: String = String::from("org_file_with_no_tags.org");

    assert_eq!(get_tags_from_file(&cfg, file), None)
}
