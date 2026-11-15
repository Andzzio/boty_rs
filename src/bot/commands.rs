use poise::{Framework, FrameworkOptions, command};
use serenity::model::user::User;

pub struct Data {}
pub type Error = anyhow::Error;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub fn build_framework() -> Framework<Data, Error> {
    poise::Framework::builder()
        .options(FrameworkOptions {
            commands: vec![hola(), hablar()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build()
}

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
pub async fn hablar(ctx: Context<'_>) -> anyhow::Result<()> {
    Ok(())
}
