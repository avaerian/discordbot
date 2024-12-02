use crate::command::{CommandHandler, CommandRegistry};
use serenity::all::Context;
use serenity::Error;
use crate::component::ComponentRegistry;

pub struct BotData {
    pub cmds: CommandRegistry,
    pub components: ComponentRegistry,
}

impl BotData {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn register_global_command(&self, ctx: &Context, handler: impl CommandHandler + 'static) -> Result<(), Error> {
        self.cmds.register_global(ctx, handler).await
    }
}

impl Default for BotData {
    fn default() -> Self {
        Self {
            cmds: CommandRegistry::new(),
            components: ComponentRegistry::new(),
        }
    }
}