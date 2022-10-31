use std::time::Duration;

use chrono::{Local, NaiveDateTime, NaiveDate};
use serenity::builder::CreateMessage;
use serenity::collector::CollectModalInteraction;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::model::prelude::component::{InputTextStyle, ButtonStyle, InputText};
use serenity::prelude::*;
use serenity::model::application::interaction::InteractionResponseType;
use crate::bot::commands::event::component::ActionRowComponent;
use crate::macros::cast;

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

#[command]
pub async fn create(ctx: &Context, msg: &Message) -> CommandResult {
    let m = msg.channel_id.send_message(&ctx, |m| {
        m.content("Click to create an event!").components(|c| {
            c.create_action_row(|row| {
                row.create_button(|button| {
                    button.custom_id("button");
                    button.label("Create");
                    button.style(ButtonStyle::Success)
                })
            })
        })
    }).await.unwrap();

    let interaction = match m.await_component_interaction(&ctx)
        .author_id(msg.author.id)
        .timeout(Duration::from_secs(60 * 10)).await {
        Some(x) => x,
        None => {
            msg.reply(&ctx, "Event creation timed out").await.unwrap();
            m.delete(&ctx).await.unwrap();
            return Ok(());
        }
    };

    interaction.create_interaction_response(&ctx, |r| {
        r.kind(InteractionResponseType::Modal).interaction_response_data(|d| {
            d.title("Event Info");
            d.custom_id("modal");
            d.components(|c| {
                c.create_action_row(|row| {
                    row.create_input_text(|i| {
                        i.custom_id("input_text_0");
                        i.style(InputTextStyle::Short);
                        i.label("Name");
                        i.placeholder("Ex: \"Robotics Meeting\"");
                        i.required(true)
                    })
                });
                c.create_action_row(|row| {
                    row.create_input_text(|i| {
                        i.custom_id("input_text_1");
                        i.style(InputTextStyle::Paragraph);
                        i.label("Info (Not Required)");
                        i.placeholder("Ex: \"WEAR YOUR SAFETY GLASSES!!!\"");
                        i.required(false)
                    })
                });
                c.create_action_row(|row| {
                    row.create_input_text(|i| {
                        i.custom_id("input_text_2");
                        i.style(InputTextStyle::Short);
                        i.label("Date (M/D/YYYY ONLY)");
                        i.placeholder("Ex: \"12/25/2022\"");
                        i.required(true);
                        i.min_length(8);
                        i.max_length(10)
                    })
                });
                c.create_action_row(|row| {
                    row.create_input_text(|i| {
                        i.custom_id("input_text_3");
                        i.style(InputTextStyle::Short);
                        i.label("Start Time (X:XX am/pm ONLY)");
                        i.placeholder("Ex: \"9:00am\"");
                        i.required(true);
                        i.min_length(6);
                        i.max_length(7)
                    })
                });
                c.create_action_row(|row| {
                    row.create_input_text(|i| {
                        i.custom_id("input_text_4");
                        i.style(InputTextStyle::Short);
                        i.label("End Time (X:XX am/pm ONLY)");
                        i.placeholder("Ex: \"5:00pm\"");
                        i.required(true);
                        i.min_length(6);
                        i.max_length(7)
                    })
                })
            })
        })
    }).await.unwrap();

    let possible_modal_interaction = m.await_modal_interaction(&ctx)
        .author_id(msg.author.id)
        .timeout(Duration::from_secs(60 * 10)).await;
    
    if let Some(modal_interaction) = possible_modal_interaction {
        let name = cast!(&modal_interaction.data.components[0].components[0], ActionRowComponent::InputText).value.as_str();
        let info = cast!(&modal_interaction.data.components[1].components[0], ActionRowComponent::InputText).value.as_str();
        let date = cast!(&modal_interaction.data.components[2].components[0], ActionRowComponent::InputText).value.as_str();
        let start_time = cast!(&modal_interaction.data.components[3].components[0], ActionRowComponent::InputText).value.as_str();
        let end_time = cast!(&modal_interaction.data.components[4].components[0], ActionRowComponent::InputText).value.as_str();

        // make util funcs to help parse, give &str, return Result<String> of error msg, doesnt need to be async

        {
            let date_split = date.split('/').collect::<Vec<_>>();
            if date_split.len() != 3 {
                msg.reply(&ctx, "Date must have three numbers").await.unwrap();
                m.delete(&ctx).await.unwrap();
                return Ok(());
            }
            let month = date_split[0].parse::<u32>();
            if let Ok(month) = month {

            }
            else {
                msg.reply(&ctx, "Date month must be a number").await.unwrap();
                m.delete(&ctx).await.unwrap();
                return Ok(());
            }
        }
        let start_time = NaiveDate::from_ymd_opt(1, 1, 1);

        match &modal_interaction.data.components[0].components[0] {
            ActionRowComponent::InputText(text) => {
                modal_interaction.create_interaction_response(&ctx, |r| {
                    r.kind(InteractionResponseType::ChannelMessageWithSource);
                    r.interaction_response_data(|d| {
                        d.ephemeral(false);
                        d.content(format!("{}", text.value))
                    })
                }).await.unwrap();
            }
            _ => unreachable!(),
        }
    }
    else {
        msg.reply(&ctx, "Event creation timed out").await.unwrap();
        m.delete(&ctx).await.unwrap();
        return Ok(());
    }

    m.delete(&ctx).await.unwrap();

    Ok(())
}
