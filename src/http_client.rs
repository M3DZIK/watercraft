use once_cell::sync::Lazy;
use reqwest::Client;

pub static CLIENT: Lazy<Client> = Lazy::new(Client::new);

/// Build a request to the given URL.
macro_rules! build_request {
    ($method: expr, $uri: expr) => {
        crate::http_client::CLIENT
            .request($method, $uri)
            .header(
                "User-Agent",
                format!("Watercraft/{:?}", env!("CARGO_PKG_VERSION")),
            )
            .build()
    };
}

/// Send a request to the given URL.
macro_rules! send_http {
    ($req: expr) => {
        crate::http_client::CLIENT.execute($req)
    };
    ($method: expr, $uri: expr) => {
        crate::http_client::CLIENT.execute(crate::http_client::build_request!($method, $uri)?)
    };
}

/// Download a file from the given URL.
macro_rules! download {
    ($uri: expr) => {
        // get the file content from the URL
        std::io::Cursor::new(send_http!(Method::GET, $uri).await?.bytes().await?)
    };
    ($writer: expr, $uri: expr) => {
        // get the file content from the URL
        let mut res = crate::http_client::download!($uri);

        // write content to writer
        std::io::copy(&mut res, $writer)?;
    };
}

macro_rules! download_file {
    ($uri: expr, $size: expr, $($path:tt)*) => {
        let path = format!($($path)*);

        if std::path::Path::new(&path).exists() {
            println!(
                "{GREY}{path} {GREEN}already exists, skipping download{RESET}",
                GREY = crate::colors::GREY,
                GREEN = crate::colors::GREEN,
                RESET = crate::colors::RESET
            );
        } else {
            let unit = byte_unit::Byte::from_bytes($size.into()).get_appropriate_unit(false).to_string();
            println!(
                "{MAGENTA}Downloading {GREY}{path} ... {MAGENTA}({unit}){RESET}",
                MAGENTA = crate::colors::MAGENTA,
                GREY = crate::colors::GREY,
                RESET = crate::colors::RESET
            );

            // create the directory if it doesn't exist
            let mut dir = path.split('/').collect::<Vec<&str>>();
            dir.pop();
            std::fs::create_dir_all(dir.join("/"))?;

            // create the file and open it for writing
            let mut file = fs::File::create(format!("{}", path))?;

            crate::http_client::download!(&mut file, $uri);
        }
    };
}

macro_rules! download_and_extract {
    ($uri: expr, $size: expr, $($path:tt)*) => {
        let path = format!($($path)*);

        let unit = byte_unit::Byte::from_bytes($size.into()).get_appropriate_unit(false).to_string();
        println!(
            "{MAGENTA}Downloading and Extracting {GREY}{url} ... {MAGENTA}({unit}){RESET}",
            url = $uri,
            MAGENTA = crate::colors::MAGENTA,
            GREY = crate::colors::GREY,
            RESET = crate::colors::RESET
        );

        // create the directory if it doesn't exist
        let mut dir = path.split('/').collect::<Vec<&str>>();
        dir.pop();
        std::fs::create_dir_all(dir.join("/"))?;

        let res = crate::http_client::download!($uri);

        // extract the zip archive
        let mut archive = zip::ZipArchive::new(res)?;

        archive.extract(path)?;
    };
}

pub(crate) use build_request;
pub(crate) use download;
pub(crate) use download_and_extract;
pub(crate) use download_file;
pub(crate) use send_http;
