use super::*;

#[test]
fn check_source_folders_works() {
    let mut config = Config {
        build: "tests/assets/build".to_string(),
        templates: "tests/assets/templates".to_string(),
        styles: "tests/assets/styles".to_string(),
        public: "tests/assets/public".to_string(),
        ..Config::default()
    };

    assert!(check_source_folders(&config).is_ok());

    config.templates = "what".to_string();
    assert!(matches!(
        check_source_folders(&config),
        Err(Error::SourceDirectoryNotExist(name)) if name == "what",
    ));
}

#[test]
fn read_folder_recurse_works() {
    let files = read_folder_recurse("tests/assets/styles/").unwrap();

    println!("{:#?}", files);
    assert_eq!(files.len(), 2);
    assert_eq!(
        files.get("global").unwrap(),
        include_str!("../../tests/assets/styles/global.scss")
    );
    assert_eq!(
        files.get("scoped/stylish").unwrap(),
        include_str!("../../tests/assets/styles/scoped/stylish.scss")
    );
}

#[test]
fn clean_build_dir_works() {
    let config = Config {
        build: "tests/assets/build".to_string(),
        templates: "tests/assets/templates".to_string(),
        styles: "tests/assets/styles".to_string(),
        public: "tests/assets/public".to_string(),
        ..Config::default()
    };

    assert!(clean_build_dir(&config, false).is_ok());
}

#[test]
fn get_filename_works() {
    assert_eq!(get_filename("abc"), "abc");
    assert_eq!(get_filename("abc.txt"), "abc");
    assert_eq!(get_filename("abc.def.txt"), "abc");
    assert_eq!(get_filename(""), "");
}
