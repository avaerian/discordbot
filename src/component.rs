use crate::bot::BotData;
use serenity::all::{ComponentInteraction, Context, CreateActionRow};
use serenity::builder::CreateButton;
use serenity::{async_trait, Error};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// for caching, as an idea:
#[non_exhaustive] // ??
enum Component {
    Button(CreateButton),
    Row(CreateActionRow),
    Rows(Vec<CreateActionRow>),
}

#[async_trait]
pub trait ComponentCallback: Send + Sync {
    async fn run<'a>(&self, ctx: ComponentContext<'a>) -> Result<(), Error>;
}

pub struct ComponentRegistry {
    handlers: RwLock<HashMap<String, Arc<Box<dyn ComponentCallback>>>>,
    // include cache for Components ???
}

pub struct ComponentContext<'a> {
    pub bot: Arc<BotData>,
    pub ctx: &'a Context,
    pub interaction: &'a ComponentInteraction
}

impl<'a> ComponentContext<'a> {
    pub fn new(bot: &Arc<BotData>, ctx: &'a Context, interaction: &'a ComponentInteraction) -> Self {
        Self {
            bot: bot.clone(),
            ctx,
            interaction,
        }
    }
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn register_component(&self, comp_id: impl AsRef<str>, handle: impl ComponentCallback + 'static) {
        let mut handlers = self.handlers.write().unwrap();
        handlers.insert(comp_id.as_ref().to_string(), Arc::new(Box::new(handle)));
    }

    pub fn get_component_handle(&self, comp_id: impl AsRef<str>) -> Option<Arc<Box<dyn ComponentCallback + 'static>>> {
        let handlers = self.handlers.read().unwrap();
        handlers.get(comp_id.as_ref()).cloned()
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self {
            handlers: RwLock::new(HashMap::new())
        }
    }
}