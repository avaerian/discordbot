use serenity::async_trait;
use serenity::prelude::*;
use serenity::all::*;

struct Handler;

const MEMBER_ROLE_ID: u64 = 1558778515787218974;

#[tokio::main]
async fn main() {
    let token = dotenv::var("DISCORD_TOKEN").expect("Expected token environment variable");
    let intents = GatewayIntents::GUILD_MESSAGES 
        | GatewayIntents::DIRECT_MESSAGES 
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;

    let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Error creating client");

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Failed to register ctrl+c handler");
        shard_manager.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" && let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
            println!("Error replying: {why:?}");
        }
    }

    async fn guild_member_addition(&self, ctx: Context, member: Member) {
        let guild_name = &ctx.http.get_guild(member.guild_id).await
            .unwrap_or_else(|_| panic!("Failed to retrieve guild data for guild {}", member.guild_id)).name;

        println!("{} joined the server {} ({})!", member.user.name, guild_name, member.guild_id);

        member.add_role(&ctx.http, RoleId::new(MEMBER_ROLE_ID)).await.expect("Failed to add member role");
    }

}
