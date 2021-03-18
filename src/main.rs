use directories_next::ProjectDirs;
use serde::Deserialize;
use serenity::Client;

mod cogs;
mod db;

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct Config {
    secret: String,
}

#[tokio::main]
async fn main() {
    let config: String = ProjectDirs::from("", "", "valheim_manager")
        .map(|project_dirs| project_dirs.config_dir().to_owned())
        .map(|mut config_dir| {
            config_dir.push("config");
            config_dir.set_extension("toml");
            config_dir
        })
        .map(|config_path| {
            std::fs::read_to_string(config_path).unwrap_or_else(|e| {
                eprintln!("Failed to read config.toml with Error:\n{}", e);
                std::process::exit(-1)
            })
        })
        .unwrap_or_else(|| {
            eprintln!("Failed to get project directories!");
            std::process::exit(-1);
        });

    let config: Config = toml::from_str(&config).unwrap_or_else(|e| {
        eprintln!(
            "Failed to read config. Is it in the right format? Error:\n {}",
            e
        );
        std::process::exit(-1)
    });

    let mut client = Client::builder(&config.secret).await.unwrap();
    client.start().await.unwrap();
}
