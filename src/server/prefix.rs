use serde::Deserialize;
use serenity::builder::CreateMessage;
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::functions::embed::build_embed;

use config::Config;

#[derive(Debug, Deserialize)]
struct DiscordConfig {
    prefix: String,
}

#[derive(Debug, Deserialize)]
struct Settings {
    discord: DiscordConfig,
}

pub async fn run(ctx: &Context, msg: &Message, _args: &[&str]) {
    let settings: Settings = Config::builder()
            .add_source(config::File::with_name("config"))
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap();
    
    let prefix_string = &format!("my prefix is: {}", settings.discord.prefix);

    let embed = build_embed(
        None,
        Some(prefix_string), 
        Some(0x00_FFAA), 
        None,
        true
    );

    let builder = CreateMessage::new()
        .embed(embed);
    let msg = msg.channel_id.send_message(&ctx.http, builder).await;

    if let Err(why) = msg {
        println!("Error sending message: {why:?}");
    }
}
