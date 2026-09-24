use std::path::Path;
use reqwest_dav::{Auth, ClientBuilder, Depth, list_cmd::ListEntity};

use crate::{configuration::Configuration, ogre::{file, folder}, session::Credentials};

pub async fn sync(config: Configuration, credentials: Credentials) -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::new()
        .set_host(config.hostname)
        .set_auth(Auth::Basic(credentials.username, credentials.pass))
        .build()?;

    let remote_path_root_folder = Path::new(&config.remote_path).file_name().map(|path| path.to_string_lossy().into_owned()).unwrap();

    let data = &client.list(&config.remote_path, Depth::Infinity).await?;
    for d in data {
        if let ListEntity::Folder(folder) = d.clone() {
            folder::create_folder_if_not_exists(
                &config.local_path,
                folder,
                &remote_path_root_folder
            );
        }
        if let ListEntity::File(file) = d.clone() {
            file::download_upload_file(
                &config.local_path,
                file,
                &remote_path_root_folder,
                &client
            ).await?
        }
    }
    Ok(())
}
