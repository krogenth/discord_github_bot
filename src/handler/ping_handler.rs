use serenity::model::channel::Message;
use serenity::prelude::*;

pub async fn parse_command(ctx:Context, msg: Message) {
  match msg.content.as_str(){
    "!ping" => ping(ctx, msg).await,
    _ => return,
  }
}

async fn ping(ctx: Context, msg: Message) {
  if let Err(why) = msg.channel_id.say(&ctx.http, msg.author).await {
    println!("Error sending message: {:?}", why);
  }
}