use serenity::framework::standard::CommandResult;
use serenity::{client::Context, model::channel::Message};
use sysinfo::System;

use crate::commands::helpers::command_formatter::format_uptime;
use crate::commands::helpers::command_formatter::vector_to_embed;

pub async fn uptime_command(ctx: &Context, msg: &Message) -> CommandResult {
    let uptime = vec![format!("**{}**", format_uptime(System::uptime()))];

    let embed = vector_to_embed(uptime, "System Uptime".to_string()).await;

    if let Err(err) = msg
        .channel_id
        .send_message(&ctx.http, |msg| msg.set_embed(embed))
        .await
    {
        println!("Failed to send uptime{:?}", err);
    }

    Ok(())
}
