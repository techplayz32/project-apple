use config::Config;
use serde::Deserialize;

use serenity::{
    async_trait,
    model::channel::Message,
    prelude::*,
    all::GatewayIntents,
};

mod information;
mod server;
mod functions;
struct Handler;

// For 'Config.toml' file, gotta put it
#[derive(Debug, Deserialize)]
struct DiscordConfig {
    token: String,
    prefix: String,
}

#[derive(Debug, Deserialize)]
struct Settings {
    discord: DiscordConfig,
}

// Main thingy thingy

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let settings: Settings = Config::builder()
            .add_source(config::File::with_name("config"))
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap();

        if msg.author.bot { return; }

        // simple split: "!cmd arg1 arg2"
        let content = msg.content.trim();
        let prefix = settings.discord.prefix;
        let (cmd, args) = if let Some(rest) = content.strip_prefix(&prefix) {
            let mut parts = rest.split_whitespace();
            let c = parts.next().unwrap_or("");
            (c, parts.collect::<Vec<_>>())
        } else {
            return;
        };

        // route to the right category
        if information::owns(cmd) {
            information::dispatch(&ctx, &msg, cmd, &args).await;
        } else if server::owns(cmd) {
            server::dispatch(&ctx, &msg, cmd, &args).await;
        }
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let settings: Settings = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap();

    let token = settings.discord.token;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}