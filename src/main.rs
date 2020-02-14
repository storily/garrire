use futures::StreamExt;
use dawn::{
    gateway::shard::{Config, Event, Shard},
    http::Client as HttpClient,
};
use std::{
    env,
    error::Error,
    sync::Arc,
};

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
            println!("message: {}", msg.content);
            if msg.content == "!!!ping" {
                http.create_message(msg.channel_id).content("Pong!").await?;
                http.create_message(msg.channel_id).content("!!!ping").await?;
            }
        },
        _ => {},
    }

    Ok(())
}

