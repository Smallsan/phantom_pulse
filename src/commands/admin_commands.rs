use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use sysinfo::{Components, Cpu, Disks, Networks, System};

use crate::tools::uptime_formattter::format_uptime;
use crate::{ShardManagerContainer, MachineInfoContainer};
use crate::commands::command_functions::command_helper::*;
use crate::commands::command_functions::usage_command::usage_command;
use crate::commands::command_functions::sysinfo_command::sys_info_command;


#[command]
#[owners_only]
async fn quit(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        msg.reply(ctx, "Shutting down!").await?;

        manager.lock().await.shutdown_all().await;
    } else {
        msg.reply(ctx, "There was a problem getting the shard manager")
            .await?;

        return Ok(());
    }

    Ok(())
}

#[command]
#[owners_only]
async fn uptime(ctx: &Context, msg: &Message) -> CommandResult {
    let uptime = vec![format!("**{}**", format_uptime(System::uptime()))];

    let embed = format_to_embed(uptime, "System Uptime".to_string()).await;

    if let Err(err) = msg
        .channel_id
        .send_message(&ctx.http, |msg| msg.set_embed(embed))
        .await
    {
        println!("Failed to send uptime{:?}", err);
    }

    Ok(())
}


#[command]
#[owners_only]
async fn sysinfo(ctx: &Context, msg: &Message) -> CommandResult {
    sys_info_command(ctx, msg).await
}


#[command]
#[owners_only]
async fn usage(ctx: &Context, msg: &Message) -> CommandResult {
   usage_command(ctx, msg).await
}

