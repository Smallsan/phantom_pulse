use std::sync::Arc;

use serenity::{client::Context, model::channel::Message, framework::standard::CommandResult};
use tokio::sync::Mutex;

use crate::{commands::command_functions::command_helper::*, MachineInfoContainer};

pub async fn usage_command(ctx: &Context, msg: &Message) -> CommandResult {

    let machine = fetch_machine(ctx).await;

    let usage = fetch_usage(machine).await;

    if let Err(err) = send_usage(ctx, msg, usage).await {
        println!("Failed to send sysinfo: {:?}", err);
    }

    return Ok(());

}

async fn fetch_usage(machine: Arc<Mutex<MachineInfoContainer>>) -> Vec<String> {
    let usage: Vec<String>;

        let mut locked_machine = machine.lock().await;

        locked_machine.system.refresh_all();

        let system = &locked_machine.system;

        usage = vec![
            format!(
                "CPU usage:             **{}%**",
                system.cpus().first().unwrap().cpu_usage().round()
            ),
            format!(
                "RAM usage:             **{}GB/{}GB**",
                bytes_to_gigabytes(system.used_memory()),
                bytes_to_gigabytes(system.total_memory())
            ),
        ];

        return usage;
    }

async fn send_usage(ctx: &Context, msg: &Message, usage:Vec<String>) -> CommandResult{
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