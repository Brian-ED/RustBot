use std::env;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;


#[async_trait]
impl EventHandler for Handler {
    // Exectutes every message
    async fn message(&self, ctx: Context, msg: Message) {
        let prefix="test!";

        let all_args: Vec<&str> = msg.content.split_whitespace().collect();
        if all_args.len()==0 || !all_args[0].starts_with(prefix){
            return message_not_cmd(ctx,msg).await
        }
        let cmd = &*all_args[0][prefix.len()..].to_owned();
        let args = all_args[1..].to_owned();
        println!("{}, {:?}, {:?}", cmd, args,all_args);

        if cmd == "ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn message_not_cmd(_ctx: Context, _msg: Message){
    return
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}