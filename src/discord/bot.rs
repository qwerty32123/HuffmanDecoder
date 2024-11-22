use serenity::all::{Context, EventHandler, Ready};
use serenity::async_trait;
use crate::CHANNEL_ID;

struct Bot;

impl Bot {
    async fn ping_channel(ctx: &Context) {
        if let Err(why) = CHANNEL_ID.say(&ctx.http, "@everyone Ping!").await {
            println!("Error sending message: {:?}", why);
        }
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        Bot::ping_channel(&ctx).await;
    }
}
