use serenity::{builder::CreateEmbed, utils::Colour};

pub async fn vector_to_embed(desc: Vec<String>, title: String) -> CreateEmbed {
    let mut message_embed = CreateEmbed::default();

    let formatted_desc = desc.join("\n");

    message_embed
        .title(title)
        .description(formatted_desc)
        .colour(Colour::DARK_GREY);

    return message_embed;
}

pub fn format_uptime(total_seconds: u64) -> String {
    let days = total_seconds / (24 * 60 * 60);

    let hours = (total_seconds % (24 * 60 * 60)) / (60 * 60);

    let minutes = (total_seconds % (60 * 60)) / 60;

    let seconds = total_seconds % 60;

    let mut result = String::new();

    if days > 0 {
        result.push_str(&format!("{} days ", days));
    }

    if hours > 0 {
        result.push_str(&format!("{} hours ", hours));
    }

    if minutes > 0 {
        result.push_str(&format!("{} minutes ", minutes));
    }

    if seconds > 0 {
        result.push_str(&format!("{} seconds", seconds));
    }

    let uptime = result.trim().to_string();

    return uptime;
}
