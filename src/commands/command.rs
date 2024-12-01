use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serenity::all::{Command, CommandInteraction, Context, CreateCommand};
use serenity::{async_trait, Error};

#[async_trait]
pub trait CommandHandler: Send + Sync {
    fn register(&self) -> CreateCommand;
    async fn run(&self, ctx: &Context, interaction: &CommandInteraction) -> Result<(), Error>;
}

pub struct CommandRegistry {
    // RwLock required only for initial registering of commands :\
    cmds: RwLock<HashMap<String, Arc<dyn CommandHandler>>>,
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self {
            cmds: RwLock::new(HashMap::new())
        }
    }
}

impl CommandRegistry {

    pub fn new() -> Self {
        Self::default()
    }

    pub async fn register_global(&self, ctx: &Context, handler: impl CommandHandler + 'static) -> Result<(), Error> {
        match Command::create_global_command(&ctx.http, handler.register()).await {
            Ok(cmd) => {
                let mut cmds = self.cmds.write().unwrap();
                cmds.insert(cmd.name, Arc::new(handler));
            }
            Err(err) => { return Err(err); }
        }

        Ok(())
    }

    pub fn get_command_handler(&self, cmd: impl AsRef<str>) -> Option<Arc<dyn CommandHandler>> {
        let cmds = self.cmds.read().unwrap();
        cmds.get(cmd.as_ref()).cloned()
    }

    // register_guild??
}