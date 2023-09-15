use dotenv::from_path;
use std::{env, path::Path};

const DEFAULT_CONFIG_PATH: &str = "../.env";

#[derive(Debug)]
pub struct Config {
    pub discord_app_id: String,
    pub discord_pub_key: String,
    pub discord_bot_token: String,
    pub discord_bot_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        from_path(Path::new(DEFAULT_CONFIG_PATH)).ok();

        return Config {
            discord_app_id: mandatory_env_var("DISCORD_APP_ID"),
            discord_pub_key: mandatory_env_var("DISCORD_PUB_KEY"),
            discord_bot_token: mandatory_env_var("DISCORD_BOT_TOKEN"),
            discord_bot_url: mandatory_env_var("DISCORD_BOT_URL"),
        };
    }
}

fn mandatory_env_var(var_name: &str) -> String {
    env::var(var_name).unwrap_or_else(|_| {
        panic!(
            "\n\n⚠️ Missing config in {}.\n{} must be set\n\n",
            Path::new(DEFAULT_CONFIG_PATH)
                .canonicalize()
                .unwrap()
                .display(),
            var_name
        )
    })
}
