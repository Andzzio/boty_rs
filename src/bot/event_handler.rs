use anyhow::{Error, Ok};
use serenity::client::FullEvent;

use crate::bot::{
    events::{message::message, ready::ready},
    framework::Data,
};

pub async fn event_handler(
    framework: poise::FrameworkContext<'_, Data, Error>,
    event: &FullEvent,
) -> anyhow::Result<()> {
    match event {
        FullEvent::Ready { data_about_bot, .. } => ready(data_about_bot).await,
        FullEvent::Message { new_message } => message(framework, new_message).await?,
        _ => {}
    }
    Ok(())
}
