use std::{fs, path::Path};

use reqwest_dav::list_cmd::ListFolder;

use crate::ogre::should_debug;


pub fn create_folder_if_not_exists(
    local_path: &String,
    folder: ListFolder, 
    remote_path_root_folder: &String
) {
    let folder_name = Path::new(&folder.clone().href)
    .file_name()
    .map(|name| name.to_string_lossy().into_owned())
    .unwrap();
    if folder_name.clone() == *remote_path_root_folder {
        return;
    }
    let local_path = format!("{}/{}", local_path, folder_name.clone());
    let local_folder = Path::new(local_path.as_str());
    if local_folder.exists() {
        return;
    }
    let _ = fs::create_dir(local_path);
    if should_debug() {
        println!("Folder with name: {:#?} created successfully!", folder_name);
    }
}