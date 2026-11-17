use serenity::model::gateway::Ready;

pub async fn ready(data_about_bot: &Ready) {
    println!("Connected as {}", data_about_bot.user.name);
}
