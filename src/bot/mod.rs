use tokio::task::JoinHandle;

use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::db::DbConnection;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
        else if msg.content == "animal" {
            let m = msg.channel_id.send_message(&ctx, |m| {
                m.content("Select something!").components(|c| {
                    c.create_action_row(|row| {
                        row.create_select_menu(|menu| {
                            menu.custom_id("animal_select");
                            menu.placeholder("No animal selected");
                            menu.options(|f| {
                                f.create_option(|o| o.label("a").value("A"))
                            })
                        })
                    })
                })
            }).await.unwrap();

            let interaction = match m.await_component_interaction() {
                
            };
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub struct Bot {
    db: DbConnection,
}

impl Bot {
    pub fn new(db: DbConnection) -> Self {
        Self {
            db
        }
    }

    pub fn start(mut self) -> JoinHandle<()> {
        tokio::spawn(async move {
            let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
            
            let intents = GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::DIRECT_MESSAGES
                | GatewayIntents::MESSAGE_CONTENT;

            let mut client = 
                Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

            if let Err(why) = client.start().await {
                println!("Client error: {:?}", why);
            }
        })
    }
}