use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub static HOME: Lazy<String> = Lazy::new(|| {
    let path_buf = dirs::home_dir().unwrap();
    let path_str = path_buf.to_str();
    path_str.unwrap().to_string()
});

macro_rules! game_dir {
    () => {
        format!("{home}/.minecraft", home = *crate::config::HOME)
    };
    ($($path:tt)*) => {
        format!("{home}/.minecraft/{}", format!($($path)*), home = *crate::config::HOME)
    };
}

macro_rules! version_dir {
    ($version: expr) => {
        crate::config::game_dir!("versions/{version}", version = $version)
    };
}

macro_rules! libraries_dir {
    ($version: expr) => {
        crate::config::game_dir!("versions/{version}/libraries", version = $version)
    };
}

macro_rules! libraries_natives_dir {
    ($version: expr) => {
        crate::config::game_dir!("versions/{version}/libraries-natives", version = $version)
    };
}

macro_rules! assets_dir {
    () => {
        format!("{home}/.minecraft/assets", home = *crate::config::HOME)
    };
    ($path: expr) => {
        format!(
            "{home}/.minecraft/assets/{path}",
            home = *crate::config::HOME,
            path = $path
        )
    };
}

macro_rules! assets_indexes_dir {
    () => {
        crate::config::assets_dir!("indexes")
    };
}

macro_rules! assets_objects_dir {
    () => {
        crate::config::assets_dir!("objects")
    };
}

macro_rules! profile_dir {
    ($username: expr) => {
        crate::config::game_dir!("profiles/{username}", username = $username)
    };
}

pub(crate) use assets_dir;
pub(crate) use assets_indexes_dir;
pub(crate) use assets_objects_dir;
pub(crate) use game_dir;
pub(crate) use libraries_dir;
pub(crate) use libraries_natives_dir;
pub(crate) use profile_dir;
pub(crate) use version_dir;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub assets_root: String,
    pub auth_uuid: u64,
    pub auth_access_token: u64,
    pub clientid: u64,
    pub auth_xuid: u64,
    pub version_type: String,
    pub user_type: String,
    pub launcher_name: String,
    pub launcher_version: String,
    pub main: String,
    pub assets_index_name: String,
    pub natives_directory: String,
    pub log_path: String,
    pub classpath: Vec<String>,
    pub java: String,
    pub jvm_opts: String,
    pub game_args: String,
}
