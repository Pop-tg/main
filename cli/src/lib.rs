pub mod api;
pub mod error;
pub mod model;

use std::borrow::Cow;
use std::fmt::Display;

pub use api::*;
pub use error::*;
pub use model::*;

pub struct Linked {
    url: String,
    text: String,
}

impl Display for Linked {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\u{1b}]8;;{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\",
            self.url, self.text
        )
    }
}
pub trait Link: Sized {
    fn link<T>(self, url: T) -> Linked
    where
        std::string::String: From<Self>,
        T: Into<String>,
    {
        let url: String = url.into();
        let text: String = self.into();
        Linked { url, text }
    }
}

impl Link for String {}

impl Link for &str {}

impl Link for Cow<'_, String> {}

impl Link for Cow<'_, &str> {}

#[cfg(feature = "token")]
pub fn gen_key() -> String {
    use rand::{distributions::Alphanumeric, Rng};
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(4)
        .map(char::from)
        .collect()
}

#[cfg(feature = "file")]
pub mod file {

    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    use crate::{Error, Result, UrlStored};

    pub fn read_json<T: AsRef<Path>>(dir: T) -> Result<Vec<UrlStored>> {
        use log::info;

        let path = dir.as_ref();

        if path.exists() {
            Ok(
                serde_json::from_reader(File::open(path).map_err(Error::ReadError)?)
                    .map_err(Error::Parse)?,
            )
        } else {
            info!("{} does not exist, creating", &path.display());
            let mut file = File::create(&path).map_err(Error::ReadError)?;
            file.write_all(b"[]").map_err(Error::ReadError)?;
            Ok(Vec::new())
        }
    }

    pub fn save_json<T: AsRef<Path>>(dir: T, value: &Vec<UrlStored>) -> Result<()> {
        let path = dir.as_ref();
        let file = File::create(&path).map_err(Error::WriteError)?;
        serde_json::to_writer(file, &value).map_err(Error::Serialize)?;
        Ok(())
    }

    #[test]
    fn test_read_json() {
        use log::info;
        use std::env;

        env::set_var("RUST_LOG", "debug");
        pretty_env_logger::init();
        info!("{:#?}", read_json("/tmp/test.json").unwrap())
    }
}

#[cfg(feature = "file")]
pub use file::*;
