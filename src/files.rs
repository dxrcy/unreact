use std::{fs, path::Path};

use crate::error::MyResult;

pub fn create_build_dir(path: &str) -> MyResult {
    if Path::new(path).exists() {
        try_else!(
            try fs::remove_dir_all(path),
            else err: throw!("[io] Failed to remove directory at '{}': {err:?}", path),
        );
    }

    try_else!(
        try fs::create_dir_all(path),
        else err: throw!("[io] Failed to create directory at '{}': {err:?}", path),
    );

    Ok(())
}

pub fn create_dir_all_for_file(path: &str) -> MyResult {
    let mut split = path.split('/');
    split.next_back();

    let path = split.collect::<Vec<&str>>().join("/");

    try_else!(
        try fs::create_dir_all(&path),
        else err: throw!("[io] Failed to recursively create directory at '{}': {err:?}", path),
    );

    Ok(())
}
