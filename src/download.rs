use crate::config::{
    assets_dir, assets_indexes_dir, assets_objects_dir, libraries_dir, libraries_natives_dir,
    version_dir, Config,
};
use anyhow::anyhow;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;

use crate::http_client::{download_and_extract, download_file, send_http};

const MAINLINE_VERSIONS: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";
const OBJ_SERVER: &str = "https://resources.download.minecraft.net";

#[cfg(target_os = "linux")]
const OS_NAME: &str = "linux";
#[cfg(target_os = "windows")]
const OS_NAME: &str = "windows";
#[cfg(target_os = "macos")]
const OS_NAME: &str = "osx";

#[derive(Debug, Serialize, Deserialize)]
struct VersionManifest {
    latest: VersionManifestLatest,
    versions: Vec<VersionManifestVersions>,
}
#[derive(Debug, Serialize, Deserialize)]
struct VersionManifestLatest {
    release: String,
    snapshot: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct VersionManifestVersions {
    id: String,
    #[serde(rename = "type")]
    typ: String,
    url: String,
    time: String,
    #[serde(rename = "releaseTime")]
    release_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct VersionDetails {
    id: String,
    assets: String,
    #[serde(rename = "mainClass")]
    main_class: String,
    arguments: Option<VersionDetailsArguments>,
    #[serde(rename = "minecraftArguments")]
    minecraft_arguments: Option<String>,
    #[serde(rename = "assetIndex")]
    asset_index: VersionDetailsAssetIndex,
    downloads: VersionDetailsDownloads,
    libraries: Vec<VersionDetailsLibraries>,
    logging: VersionDetailsLogging,
    #[serde(rename = "releaseTime")]
    release_time: String,
    time: String,
    #[serde(rename = "type")]
    typ: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct VersionDetailsArguments {
    game: Vec<VersionDetailsArgumentsGame>,
    // jvm
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum VersionDetailsArgumentsGame {
    String(String),
    Rule(VersionDetailsArgumentsGameCustom),
}
#[derive(Debug, Serialize, Deserialize)]
struct VersionDetailsArgumentsGameCustom {
    rules: Vec<VersionDetailsArgumentsGameCustomRule>,
}
#[derive(Debug, Serialize, Deserialize)]
struct VersionDetailsArgumentsGameCustomRule {
    action: String,
    features: HashMap<String, bool>,
}
#[derive(Debug, Serialize, Deserialize)]
struct VersionDetailsDownloads {
    client: VersionDetailsDownloadsEntry,
    server: VersionDetailsDownloadsEntry,
}
#[derive(Debug, Serialize, Deserialize)]
struct VersionDetailsDownloadsEntry {
    sha1: String,
    size: u32,
    url: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct VersionDetailsLibraries {
    name: String,
    downloads: VersionDetailsLibrariesDownloads,
    rules: Option<Vec<VersionDetailsLibrariesDownloadsRules>>,
    natives: Option<HashMap<String, String>>,
}
#[derive(Debug, Serialize, Deserialize)]
struct VersionDetailsLibrariesDownloads {
    artifact: Option<VersionDetailsLibrariesDownloadsArtifact>,
    classifiers: Option<HashMap<String, VersionDetailsLibrariesDownloadsArtifact>>,
}
#[derive(Debug, Serialize, Deserialize)]
struct VersionDetailsLibrariesDownloadsArtifact {
    path: String,
    sha1: String,
    size: u32,
    url: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct VersionDetailsLibrariesDownloadsRules {
    action: String,
    os: Option<VersionDetailsLibrariesDownloadsRulesOs>,
}
#[derive(Debug, Serialize, Deserialize)]
struct VersionDetailsLibrariesDownloadsRulesOs {
    name: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct VersionDetailsLogging {
    client: VersionDetailsLoggingClient,
}
#[derive(Debug, Serialize, Deserialize)]
struct VersionDetailsLoggingClient {
    argument: String,
    #[serde(rename = "type")]
    typ: String,
    file: VersionDetailsLoggingClientFile,
}
#[derive(Debug, Serialize, Deserialize)]
struct VersionDetailsLoggingClientFile {
    id: String,
    sha1: String,
    size: u32,
    url: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct VersionDetailsAssetIndex {
    id: String,
    sha1: String,
    size: u32,
    #[serde(rename = "totalSize")]
    total_size: u32,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AssetsIndex {
    objects: HashMap<String, AssetsIndexObject>,
}
#[derive(Debug, Serialize, Deserialize)]
struct AssetsIndexObject {
    hash: String,
    size: u64,
}

pub async fn download(version: String) -> anyhow::Result<()> {
    println!("Downloading version {version} ...");

    // get the version manifest from mojang servers
    let res = send_http!(Method::GET, MAINLINE_VERSIONS).await?;

    // parse the version manifest
    let manifest = res.json::<VersionManifest>().await?;

    // find the version we want in the manifest
    let version_manifest = manifest.versions.iter().find(|x| x.id == version);

    // if the version is not found, return an error
    if version_manifest.is_none() {
        let available_versions = manifest
            .versions
            .iter()
            .filter(|x| x.typ == "release")
            .map(|x| x.id.clone())
            .collect::<Vec<String>>()
            .join("\n");

        return Err(anyhow!(
            "No mainline version {} exists.\nAvailable versions:\n{}",
            version,
            available_versions
        ));
    }

    // get the version details from mojang servers
    let res = send_http!(Method::GET, version_manifest.unwrap().url.clone()).await?;

    // parse the version details
    let version_details = res.json::<VersionDetails>().await?;

    let version_dir = version_dir!(version);
    let libraries_dir = libraries_dir!(version);
    let libraries_natives_dir = libraries_natives_dir!(version);
    let assets_dir = assets_dir!();
    let assets_indexes_dir = assets_indexes_dir!();
    let assets_objects_dir = assets_objects_dir!();

    // create the version directory
    fs::create_dir_all(&version_dir)?;

    // download minecraft client jar
    download_file!(
        version_details.downloads.client.url,
        version_details.downloads.client.size,
        "{version_dir}/{version}.jar",
    );

    // download assets index
    download_file!(
        version_details.asset_index.url,
        version_details.asset_index.size,
        "{assets_indexes_dir}/{id}.json",
        id = version_details.asset_index.id
    );

    // download logging client file
    download_file!(
        version_details.logging.client.file.url,
        version_details.logging.client.file.size,
        "{version_dir}/logging-{id}",
        id = version_details.logging.client.file.id
    );

    let mut classpath = vec![];

    // download libraries
    for lib in version_details.libraries {
        // if the library has rules, check if the rules apply to this system
        if lib.rules.is_some() {
            let mut allowed = "disallow".to_string();

            for rule in lib.rules.unwrap() {
                match &rule.os {
                    Some(os) => {
                        if os.name == OS_NAME {
                            allowed = rule.action;
                            break;
                        }
                    }
                    None => allowed = rule.action,
                }
            }

            if allowed != "allow" {
                continue;
            }
        }

        // download the library artifact if it exists
        if lib.downloads.artifact.is_some() {
            let lib = lib.downloads.artifact.unwrap();

            download_file!(
                lib.url,
                lib.size,
                "{libraries_dir}/{lib_path}",
                lib_path = lib.path
            );

            classpath.push(format!("libraries/{lib_path}", lib_path = lib.path));
        }

        // download the library classifiers (natives) if they exist
        if lib.downloads.classifiers.is_some() {
            let lib_natives = lib.natives.unwrap();
            let natives = lib_natives.get(OS_NAME);

            if natives.is_none() {
                continue;
            }

            let classifier = lib.downloads.classifiers.unwrap();
            let artifact = classifier.get(natives.unwrap());

            if artifact.is_some() {
                let artifact = artifact.unwrap();

                download_and_extract!(&artifact.url, artifact.size, "{libraries_natives_dir}");
            }
        }
    }

    // open the assets index file
    let file = File::open(format!(
        "{assets_indexes_dir}/{id}.json",
        id = version_details.asset_index.id
    ))?;

    // parse the assets index
    let assets_index: AssetsIndex = serde_json::from_reader(file)?;

    // download assets objects
    for (path, object) in assets_index.objects {
        // get the first two characters of the hash
        let id = object.hash.chars().take(2).collect::<String>();

        download_file!(
            format!("{OBJ_SERVER}/{id}/{hash}", hash = object.hash),
            object.size,
            "{assets_objects_dir}/{path}"
        );
    }

    let game_args = if version_details.minecraft_arguments.is_some() {
        version_details.minecraft_arguments.unwrap()
    } else {
        let mut args = vec![];

        for arg in version_details.arguments.unwrap().game {
            if let VersionDetailsArgumentsGame::String(x) = arg {
                args.push(x)
            }
        }

        args.join(" ")
    };

    classpath.push(format!("{version}.jar"));

    let config = Config {
        version: version.clone(),
        assets_root: assets_dir,
        auth_uuid: 0,
        auth_access_token: 0,
        clientid: 0,
        auth_xuid: 0,
        version_type: version_manifest.unwrap().typ.clone(),
        user_type: "legacy".to_string(),
        launcher_name: "minecraft-launcher".to_string(),
        launcher_version: "2.1.1349".to_string(),
        main: version_details.main_class,
        assets_index_name: version_details.asset_index.id.clone(),
        natives_directory: "libraries-natives".to_string(),
        log_path: format!("logging-{id}", id = version_details.logging.client.file.id),
        classpath,
        java: "".to_string(),
        jvm_opts: "-Xss1M -Djava.library.path=${natives_directory} -Dminecraft.launcher.brand=${launcher_name} -Dminecraft.launcher.version=${launcher_version} -Dlog4j.configurationFile=${log_path} -cp ${classpath}".to_string(),
        game_args,
    };

    let config_str = toml::to_string(&config)?;

    let mut config_file = File::create(format!("{version_dir}/config.toml"))?;
    config_file.write_all(config_str.as_bytes())?;

    Ok(())
}
