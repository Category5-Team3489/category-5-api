use std::time::Duration;

use serenity::collector::CollectModalInteraction;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::model::prelude::component::{InputTextStyle, ButtonStyle, InputText};
use serenity::prelude::*;
use serenity::model::application::interaction::InteractionResponseType;
use crate::bot::commands::math::component::ActionRowComponent;

#[command]
pub async fn multiply(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    //let one = args.single::<f64>()?;
    //let two = args.single::<f64>()?;

    //let product = one * two;

    //msg.channel_id.say(&ctx.http, product).await?;

    let m = msg.channel_id.send_message(&ctx, |m| {
        m.content("Select something!").components(|c| {
            c.create_action_row(|row| {
                /*
                row.create_input_text(|text| {
                    text.custom_id("input_text");
                    text.style(InputTextStyle::Short);
                    text.label("Input Text");
                    text.placeholder("Input Text Placeholder")
                })
                */
                row.create_button(|button| {
                    button.custom_id("button");
                    button.label("Create");
                    button.style(ButtonStyle::Success)
                })
                /*
                row.create_button(|button| {
                    button.custom_id("2");
                    button.label("Button");
                    button.style(ButtonStyle::Secondary)
                });
                row.create_button(|button| {
                    button.custom_id("3");
                    button.label("Button");
                    button.style(ButtonStyle::Danger)
                })
                */
                /*
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
                */
            })
        })
    }).await.unwrap();

    let interaction = match m.await_component_interaction(&ctx).author_id(msg.author.id).timeout(Duration::from_secs(60 * 3)).await {
        Some(x) => x,
        None => {
            msg.reply(&ctx, "Timed out").await.unwrap();
            return Ok(());
        }
    };

    interaction.create_interaction_response(&ctx, |r| {
        r.kind(InteractionResponseType::Modal).interaction_response_data(|d| {
            d.title("e");
            d.custom_id("data");
            d.content("You selected something!").components(|c| {
                c.create_action_row(|row| {
                    row.create_input_text(|text| {
                        text.custom_id("input_text");
                        text.style(InputTextStyle::Short);
                        text.label("Input Text");
                        text.placeholder("Input Text Placeholder")
                    })
                })
            })
        })
    }).await.unwrap();

    let modal = m.await_modal_interaction(&ctx).author_id(msg.author.id).timeout(Duration::from_secs(60 * 3)).await;
    if let Some(i) = modal {
        match &i.data.components[0].components[0] {
            ActionRowComponent::InputText(text) => {
                msg.reply(&ctx, format!("{}", text.value)).await.unwrap();
                ht
            }
            _ => panic!("why did this happen??"),
        }
    }


    // other event kind, have them specify, in modal panel, drop down + other and specify

    //m.delete(&ctx).await.unwrap();

    Ok(())
}
