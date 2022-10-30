mod commands;

use tokio::task::JoinHandle;

use std::collections::HashSet;
use std::env;
use std::sync::Arc;

use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::db::DbConnection;
use crate::bot::commands::math::*;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    /*
    async fn message(&self, ctx: Context, msg: Message) {
        /*
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
                    })s
                })
            }).await.unwrap();

            let interaction = match m.await_component_interaction() {
                
            };
        }
        */
    }
    */

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
#[commands(multiply)]
struct General;

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
            self._start().await;
        })
    }

    async fn _start(mut self) {
        dotenv::dotenv().expect("Failed to load .env file");

        tracing_subscriber::fmt::init();


        let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

        let http = Http::new(&token);

        let (owners, _bot_id) = match http.get_current_application_info().await {
            Ok(info) => {
                let mut owners = HashSet::new();
                owners.insert(info.owner.id);

                (owners, info.id)
            },
            Err(why) => panic!("Could not access application info: {:?}", why),
        };

        let framework = StandardFramework::new()
            .configure(|c| c
                .owners(owners)
                .prefix("!"))
            .group(&GENERAL_GROUP);
        
        let intents = GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT;

        let mut client = Client::builder(&token, intents)
            .framework(framework)
            .event_handler(Handler)
            .await.expect("Err creating client");

        {
            let mut data = client.data.write().await;
            data.insert::<ShardManagerContainer>(client.shard_manager.clone());
            //data.insert::<DbConnection>(self.db);
        }

        let shard_manager = client.shard_manager.clone();

        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
            shard_manager.lock().await.shutdown_all().await;
        });

        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        }
    }
}