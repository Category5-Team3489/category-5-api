use std::time::Duration;

use chrono::NaiveDateTime;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::model::prelude::component::{InputTextStyle, ButtonStyle};
use serenity::prelude::*;
use serenity::model::application::interaction::InteractionResponseType;
use crate::bot::commands::event::component::ActionRowComponent;
use crate::bot::{parse_utils, DbConnectionContainer};
use crate::db::{Db, DbInput, data, DbOutput};
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
                        i.required(true);
                        i.max_length(100)
                    })
                });
                c.create_action_row(|row| {
                    row.create_input_text(|i| {
                        i.custom_id("input_text_1");
                        i.style(InputTextStyle::Paragraph);
                        i.label("Info (Not Required)");
                        i.placeholder("Ex: \"WEAR YOUR SAFETY GLASSES!!!\"");
                        i.required(false);
                        i.max_length(1000)
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

        let date = match parse_utils::parse_date(date) {
            Ok(date) => {
                date
            }
            Err(err) => {
                msg.reply(&ctx, err).await.unwrap();
                m.delete(&ctx).await.unwrap();
                return Ok(());
            }
        };

        let start_time = match parse_utils::parse_time(start_time) {
            Ok(start_time) => {
                start_time
            }
            Err(err) => {
                msg.reply(&ctx, err).await.unwrap();
                m.delete(&ctx).await.unwrap();
                return Ok(());
            }
        };

        let end_time = match parse_utils::parse_time(end_time) {
            Ok(end_time) => {
                end_time
            }
            Err(err) => {
                msg.reply(&ctx, err).await.unwrap();
                m.delete(&ctx).await.unwrap();
                return Ok(());
            }
        };

        if start_time > end_time {
            msg.reply(&ctx, "Start time cannot be greater than end time").await.unwrap();
            m.delete(&ctx).await.unwrap();
            return Ok(());
        }

        let start_time = NaiveDateTime::new(date, start_time);

        let end_time = NaiveDateTime::new(date, end_time);

        let event = {
            let data = ctx.data.read().await;
            let db = data.get::<DbConnectionContainer>().unwrap();
            let event = data::event::Event::new(name.to_string(), info.to_string(), start_time, end_time);
            let input = DbInput::CreateEvent(event, true);
            let output = Db::call(db, input).await;
            cast!(output, DbOutput::CreateEvent).unwrap()
        };

        modal_interaction.create_interaction_response(&ctx, |r| {
            r.kind(InteractionResponseType::ChannelMessageWithSource);
            r.interaction_response_data(|d| {
                d.content("Created event!").embed(|e| {
                    e.title(event.name);
                    e.description(event.info);
                    // https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDateTime.html#method.format_with_items
                    e.field("Date", event.start_time.format("%m/%d/%Y"), false);
                    e.field("Start time", event.start_time.format("%l:%M%p"), true);
                    e.field("End time", event.end_time.format("%l:%M%p"), true);
                    e.color(serenity::utils::Colour::DARK_GREEN)
                })
            })
        }).await.unwrap();
    }
    else {
        msg.reply(&ctx, "Event creation timed out").await.unwrap();
        m.delete(&ctx).await.unwrap();
        return Ok(());
    }

    m.delete(&ctx).await.unwrap();

    Ok(())
}
