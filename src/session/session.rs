use std::{fs, io::{Result, Write, stdin, stdout}, path::Path};

use crate::session::credentials::Credentials;

fn prompt(label: &str) -> Result<String> {
    print!("{label}");
    stdout().flush()?;
    let mut value = String::new();
    stdin().read_line(&mut value)?;
    Ok(value.trim_end().to_owned())
}

fn write_credentials(creds_file_path: &str) -> Result<()> {
    let username = prompt("Username: ")?;
    let pass = prompt("Password: ")?;
    let credentials = Credentials::new(username, pass);
    let mut session_file = fs::File::create(creds_file_path)?;
    session_file.write_all(credentials.encode().as_bytes())?;
    stdout().flush()?;
    Ok(())
}

fn write_login_prompt() {
    println!("------------------------------------------------------------");
    println!("Credentials were cleared or removed. You need to login again!");
    println!("------------------------------------------------------------");
}

pub fn login() -> Result<Credentials> {
    match dirs::cache_dir() {
        Some(path) => {
            let creds_file_path = format!(
                    "{}/{}",
                    path.as_os_str().to_str().unwrap(),
                    "ogre"
            );
            if Path::new(&creds_file_path).exists() {
                let encoded_credentials = fs::read_to_string(creds_file_path.clone())?;
                if encoded_credentials.is_empty() {
                    write_login_prompt();
                    write_credentials(&creds_file_path)?;
                    let encoded_credentials = fs::read_to_string(creds_file_path.clone())?;
                    Ok(Credentials::decode(encoded_credentials))
                } else {
                    return Ok(Credentials::decode(encoded_credentials));
                }
            } else {
                write_login_prompt();
                write_credentials(&creds_file_path)?;
                let encoded_credentials = fs::read_to_string(creds_file_path.clone())?;
                Ok(Credentials::decode(encoded_credentials))
            }
        }
        None => {
            eprintln!("Cache dir error");
            Ok(Credentials { username: "".to_string(), pass: "".to_string() })
        }
    }
}