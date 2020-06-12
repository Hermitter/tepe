use clap::ArgMatches;
use std::sync::Arc;
use teloxide::{prelude::*, requests::Request, utils::markdown};
mod file_ext;
mod send;

pub struct TelegramBot {
    /// Teleoxide representation of a Telegram bot
    pub bot: Arc<Bot>,

    /// Default destination for messages
    pub chat_ids: Vec<i64>,
}

impl TelegramBot {
    /// Instantiate a Telegram bot from function arguments.
    pub fn from(command: &ArgMatches) -> TelegramBot {
        let mut chat_ids = Vec::<i64>::new();

        // chat_id from flags
        if let Some(args) = command.args.get("chat_ids") {
            args.vals.iter().for_each(|id| {
                chat_ids.push(
                    id.clone()
                        .into_string()
                        .expect(&format!("Error parsing chat_id: {:?}", id))
                        .parse::<i64>()
                        .expect(&format!("Error parsing chat_id: {:?}", id)),
                );
            });
        }

        // chat_id from environment
        if let Some(default_chat_id) = std::env::var("TEPE_TELEGRAM_CHAT_ID").ok() {
            chat_ids.push(
                default_chat_id
                    .parse::<i64>()
                    .expect("Error parsing TEPE_TELEGRAM_CHAT_ID"),
            );
        }

        // token from flag or environment variable.
        let token = std::env::var("TEPE_TELEGRAM_BOT_TOKEN").unwrap_or({
            match command.args.get("token") {
                Some(arg) => arg.vals[0]
                    .clone()
                    .into_string()
                    .expect("Could not read (--token, -t) argument"),
                None => panic!("TEPE_TELEGRAM_BOT_TOKEN has not been set"),
            }
        });

        TelegramBot {
            bot: Bot::new(token),
            chat_ids,
        }
    }

    /// A one time response to a chat with the current chat_id
    pub async fn reply_chat_id(&self) {
        println!("*********************************************************************\nYour Telegram bot is now running! Try sending a message.\nOn success, the chat_id is printed.");

        Dispatcher::new(self.bot.clone())
            .messages_handler(|rx: DispatcherHandlerRx<Message>| {
                rx.for_each(|message| async move {
                    let response = format!(
                        "{} `{}`",
                        markdown::escape("This bot is good to go! This chat_id is"),
                        &message.chat_id().to_string()
                    );

                    message
                        .answer(response)
                        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                        .send()
                        .await
                        .log_on_error()
                        .await;

                    println!("{}", format!("\nSuccessful reply from chat_id: {}\n*********************************************************************", &message.chat_id()));
                    std::process::exit(0);
                })
            })
            .dispatch()
            .await;
    }

    /// Send a text message to the default group_id.
    pub async fn send_text_message(&self, text: &str) {
        for chat_id in &self.chat_ids {
            self.bot
                .send_message(chat_id.clone(), text)
                .send()
                .await
                .log_on_error()
                .await;
        }
    }
}
