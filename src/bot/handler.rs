use serenity::{all::Message, async_trait, prelude::*};

pub struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: serenity::model::gateway::Ready) {
        println!("Connected as {}", ready.user.name);
    }
    async fn message(&self, ctx: Context, msg: Message) {
        println!("Received message: {}", msg.content);
        if msg.content == "hola" {
            println!("Thinking...");
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, "Hola voy a revivir el server")
                .await
            {
                println!("Error sending message {why:?}");
            }
        }
    }
}
