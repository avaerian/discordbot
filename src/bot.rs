use std::sync::Arc;
use crate::command::{CommandHandler, CommandRegistry};
use serenity::all::Context;
use serenity::Error;
use crate::component::{ComponentCallback, ComponentRegistry};

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

    pub fn get_component_handle(&self, comp_id: impl AsRef<str>) -> Option<Arc<Box<dyn ComponentCallback + 'static>>>{
        self.components.get_component_handle(comp_id)
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