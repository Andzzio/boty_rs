use serenity::model::channel::Message;

use crate::bot::commands::{Data, Error};

pub async fn message(
    framework: poise::FrameworkContext<'_, Data, Error>,
    new_message: &Message,
) -> anyhow::Result<()> {
    if !new_message.content.is_empty() {
        println!("Received message: {}", new_message.content);
    }
    match new_message.content.to_lowercase().trim() {
        "hola" => {
            println!("Thinking...");
            if let Err(why) = new_message
                .channel_id
                .say(
                    &framework.serenity_context.http,
                    format!("Hola {}!!!", new_message.author.name),
                )
                .await
            {
                println!("Error sending message {why:?}");
            }
            Ok(())
        }
        "bye" | "chau" => {
            println!("Thinking...");
            if let Err(why) = new_message
                .channel_id
                .say(
                    &framework.serenity_context.http,
                    format!("Bye {}", new_message.author.name),
                )
                .await
            {
                println!("Error sending message {why:?}");
            }
            Ok(())
        }
        msg if msg.contains("boty") => {
            let _typing = new_message
                .channel_id
                .start_typing(&framework.serenity_context.http);
            let llm_provider = framework.user_data.llm.as_ref();
            let full_prompt = format!(
                "Name of the user: {}\nUser message: {}",
                new_message.author.name, new_message.content
            );
            let response: String = match llm_provider.generate(&full_prompt).await {
                Ok(text) => text,
                Err(err) => {
                    eprintln!("{:#}", err);
                    new_message
                        .channel_id
                        .say(
                            &framework.serenity_context.http,
                            "Ocurrió un error bro programame bien.",
                        )
                        .await?;
                    return Ok(());
                }
            };
            new_message
                .channel_id
                .say(&framework.serenity_context.http, response)
                .await?;
            Ok(())
        }
        _ => Ok(()),
    }
}
