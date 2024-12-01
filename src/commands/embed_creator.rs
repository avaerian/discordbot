use serenity::all::{CommandInteraction, Context, CreateCommand};
use serenity::{async_trait, Error};
use crate::commands::command::CommandHandler;

pub struct EmbedCreatorCommand;

#[async_trait]
impl CommandHandler for EmbedCreatorCommand {
    fn register(&self) -> CreateCommand {
        todo!()
    }

    async fn run(&self, ctx: &Context, interaction: &CommandInteraction) -> Result<(), Error> {
        todo!()
    }
}