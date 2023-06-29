use crate::roastbotai::RoastBotAi;
use anyhow::anyhow;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::UserId;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

mod dto;
mod roastbotai;

struct Bot {
    ai: roastbotai::RoastBotAi,
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        info!("{}: {}", msg.author.name, msg.content);
        let mess_250_chars = msg.content.clone().chars().take(250).collect::<String>();
        let res = self.ai.send_message(&mess_250_chars).await;
        let bot_id = UserId(1120025595376586843);
        if res.is_some() && msg.author.id != bot_id {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, res.unwrap()).await {
                error!("Error sending message: {:?}", why);
            }
        } else {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, "couldn reach roastbot !")
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot {
            ai: RoastBotAi::new(),
        })
        .await
        .expect("Err creating client");

    Ok(client.into())
}
