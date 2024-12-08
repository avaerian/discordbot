use std::fmt::Error;
use std::str::FromStr;
use crate::command::{CommandContext, CommandHandler};
use crate::component::ComponentCallback;
use crate::component::ComponentContext;
use serenity::all::{ButtonStyle, CreateActionRow, CreateCommand, CreateInputText, CreateInteractionResponse, CreateInteractionResponseMessage, CreateModal, InputTextStyle};
use serenity::builder::CreateButton;
use serenity::{async_trait, Error};


/*enum EmbedButton {
    Author,
    Title,
    Description,
    Color,
    Image,
    Thumbnail,
    Footer,

    // Submit
    // Cancel
}*/

// review https://stackoverflow.com/questions/37006835/building-an-enum-inside-a-macro for more complex enums for macros
macro_rules! define_embed_btns {
    ( $($v:vis,)? $($btn:ident),+ ) => {
        v enum EmbedButton {
            $($btn),+
        }

        impl FromStr for EmbedButton {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.to_lowercase() {
                    $(
                       str::to_lowercase(stringify!($btn)) => { Ok(EmbedButton::$btn) }
                    )+
                    _ => { Err(())}
                }
            }
        }

        /*impl Into<String> for EmbedButton {

        }*/

        // TODO: Into<String>, Iterator, size, additional fns for associated data
    };
}

/*
    To associate values/data with the enum, the macro will generate
    functions with match statements that return the appropriate data for each enum
*/

fn test() {
    match EmbedButton::from_str("author") {
        Ok(btn_type) => {}
    }
}

// TODO: refactor to define_btns! macro (contain namespace, custom_id, etc.)
define_embed_btns!(Author, Title, Description, Color, Image, Thumbnail, Footer, Submit, Cancel);

struct EmbedButtonCallback;

#[async_trait]
impl ComponentCallback for EmbedButtonCallback {
    async fn run<'a>(&self, ctx: ComponentContext<'a>) -> Result<(), Error> {
        let interaction = ctx.interaction;
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

        interaction.create_response(&ctx.ctx.http, CreateInteractionResponse::Modal(modal)).await
    }
}

pub struct EmbedCreatorCommand;

#[async_trait]
impl CommandHandler for EmbedCreatorCommand {
    fn register(&self) -> CreateCommand {
        CreateCommand::new("embed").description("Create an embed template")
            .dm_permission(false)
    }

    async fn run<'a>(&self, ctx: CommandContext<'a>) -> Result<(), Error> {
        let bot = ctx.bot;
        let interaction = ctx.interaction;
        let http = &ctx.ctx.http;

        // TODO: Create namespaces for component system !!

        // TODO: eventually refactor into EmbedTemplate for storing on server/dumping to file
        // TODO: introduce caching for these types of messages? (not priority)
        let mut edit_btns: Vec<CreateButton> = Vec::new(); // for caching, associate with id and if None, add buttons and cache result
        // EmbedButton::Author::create().label("Set Author") ... ??
        edit_btns.push(CreateButton::new("author").label("Set Author").style(ButtonStyle::Secondary));
        edit_btns.push(CreateButton::new("title").label("Set Title").style(ButtonStyle::Secondary));
        edit_btns.push(CreateButton::new("desc").label("Set Description").style(ButtonStyle::Secondary));
        edit_btns.push(CreateButton::new("color").label("Set Color").style(ButtonStyle::Secondary));
        edit_btns.push(CreateButton::new("image").label("Set Image").style(ButtonStyle::Secondary));
        edit_btns.push(CreateButton::new("thumbnail").label("Set Thumbnail").style(ButtonStyle::Secondary));
        edit_btns.push(CreateButton::new("footer").label("Set Footer").style(ButtonStyle::Secondary));

        bot.components.register_component("author", EmbedButtonCallback).await; // move to register ?

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