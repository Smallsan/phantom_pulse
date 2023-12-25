use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::futures::lock;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::result;
use std::sync::Arc;
use sysinfo::{Components, Cpu, Disks, Networks, System};
use tokio::sync::MutexGuard;

use crate::tools::embed_builder::format_to_embed;
use crate::tools::uptime_formattter::format_uptime;
use crate::{ShardManagerContainer, SystemContainer};

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
async fn sysinfo(ctx: &Context, msg: &Message) -> CommandResult {
    let sys_info = vec![
        format!(
            "System name:             **{}**",
            System::host_name().unwrap()
        ),
        format!(
            "System OS:            **{} {}**",
            System::name().unwrap(),
            System::os_version().unwrap()
        ),
    ];

    let embed = format_to_embed(sys_info, "System Information".to_string()).await;

    if let Err(err) = msg
        .channel_id
        .send_message(&ctx.http, |msg| msg.set_embed(embed))
        .await
    {
        println!("Failed to send sysinfo{:?}", err);
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
async fn usage(ctx: &Context, msg: &Message) -> CommandResult {
    let system = fetch_system(ctx).await;

    let usage: Vec<String>;

    {
        let mut locked_system = system.lock().await;

        locked_system.refresh_all();

        usage = vec![
            format!(
                "CPU usage:             **{}%**",
                locked_system.cpus().first().unwrap().cpu_usage().round()
            ),
            format!(
                "RAM usage:             **{}GB/{}GB**",
                (locked_system.used_memory() as f64 / 1024_f64.powi(3)).round(),
                (locked_system.total_memory() as f64 / 1024_f64.powi(3)).round()
            ),
        ];
    }

    let embed = format_to_embed(usage, "System Usage".to_string()).await;

    if let Err(err) = msg
        .channel_id
        .send_message(&ctx.http, |msg| msg.set_embed(embed))
        .await
    {
        println!("Failed to send usage{:?}", err);
    }

    return Ok(());
}

async fn fetch_system(ctx: &Context) -> Arc<Mutex<System>> {
    let system_raw = {
        let data_read = ctx.data.read().await;

        data_read
            .get::<SystemContainer>()
            .expect("Expected System Container In TypeMap")
            .clone()
    };

    return system_raw;
}
