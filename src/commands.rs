use crate::{Context, Error};

#[poise::command(prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let response = "pawng";
    ctx.say(response).await?;
    Ok(())
}
