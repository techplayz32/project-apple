pub mod prefix;

use serenity::prelude::*;
use serenity::model::channel::Message;

pub fn owns(cmd: &str) -> bool {
    matches!(cmd, "prefix")
}

pub async fn dispatch(ctx: &Context, msg: &Message, cmd: &str, args: &[&str]) {
    match cmd {
        "prefix" => prefix::run(ctx, msg, args).await,
        _ => {},
    }
}
