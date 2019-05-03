use crate::interaction::wait::{Action, WaitEvent};
use crate::models::bank::Bank;
use crate::schema::banks::dsl::*;
use crate::DatabaseConnection;
use crate::Waiter;
use chrono::prelude::*;
use diesel::prelude::*;
use rand::prelude::*;
use serenity::model::{channel::Message, channel::ReactionType, id::ChannelId, id::MessageId};
use serenity::utils::{content_safe, ContentSafeOptions};
use std::fmt;
use crate::BlackjackState;

command!(play(ctx, msg, args) {
    let data = ctx.data.lock();
    let conn = match data.get::<DatabaseConnection>() {
        Some(v) => v.clone(),
        None => {
            let _ = msg.channel_id.say("Datenbankfehler, bitte informiere einen Moderator!");
            return Ok(());
        }
    };
    let amount_to_bet = match args.single::<i64>() {
        Ok(v) if v > 0 => v,
        Ok(_) => {
            // log
            let _ = msg.channel_id.say("Ungültiger Wetteinsatz!");
            return Ok(());
        }
        Err(e) => {
            // log
            let _ = msg.channel_id.say("Ungültiger Wetteinsatz!");
            return Ok(());
        }
    };
    let blackjack_state = match data.get::<BlackjackState>() {
        Some(v) => v.clone(),
        None => {
            let _ = msg.reply("Could not retrieve the blackjack state!");
            return Ok(());
        }
    };

    // check if user already owns a bank & has enough balance
    let results = banks.filter(user_id.eq(*msg.author.id.as_u64() as i64)).load::<Bank>(&*conn.lock()).expect("could not retrieve banks");
    
    if !results.is_empty() && results[0].amount >= amount_to_bet {
        // remove betted amount

        // create blackjack game message and add it to blackjack state
        let blackjack_msg = msg.channel_id.send_message(|m| m.content("Starting Blackjack game...")
            .reactions(vec![
                ReactionType::Unicode("👆".to_string()),
                ReactionType::Unicode("✋".to_string()),
                ReactionType::Unicode("🌀".to_string()),
            ])
            ).expect("Failed to create blackjack message");
        blackjack_state.lock().add_game(conn.clone(), *msg.author.id.as_u64(), amount_to_bet, *blackjack_msg.channel_id.as_u64(), *blackjack_msg.id.as_u64());
    } else {
        let _ = msg.channel_id.say("Du besitzt entweder keine Bank, oder nicht genügend credits!");
    }
});