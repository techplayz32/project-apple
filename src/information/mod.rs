pub mod help;

use serenity::prelude::*;
use serenity::model::channel::Message;

pub fn owns(cmd: &str) -> bool {
    matches!(cmd, "help")
}

pub async fn dispatch(ctx: &Context, msg: &Message, cmd: &str, args: &[&str]) {
    match cmd {
        "help" => help::run(ctx, msg, args).await,
        _ => {},
    }
}
