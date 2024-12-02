use std::sync::Arc;
use crate::command::{CommandContext, CommandHandler};
use serenity::all::{ButtonStyle, CommandInteraction, ComponentInteraction, Context, CreateActionRow, CreateCommand, CreateInputText, CreateInteractionResponse, CreateInteractionResponseMessage, CreateModal, CreateQuickModal, InputTextStyle};
use serenity::{async_trait, Error};
use serenity::builder::CreateButton;
use crate::bot::BotData;

pub struct EmbedCreatorCommand;

pub async fn handle_button(bot: Arc<BotData>, ctx: &Context, interaction: &ComponentInteraction) -> Result<(), Error> {
    let btn_id = &interaction.data.custom_id;
    let modal = match btn_id.as_str() {
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
        _ => unimplemented!()
    };

    Ok(())
}

#[async_trait]
impl CommandHandler for EmbedCreatorCommand {
    fn register(&self) -> CreateCommand {
        CreateCommand::new("embed").description("Create an embed template")
            .dm_permission(false)
    }

    async fn run(&self, ctx: CommandContext) -> Result<(), Error> {
        let bot = ctx.bot;
        let interaction = ctx.interaction;
        let http = &ctx.ctx.http;

        // TODO: eventually refactor into EmbedTemplate for storing on server/dumping to file
        // TODO: introduce caching for these types of messages? (not priority)
        let mut edit_btns: Vec<CreateButton> = Vec::new(); // for caching, associate with id and if None, add buttons and cache result
        edit_btns.push(CreateButton::new("author").label("Set Author").style(ButtonStyle::Secondary));
        edit_btns.push(CreateButton::new("title").label("Set Title").style(ButtonStyle::Secondary));
        edit_btns.push(CreateButton::new("desc").label("Set Description").style(ButtonStyle::Secondary));
        edit_btns.push(CreateButton::new("color").label("Set Color").style(ButtonStyle::Secondary));
        edit_btns.push(CreateButton::new("image").label("Set Image").style(ButtonStyle::Secondary));
        edit_btns.push(CreateButton::new("thumbnail").label("Set Thumbnail").style(ButtonStyle::Secondary));
        edit_btns.push(CreateButton::new("footer").label("Set Footer").style(ButtonStyle::Secondary));

        bot.components.register_component("author", handle_button).await;

        let mut submit_btns: Vec<CreateButton> = Vec::new();
        submit_btns.push(CreateButton::new("cancel").label("Cancel").style(ButtonStyle::Danger));
        submit_btns.push(CreateButton::new("submit").label("Submit").style(ButtonStyle::Success));

        let mut rows = Vec::new();
        rows.push(CreateActionRow::Buttons(Vec::from(&edit_btns[0..4])));
        rows.push(CreateActionRow::Buttons(Vec::from(&edit_btns[4..])));
        rows.push(CreateActionRow::Buttons(submit_btns));

        let msg = CreateInteractionResponseMessage::new()
            .components(rows).ephemeral(true);

        interaction.create_response(http, CreateInteractionResponse::Message(msg)).await

        /*let modal = CreateQuickModal::new("Embed Template Creator")
            .short_field("Set Author")
            .short_field("Set Title")
            .paragraph_field("Set Description")
            .short_field("Set Image")
            .short_field("Set Thumbnail")
            //.short_field("Set Footer")
            .timeout(std::time::Duration::from_secs(10 * 60));

        let response = interaction.quick_modal(ctx, modal).await.expect("Failed to get modal response");
        let message = CreateInteractionResponseMessage::new().content("debug response").ephemeral(true);
        response.unwrap().interaction.create_response(ctx, CreateInteractionResponse::Message(message)).await*/

    }
}