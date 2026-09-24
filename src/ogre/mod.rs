mod ogre;
mod file;
mod folder;

pub use self::ogre::*;

pub fn should_debug() -> bool {
    let should_debug_str = std::env::var("OGRE_DEBUG");
    match should_debug_str {
        Ok(str_bool) => matches!(str_bool.trim(), "true" | "TRUE" | "True"),
        Err(_) => false
    }
}
