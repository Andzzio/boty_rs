use serenity::model::channel::Message;

use crate::bot::framework::{Data, Error, get_message_history, record_message};

pub async fn message(
    framework: poise::FrameworkContext<'_, Data, Error>,
    new_message: &Message,
) -> anyhow::Result<()> {
    if new_message.author.bot {
        return Ok(());
    }
    if !new_message.content.is_empty() {
        let guild_id = new_message.guild_id;
        let message = new_message;
        record_message(
            framework,
            guild_id,
            message.content.clone(),
            message.author.name.clone(),
        )
        .await;
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
            let guild_id = new_message.guild_id;
            let llm_provider = framework.user_data.llm.as_ref();
            let full_prompt = get_message_history(framework, guild_id).await;
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
                .say(&framework.serenity_context.http, response.clone())
                .await?;
            record_message(framework, guild_id, response, String::from("boty")).await;
            Ok(())
        }
        _ => Ok(()),
    }
}
