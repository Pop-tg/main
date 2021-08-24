use std::env;
use std::path::PathBuf;
use std::str::FromStr;

use clap::{AppSettings, Clap};
use colored::*;
use home::home_dir;
use log::{debug, info, warn};
use poptg::{
    gen_key, read_json, save_json, Client, Error, PostRequest, Response, Result, UrlStored,
};
use regex::Regex;

#[derive(Clap, Debug)]
#[clap(
    version = "1.0",
    author = "George Miao <gm@miao.dev>",
    about = "CLI for interacting with URL-shortener Pop.tg"
)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    command: SubCommand,
    #[clap(short, long, about = "Local file to keep history records")]
    local: Option<PathBuf>,
}

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
enum SubCommand {
    #[clap(about = "Create a new record")]
    New(NewOpt),
    #[clap(about = "List all records owned")]
    List,
}

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
struct NewOpt {
    #[clap(index = 1)]
    url: url::Url,
    #[clap(short, long, about = "Identifier of the record")]
    key: Option<Key>,
    #[clap(short, long, about = "Time to live, in minutes")]
    ttl: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Key(String);

impl FromStr for Key {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if Regex::new("^[a-zA-Z0-9_-]{3,12}$").unwrap().is_match(s) {
            Ok(Self(s.to_owned()))
        } else {
            Err(Error::BadKey)
        }
    }
}

trait Handled<T> {
    fn handled(self) -> Result<T>;
}

impl<T> Handled<T> for Response<T> {
    fn handled(self) -> Result<T> {
        match self {
            Self::Error {
                error_code, reason, ..
            } => Err(Error::ApiError(error_code, reason.join(", "))),
            Self::Success { result, .. } => Ok(result),
        }
    }
}

async fn exec() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    debug!("Opts: {:?}", opts);
    let file_path = opts
        .local
        .or_else(|| home_dir().and_then(|home| Some(home.join(".poptg.json"))))
        .expect("Cannot determine the path of storage file");
    info!("Local storage path: {:?}", file_path.display());
    let mut histories = read_json(&file_path)?;
    debug!("histories: {:#?}", &histories);
    let client = Client::new();

    match opts.command {
        SubCommand::New(new) => {
            let key = new.key.or_else(|| Some(Key(gen_key()))).unwrap();
            let req = PostRequest {
                key: key.0,
                value: new.url.to_string(),
                ttl: new.ttl,
            };
            debug!("Req: {:#?}", req);
            info!("Creating");
            let result = client.request(req).await?.handled()?;
            let new_url = format!("https://pop.tg/{}", result.key).green();
            info!("New record created: {} -> {}", new_url, result.value);
            histories.push(UrlStored {
                key: result.key,
                value: result.value,
                token: result.token,
                expire: result.expire,
            });
            save_json(file_path, &histories)?;
        }
        SubCommand::List => {
            for (index, history) in histories.iter().enumerate() {
                let pref = format!("[ Record #{} ]", index + 1).blue();
                let url = format!("https://pop.tg/{}", history.key);
                info!("{} {} -> {}", pref, url.green(), history.value.magenta())
            }
        }
    }
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env::var("RUST_LOG").unwrap_or_else(|_| {
        let default = "info".to_owned();
        env::set_var("RUST_LOG", &default);
        default
    });
    pretty_env_logger::init();
    match exec().await {
        Err(e) => warn!("{}", e),
        _ => {}
    }
}
