use std::collections::HashMap;
use std::future::Future;
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use serenity::all::{ComponentInteraction, Context, CreateActionRow};
use serenity::builder::CreateButton;
use serenity::Error;
use crate::bot::BotData;

// for caching, as an idea:
#[non_exhaustive] // ??
enum Component {
    Button(CreateButton),
    Row(CreateActionRow),
    Rows(Vec<CreateActionRow>),
}

//type ComponentHandle = dyn Future<Output=dyn Fn(Arc<BotData>, &Context, &ComponentInteraction) -> Result<(), Error>> + 'static + Send + Sync;
type ComponentHandle = dyn Fn(Arc<BotData>, &Context, &ComponentInteraction) -> (dyn Future<Output=Result<(), Error>> + Send + Sync);

pub struct ComponentRegistry {
    handlers: RwLock<HashMap<String, Box<ComponentHandle>>>
    // include cache for Components ???
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn register_component(&self, comp_id: impl AsRef<str>, handle: Box<ComponentHandle>) {
        let mut handlers = self.handlers.write().unwrap();
        handlers.insert(comp_id.as_ref().to_string(), handle);
    }

    pub async fn get_component_handle(&self, comp_id: impl AsRef<str>) -> Option<Box<ComponentHandle>> {
        let handlers = self.handlers.read().unwrap();
        match handlers.get(comp_id.as_ref()) {
            Some(&handle) => {
                Some(Box::new(handle)) // TODO: come back to here; review Boxing and "Pin"-ning
            },
            None => None
        }
        //handlers.get(comp_id.as_ref()).cloned()
    }

}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self {
            handlers: RwLock::new(HashMap::new())
        }
    }
}