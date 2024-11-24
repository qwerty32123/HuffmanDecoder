use serenity::all::{ChannelId, Http};
use std::sync::Arc;

const CHANNEL_1: u64 = 1309587781252546710;
const CHANNEL_2: u64 = 1309907135949180988;
const CHANNEL_3: u64 = 1309907121663508601;
const CHANNEL_4: u64 = 1309907129347473478;

pub struct DiscordChannels {
    http: Arc<Http>,
    channels: [ChannelId; 4],
}

impl DiscordChannels {
    pub fn new(http: Arc<Http>) -> Self {
        Self {
            http,
            channels: [
                ChannelId::new(CHANNEL_1),
                ChannelId::new(CHANNEL_2),
                ChannelId::new(CHANNEL_3),
                ChannelId::new(CHANNEL_4),
            ],
        }
    }

    pub fn send_message(&self, client_id: u32, message: impl std::fmt::Display + Send + 'static) {
        let channel = self.channels.get(client_id as usize).copied().unwrap_or(self.channels[0]);
        let http = Arc::clone(&self.http);

        tokio::spawn(async move {
            if let Err(why) = channel.say(&http, message.to_string()).await
            {
                eprintln!("Error sending Discord message: {:?}", why);
            }
        });
    }
}