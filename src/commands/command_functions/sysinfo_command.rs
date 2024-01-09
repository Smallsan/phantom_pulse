use std::sync::Arc;

use serenity::prelude::*;
use serenity::{client::Context, framework::standard::CommandResult, model::channel::Message};
use sysinfo::System;

use crate::commands::helpers::command_formatter::vector_to_embed;
use crate::commands::helpers::machine_fetcher::fetch_machine;
use crate::commands::helpers::unit_converter::bytes_to_gigabytes;
use crate::MachineInfoContainer;

pub async fn sys_info_command(ctx: &Context, msg: &Message) -> CommandResult {
    let machine = fetch_machine(ctx).await;
    let sys_info = fetch_sys_info(machine).await;

    if let Err(err) = send_sys_info(ctx, msg, sys_info).await {
        println!("Failed to send sysinfo: {:?}", err);
    }

    return Ok(());
}

async fn fetch_sys_info(machine: Arc<Mutex<MachineInfoContainer>>) -> Vec<String> {
    let mut locked_machine = machine.lock().await;

    locked_machine.system.refresh_all();

    locked_machine.disks.refresh();

    let system = &locked_machine.system;

    let disks = &locked_machine.disks;

    let mut disk_info: Vec<String> = vec![];

    for disk in disks.iter() {
        disk_info.push(format!(
            "{}: **{}GB**",
            disk.name().to_string_lossy(),
            bytes_to_gigabytes(disk.total_space())
        ));
    }

    let mut sys_info = vec![
        format!(
            "System name:             **{}**",
            System::host_name().unwrap()
        ),
        format!(
            "System OS:            **{} {}**",
            System::name().unwrap(),
            System::os_version().unwrap()
        ),
        format!(
            "Processor:         **{}**",
            system.cpus().first().unwrap().brand()
        ),
        format!(
            "Memory:         **{}GB**",
            bytes_to_gigabytes(system.total_memory())
        ),
    ];

    for info in disk_info {
        sys_info.push(info)
    }

    return sys_info;
}

async fn send_sys_info(
    ctx: &Context,
    msg: &Message,
    sys_info: Vec<String>,
) -> serenity::Result<()> {
    let embed = vector_to_embed(sys_info, "System Information".to_string()).await;
    msg.channel_id
        .send_message(&ctx.http, |msg| msg.set_embed(embed))
        .await?;
    return Ok(());
}
