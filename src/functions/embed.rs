use serenity::builder::CreateEmbed;
use serenity::model::Timestamp;

pub fn build_embed(
    title: Option<&str>,
    description: Option<&str>,
    color: Option<u32>,
    fields: Option<Vec<(&str, &str, bool)>>,
    include_timestamp: bool,
) -> CreateEmbed {
    let mut embed = CreateEmbed::default();

    if let Some(t) = title {
        embed = embed.title(t);
    }

    if let Some(desc) = description {
        embed = embed.description(desc);
    }

    if let Some(c) = color {
        embed = embed.color(c);
    }

    if let Some(flds) = fields {
        for (name, value, inline) in flds {
            embed = embed.field(name, value, inline);
        }
    }

    if include_timestamp {
        embed = embed.timestamp(Timestamp::now());
    }

    embed
}
