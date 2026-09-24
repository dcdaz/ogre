use std::{error::Error, fs, io::Write, path::Path};

use chrono::{DateTime, Utc};
use filetime::{FileTime, set_file_mtime};
use reqwest_dav::{Client, list_cmd::ListFile};

use crate::ogre::should_debug;

pub async fn download_upload_file(
    local_path: &String,
    file: ListFile,
    remote_path_root_folder: &String,
    client: &Client
) -> Result<(), Box<dyn Error>> {
    let href = file.href.as_str();
    let remote_path = Path::new(href);
    let file_name = remote_path.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap();
    let parent_folder = remote_path.parent().unwrap().file_name().map(|name| name.to_string_lossy().into_owned()).unwrap();
    let local_file_path = if parent_folder == *remote_path_root_folder {
        format!("{}/{}", local_path, file_name.clone())
    } else {
        format!("{}/{}/{}", local_path, parent_folder, file_name.clone())
    };

    let local_modified_date: DateTime<Utc> = get_local_file_modified_date(&local_file_path);
    let should_debug = should_debug();

    if file.last_modified > local_modified_date {
        if should_debug {
            println!(
                "Remote file '{}' is newer than local with dates Remote:{} -- Local:{}",
                file_name,
                file.last_modified,
                local_modified_date
            );
        }
        download_file(href, file_name, &local_file_path, client).await?;
        set_date(&local_file_path.clone(), file.last_modified)?;
        Ok(())
    } else if  file.last_modified < local_modified_date {
        if should_debug {
            println!(
                "Local file '{}' is newer than remote with dates Remote:{} -- Local:{}",
                file_name,
                local_modified_date,
                file.last_modified
            );
        }
        upload_file(href, file_name, &local_file_path, client).await?;
        Ok(())
    } else {
        if should_debug {
            println!("No changes on file '{file_name}' between remote and local");
        }
        Ok(())
    }
}

fn get_local_file_modified_date(local_file_path: &String) -> DateTime<Utc> {
    let local_path = Path::new(local_file_path);
    if local_path.exists() {
        let local_file_metadata = fs::metadata(local_path).unwrap();
        local_file_metadata.modified().unwrap().into()
    } else {
        DateTime::<Utc>::MIN_UTC
    }
}

fn set_date(file_path: &String, date: DateTime<Utc>) -> Result<(), Box<dyn Error>> {
    set_file_mtime(file_path, FileTime::from_unix_time(date.timestamp(), date.timestamp_subsec_nanos()))?;
    Ok(())
}

async fn download_file(
    href: &str,
    file_name: String,
    local_file_path: &String,
    client: &Client
) -> Result<(), Box<dyn Error>> {
    println!("==> Downloading file '{file_name}'");
    let mut local_file = fs::File::create(local_file_path.clone())?;
    let mut file = client.get(href).await?;
    if !file.status().is_success() {
        return Err(format!("Download failed: {}", file.status()).into());
    }

    while let Some(chunk) = file.chunk().await? {
        local_file.write_all(&chunk)?;
    }
    local_file.flush()?;
    if should_debug() {
        println!("File with name: '{file_name}' downloaded successfully in path {local_file_path}");
    }
    Ok(())
}

async fn upload_file(
    href: &str,
    file_name: String,
    local_file_path: &String,
    client: &Client
) -> Result<(), Box<dyn Error>> {
    println!("==> Uploading file '{file_name}'");
    let local_file = fs::read(local_file_path)?;
    client.put(href, local_file).await?;
    Ok(())
}