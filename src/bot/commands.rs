use poise::command;
use serenity::model::user::User;

use crate::bot::framework::Context;

#[command(slash_command)]
pub async fn hola(
    ctx: Context<'_>,
    #[description = "Hola"] user: Option<User>,
) -> anyhow::Result<()> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("Hola {}", u.name);
    ctx.say(response).await?;
    Ok(())
}
#[command(slash_command)]
pub async fn hablar(
    ctx: Context<'_>,
    #[description = "Escribe el mensaje que quieras preguntarle"] prompt: String,
) -> anyhow::Result<()> {
    ctx.defer().await?;
    let llm_provider = ctx.data().llm.as_ref();
    let full_prompt = format!(
        "Name of the user: {}\nUser message: {}",
        ctx.author().name,
        prompt
    );
    let response: String = match llm_provider.generate(&full_prompt).await {
        Ok(text) => text,
        Err(err) => {
            eprintln!("{:#}", err);
            ctx.say("Ocurrió un error bro programame bien.").await?;
            return Ok(());
        }
    };
    ctx.say(format!(
        "**{}**: {}\n\n{}",
        ctx.author().name,
        prompt,
        response
    ))
    .await?;
    Ok(())
}
