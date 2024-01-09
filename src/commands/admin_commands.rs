use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::command_functions::sysinfo_command::sys_info_command;
use crate::commands::command_functions::uptime_command::uptime_command;
use crate::commands::command_functions::usage_command::usage_command;
use crate::ShardManagerContainer;

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
    uptime_command(ctx, msg).await
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
