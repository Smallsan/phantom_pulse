use serenity::builder::CreateEmbed;
use serenity::utils::Colour;

pub async fn format_to_embed(desc: Vec<String>, title: String) -> CreateEmbed {
    let mut message_embed = CreateEmbed::default();

    let formatted_desc = desc.join("\n");

    message_embed
        .title(title)
        .description(formatted_desc)
        .colour(Colour::DARK_GREY);

    return message_embed;
}
