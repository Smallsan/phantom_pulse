use std::collections::HashSet;
use std::sync::Arc;

use serenity::client::bridge::gateway::ShardManager;
use serenity::client::{Context, EventHandler};
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::{GatewayIntents, Ready};
use serenity::{async_trait, Client};

use sysinfo::System;
use tracing::error;

mod tools;
use serenity::prelude::TypeMapKey;
use tokio::sync::Mutex;
use tools::config_manager::fetch_key;

mod commands;
use commands::admin_commands::*;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct SystemContainer;

impl TypeMapKey for SystemContainer {
    type Value = Arc<Mutex<System>>;
}

#[group]
#[commands(quit, sysinfo, usage, uptime)]
struct Command;
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _ctx: Context, _resume: ResumedEvent) {
        println!("Resumed")
    }
}

#[tokio::main]
async fn main() {
    let token = fetch_key();

    let http = Http::new(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|config| config.owners(owners).prefix("!"))
        .group(&COMMAND_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<SystemContainer>(Arc::new(Mutex::new(System::new_all())));
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
