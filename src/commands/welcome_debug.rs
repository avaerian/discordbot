use serde_json::Value;
use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, Embed};
use serenity::{async_trait, Error};
use std::collections::HashMap;
use std::fs;
use crate::commands::command::CommandHandler;

pub struct DebugWelcomeCommand;

#[async_trait]
impl CommandHandler for DebugWelcomeCommand {
    fn register(&self) -> CreateCommand {
        CreateCommand::new("debug").description("Debug command for printing welcome embeds")
            .add_option(CreateCommandOption::new(CommandOptionType::User, "user", "The user to test the welcome embed"))
    }

    async fn run(&self, ctx: &Context, interaction: &CommandInteraction) -> Result<(), Error>{
        let json = fs::read_to_string("test_embed.json").expect("Failed to read test_embed.json");
        let json_parts: HashMap<String, Value> = serde_json::from_str(json.as_str()).expect("Failed to read json sections");

        let embeds_part = json_parts.get("embeds").expect("Failed to read embeds section");

        let embeds: Vec<CreateEmbed> = embeds_part.as_array().unwrap().iter()
            .map(|embed| serde_json::from_value(embed.clone()).expect("Failed to read raw embed"))
            .map(|embed: Embed| CreateEmbed::from(embed))
            .collect();

        let data = CreateInteractionResponseMessage::new().add_embeds(embeds);
        let builder = CreateInteractionResponse::Message(data);
        interaction.create_response(&ctx.http, builder).await
    }
}