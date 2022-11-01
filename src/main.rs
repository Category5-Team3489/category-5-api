mod db;
mod bot;

mod macros;

use std::sync::Arc;

use axum::{routing::get, Extension, Router, extract::Path};
use bot::parse_utils;
use db::{DbConnection, DbInput, DbOutput, data::student::Student};
use macros::cast;
use tokio::{
    join
};

use crate::{db::{
    Db,
    DbFunction
}, bot::Bot};

// TODO todo every meeting, send out a message in the attendance channel, react to mark attendance??
// listen for reactions to attend it??? say when your attendance is recorded, message them
// 

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // let timer = Instant::now();
    // let string = format!("{}", timer.elapsed().as_secs_f64());

    let (tx, rx) = flume::unbounded::<DbFunction>();

    let db = Db::new(rx);
    let server = Router::new()
        .route("/", get(index))
        .route("/create_student/:name", get(create_student))
        .layer(Extension(Arc::new(tx.clone())));
    // Arc is used for simplicity, not needed though
    let bot = Bot::new(Arc::new(tx));

    let db_task = db.start();
    let server_task =
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(server.into_make_service());
    let bot_task = bot.start();

    let tasks = join!(db_task, server_task, bot_task);
    tasks.0.unwrap();
    tasks.1.unwrap();
    tasks.2.unwrap();
}

async fn index(db: Extension<DbConnection>) -> String {
    let input = DbInput::Clone;
    let output = Db::ext_call(db, input).await;

    let output = macros::cast!(output, DbOutput::Clone);
    serde_json::to_string(&output).unwrap()
}

async fn create_student(db: Extension<DbConnection>, Path(name): Path<String>) -> String {
    let input = DbInput::CreateStudent(Student::new(name), true);
    let output = Db::ext_call(db, input).await;

    let output = cast!(output, DbOutput::CreateStudent).unwrap();
    serde_json::to_string(&output).unwrap()
}