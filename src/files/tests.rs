use super::*;

#[test]
fn check_source_folders_works() {
    let mut config = Config::default();
    assert!(check_source_folders(&config).is_ok());

    config.templates = "what".to_string();
    assert!(matches!(
        check_source_folders(&config),
        Err(Error::SourceDirectoryNotExist(name)) if name == "what",
    ));
}

#[test]
fn read_folder_recurse_works() {
    let files = read_folder_recurse("styles/").unwrap();

    println!("{:#?}", files);
    assert_eq!(files.len(), 2);
    assert_eq!(
        files.get("global").unwrap(),
        include_str!("../../styles/global.scss")
    );
    assert_eq!(
        files.get("scoped/stylish").unwrap(),
        include_str!("../../styles/scoped/stylish.scss")
    );
}

#[test]
fn clean_build_dir_works() {
    assert!(clean_build_dir(&Config::default()).is_ok());
}

#[test]
fn get_filename_works() {
    assert_eq!(get_filename("abc"), "abc");
    assert_eq!(get_filename("abc.txt"), "abc");
    assert_eq!(get_filename("abc.def.txt"), "abc");
    assert_eq!(get_filename(""), "");
}
