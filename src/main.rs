use crate::configuration::Configuration;

mod configuration;
mod ogre;
mod session;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration::new();
    let credentials = session::login()?;
    ogre::sync(config, credentials).await?;
    Ok(())
}
