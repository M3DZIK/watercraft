use std::fs;
use std::process::Command;

use crate::config::{profile_dir, version_dir, Config};

pub async fn launch(version: String, username: String, java: Option<String>) -> anyhow::Result<()> {
    println!("Launching version {version} as {username}");

    let version_dir = version_dir!(version);

    // parse config file
    let cfg_file = fs::read_to_string(format!("{version_dir}/config.toml"))?;
    let config: Config = toml::from_str(&cfg_file)?;

    // create the profile directory if it doesn't exist
    let profile_dir = profile_dir!(username);
    fs::create_dir_all(&profile_dir)?;

    let mut cmd = format!(
        "{java} {jvm_opts} {main} {game_args}",
        java = java.unwrap_or(config.java),
        jvm_opts = config.jvm_opts,
        main = config.main,
        game_args = config.game_args
    );

    // replace the variables in the command with the correct values from the config file
    cmd = cmd
        .replace("${natives_directory}", &config.natives_directory)
        .replace("${launcher_name}", &config.launcher_name)
        .replace("${launcher_version}", &config.launcher_version)
        .replace("${log_path}", &config.log_path)
        .replace("${classpath}", &config.classpath.join(":"))
        .replace("${auth_player_name}", &username)
        .replace("${version_name}", &version)
        .replace("${game_directory}", &profile_dir)
        .replace("${assets_root}", &config.assets_root)
        .replace("${assets_index_name}", &config.assets_index_name)
        .replace("${auth_uuid}", &config.auth_uuid.to_string())
        .replace(
            "${auth_access_token}",
            &config.auth_access_token.to_string(),
        )
        .replace("${clientid}", &config.clientid.to_string())
        .replace("${auth_xuid}", &config.auth_xuid.to_string())
        .replace("${user_type}", &config.user_type)
        .replace("${version_type}", &config.version_type);

    let command_vec: Vec<&str> = cmd.split_whitespace().collect();

    Command::new(command_vec[0])
        .args(&command_vec[1..])
        .current_dir(version_dir)
        .spawn()?
        .wait()?;

    Ok(())
}
