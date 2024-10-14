use futures::future::join_all;
use kuchiki::traits::*;
use kuchiki::NodeRef;
use log::log;
use std::io;
use std::io::Error;
use teloxide::{prelude::*, utils::command::BotCommands};
use tokio;

// Function to search for <a> tag with the specific title and return title and href
fn find_travel_advisory_link(node: &NodeRef, country: &str) -> Option<(String, String)> {
    // Check if the node is an element
    if let Some(element) = node.as_element() {
        // Check if the element is an <a> tag
        if element.name.local.as_ref() == "a" {
            let attributes = element.attributes.borrow();
            let title = attributes.get("title");
            let href = attributes.get("href");

            // Check if title contains "Travel Advisory <country>" and href exists
            if let (Some(title), Some(href)) = (title, href) {
                if title.contains(&format!("Travel Advisory {}", country)) {
                    return Some((title.to_string(), href.to_string()));
                }
            }
        }
    }

    // Recursively check child nodes
    for child in node.children() {
        if let Some(result) = find_travel_advisory_link(&child, country) {
            return Some(result);
        }
    }
    None
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    tokio::join!(get_country_status(), telegram_bot());
}

fn set_url(country: &str) -> String {
    format!("https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/{}.html", country)
}

async fn get_data(url: String, country: &str) -> Result<String, io::Error> {
    let resp = reqwest::get(url).await.unwrap();
    let html = resp.text().await.unwrap();

    // Parse the HTML
    let document = kuchiki::parse_html().one(html);

    // Find and print the travel advisory title and link
    if let Some((title, _link)) = find_travel_advisory_link(&document, country) {
        log::info!("Found travel advisory title for {}: {}", country, &title);
        Ok(title)
    } else {
        log::info!("Travel advisory link not found for {}", country);
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Travel advisory not found",
        ))
    }
}

async fn telegram_bot() {
    log::info!("Starting echo bot...");

    // Initialize the bot with your bot token
    let bot = Bot::from_env();
    Command::repl(bot, answer).await;
}
#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
    #[command(description = "Check LT travel status.", parse_with = "split")]
    CheckLT,
}
async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Username(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}."))
                .await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(
                msg.chat.id,
                format!("Your username is @{username} and age is {age}."),
            )
            .await?
        }
        Command::CheckLT => {
            let results = get_country_status().await;
            bot.send_message(
                msg.chat.id,
                format!(
                    "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
                    results[0],
                    results[1],
                    results[2],
                    results[3],
                    results[4],
                    results[5],
                    results[6],
                    results[7]
                ),
            )
            .await?
        }
    };

    Ok(())
}

async fn get_country_status() -> Vec<String> {
    let countries = [
        "Lithuania",
        "Latvia",
        "Estonia",
        "Poland",
        "Ukraine",
        "Portugal",
        "Finland",
        "Hungary",
    ];

    let mut responses: Vec<String> = Vec::with_capacity(countries.len());

    // Create a vector of futures
    let futures = countries.iter().map(|&country| {
        let url = set_url(country);
        get_data(url, country)
    });

    // Run all futures concurrently and wait for them to complete
    let results = join_all(futures).await;

    // Handle results
    for (i, result) in results.into_iter().enumerate() {
        if let Err(e) = result {
            println!("Error fetching data for {}: {}", countries[i], e);
        } else {
            responses.push(result.unwrap())
        }
    }
    responses
}
