use crate::download::Config;
use std::fs;
use std::process::Command;

pub async fn launch(version: String, username: String, java: String) -> anyhow::Result<()> {
    println!("Launching version {version} as {username}");

    let cfg_file = fs::read_to_string(format!("versions/{version}/{version}-config.toml"))?;
    let config: Config = toml::from_str(&cfg_file)?;

    let profile_path = format!("profiles/{username}");
    fs::create_dir_all(&profile_path)?;

    let mut cmd = format!(
        "{} {} {} {}",
        java, config.jvm_opts, config.main, config.game_args
    );

    cmd = cmd
        .replace("${natives_directory}", &config.natives_directory)
        .replace("${launcher_name}", &config.launcher_name)
        .replace("${launcher_version}", &config.launcher_version)
        .replace("${log_path}", &config.log_path)
        .replace("${classpath}", &config.classpath.join(":"))
        .replace("${auth_player_name}", &username)
        .replace("${version_name}", &version)
        .replace("${game_directory}", &format!("../../{profile_path}"))
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

    Command::new("bash")
        .args(["-c", &cmd])
        .current_dir(format!("versions/{version}"))
        .spawn()
        .expect("failed to run minecraft");

    Ok(())
}

// fn create_command(exe: &str, args: Vec<&str>) -> anyhow::Result<Command> {
//     let mut full_command: Vec<String> = vec![];
//
//     full_command.push(exe.to_owned());
//     for arg in args {
//         full_command.push(arg.to_owned());
//     }
//
//     let mut command = Command::new(full_command[0].clone());
//     command.args(full_command[1..full_command.len()].to_vec());
//
//     Ok(command)
// }
