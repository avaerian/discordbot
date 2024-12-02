pub mod commands;
pub mod bot;
pub mod command;
pub mod component;

use discordbot::{
    bot::BotData,
    commands::purge::PurgeCommand,
    commands::welcome_debug::DebugWelcomeCommand
};
use serenity::all::{ActivityData, CreateActionRow, CreateInputText, CreateInteractionResponse, CreateModal, EventHandler, GatewayIntents, InputTextStyle, Interaction, Message, MessageUpdateEvent, OnlineStatus, Ready, ResumedEvent};
use serenity::client::Context;
use serenity::{async_trait, Client};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use discordbot::commands::embed_creator::EmbedCreatorCommand;
use crate::command::CommandContext;

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
        bot.register_global_command(&ctx, EmbedCreatorCommand).await.expect("Failed to register command");

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
        match interaction {
            Interaction::Modal(modal) => {
                let data = &modal.data;
                println!("{data:#?}");

                modal.create_response(&ctx.http, CreateInteractionResponse::Acknowledge).await.unwrap()
                // on modal submit, update message with embed preview
            }

            Interaction::Command(cmd) => {
                let cmd_name = &cmd.data.name;
                match self.bot.cmds.get_command_handler(cmd_name) {
                    Some(handle) => {
                        let ctx = CommandContext::new(&self.bot, &ctx, &cmd);
                        handle.run(ctx).await.expect("Error running command");
                    }
                    None => { println!("Unrecognized command"); }
                }
            }

            // Prototype modal response for button
            // Prototype on modal submit, print debug data

            Interaction::Component(comp) => {
                let data = &comp.data;
                println!("{data:#?}");

                let timeout = Duration::from_secs(10 * 60);

                // match button id
                let modal = match data.custom_id.as_str() {
                    "author" => {
                        let set_author_field = CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Short, "Set embed author", "embed:author"));
                        let mut comps = Vec::new();
                        comps.push(set_author_field);
                        CreateModal::new("modal:author", "Test modal")
                            .components(comps)
                    }
                    "title" => {
                        let set_title_field = CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Short, "Set embed title", "embed:title"));
                        let mut comps = Vec::new();
                        comps.push(set_title_field);
                        CreateModal::new("modal:title", "Test modal")
                            .components(comps)
                    }
                    _ => unimplemented!("temp debug code")
                };

                comp.create_response(&ctx.http, CreateInteractionResponse::Modal(modal)).await.unwrap()
            }

            _ => {}
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