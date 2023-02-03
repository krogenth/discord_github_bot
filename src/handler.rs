use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use regex::Regex;

mod github_handler;
mod ping_handler;

async fn parse_message(ctx: Context, msg: Message) {
  let github_regex: regex::Regex = Regex::new(r"(?:github.com.*)(?:#L)(\d+)(?:-L)?(\d+)?").unwrap();

  if msg.content.chars().nth(0) == Some('!') {
    ping_handler::parse_command(ctx, msg).await;
  } else if github_regex.is_match(&msg.content) {
    github_handler::parse_github_link(ctx, msg).await;
  }
}

pub struct Handler;
#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    parse_message(ctx, msg).await;
  }

  async fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
  }
}
