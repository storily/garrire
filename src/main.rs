#![feature(str_strip)]
#[macro_use]
extern crate pest_derive;

use futures::StreamExt;
use dawn_model::channel::embed::Embed;
use dawn_gateway::shard::{Config, Event, Shard};
use dawn_http::Client as HttpClient;
use pest::Parser;
use std::{
    env,
    error::Error,
    fmt::Display,
    sync::Arc,
};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let token = env::var("DISCORD_TOKEN")?;

    let http = Arc::new(HttpClient::new(&token));

    let config = Config::builder(&token).build();
    let shard = Shard::new(config).await?;
    let mut events = shard.events().await;

    println!("listening for events...");
    while let Some(event) = events.next().await {
        tokio::spawn(handler(http.clone(), event));
    }

    Ok(())
}

async fn handler(http: Arc<HttpClient>, event: Event) {
    if let Err(err) = handle_event(http, event).await {
        eprintln!("handler: {:?}", err);
    }
}

async fn handle_event(http: Arc<HttpClient>, event: Event) -> Result<(), Box<dyn Error>> {
    match event {
        Event::ShardConnected(connected) => {
            println!("Connected on shard {}", connected.shard_id);
        },
        Event::MessageCreate(msg) => {
            println!("message: ts={} author=(id={} name={}) content={}",
                msg.timestamp,
                msg.author.id,
                msg.author.name,
                msg.content
            );

            let mut resp = Vec::new();

            let prefix = "!";
            if let Some(body) = msg.content.trim().strip_prefix(prefix) {
                let prefixed = format!("\u{F8F8}{}", body);
                match Grammar::parse(Rule::command, &prefixed) {
                    Ok(parsed) => {
                        resp.push(http.create_message(msg.channel_id).embed(colour_block(
                            0x00FF00,
                            format!("{:#?}", parsed),
                        )));
                    }
                    Err(err) => {
                        let reprefixed = err.to_string().replace("\u{F8F8}", prefix);

                        eprintln!("{}", reprefixed);
                        resp.push(http.create_message(msg.channel_id).embed(colour_block(
                            0xFF0000,
                            reprefixed,
                        )));
                    }
                }
            }

            for r in resp {
                r.await?;
            }
        },
        _ => {},
    }

    Ok(())
}

fn colour_block(colour: u32, content: impl Display) -> Embed {
    Embed {
        author: None,
        color: Some(colour),
        fields: vec![],
        footer: None,
        image: None,
        kind: "".into(),
        provider: None,
        thumbnail: None,
        timestamp: None,
        url: None,
        video: None,

        title: None,
        description: Some(format!("```\n{}\n```", content)),
    }
}
