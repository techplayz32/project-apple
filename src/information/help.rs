use serenity::builder::CreateMessage;
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::functions::embed::build_embed;

pub async fn run(ctx: &Context, msg: &Message, _args: &[&str]) {
    let mention = ctx.cache.current_user().mention();
    let mention_string = &format!("`pa!config` • Configure the {}", mention);

    let fields = vec![
        ("help", "`pa!help <command>` • Shows information about a specific command\n\
            `pa!help <feature>` • Shows information about a specific feature", true),

        ("listing", "`pa!list commands` • Lists all available commands\n\
            `pa!list features` • Lists all available features", true),

        ("configuration", mention_string, true),
        ("links", "`GitHub` • https://github.com/techplayz32/project-apple", true),
    ];

    let embed = build_embed(
        Some("overview"), 
        Some("quick overview of some commands:"), 
        Some(0x00_FFAA), 
        Some(fields), 
        true
    ); // overview 0x00_FFAA 

    let builder = CreateMessage::new()
        .embed(embed);
    let msg = msg.channel_id.send_message(&ctx.http, builder).await;

    if let Err(why) = msg {
        println!("Error sending message: {why:?}");
    }
}
