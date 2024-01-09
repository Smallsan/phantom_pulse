use serenity::client::Context;
use std::sync::Arc;
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
