use std::env;
use crate::roastbotai::RoastBotAi;
use anyhow::anyhow;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::UserId;
use serenity::prelude::*;
use tracing::{error, info};
use rand::Rng;

mod dto;
mod roastbotai;

struct Bot {
    ai: RoastBotAi,
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        info!("{}: {}", msg.author.name, msg.content);
        let bot_id = UserId(1120025595376586843);
        let random = {
            let mut rng = rand::thread_rng();
            rng.gen_range(0..3)
        };
        if msg.author.id == bot_id || random == 1 {
            return;
        }

        let mess_250_chars = msg.content.clone().chars().take(250).collect::<String>();
        let res = self.ai.send_message(&mess_250_chars).await;
        if let Some(ai_message) = res  {
            if let Err(why) = msg.channel_id.say(&ctx.http, ai_message).await {
                error!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    // Get the discord token set in `Secrets.toml`
    let token = if let Ok(token) = env::var("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Bot {
            ai: RoastBotAi::new(),
        })
        .await
        .expect("Err creating client");

    client.start().await?;
    Ok(())
}
