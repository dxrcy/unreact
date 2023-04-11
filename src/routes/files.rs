use std::{collections::HashMap, fs, path::Path};

use crate::{error::MyResult, Error};

type FileMap = HashMap<String, String>;

/// Load all files of a folder recursively into an existing hashmap
///
/// - For every *file* in the given directory, read and insert to hashmap
/// - For every *folder* in the given directory, recurse this function, with the 'parent' folder as this folder
///
/// Returns `Err` if cannot read a file or folder children
pub fn load_filemap(root: &str) -> MyResult<FileMap> {
    let mut filemap = FileMap::new();
    load_filemap_recurse(&mut filemap, root, "")?;
    Ok(filemap)
}

//TODO docs
fn load_filemap_recurse(map: &mut FileMap, root: &str, parent: &str) -> MyResult<()> {
    // Full path relative to working directory
    let full_path = format!("{root}/{parent}/");

    // Children of current directory
    let children = try_else!(
        try fs::read_dir(&full_path),
        else err: throw!("Could not read directory '{}': {err}", full_path),
    );

    // Loop child files and folders
    for file in children.flatten() {
        // Get file path and file name
        let (path, full_name) = (file.path(), file.file_name());
        let Some((path, name)) = path.to_str().zip(full_name.to_str()) else {
            continue;
        };
        // Normalize slashes in path
        let path = path.replace('\\', "/");

        // If child is a folder, recurse this function
        if Path::new(&path).is_dir() {
            load_filemap_recurse(map, root, &format!("{parent}{name}/"))?;
            continue;
        }

        // Read file contents
        let content = try_else!(
            try fs::read_to_string(&path),
            else err: throw!("Could not read file '{}': {err}", path),
        );

        // Insert file and contents to hashmap
        map.insert(format!("{parent}{name}"), content);
    }

    Ok(())
}
