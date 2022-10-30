use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn multiply(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let one = args.single::<f64>()?;
    let two = args.single::<f64>()?;

    let product = one * two;

    msg.channel_id.say(&ctx.http, product).await?;

    let m = msg.channel_id.send_message(&ctx, |m| {
        m.content("Select something!").components(|c| {
            c.create_action_row(|row| {
                row.create_select_menu(|menu| {
                    menu.custom_id("animal_select");
                    menu.placeholder("No animal selected");
                    menu.options(|f| {
                        f.create_option(|o| o.label("1").value("1"));
                        f.create_option(|o| o.label("2").value("2"));
                        f.create_option(|o| o.label("3").value("3"));
                        f.create_option(|o| o.label("4").value("4"));
                        f.create_option(|o| o.label("5").value("5"));
                        f.create_option(|o| o.label("6").value("6"));
                        f.create_option(|o| o.label("7").value("7"));
                        f.create_option(|o| o.label("8").value("8"));
                        f.create_option(|o| o.label("9").value("9"));
                        f.create_option(|o| o.label("10").value("10"));
                        f.create_option(|o| o.label("11").value("11"));
                        f.create_option(|o| o.label("12").value("12"));
                        f.create_option(|o| o.label("13").value("13"));
                        f.create_option(|o| o.label("14").value("14"));
                        f.create_option(|o| o.label("15").value("15"));
                        f.create_option(|o| o.label("16").value("16"))
                    })
                })
            })
        })
    }).await.unwrap();

    Ok(())
}
