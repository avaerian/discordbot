use serenity::all::{Channel, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage, GetMessages, GuildChannel, Message, MessageId, ResolvedOption, ResolvedValue};
use serenity::{async_trait, Error};
use crate::command::{CommandContext, CommandHandler};

pub struct PurgeCommand;

#[async_trait]
impl CommandHandler for PurgeCommand {
    fn register(&self) -> CreateCommand {
        CreateCommand::new("purge").description("Delete a mass number of messages at a time. Can't delete messages older than 2 weeks.")
            .add_option(CreateCommandOption::new(CommandOptionType::Integer, "messages", "Number of messages to delete")
                .min_int_value(1)
                .max_int_value(100)
                .required(true))
            .dm_permission(false)
    }

    async fn run(&self, ctx: CommandContext) -> Result<(), Error> {
        let interaction = ctx.interaction;
        let http = &ctx.ctx.http;

        let channel = unwrap_guild_channel(interaction.channel_id.to_channel(http).await.expect("Failed to retrieve channel"));

        if let Some(ResolvedOption {
                        value: ResolvedValue::Integer(count),
                        ..
                    }) = interaction.data.options().first()
        {
            match channel.last_message_id {
                Some(id) =>  {
                    let builder = GetMessages::new().before(id).limit((*count - 1) as u8);
                    let messages = channel.messages(http, builder).await.expect("Failed to retrieve messages");
                    let mut messages: Vec<MessageId> = messages.iter().map(|msg| msg.id).collect();
                    messages.push(id); // include most recent message
                    println!("{messages:#?}");
                    channel.delete_messages(http, &messages).await.expect("Failed to delete messages");
                    println!("Deleted messages successfully!");

                    let data = CreateInteractionResponseMessage::new().content(format!("Deleted {} messages", messages.len())).ephemeral(true);
                    let builder = CreateInteractionResponse::Message(data);
                    return interaction.create_response(http, builder).await;
                },
                None => {
                    println!("No messages to purge in channel");
                }
            }
        } else {
            println!("Incorrect argument provided");
        }

        // TODO: create bot response for user

        Ok(())
    }
}

// TODO: move somewhere else
pub fn unwrap_guild_channel(channel: Channel) -> GuildChannel {
    if let Channel::Guild(channel) = channel {
        channel
    } else {
        panic!("Channel must be a guild channel. WTF?");
    }
}