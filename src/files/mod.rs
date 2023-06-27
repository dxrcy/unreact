#[cfg(test)]
mod tests;

use std::{fs, path::Path};

use crate::{Config, Error, FileMap};

/// Returns `Err` if source folders are not found in the working directory
pub fn check_source_folders(config: &Config) -> Result<(), Error> {
    let src_folders = [&config.templates, &config.public, &config.styles];
    for folder in src_folders {
        if !Path::new(&folder).is_dir() {
            return Err(Error::SourceDirectoryNotExist(folder.to_string()));
        }
    }
    Ok(())
}

/// Read a folder recursively, and read every file contents
///
/// Returns a hashmap of filepath strings (relative to the given directory), and file contents
///
/// Returns `Err` if cannot read a file or folder children
pub fn read_folder_recurse(folder: &str) -> Result<FileMap, Error> {
    let mut filemap = FileMap::new();
    load_filemap(&mut filemap, folder, "")?;
    Ok(filemap)
}

/// Load all files of a folder recursively into an existing hashmap
///
/// - For every *file* in the given directory, read and insert to hashmap
/// - For every *folder* in the given directory, recurse this function, with the 'parent' folder as this folder
///
/// Returns `Err` if cannot read a file or folder children
fn load_filemap(map: &mut FileMap, root: &str, parent: &str) -> Result<(), Error> {
    // Full path relative to working directory
    let full_path = format!("{root}/{parent}/");

    // Children of current directory
    let children = try_unwrap!(
        fs::read_dir(&full_path),
        else Err(err) => return io_fail!(ReadDir, full_path, err),
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
            load_filemap(map, root, &format!("{parent}{name}/"))?;
            continue;
        }

        // Get name (not file extension) of child file
        let name = get_filename(name);

        // Read file contents
        let content = try_unwrap!(
            fs::read_to_string(&path),
            else Err(err) => return io_fail!(ReadFile, path, err),
        );

        // Insert file and contents to hashmap
        map.insert(format!("{parent}{name}"), content);
    }

    Ok(())
}

/// Remove files recursively from build directory, and create empty folders to be filled
///
/// All paths are treated relative to working directory
///
/// 1. Removes build folder (`./build/` or otherwise specified), if exists
/// 2. Creates build folder
/// 3. Creates `styles/` and `public/` inside build folder
/// 4. Copies all files from public source folder into `public/` inside build folder
pub fn clean_build_dir(config: &Config, is_dev: bool) -> Result<(), Error> {
    // Remove build folder (if exists)
    if Path::new(&config.build).exists() {
        try_unwrap!(
            fs::remove_dir_all(&config.build),
            else Err(err) => return io_fail!(RemoveDir, config.build.clone(), err),
        );
    }

    // Create output folders (build and subfolders)
    let out_folders = ["", "styles", "public"];
    for folder in out_folders {
        let path = format!("{}/{}/", config.build, folder);

        try_unwrap!(
            fs::create_dir_all(&path),
            else Err(err) => return io_fail!(CreateDir, path, err),
        );
    }

    // Public directory
    // Only *copy* directory in production
    if !is_dev {
        // Recursively copy public directory
        try_unwrap!(
            dircpy::copy_dir(&config.public, format!("{}/public", config.build)),
            else Err(err) => return io_fail!(CopyDir, config.public.clone(), err),
        );
    } else {
        // Create dummy note file
        try_unwrap!(
            fs::write(format!("{}/public/EMPTY", config.build), "'public' folder should always be empty in dev mode"),
            else Err(err) => return io_fail!(CopyDir, config.public.clone(), err),
        )
    }

    Ok(())
}

/// Get file 'name' from full file
///
/// Returns everything before the first `.` period
///
/// Returns empty string if nothing found
pub fn get_filename(full_name: &str) -> &str {
    full_name.split('.').next().unwrap_or("")
}
