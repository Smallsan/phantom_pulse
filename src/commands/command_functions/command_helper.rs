use std::sync::Arc;

use serenity::{client::Context, builder::CreateEmbed, utils::Colour};
use tokio::sync::Mutex;

use crate::MachineInfoContainer;

pub async fn fetch_machine(ctx: &Context) -> Arc<Mutex<MachineInfoContainer>> {
    let machine = {
        let data_read = ctx.data.read().await;

        data_read
            .get::<MachineInfoContainer>()
            .expect("Expected System Container In TypeMap")
            .clone()
    };

    return machine;
}


pub fn bytes_to_gigabytes(bytes: u64) -> f64 {
    return (bytes as f64 / 1024_f64.powi(3)).ceil()
}

pub async fn format_to_embed(desc: Vec<String>, title: String) -> CreateEmbed {
    let mut message_embed = CreateEmbed::default();

    let formatted_desc = desc.join("\n");

    message_embed
        .title(title)
        .description(formatted_desc)
        .colour(Colour::DARK_GREY);

    return message_embed;
}