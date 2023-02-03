use serenity::model::channel::Message;
use serenity::prelude::*;
use reqwest::Client;
use scraper::{Html, Selector};
use regex::Regex;

pub async fn parse_github_link(ctx: Context, msg: Message) {
  let lines = get_github_lines(&msg.content.clone()).await;
  if lines == None {
    return;
  }

  let html = get_github_page_content(&msg.content.clone()).await;
  if html == None {
    return;
  }

  let content = get_github_line_content(&html.unwrap(), &lines.unwrap()).await;
  let code_type = determine_code_type(&msg.content.clone()).await;

  let return_content = format!("```{}{}\n```", code_type, content);
  if let Err(why) = msg.channel_id.say(&ctx.http, return_content).await {
    println!("Error sending message: {:?}", why);
  }
}

async fn get_github_lines(url: &String) -> Option<[i32; 2]> {
  let github_regex: regex::Regex = Regex::new(r"(?:github.com.*)(?:#L)(\d+)(?:-L)?(\d+)?").unwrap();
  let mut indices: [i32; 2] = Default::default();

  if !github_regex.is_match(&url) {
    return None;
  }

  let captures = github_regex.captures(&url).unwrap();

  // guaranteed at least 1 line
  indices[0] = captures.get(1).map_or("".to_string(), |m| m.as_str().to_string()).parse::<i32>().unwrap();

  if captures.len() > 2 && captures.get(2) != None {
    indices[1] = captures.get(2).map_or("".to_string(), |m| m.as_str().to_string()).parse::<i32>().unwrap();
  } else {
    indices[1] = indices[0];
  }

  return Some(indices);
}

async fn get_github_page_content(url: &String) -> Option<String> {
  let client = Client::new();
  let result = client.get(url.as_str()).send().await;
  if result.is_err() {
    return None;
  }

  let response = result.unwrap();
  return Some(response.text().await.unwrap());
}

async fn get_github_line_content(html: &String, lines: &[i32; 2]) -> String {
  let document = Html::parse_document(&html);
  let mut content: String = Default::default();

  for line in lines[0]..lines[1]+1 {
    // for each line, add a new line to the content to be returned
    content.push_str("\n");

    let line_id = format!("#LC{}", line.to_string());
    let line_selector = Selector::parse(line_id.as_str()).unwrap();
    for selected in document.select(&line_selector) {
      // convert the text of the selected element into a vector that can be converted to a string to be appended
      let selected_vec = selected.text().collect::<Vec<_>>();
      content.push_str(selected_vec.iter().cloned().collect::<String>().as_str());
    }
  }

  return content;
}

async fn determine_code_type(url: &String) -> String {
  let github_filetype_regex = Regex::new(r"(?:github.com.*)(?:\.)(.*)(?:#L)").unwrap();
  let mut filetype: String = Default::default();

  for captures in github_filetype_regex.captures_iter(&url) {
    filetype = captures.get(1).map_or("".to_string(), |m| m.as_str().to_string());
  }
  filetype.make_ascii_lowercase();

  return match filetype.as_str(){
    "h" | "hpp" | "c" | "cpp" | "cxx" => "cpp".to_string(),
    "java" => "java".to_string(),
    "rs" => "rust".to_string(),
    "cs" => "csharp".to_string(),
    "js" | "jsx" | "ts" => "js".to_string(),
    "json" => "json".to_string(),
    "go" => "golang".to_string(),
    "xml" => "xml".to_string(),
    "py" => "python".to_string(),
    "html" => "html".to_string(),
    "css" => "css".to_string(),
    "sql" => "sql".to_string(),
    _ => "".to_string(),
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[tokio::test]
  async fn test_get_github_lines_single() {
    let url: String = "https://github.com/krogenth/discord_github_bot/blob/main/src/main.rs#L1".to_string();
    let res = get_github_lines(&url).await;
    assert_ne!(res, None);
    assert_eq!(res, Some([1, 1]));
  }

  #[tokio::test]
  async fn test_get_github_lines_multiple() {
    let url: String = "https://github.com/krogenth/discord_github_bot/blob/main/src/main.rs#L1-L10".to_string();
    let res = get_github_lines(&url).await;
    assert_ne!(res, None);
    assert_eq!(res, Some([1, 10]));
  }

  #[tokio::test]
  async fn test_get_github_lines_none() {
    let url: String = "https://github.com/krogenth/discord_github_bot/blob/main/src/main.rs".to_string();
    let res = get_github_lines(&url).await;
    assert_eq!(res, None);
  }
}