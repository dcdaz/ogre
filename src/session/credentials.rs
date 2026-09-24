use data_encoding::BASE64;

pub struct Credentials {
    pub username: String,
    pub pass: String,
}

impl Credentials {

    pub fn new(username: String, pass: String) -> Self {
        Credentials { username: username, pass: pass }
    }

    pub fn encode(&self) -> String {
        let encoded_username = BASE64.encode(self.username.as_bytes());
        let encoded_pass = BASE64.encode(self.pass.as_bytes());
        format!("{encoded_username}\n{encoded_pass}")
    }

    pub fn decode(encoded_creds: String) -> Self {
        let creds = encoded_creds.lines().collect::<Vec<_>>();
        if creds.len() != 2 {
            panic!("Credentials cache is corrupted, size {}", creds.len());
        }
        let username = String::from_utf8(BASE64.decode(creds[0].as_bytes()).unwrap()).unwrap();
        let pass = String::from_utf8(BASE64.decode(creds[1].as_bytes()).unwrap()).unwrap();
        Credentials { username: username, pass: pass }
    }
}