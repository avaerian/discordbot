mod commands;
mod bot;

use discordbot::{
    bot::BotData,
    commands::purge::PurgeCommand,
    commands::welcome_debug::DebugWelcomeCommand
};
use serenity::all::{ActivityData, EventHandler, GatewayIntents, Interaction, Message, MessageUpdateEvent, OnlineStatus, Ready, ResumedEvent};
use serenity::client::Context;
use serenity::{async_trait, Client};
use std::env;
use std::sync::Arc;

struct BotEventHandler {
    bot: Arc<BotData>
}

impl From<Arc<BotData>> for BotEventHandler {
    fn from(value: Arc<BotData>) -> Self {
        Self {
            bot: value.clone()
        }
    }
}

#[async_trait]
impl EventHandler for BotEventHandler {
    async fn ready(&self, ctx: Context, event: Ready) {

        let bot = self.bot.clone();

        // Register commands
        bot.register_global_command(&ctx, DebugWelcomeCommand).await.expect("Failed to register command");
        bot.register_global_command(&ctx, PurgeCommand).await.expect("Failed to register command");

        println!("Ready!")
    }

    async fn resume(&self, ctx: Context, event: ResumedEvent) {

    }

    async fn message(&self, ctx: Context, event: Message) {
        let author = event.author;
        let userid = author.id;
        let content = event.content;

        let author = author.name;
        let userid = userid.get();
        let channel = event.channel_id.name(&ctx.http).await.expect("Failed to get channel from id");
        println!("{author}({userid}) said: \"{content}\" in channel {channel}");

    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let cmd_name = &command.data.name;
            match self.bot.cmds.get_command_handler(cmd_name) {
                Some(handle) => {
                    handle.run(&ctx, &command).await.expect("Error running command");
                }
                None => { println!("Unrecognized command"); }
            }
        }
    }

    async fn message_update(&self, ctx: Context, old_if_available: Option<Message>, new: Option<Message>, event: MessageUpdateEvent) {
        println!("Messaged updated")
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    dotenv::dotenv().expect("Failed to read .env file");
    let token = env::var("BOT_TOKEN").expect("Expected bot token in env");

    let bot = Arc::new(BotData::new());

    let intents = GatewayIntents::all();
    let mut client = Client::builder(&token, intents)
        .status(OnlineStatus::DoNotDisturb)
        .activity(ActivityData::playing("on Minerift"))
        .event_handler(BotEventHandler::from(bot))
        .await.expect("Failed to create client"); // TODO: properly log error

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Failed to register ctrl+c signal");
        shard_manager.shutdown_all().await;
    });

    client.start().await?;

    println!("Closing...");
    Ok(())
}